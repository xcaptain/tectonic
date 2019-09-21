/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew

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
#ifndef XETEX_MAC
#include "xetex-XeTeXFontMgr_FC.h"

/* allow compilation with old Fontconfig header */
#ifndef FC_FULLNAME
#define FC_FULLNAME "fullname"
#endif

#include FT_SFNT_NAMES_H
#include FT_TRUETYPE_IDS_H

#include <unicode/ucnv.h>

#define kFontFamilyName 1
#define kFontStyleName  2
#define kFontFullName   4
#define kPreferredFamilyName    16
#define kPreferredSubfamilyName 17

static UConverter* macRomanConv = NULL;
static UConverter* utf16beConv = NULL;
static UConverter* utf8Conv = NULL;

static char*
convertToUtf8(UConverter* conv, const unsigned char* name, int len)
{
    char* buffer1 = NULL;
    char* buffer2 = NULL;
    int bufSize = -1;

    if (2 * (len + 1) > bufSize) {
        if (buffer1 != NULL) {
            free(buffer1);
            free(buffer2);
        }
        bufSize = 2 * len + 100;
        buffer1 = malloc(sizeof(char) * bufSize);
        buffer2 = malloc(sizeof(char) * bufSize);
    }

    UErrorCode status = U_ZERO_ERROR;
    len = ucnv_toUChars(conv, (UChar*)buffer1, bufSize, (const char*)name, len, &status);
    len = ucnv_fromUChars(utf8Conv, buffer2, bufSize, (UChar*)buffer1, len, &status);
    buffer2[len] = 0;

    free(buffer1);
    return buffer2;
}

XeTeXFontMgrNameCollection*
XeTeXFontMgr_FC_readNames(XeTeXFontMgr* self, FcPattern* pat)
{
    XeTeXFontMgrNameCollection* names = XeTeXFontMgrNameCollection_create();

    char* pathname;
    if (FcPatternGetString(pat, FC_FILE, 0, (FcChar8**)&pathname) != FcResultMatch)
        return names;
    int index;
    if (FcPatternGetInteger(pat, FC_INDEX, 0, &index) != FcResultMatch)
        return names;

    FT_Face face;
    if (FT_New_Face(gFreeTypeLibrary, pathname, index, &face) != 0)
        return names;

    const char* name = FT_Get_Postscript_Name(face);
    if (name == NULL)
        return names;
    CppStdString_assign_from_const_char_ptr(names->m_psName, name);

    // for sfnt containers, we'll read the name table ourselves, not rely on Fontconfig
    if (FT_IS_SFNT(face)) {
        unsigned int i;
        CppStdListOfString* familyNames = CppStdListOfString_create();
        CppStdListOfString* subFamilyNames = CppStdListOfString_create();
        FT_SfntName nameRec;
        for (i = 0; i < FT_Get_Sfnt_Name_Count(face); ++i) {
            char* utf8name = NULL;
            if (FT_Get_Sfnt_Name(face, i, &nameRec) != 0)
                continue;
            switch (nameRec.name_id) {
                case kFontFullName:
                case kFontFamilyName:
                case kFontStyleName:
                case kPreferredFamilyName:
                case kPreferredSubfamilyName:
                    {
                        bool preferredName = false;
                        if (nameRec.platform_id == TT_PLATFORM_MACINTOSH
                                && nameRec.encoding_id == TT_MAC_ID_ROMAN && nameRec.language_id == 0) {
                            utf8name = convertToUtf8(macRomanConv, nameRec.string, nameRec.string_len);
                            preferredName = true;
                        }
                        else if ((nameRec.platform_id == TT_PLATFORM_APPLE_UNICODE)
                                || (nameRec.platform_id == TT_PLATFORM_MICROSOFT))
                            utf8name = convertToUtf8(utf16beConv, nameRec.string, nameRec.string_len);

                        if (utf8name != NULL) {
                            CppStdListOfString* nameList = NULL;
                            switch (nameRec.name_id) {
                                case kFontFullName:
                                    nameList = names->m_fullNames;
                                    break;
                                case kFontFamilyName:
                                    nameList = names->m_familyNames;
                                    break;
                                case kFontStyleName:
                                    nameList = names->m_styleNames;
                                    break;
                                case kPreferredFamilyName:
                                    nameList = familyNames;
                                    break;
                                case kPreferredSubfamilyName:
                                    nameList = subFamilyNames;
                                    break;
                            }
                            if (preferredName)
                                XeTeXFontMgr_prependToList(self, nameList, utf8name);
                            else
                                XeTeXFontMgr_appendToList(self, nameList, utf8name);
                            free(utf8name);
                        }
                    }
                    break;
            }
        }
        if (CppStdListOfString_size(familyNames) > 0)
            CppStdListOfString_assign(names->m_familyNames, familyNames);
        if (CppStdListOfString_size(subFamilyNames) > 0)
            CppStdListOfString_assign(names->m_styleNames,subFamilyNames);
		CppStdListOfString_delete(subFamilyNames);
		CppStdListOfString_delete(familyNames);
    } else {
        index = 0;
        while (FcPatternGetString(pat, FC_FULLNAME, index++, (FcChar8**)&name) == FcResultMatch)
            XeTeXFontMgr_appendToList(self, names->m_fullNames, name);
        index = 0;
        while (FcPatternGetString(pat, FC_FAMILY, index++, (FcChar8**)&name) == FcResultMatch)
            XeTeXFontMgr_appendToList(self, names->m_familyNames, name);
        index = 0;
        while (FcPatternGetString(pat, FC_STYLE, index++, (FcChar8**)&name) == FcResultMatch)
            XeTeXFontMgr_appendToList(self, names->m_styleNames, name);

        if (CppStdListOfString_size(names->m_fullNames) == 0) {
			CppStdString* fullName = CppStdString_create();
			CppStdString_append_const_char_ptr(fullName, CppStdListOfString_front_const_char_ptr(names->m_familyNames));
            if (CppStdListOfString_size(names->m_styleNames) > 0) {
				CppStdString_append_const_char_ptr(fullName, " ");
				CppStdString_append_const_char_ptr(fullName, CppStdListOfString_front_const_char_ptr(names->m_styleNames));
            }
			CppStdListOfString_append_copy_CppStdString(names->m_fullNames, fullName);
			CppStdString_delete(fullName);
        }
    }

    FT_Done_Face(face);

    return names;
}

void
XeTeXFontMgr_FC_getOpSizeRecAndStyleFlags(XeTeXFontMgr* self, XeTeXFontMgrFont* theFont)
{
    XeTeXFontMgr_base_getOpSizeRecAndStyleFlags(self, theFont);

    if (theFont->weight == 0 && theFont->width == 0) {
        // try to get values from FontConfig, as it apparently wasn't an sfnt
        FcPattern* pat = theFont->fontRef;
        int value;
        if (FcPatternGetInteger(pat, FC_WEIGHT, 0, &value) == FcResultMatch)
            theFont->weight = value;
        if (FcPatternGetInteger(pat, FC_WIDTH, 0, &value) == FcResultMatch)
            theFont->width = value;
        if (FcPatternGetInteger(pat, FC_SLANT, 0, &value) == FcResultMatch)
            theFont->slant = value;
    }
}

void
XeTeXFontMgr_FC_cacheFamilyMembers(XeTeXFontMgr* self, const CppStdListOfString* familyNames)
{
	XeTeXFontMgr_FC* real_self = (XeTeXFontMgr_FC*)self;
    if (CppStdListOfString_size(familyNames) == 0)
        return;
    for (int f = 0; f < real_self->allFonts->nfont; ++f) {
        FcPattern* pat = real_self->allFonts->fonts[f];
		if (CppStdMapFontRefToFontPtr_contains(self->m_platformRefToFont, pat))
			continue;
        char* s;
        for (int i = 0; FcPatternGetString(pat, FC_FAMILY, i, (FcChar8**)&s) == FcResultMatch; ++i) {
			if (!CppStdListOfString_contains_const_char_ptr(familyNames, s))
				continue;
			XeTeXFontMgrNameCollection* names = XeTeXFontMgr_readNames(self, pat);
			XeTeXFontMgr_addToMaps(self, pat, names);
			XeTeXFontMgrNameCollection_delete(names);
			break;
        }
    }
}

void
XeTeXFontMgr_FC_searchForHostPlatformFonts(XeTeXFontMgr* self, const char* name)
{
	XeTeXFontMgr_FC* real_self = (XeTeXFontMgr_FC*)self;
    if (real_self->cachedAll) // we've already loaded everything on an earlier search
        return;

	CppStdString* famName = CppStdString_create();
	char* hyph_pos = strchr(name, '-');
    int hyph;
	if (hyph_pos)
	{
		hyph = hyph_pos - name;
		CppStdString_assign_n_chars(famName, name, hyph);
	}
	else
	{
		hyph = 0;
	}

    bool found = false;
    while (1) {
        for (int f = 0; f < real_self->allFonts->nfont; ++f) {
            FcPattern* pat = real_self->allFonts->fonts[f];
			if (CppStdMapFontRefToFontPtr_contains(self->m_platformRefToFont, pat))
                continue;

            if (real_self->cachedAll) {
                // failed to find it via FC; add everything to our maps (potentially slow) as a last resort
                XeTeXFontMgrNameCollection* names = XeTeXFontMgr_readNames(self, pat);
                XeTeXFontMgr_addToMaps(self, pat, names);
                XeTeXFontMgrNameCollection_delete(names);
                continue;
            }

            char* s;
            int i;
            for (i = 0; FcPatternGetString(pat, FC_FULLNAME, i, (FcChar8**)&s) == FcResultMatch; ++i) {
                if (CppStdString_const_char_ptr_equal_const_char_ptr(name,s)) {
                    XeTeXFontMgrNameCollection* names = XeTeXFontMgr_readNames(self, pat);
                    XeTeXFontMgr_addToMaps(self, pat, names);
                    XeTeXFontMgr_cacheFamilyMembers(self, names->m_familyNames);
                    XeTeXFontMgrNameCollection_delete(names);
                    found = true;
                    goto next_font;
                }
            }

            for (i = 0; FcPatternGetString(pat, FC_FAMILY, i, (FcChar8**)&s) == FcResultMatch; ++i) {
                if (CppStdString_const_char_ptr_equal_const_char_ptr(name,s) || (hyph && CppStdString_equal_const_char_ptr(famName, s))) {
                    XeTeXFontMgrNameCollection* names = XeTeXFontMgr_readNames(self, pat);
                    XeTeXFontMgr_addToMaps(self, pat, names);
                    XeTeXFontMgr_cacheFamilyMembers(self, names->m_familyNames);
                    XeTeXFontMgrNameCollection_delete(names);
                    found = true;
                    goto next_font;
                }
                char* t;
                for (int j = 0; FcPatternGetString(pat, FC_STYLE, j, (FcChar8**)&t) == FcResultMatch; ++j) {
					CppStdString* full = CppStdString_create();
					CppStdString_append_const_char_ptr(full, s);
					CppStdString_append_const_char_ptr(full, " ");
					CppStdString_append_const_char_ptr(full, t);
					bool matched = CppStdString_equal_const_char_ptr(full, name);
					CppStdString_delete(full);
                    if (matched) {
                        XeTeXFontMgrNameCollection* names = XeTeXFontMgr_readNames(self, pat);
                        XeTeXFontMgr_addToMaps(self, pat, names);
                        XeTeXFontMgr_cacheFamilyMembers(self, names->m_familyNames);
                        XeTeXFontMgrNameCollection_delete(names);
                        found = true;
                        goto next_font;
                    }
                }
            }

        next_font:
            ;
        }

        if (found || real_self->cachedAll)
            break;
        real_self->cachedAll = true;
    }
	CppStdString_delete(famName);
}

void
XeTeXFontMgr_FC_initialize(XeTeXFontMgr* self)
{
	XeTeXFontMgr_FC* real_self = (XeTeXFontMgr_FC*)self;
    if (FcInit() == FcFalse)
        _tt_abort("fontconfig initialization failed");

    if (gFreeTypeLibrary == 0 && FT_Init_FreeType(&gFreeTypeLibrary) != 0)
        _tt_abort("FreeType initialization failed");

    UErrorCode err = U_ZERO_ERROR;
    macRomanConv = ucnv_open("macintosh", &err);
    utf16beConv = ucnv_open("UTF16BE", &err);
    utf8Conv = ucnv_open("UTF8", &err);
    if (err)
        _tt_abort("cannot read font names");

    FcPattern* pat = FcNameParse((const FcChar8*)":outline=true");
    FcObjectSet* os = FcObjectSetBuild(FC_FAMILY, FC_STYLE, FC_FILE, FC_INDEX,
                                       FC_FULLNAME, FC_WEIGHT, FC_WIDTH, FC_SLANT, FC_FONTFORMAT, NULL);
    real_self->allFonts = FcFontList(FcConfigGetCurrent(), pat, os);
    FcObjectSetDestroy(os);
    FcPatternDestroy(pat);

    real_self->cachedAll = false;
}

void
XeTeXFontMgr_FC_terminate(XeTeXFontMgr* self)
{
	XeTeXFontMgr_FC* real_self = (XeTeXFontMgr_FC*)self;
    FcFontSetDestroy(real_self->allFonts);
    real_self->allFonts = NULL;

    if (macRomanConv != NULL) {
        ucnv_close(macRomanConv);
        macRomanConv = NULL;
    }
    if (utf16beConv != NULL) {
        ucnv_close(utf16beConv);
        utf16beConv = NULL;
    }
    if (utf8Conv != NULL) {
        ucnv_close(utf8Conv);
        utf8Conv = NULL;
    }
}

char*
XeTeXFontMgr_FC_getPlatformFontDesc(const XeTeXFontMgr* self, PlatformFontRef font)
{
    FcChar8* s;
	char* path;
    if (FcPatternGetString(font, FC_FILE, 0, (FcChar8**)&s) == FcResultMatch)
        path = strdup(s);
    else
        path = strdup("[unknown]");
    return path;
}

void XeTeXFontMgr_FC_ctor(XeTeXFontMgr_FC* self) {
	XeTeXFontMgr_base_ctor(&self->super_);
	self->super_.m_memfnInitialize = XeTeXFontMgr_FC_initialize;
	self->super_.m_memfnTerminate = XeTeXFontMgr_FC_terminate;
	self->super_.m_memfnGetOpSizeRecAndStyleFlags = XeTeXFontMgr_FC_getOpSizeRecAndStyleFlags;
	self->super_.m_memfnGetPlatformFontDesc = XeTeXFontMgr_FC_getPlatformFontDesc;
	self->super_.m_memfnSearchForHostPlatformFonts = XeTeXFontMgr_FC_searchForHostPlatformFonts;
	self->super_.m_memfnReadNames = XeTeXFontMgr_FC_readNames;
}

XeTeXFontMgr_FC* XeTeXFontMgr_FC_create() {
	XeTeXFontMgr_FC* self = malloc(sizeof(XeTeXFontMgr_FC));
	XeTeXFontMgr_FC_ctor(self);
	return self;
}

#endif

