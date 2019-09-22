/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009-2014 by Jonathan Kew

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/

#include "xetex-core.h"
#include "xetex-web.h"

#ifdef XETEX_MAC
#include "xetex-XeTeXFontMgr_Mac.h"
#else
#include "xetex-XeTeXFontMgr_FC.h"
#endif
#include "xetex-XeTeXFontInst.h"

#include <harfbuzz/hb-ot.h>

// see cpascal.h
#define printcstring(STR)        \
  do {                           \
    const char* ch_ptr = (STR);  \
    while (*ch_ptr)              \
      print_char(*(ch_ptr++));    \
  } while (0)

XeTeXFontMgr* XeTeXFontMgr_sFontManager = NULL;
char XeTeXFontMgr_sReqEngine = 0;

/* use our own fmax function because it seems to be missing on certain platforms
   (solaris2.9, at least) */
static inline double
my_fmax(double x, double y)
{
    return (x > y) ? x : y;
}

XeTeXFontMgr*
XeTeXFontMgr_GetFontManager()
{
    if (XeTeXFontMgr_sFontManager == NULL) {
#ifdef XETEX_MAC
        XeTeXFontMgr_sFontManager = &XeTeXFontMgr_Mac_create()->super_;
#else
        XeTeXFontMgr_sFontManager = &XeTeXFontMgr_FC_create()->super_;
#endif
        XeTeXFontMgr_initialize(XeTeXFontMgr_sFontManager);
    }

    return XeTeXFontMgr_sFontManager;
}

void
XeTeXFontMgr_Terminate()
{
    if (XeTeXFontMgr_sFontManager != NULL) {
		XeTeXFontMgr_terminate(XeTeXFontMgr_sFontManager);
        // we don't actually deallocate the manager, just ask it to clean up
        // any auxiliary data such as the cocoa pool or freetype/fontconfig stuff
        // as we still need to access font names after this is called
    }
}

void
XeTeXFontMgr_Destroy()
{
    // Here we actually fully destroy the font manager.

    if (XeTeXFontMgr_sFontManager != NULL) {
		XeTeXFontMgr_delete(XeTeXFontMgr_sFontManager);
        XeTeXFontMgr_sFontManager = NULL;
    }
}

char XeTeXFontMgr_getReqEngine(const XeTeXFontMgr* self) {
    // return the requested rendering technology for the most recent findFont
    // or 0 if no specific technology was requested	
	
	return XeTeXFontMgr_sReqEngine;
}

void XeTeXFontMgr_setReqEngine(const XeTeXFontMgr* self, char reqEngine)
{
	XeTeXFontMgr_sReqEngine = reqEngine;
}

// above are singleton operation.
///////////////

void XeTeXFontMgr_delete(XeTeXFontMgr* self) {
	if (!self)
		return;
	if (self->m_subdtor)
		(self->m_subdtor)(self);
	CppStdMapStringToFontPtr_delete(self->m_nameToFont);
	CppStdMapStringToFamilyPtr_delete(self->m_nameToFamily);
	CppStdMapFontRefToFontPtr_delete(self->m_platformRefToFont);
	CppStdMapStringToFontPtr_delete(self->m_psNameToFont);
	free(self);
}

PlatformFontRef
XeTeXFontMgr_findFont(XeTeXFontMgr* self, const char* name, char* variant, double ptSize)
{
	// 1st arg is name as specified by user (C string, UTF-8)
        // 2nd is /B/I/AAT/OT/ICU/GR/S=## qualifiers
        // 1. try name given as "full name"
        // 2. if there's a hyphen, split and try "family-style"
        // 3. try as PostScript name
        // 4. try name as family with "Regular/Plain/Normal" style
        // apply style qualifiers and optical sizing if present

        // SIDE EFFECT: sets sReqEngine to 'A' or 'O' or 'G' if appropriate,
        //   else clears it to 0

        // SIDE EFFECT: updates TeX variables /nameoffile/ and /namelength/,
        //   to match the actual font found

        // SIDE EFFECT: edits /variant/ string in-place removing /B or /I

    // ptSize is in TeX points, or negative for 'scaled' factor
    // "variant" string will be shortened (in-place) by removal of /B and /I if present
		
	CppStdString* nameStr = CppStdString_create();
	CppStdString_assign_from_const_char_ptr(nameStr, name);
    XeTeXFontMgrFont* font = NULL;
    int dsize = 100;
    loaded_font_design_size = 655360L;

    for (int pass = 0; pass < 2; ++pass) {
        // try full name as given
        CppStdMapStringToFontPtr_Iter i = CppStdMapStringToFontPtr_find(self->m_nameToFont, nameStr);
        if (CppStdMapStringToFontPtr_Iter_neq(i,CppStdMapStringToFontPtr_end(self->m_nameToFont))) {
            font = CppStdMapStringToFontPtr_Iter_second(i);
            if (font->opSizeInfo.designSize != 0)
                dsize = font->opSizeInfo.designSize;
            break;
        }

        // if there's a hyphen, split there and try Family-Style
		const char* nameStr_cstr = CppStdString_cstr(nameStr);
		int nameStr_len = strlen(nameStr_cstr);
		const char* hyph_pos = strchr(nameStr_cstr, '-');
        int hyph = hyph_pos ? hyph_pos - nameStr_cstr : -1;
        if (hyph > 0 && hyph < (int) (nameStr_len - 1)) {
			CppStdString* family = CppStdString_create();
			CppStdString_assign_n_chars(family, nameStr_cstr, hyph);
            CppStdMapStringToFamilyPtr_Iter f = CppStdMapStringToFamilyPtr_find(self->m_nameToFamily, family);
			CppStdString_delete(family);
            if (CppStdMapStringToFamilyPtr_Iter_neq(f, CppStdMapStringToFamilyPtr_end(self->m_nameToFamily))) {
				CppStdString* style = CppStdString_create();
				CppStdString_assign_n_chars(style, nameStr_cstr + hyph + 1, nameStr_len - hyph - 1);
				CppStdString_delete(style);
                i = CppStdMapStringToFontPtr_find(CppStdMapStringToFamilyPtr_Iter_second(f)->styles, style);
                if (CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(CppStdMapStringToFamilyPtr_Iter_second(f)->styles))) {
                    font = CppStdMapStringToFontPtr_Iter_second(i);
                    if (font->opSizeInfo.designSize != 0)
                        dsize = font->opSizeInfo.designSize;
                    break;
                }
            }
        }

        // try as PostScript name
        i = CppStdMapStringToFontPtr_find(self->m_psNameToFont, nameStr);
        if (CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(self->m_psNameToFont))) {
            font = CppStdMapStringToFontPtr_Iter_second(i);
            if (font->opSizeInfo.designSize != 0)
                dsize = font->opSizeInfo.designSize;
            break;
        }

        // try for the name as a family name
        CppStdMapStringToFamilyPtr_Iter f = CppStdMapStringToFamilyPtr_find(self->m_nameToFamily, nameStr);

        if (CppStdMapStringToFamilyPtr_Iter_neq(f, CppStdMapStringToFamilyPtr_end(self->m_nameToFamily))) {
            // look for a family member with the "regular" bit set in OS/2
            int regFonts = 0;
            for (i = CppStdMapStringToFontPtr_begin(CppStdMapStringToFamilyPtr_Iter_second(f)->styles);
			CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(CppStdMapStringToFamilyPtr_Iter_second(f)->styles)); CppStdMapStringToFontPtr_Iter_inc(&i))
                if (CppStdMapStringToFontPtr_Iter_second(i)->isReg) {
                    if (regFonts == 0)
                        font = CppStdMapStringToFontPtr_Iter_second(i);
                    ++regFonts;
                }

            // families with Ornament or similar fonts may flag those as Regular,
            // which confuses the search above... so try some known names
            if (font == NULL || regFonts > 1) {
                // try for style "Regular", "Plain", "Normal", "Roman"
                i = CppStdMapStringToFontPtr_find_const_char_ptr(CppStdMapStringToFamilyPtr_Iter_second(f)->styles, "Regular");
                if (CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(CppStdMapStringToFamilyPtr_Iter_second(f)->styles)))
                    font = CppStdMapStringToFontPtr_Iter_second(i);
                else {
                    i = CppStdMapStringToFontPtr_find_const_char_ptr(CppStdMapStringToFamilyPtr_Iter_second(f)->styles, "Plain");
                    if (CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(CppStdMapStringToFamilyPtr_Iter_second(f)->styles)))
                        font = CppStdMapStringToFontPtr_Iter_second(i);
                    else {
                        i = CppStdMapStringToFontPtr_find_const_char_ptr(CppStdMapStringToFamilyPtr_Iter_second(f)->styles, "Normal");
                        if (CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(CppStdMapStringToFamilyPtr_Iter_second(f)->styles)))
                            font = CppStdMapStringToFontPtr_Iter_second(i);
                        else {
                            i = CppStdMapStringToFontPtr_find_const_char_ptr(CppStdMapStringToFamilyPtr_Iter_second(f)->styles, "Roman");
                            if (CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(CppStdMapStringToFamilyPtr_Iter_second(f)->styles)))
                                font = CppStdMapStringToFontPtr_Iter_second(i);
                        }
                    }
                }
            }

            if (font == NULL) {
                // look through the family for the (weight, width, slant) nearest to (80, 100, 0)
                font = XeTeXFontMgr_bestMatchFromFamily(self, CppStdMapStringToFamilyPtr_Iter_second(f), 80, 100, 0);
            }

            if (font != NULL)
                break;
        }

        if (pass == 0) {
            // didn't find it in our caches, so do a platform search (may be relatively expensive);
            // this will update the caches with any fonts that seem to match the name given,
            // so that the second pass might find it
            XeTeXFontMgr_searchForHostPlatformFonts(self, CppStdString_cstr(nameStr));
        }
    }
	CppStdString_delete(nameStr);
	
    if (font == NULL)
        return 0;

    XeTeXFontMgrFamily* parent = font->parent;

    // if there are variant requests, try to apply them
    // and delete B, I, and S=... codes from the string, just retain /engine option
    XeTeXFontMgr_sReqEngine = 0;
    bool reqBold = false;
    bool reqItal = false;
    if (variant != NULL) {
		CppStdString* varString = CppStdString_create();
        char* cp = variant;
        while (*cp) {
            if (strncmp(cp, "AAT", 3) == 0) {
                XeTeXFontMgr_sReqEngine = 'A';
                cp += 3;
                if (CppStdString_length(varString) > 0 && CppStdString_last(varString) != '/')
                    CppStdString_append_const_char_ptr(varString,"/");
                CppStdString_append_const_char_ptr(varString, "AAT");
                goto skip_to_slash;
            }
            if (strncmp(cp, "ICU", 3) == 0) { // for backword compatability
                XeTeXFontMgr_sReqEngine = 'O';
                cp += 3;
                if (CppStdString_length(varString) > 0 && CppStdString_last(varString) != '/')
                    CppStdString_append_const_char_ptr(varString,"/");
                CppStdString_append_const_char_ptr(varString, "OT");
                goto skip_to_slash;
            }
            if (strncmp(cp, "OT", 2) == 0) {
                XeTeXFontMgr_sReqEngine = 'O';
                cp += 2;
                if (CppStdString_length(varString) > 0 && CppStdString_last(varString) != '/')
                    CppStdString_append_const_char_ptr(varString,"/");
                CppStdString_append_const_char_ptr(varString, "OT");
                goto skip_to_slash;
            }
            if (strncmp(cp, "GR", 2) == 0) {
                XeTeXFontMgr_sReqEngine = 'G';
                cp += 2;
                if (CppStdString_length(varString) > 0 && CppStdString_last(varString) != '/')
                    CppStdString_append_const_char_ptr(varString,"/");
                CppStdString_append_const_char_ptr(varString, "GR");
                goto skip_to_slash;
            }
            if (*cp == 'S') {
                ++cp;
                if (*cp == '=')
                    ++cp;
                ptSize = 0.0;
                while (*cp >= '0' && *cp <= '9') {
                    ptSize = ptSize * 10 + *cp - '0';
                    ++cp;
                }
                if (*cp == '.') {
                    double dec = 1.0;
                    ++cp;
                    while (*cp >= '0' && *cp <= '9') {
                        dec = dec * 10.0;
                        ptSize = ptSize + (*cp - '0') / dec;
                        ++cp;
                    }
                }
                goto skip_to_slash;
            }

            /* if the code is "B" or "I", we skip putting it in varString */
            while (1) {
                if (*cp == 'B') {
                    reqBold = true;
                    ++cp;
                    continue;
                }
                if (*cp == 'I') {
                    reqItal = true;
                    ++cp;
                    continue;
                }
                break;
            }

        skip_to_slash:
            while (*cp && *cp != '/')
                ++cp;
            if (*cp == '/')
                ++cp;
        }
        strcpy(variant, CppStdString_cstr(varString));
		CppStdString_delete(varString);

        CppStdMapStringToFontPtr_Iter i;
        if (reqItal) {
            XeTeXFontMgrFont* bestMatch = font;
            if (font->slant < parent->maxSlant)
                // try for a face with more slant
                bestMatch = XeTeXFontMgr_bestMatchFromFamily(self, parent, font->weight, font->width, parent->maxSlant);

            if (bestMatch == font && font->slant > parent->minSlant)
                // maybe the slant is negated, or maybe this was something like "Times-Italic/I"
                bestMatch = XeTeXFontMgr_bestMatchFromFamily(self, parent, font->weight, font->width, parent->minSlant);

            if (parent->minWeight == parent->maxWeight && bestMatch->isBold != font->isBold) {
                // try again using the bold flag, as we can't trust weight values
                XeTeXFontMgrFont* newBest = NULL;
                for (i = CppStdMapStringToFontPtr_begin(parent->styles); CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(parent->styles)); CppStdMapStringToFontPtr_Iter_inc(&i)) {
                    if (CppStdMapStringToFontPtr_Iter_second(i)->isBold == font->isBold) {
                        if (newBest == NULL && CppStdMapStringToFontPtr_Iter_second(i)->isItalic != font->isItalic) {
                            newBest = CppStdMapStringToFontPtr_Iter_second(i);
                            break;
                        }
                    }
                }
                if (newBest != NULL)
                    bestMatch = newBest;
            }

            if (bestMatch == font) {
                // maybe slant values weren't present; try the style bits as a fallback
                bestMatch = NULL;
                for (i = CppStdMapStringToFontPtr_begin(parent->styles); 
				CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(parent->styles)); CppStdMapStringToFontPtr_Iter_inc(&i)) {
                    if (CppStdMapStringToFontPtr_Iter_second(i)->isItalic == !font->isItalic) {
                        if (parent->minWeight != parent->maxWeight) {
                            // weight info was available, so try to match that
                            if (bestMatch == NULL || XeTeXFontMgr_weightAndWidthDiff(self, CppStdMapStringToFontPtr_Iter_second(i), font) < XeTeXFontMgr_weightAndWidthDiff(self, bestMatch, font))
                                bestMatch = CppStdMapStringToFontPtr_Iter_second(i);
                        } else {
                            // no weight info, so try matching style bits
                            if (bestMatch == NULL && CppStdMapStringToFontPtr_Iter_second(i)->isBold == font->isBold) {
                                bestMatch = CppStdMapStringToFontPtr_Iter_second(i);
                                break;  // found a match, no need to look further as we can't distinguish!
                            }
                        }
                    }
                }
            }
            if (bestMatch != NULL)
                font = bestMatch;
        }

        if (reqBold) {
            // try for more boldness, with the same width and slant
            XeTeXFontMgrFont* bestMatch = font;
            if (font->weight < parent->maxWeight) {
                // try to increase weight by 1/2 x (max - min), rounding up
                bestMatch = XeTeXFontMgr_bestMatchFromFamily(self, parent,
                    font->weight + (parent->maxWeight - parent->minWeight) / 2 + 1,
                    font->width, font->slant);
                if (parent->minSlant == parent->maxSlant) {
                    // double-check the italic flag, as we can't trust slant values
                    XeTeXFontMgrFont* newBest = NULL;
                    for (i = CppStdMapStringToFontPtr_begin(parent->styles); 
					CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(parent->styles)); CppStdMapStringToFontPtr_Iter_inc(&i)) {
                        if (CppStdMapStringToFontPtr_Iter_second(i)->isItalic == font->isItalic) {
                            if (newBest == NULL || XeTeXFontMgr_weightAndWidthDiff(self, CppStdMapStringToFontPtr_Iter_second(i), bestMatch) < XeTeXFontMgr_weightAndWidthDiff(self, newBest, bestMatch))
                                newBest = CppStdMapStringToFontPtr_Iter_second(i);
                        }
                    }
                    if (newBest != NULL)
                        bestMatch = newBest;
                }
            }
            if (bestMatch == font && !font->isBold) {
                for (i = CppStdMapStringToFontPtr_begin(parent->styles);
				CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(parent->styles)); CppStdMapStringToFontPtr_Iter_inc(&i)) {
                    if (CppStdMapStringToFontPtr_Iter_second(i)->isItalic == font->isItalic && CppStdMapStringToFontPtr_Iter_second(i)->isBold) {
                        bestMatch = CppStdMapStringToFontPtr_Iter_second(i);
                        break;
                    }
                }
            }
            font = bestMatch;
        }
    }

    // if there's optical size info, try to apply it
    if (ptSize < 0.0)
        ptSize = dsize / 10.0;
    if (font != NULL && font->opSizeInfo.subFamilyID != 0 && ptSize > 0.0) {
        ptSize = ptSize * 10.0; // convert to decipoints for comparison with the opSize values
        double bestMismatch = my_fmax(font->opSizeInfo.minSize - ptSize, ptSize - font->opSizeInfo.maxSize);
        if (bestMismatch > 0.0) {
            XeTeXFontMgrFont* bestMatch = font;
            for (CppStdMapStringToFontPtr_Iter i = CppStdMapStringToFontPtr_begin(parent->styles);
			    CppStdMapStringToFontPtr_Iter_neq(i, CppStdMapStringToFontPtr_end(parent->styles)); CppStdMapStringToFontPtr_Iter_inc(&i)) {
                if (CppStdMapStringToFontPtr_Iter_second(i)->opSizeInfo.subFamilyID != font->opSizeInfo.subFamilyID)
                    continue;
                double mismatch = my_fmax(CppStdMapStringToFontPtr_Iter_second(i)->opSizeInfo.minSize - ptSize, ptSize - CppStdMapStringToFontPtr_Iter_second(i)->opSizeInfo.maxSize);
                if (mismatch < bestMismatch) {
                    bestMatch = CppStdMapStringToFontPtr_Iter_second(i);
                    bestMismatch = mismatch;
                }
                if (bestMismatch <= 0.0)
                    break;
            }
            font = bestMatch;
        }
    }

    if (font != NULL && font->opSizeInfo.designSize != 0)
        loaded_font_design_size = (font->opSizeInfo.designSize << 16L) / 10;

    if (get_tracing_fonts_state() > 0) {
        begin_diagnostic();
        print_nl(' ');
        printcstring("-> ");
		char* font_desc = XeTeXFontMgr_getPlatformFontDesc(self, font->fontRef);
        printcstring(font_desc);
		free(font_desc);
        end_diagnostic(0);
    }

    return font->fontRef;
}

const char*
XeTeXFontMgr_getFullName(const XeTeXFontMgr* self, PlatformFontRef font)
{
// return the full name of the font, suitable for use in XeTeX source
        // without requiring style qualifiers	
    CppStdMapFontRefToFontPtr_Iter i = CppStdMapFontRefToFontPtr_find(self->m_platformRefToFont, font);
    if (CppStdMapFontRefToFontPtr_Iter_eq(i, CppStdMapFontRefToFontPtr_end(self->m_platformRefToFont)))
        _tt_abort("internal error %d in XeTeXFontMgr", 2);
    if (CppStdMapFontRefToFontPtr_Iter_second(i)->m_fullName != NULL)
        return CppStdString_cstr(CppStdMapFontRefToFontPtr_Iter_second(i)->m_fullName);
    else
        return CppStdString_cstr(CppStdMapFontRefToFontPtr_Iter_second(i)->m_psName);
}

int
XeTeXFontMgr_weightAndWidthDiff(const XeTeXFontMgr* self, const XeTeXFontMgrFont* a, const XeTeXFontMgrFont* b)
{
    if (a->weight == 0 && a->width == 0) {
        // assume there was no OS/2 info
        if (a->isBold == b->isBold)
            return 0;
        else
            return 10000;
    }

    int widDiff = labs(a->width - b->width);
    if (widDiff < 10)
        widDiff *= 50;

    return labs(a->weight - b->weight) + widDiff;
}

int
XeTeXFontMgr_styleDiff(const XeTeXFontMgr* self, const XeTeXFontMgrFont* a, int wt, int wd, int slant) 
{
    int widDiff = labs(a->width - wd);
    if (widDiff < 10)
        widDiff *= 200;

    return labs(labs(a->slant) - labs(slant)) * 2 + labs(a->weight - wt) + widDiff;
}

XeTeXFontMgrFont*
XeTeXFontMgr_bestMatchFromFamily(const XeTeXFontMgr* self, const XeTeXFontMgrFamily* fam, int wt, int wd, int slant) 
{
    XeTeXFontMgrFont* bestMatch = NULL;
    for (CppStdMapStringToFontPtr_Iter s = CppStdMapStringToFontPtr_begin(fam->styles); 
		CppStdMapStringToFontPtr_Iter_neq(s, CppStdMapStringToFontPtr_end(fam->styles)); 
		CppStdMapStringToFontPtr_Iter_inc(&s))
        if (bestMatch == NULL || XeTeXFontMgr_styleDiff(self, CppStdMapStringToFontPtr_Iter_second(s), wt, wd, slant) < XeTeXFontMgr_styleDiff(self, bestMatch, wt, wd, slant))
            bestMatch = CppStdMapStringToFontPtr_Iter_second(s);
    return bestMatch;
}


XeTeXFontMgrOpSizeRec*
XeTeXFontMgr_getOpSize(XeTeXFontMgr* self, XeTeXFont font)
{
    hb_font_t *hbFont = XeTeXFontInst_getHbFont((XeTeXFontInst *) font);

    if (hbFont == NULL)
        return NULL;

    hb_face_t *face = hb_font_get_face(hbFont);
    XeTeXFontMgrOpSizeRec *pSizeRec = (XeTeXFontMgrOpSizeRec*) xmalloc(sizeof(XeTeXFontMgrOpSizeRec));

    bool ok = hb_ot_layout_get_size_params(face,
                                           &pSizeRec->designSize,
                                           &pSizeRec->subFamilyID,
                                           &pSizeRec->nameCode,
                                           &pSizeRec->minSize,
                                           &pSizeRec->maxSize);

    if (ok)
        return pSizeRec;

    free(pSizeRec);
    return NULL;
}


double
XeTeXFontMgr_getDesignSize(XeTeXFontMgr* self, XeTeXFont font)
{
    XeTeXFontMgrOpSizeRec* pSizeRec = XeTeXFontMgr_getOpSize(self, font);

    if (pSizeRec == NULL)
        return 10.0;

    double result = pSizeRec->designSize / 10.0;
    free(pSizeRec);
    return result;
}


void
XeTeXFontMgr_base_getOpSizeRecAndStyleFlags(XeTeXFontMgr* self, XeTeXFontMgrFont* theFont)
{
    XeTeXFont font = createFont(theFont->fontRef, 655360);
    XeTeXFontInst* fontInst = (XeTeXFontInst*) font;
    if (font != 0) {
        XeTeXFontMgrOpSizeRec* pSizeRec = XeTeXFontMgr_getOpSize(self, font);

        if (pSizeRec != NULL) {
            theFont->opSizeInfo.designSize = pSizeRec->designSize;
            if (pSizeRec->subFamilyID == 0
                && pSizeRec->nameCode == 0
                && pSizeRec->minSize == 0
                && pSizeRec->maxSize == 0) {
                free(pSizeRec);
                goto done_size; // feature is valid, but no 'size' range
            }

            theFont->opSizeInfo.subFamilyID = pSizeRec->subFamilyID;
            theFont->opSizeInfo.nameCode = pSizeRec->nameCode;
            theFont->opSizeInfo.minSize = pSizeRec->minSize;
            theFont->opSizeInfo.maxSize = pSizeRec->maxSize;
            free(pSizeRec);
        }

    done_size:
		;
        const TT_OS2* os2Table = (TT_OS2*) XeTeXFontInst_getFontTableFT(fontInst, ft_sfnt_os2);
        if (os2Table != NULL) {
            theFont->weight = os2Table->usWeightClass;
            theFont->width = os2Table->usWidthClass;
            uint16_t sel = os2Table->fsSelection;
            theFont->isReg = (sel & (1 << 6)) != 0;
            theFont->isBold = (sel & (1 << 5)) != 0;
            theFont->isItalic = (sel & (1 << 0)) != 0;
        }

        const TT_Header* headTable = (TT_Header*)XeTeXFontInst_getFontTableFT(fontInst, ft_sfnt_head);
        if (headTable != NULL) {
            uint16_t ms = headTable->Mac_Style;
            if ((ms & (1 << 0)) != 0)
                theFont->isBold = true;
            if ((ms & (1 << 1)) != 0)
                theFont->isItalic = true;
        }

        const TT_Postscript* postTable = (const TT_Postscript*)XeTeXFontInst_getFontTableFT(fontInst, ft_sfnt_post);
        if (postTable != NULL) {
            theFont->slant = (int)(1000 * (tan(Fix2D(-postTable->italicAngle) * M_PI / 180.0)));
        }
        deleteFont(font);
    }
}

// append a name but only if it's not already in the list
void
XeTeXFontMgr_appendToList(XeTeXFontMgr* self, CppStdListOfString* list, const char* str)
{
    for (CppStdListOfString_Iter i = CppStdListOfString_begin(list); 
		CppStdListOfString_Iter_neq(i, CppStdListOfString_end(list)); CppStdListOfString_Iter_inc(&i))
        if (CppStdString_equal_const_char_ptr(CppStdListOfString_Iter_deref(i),str))
            return;
	CppStdListOfString_append_copy_const_char_ptr(list, str);
}

// prepend a name, removing it from later in the list if present
void
XeTeXFontMgr_prependToList(XeTeXFontMgr* self, CppStdListOfString* list, const char* str)
{
    for (CppStdListOfString_Iter i = CppStdListOfString_begin(list); 
		CppStdListOfString_Iter_neq(i, CppStdListOfString_end(list)); CppStdListOfString_Iter_inc(&i))
        if (CppStdString_equal_const_char_ptr(CppStdListOfString_Iter_deref(i),str)) {
			CppStdListOfString_erase(list, i);
            break;
        }
	CppStdListOfString_prepend_copy_const_char_ptr(list, str);
}

void
XeTeXFontMgr_addToMaps(XeTeXFontMgr* self, PlatformFontRef platformFont, const XeTeXFontMgrNameCollection* names)
{
    if (CppStdMapFontRefToFontPtr_Iter_neq(CppStdMapFontRefToFontPtr_find(self->m_platformRefToFont, platformFont), CppStdMapFontRefToFontPtr_end(self->m_platformRefToFont)))
        return; // this font has already been cached

    if (CppStdString_length(names->m_psName) == 0)
        return; // can't use a font that lacks a PostScript name

    if (
	CppStdMapStringToFontPtr_Iter_neq(CppStdMapStringToFontPtr_find(self->m_psNameToFont, names->m_psName), CppStdMapStringToFontPtr_end(self->m_psNameToFont)))
        return; // duplicates an earlier PS name, so skip

    XeTeXFontMgrFont* thisFont = XeTeXFontMgrFont_create(platformFont);
    thisFont->m_psName = CppStdString_clone(names->m_psName);
    XeTeXFontMgr_getOpSizeRecAndStyleFlags(self, thisFont);

    CppStdMapStringToFontPtr_put(self->m_psNameToFont, names->m_psName, thisFont);
    CppStdMapFontRefToFontPtr_put(self->m_platformRefToFont, platformFont, thisFont);

    if (CppStdListOfString_size(names->m_fullNames) > 0)
        thisFont->m_fullName = CppStdString_clone_from_iter(CppStdListOfString_begin(names->m_fullNames));

    if (CppStdListOfString_size(names->m_familyNames) > 0)
        thisFont->m_familyName = CppStdString_clone_from_iter(CppStdListOfString_begin(names->m_familyNames));
    else
        thisFont->m_familyName = CppStdString_clone(names->m_psName);

    if (CppStdListOfString_size(names->m_styleNames) > 0)
        thisFont->m_styleName = CppStdString_clone_from_iter(CppStdListOfString_begin(names->m_styleNames));
    else
        thisFont->m_styleName = CppStdString_create();

    CppStdListOfString_Iter i;
    for (i = CppStdListOfString_begin(names->m_familyNames); CppStdListOfString_Iter_neq(i, CppStdListOfString_end(names->m_familyNames)); CppStdListOfString_Iter_inc(&i)) {
        CppStdMapStringToFamilyPtr_Iter iFam = CppStdMapStringToFamilyPtr_find(self->m_nameToFamily, CppStdListOfString_Iter_deref(i));
        XeTeXFontMgrFamily* family;
        if (CppStdMapStringToFamilyPtr_Iter_eq(iFam, CppStdMapStringToFamilyPtr_end(self->m_nameToFamily))) {
            family = XeTeXFontMgrFamily_create();
            CppStdMapStringToFamilyPtr_put(self->m_nameToFamily, CppStdListOfString_Iter_deref(i), family);
            family->minWeight = thisFont->weight;
            family->maxWeight = thisFont->weight;
            family->minWidth = thisFont->width;
            family->maxWidth = thisFont->width;
            family->minSlant = thisFont->slant;
            family->maxSlant = thisFont->slant;
        } else {
            family = CppStdMapStringToFamilyPtr_Iter_second(iFam);
            if (thisFont->weight < family->minWeight)
                family->minWeight = thisFont->weight;
            if (thisFont->weight > family->maxWeight)
                family->maxWeight = thisFont->weight;
            if (thisFont->width < family->minWidth)
                family->minWidth = thisFont->width;
            if (thisFont->width > family->maxWidth)
                family->maxWidth = thisFont->width;
            if (thisFont->slant < family->minSlant)
                family->minSlant = thisFont->slant;
            if (thisFont->slant > family->maxSlant)
                family->maxSlant = thisFont->slant;
        }

        if (thisFont->parent == NULL)
            thisFont->parent = family;

        // ensure all style names in the family point to thisFont
        for (CppStdListOfString_Iter j = CppStdListOfString_begin(names->m_styleNames); CppStdListOfString_Iter_neq(j, CppStdListOfString_end(names->m_styleNames)); 
			CppStdListOfString_Iter_inc(&j)) {
            CppStdMapStringToFontPtr_Iter iFont = CppStdMapStringToFontPtr_find(family->styles, CppStdListOfString_Iter_deref(j));
            if (CppStdMapStringToFontPtr_Iter_eq(iFont, CppStdMapStringToFontPtr_end(family->styles)))
                CppStdMapStringToFontPtr_put(family->styles, CppStdListOfString_Iter_deref(j), thisFont);
/*
            else if (iFont->second != thisFont)
                fprintf(stderr, "# Font name warning: ambiguous Style \"%s\" in Family \"%s\" (PSNames \"%s\" and \"%s\")\n",
                            j->c_str(), i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
*/
        }
    }

    for (i = CppStdListOfString_begin(names->m_fullNames); CppStdListOfString_Iter_neq(i, CppStdListOfString_end(names->m_fullNames)); CppStdListOfString_Iter_inc(&i)) {
        CppStdMapStringToFontPtr_Iter iFont = CppStdMapStringToFontPtr_find(self->m_nameToFont, CppStdListOfString_Iter_deref(i));
        if (CppStdMapStringToFontPtr_Iter_eq(iFont, CppStdMapStringToFontPtr_end(self->m_nameToFont)))
            CppStdMapStringToFontPtr_put(self->m_nameToFont, CppStdListOfString_Iter_deref(i), thisFont);
/*
        else if (iFont->second != thisFont)
            fprintf(stderr, "# Font name warning: ambiguous FullName \"%s\" (PSNames \"%s\" and \"%s\")\n",
                        i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
*/
    }
}

void
XeTeXFontMgr_base_terminate(XeTeXFontMgr* self)
{
}

void XeTeXFontMgr_base_ctor(XeTeXFontMgr* self) {
	self->m_subdtor = NULL;
	self->m_memfnInitialize = NULL; /*abstract*/
	self->m_memfnTerminate = XeTeXFontMgr_base_terminate;
	self->m_memfnGetPlatformFontDesc = NULL; /*abstract*/
	self->m_memfnGetOpSizeRecAndStyleFlags = XeTeXFontMgr_base_getOpSizeRecAndStyleFlags;
	self->m_memfnSearchForHostPlatformFonts = NULL; /*abstract*/
	self->m_memfnReadNames = NULL; /*abstract*/
	self->m_nameToFont = CppStdMapStringToFontPtr_create();
	self->m_nameToFamily = CppStdMapStringToFamilyPtr_create();
	self->m_platformRefToFont = CppStdMapFontRefToFontPtr_create();
	self->m_psNameToFont = CppStdMapStringToFontPtr_create();	
}
