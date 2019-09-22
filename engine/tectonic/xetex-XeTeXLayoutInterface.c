/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009-2012 by Jonathan Kew
 Copyright (c) 2012-2015 by Khaled Hosny

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

#include <unicode/platform.h>   // We need this first
#include <unicode/ubidi.h>
#include <unicode/utext.h>

#include <graphite2/Font.h>
#include <graphite2/Segment.h>
#include <harfbuzz/hb-graphite2.h>
#include <harfbuzz/hb-icu.h>
#include <harfbuzz/hb-ot.h>

#include "xetex-web.h"

#include "xetex-XeTeXLayoutInterface.h"
#include "xetex-XeTeXFontInst.h"
#ifdef XETEX_MAC
#include "xetex-XeTeXFontInst_Mac.h"
#endif
#include "xetex-XeTeXFontMgr.h"

struct XeTeXLayoutEngine_rec
{
    XeTeXFontInst*  font;
    PlatformFontRef fontRef;
    hb_tag_t        script;
    hb_language_t   language;
    hb_feature_t*   features;
    char**          ShaperList; // the requested shapers
    char*           shaper;     // the actually used shaper
    int             nFeatures;
    uint32_t        rgbValue;
    float           extend;
    float           slant;
    float           embolden;
    hb_buffer_t*    hbBuffer;
};

typedef struct XeTeXLayoutEngine_rec XeTeXLayoutEngine_rec;

/*******************************************************************/
/* Glyph bounding box cache to speed up \XeTeXuseglyphmetrics mode */
/*******************************************************************/

// key is combined value representing (font_id << 16) + glyph
// value is glyph bounding box in TeX points

struct CppStdMapU32ToGlyphBBox;
typedef struct CppStdMapU32ToGlyphBBox CppStdMapU32ToGlyphBBox;
CppStdMapU32ToGlyphBBox* CppStdMapU32ToGlyphBBox_create();

struct CppStdMapU32ToGlyphBBox_Iter {
	void* unused;
};
typedef struct CppStdMapU32ToGlyphBBox_Iter CppStdMapU32ToGlyphBBox_Iter;
CppStdMapU32ToGlyphBBox_Iter CppStdMapU32ToGlyphBBox_find(CppStdMapU32ToGlyphBBox* self, uint32_t key);
CppStdMapU32ToGlyphBBox_Iter CppStdMapU32ToGlyphBBox_end(CppStdMapU32ToGlyphBBox* self);
bool CppStdMapU32ToGlyphBBox_Iter_eq(CppStdMapU32ToGlyphBBox_Iter lhs, CppStdMapU32ToGlyphBBox_Iter rhs);
GlyphBBox CppStdMapU32ToGlyphBBox_Iter_second(CppStdMapU32ToGlyphBBox_Iter self);
void CppStdMapU32ToGlyphBBox_put(CppStdMapU32ToGlyphBBox* self, uint32_t key, GlyphBBox val);

CppStdMapU32ToGlyphBBox* getGlyphBBoxCache() {
	static CppStdMapU32ToGlyphBBox* cache = NULL;
	if (!cache)
		cache = CppStdMapU32ToGlyphBBox_create();
	return cache;
}

int
getCachedGlyphBBox(uint16_t fontID, uint16_t glyphID, GlyphBBox* bbox)
{
	CppStdMapU32ToGlyphBBox* sGlyphBoxes = getGlyphBBoxCache();
    uint32_t key = ((uint32_t)fontID << 16) + glyphID;
    CppStdMapU32ToGlyphBBox_Iter i = CppStdMapU32ToGlyphBBox_find(sGlyphBoxes, key);
    if (CppStdMapU32ToGlyphBBox_Iter_eq(i, CppStdMapU32ToGlyphBBox_end(sGlyphBoxes))) {
        return 0;
    }
    *bbox = CppStdMapU32ToGlyphBBox_Iter_second(i);
    return 1;
}

void
cacheGlyphBBox(uint16_t fontID, uint16_t glyphID, const GlyphBBox* bbox)
{
	CppStdMapU32ToGlyphBBox* sGlyphBoxes = getGlyphBBoxCache();
    uint32_t key = ((uint32_t)fontID << 16) + glyphID;
	CppStdMapU32ToGlyphBBox_put(sGlyphBoxes, key, *bbox);
}

/* The following code used to be in a file called "hz.cpp" and there's no
 * particular reason for it to be here, but it was a tiny file with a weird
 * name so I wanted to get rid of it. The functions are invoked from the C
 * code. */

struct GlyphId {
	int fontNum;
	unsigned int code;
};

typedef struct GlyphId GlyphId;

inline GlyphId GlyphId_create(int fontNum, unsigned int code)
{
	GlyphId id;
	id.fontNum = fontNum;
	id.code = code;
	return id;
}

struct CppStdMapGlyphIdToInt;
typedef struct CppStdMapGlyphIdToInt CppStdMapGlyphIdToInt;
CppStdMapGlyphIdToInt* CppStdMapGlyphIdToInt_create();

struct CppStdMapGlyphIdToInt_Iter {
	void* unused;
};
typedef struct CppStdMapGlyphIdToInt_Iter CppStdMapGlyphIdToInt_Iter;
CppStdMapGlyphIdToInt_Iter CppStdMapGlyphIdToInt_find(CppStdMapGlyphIdToInt* self, GlyphId id);
CppStdMapGlyphIdToInt_Iter CppStdMapGlyphIdToInt_end(CppStdMapGlyphIdToInt* self);
bool CppStdMapGlyphIdToInt_Iter_eq(CppStdMapGlyphIdToInt_Iter lhs, CppStdMapGlyphIdToInt_Iter rhs);
int CppStdMapGlyphIdToInt_Iter_second(CppStdMapGlyphIdToInt_Iter self);
void CppStdMapGlyphIdToInt_put(CppStdMapGlyphIdToInt* self, GlyphId key, int val);

typedef CppStdMapGlyphIdToInt ProtrusionFactor;

ProtrusionFactor* getProtrusionFactor(int side) {
	static ProtrusionFactor* leftProt = NULL;
	static ProtrusionFactor* rightProt = NULL;
    ProtrusionFactor *container = NULL;
    switch (side) {
    case LEFT_SIDE:
		if (!leftProt)
			leftProt = CppStdMapGlyphIdToInt_create();
        container = leftProt;
        break;
    case RIGHT_SIDE:
		if (!rightProt)
			rightProt = CppStdMapGlyphIdToInt_create();
        container = rightProt;
        break;
    default:
        assert(0); // we should not reach here
    }
	return container;
}

void
set_cp_code(int fontNum, unsigned int code, int side, int value)
{
    GlyphId id = GlyphId_create(fontNum, code);
    ProtrusionFactor *container = getProtrusionFactor(side);

	CppStdMapGlyphIdToInt_put(container, id, value);
}


int
get_cp_code(int fontNum, unsigned int code, int side)
{
    GlyphId id = GlyphId_create(fontNum, code);
    ProtrusionFactor *container = getProtrusionFactor(side);

    CppStdMapGlyphIdToInt_Iter it = CppStdMapGlyphIdToInt_find(container, id);
    if (CppStdMapGlyphIdToInt_Iter_eq(it, CppStdMapGlyphIdToInt_end(container)))
        return 0;

    return CppStdMapGlyphIdToInt_Iter_second(it);
}

/*******************************************************************/

void
terminate_font_manager()
{
    XeTeXFontMgr_Terminate();
}

void
destroy_font_manager()
{
    XeTeXFontMgr_Destroy();
}

XeTeXFont
createFont(PlatformFontRef fontRef, Fixed pointSize)
{
    int status = 0;
#ifdef XETEX_MAC
    XeTeXFontInst* font = &XeTeXFontInst_Mac_create(fontRef, Fix2D(pointSize), &status)->super_;
#else
    FcChar8* pathname = 0;
    FcPatternGetString(fontRef, FC_FILE, 0, &pathname);
    int index;
    FcPatternGetInteger(fontRef, FC_INDEX, 0, &index);
    XeTeXFontInst* font = XeTeXFontInst_create((const char*)pathname, index, Fix2D(pointSize), &status);
#endif
    if (status != 0) {
        XeTeXFontInst_delete(font);
        return NULL;
    }
    return (XeTeXFont)font;
}

XeTeXFont
createFontFromFile(const char* filename, int index, Fixed pointSize)
{
    int status = 0;
    XeTeXFontInst* font = XeTeXFontInst_create(filename, index, Fix2D(pointSize), &status);
    if (status != 0) {
        XeTeXFontInst_delete(font);
        return NULL;
    }
    return (XeTeXFont)font;
}

void
setFontLayoutDir(XeTeXFont font, int vertical)
{
    XeTeXFontInst_setLayoutDirVertical((XeTeXFontInst*)font, vertical != 0);
}

PlatformFontRef
findFontByName(const char* name, char* var, double size)
{
    return XeTeXFontMgr_findFont(XeTeXFontMgr_GetFontManager(), name, var, size);
}

char
getReqEngine()
{
    return XeTeXFontMgr_getReqEngine(XeTeXFontMgr_GetFontManager());
}

void
setReqEngine(char reqEngine)
{
    XeTeXFontMgr_setReqEngine(XeTeXFontMgr_GetFontManager(), reqEngine);
}

const char*
getFullName(PlatformFontRef fontRef)
{
    return XeTeXFontMgr_getFullName(XeTeXFontMgr_GetFontManager(), fontRef);
}

double
getDesignSize(XeTeXFont font)
{
    return XeTeXFontMgr_getDesignSize(XeTeXFontMgr_GetFontManager(), font);
}

char*
getFontFilename(XeTeXLayoutEngine engine, uint32_t* index)
{
    return xstrdup(XeTeXFontInst_getFilename(engine->font, index));
}

PlatformFontRef
getFontRef(XeTeXLayoutEngine engine)
{
    return engine->fontRef;
}

void
deleteFont(XeTeXFont font)
{
    XeTeXFontInst_delete((XeTeXFontInst*)font);
}

void*
getFontTablePtr(XeTeXFont font, uint32_t tableTag)
{
    return (void*)XeTeXFontInst_getFontTable(((XeTeXFontInst*)font),tableTag);
}

Fixed
getSlant(XeTeXFont font)
{
    float italAngle = XeTeXFontInst_getItalicAngle((XeTeXFontInst*)font);
    return D2Fix(tan(-italAngle * M_PI / 180.0));
}

static unsigned int
getLargerScriptListTable(XeTeXFont font, hb_tag_t** scriptList)
{
    unsigned int rval = 0;

    hb_face_t* face = hb_font_get_face(XeTeXFontInst_getHbFont((XeTeXFontInst*)font));

    hb_tag_t* scriptListSub = NULL;
    hb_tag_t* scriptListPos = NULL;

    unsigned int scriptCountSub = hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GSUB, 0, NULL, NULL);
    scriptListSub = (hb_tag_t*) xcalloc(scriptCountSub, sizeof(hb_tag_t*));
    hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GSUB, 0, &scriptCountSub, scriptListSub);

    unsigned int scriptCountPos = hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GPOS, 0, NULL, NULL);
    scriptListPos = (hb_tag_t*) xcalloc(scriptCountPos, sizeof(hb_tag_t*));
    hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GSUB, 0, &scriptCountPos, scriptListPos);

    if (scriptCountSub > scriptCountPos) {
        if (scriptList != NULL)
            *scriptList = scriptListSub;
        rval = scriptCountSub;
    } else {
        if (scriptList != NULL)
            *scriptList = scriptListPos;
        rval = scriptCountPos;
    }

    return rval;
}

unsigned int
countScripts(XeTeXFont font)
{
    return getLargerScriptListTable(font, NULL);
}

hb_tag_t
getIndScript(XeTeXFont font, unsigned int index)
{
    hb_tag_t rval = 0;

    hb_tag_t* scriptList;

    unsigned int scriptCount = getLargerScriptListTable(font, &scriptList);
    if (scriptList != NULL) {
        if (index < scriptCount)
            rval = scriptList[index];
    }

    return rval;
}

unsigned int
countLanguages(XeTeXFont font, hb_tag_t script)
{
    unsigned int rval = 0;

    hb_face_t* face = hb_font_get_face(XeTeXFontInst_getHbFont((XeTeXFontInst*)font));
    hb_tag_t* scriptList;

    unsigned int scriptCount = getLargerScriptListTable(font, &scriptList);
    if (scriptList != NULL) {
        for (unsigned int i = 0; i < scriptCount; i++) {
            if (scriptList[i] == script) {
                rval += hb_ot_layout_script_get_language_tags (face, HB_OT_TAG_GSUB, i, 0, NULL, NULL);
                rval += hb_ot_layout_script_get_language_tags (face, HB_OT_TAG_GPOS, i, 0, NULL, NULL);
                break;
            }
        }
    }

    return rval;
}

hb_tag_t
getIndLanguage(XeTeXFont font, hb_tag_t script, unsigned int index)
{
    hb_tag_t rval = 0;

    hb_face_t* face = hb_font_get_face(XeTeXFontInst_getHbFont((XeTeXFontInst*)font));
    hb_tag_t* scriptList;

    unsigned int scriptCount = getLargerScriptListTable(font, &scriptList);
    if (scriptList != NULL) {
        for (unsigned int i = 0; i < scriptCount; i++) {
            if (scriptList[i] == script) {
                unsigned int langCount;
                hb_tag_t* langList;

                langCount = hb_ot_layout_script_get_language_tags(face, HB_OT_TAG_GSUB, i, 0, NULL, NULL);
                langList = (hb_tag_t*) xcalloc(langCount, sizeof(hb_tag_t*));
                hb_ot_layout_script_get_language_tags(face, HB_OT_TAG_GSUB, i, 0, &langCount, langList);

                if (index < langCount) {
                    rval = langList[index];
                    break;
                }

                free(langList);

                langCount = hb_ot_layout_script_get_language_tags(face, HB_OT_TAG_GPOS, i, 0, NULL, NULL);
                langList = (hb_tag_t*) xcalloc(langCount, sizeof(hb_tag_t*));
                hb_ot_layout_script_get_language_tags(face, HB_OT_TAG_GPOS, i, 0, &langCount, langList);

                if (index < langCount) {
                    rval = langList[index];
                    break;
                }

                free(langList);
            }
        }
    }

    return rval;
}

unsigned int
countFeatures(XeTeXFont font, hb_tag_t script, hb_tag_t language)
{
    unsigned int rval = 0;

    hb_face_t* face = hb_font_get_face(XeTeXFontInst_getHbFont((XeTeXFontInst*)font));

    for (int i = 0; i < 2; ++i) {
        unsigned int scriptIndex, langIndex = 0;
        hb_tag_t tableTag = i == 0 ? HB_OT_TAG_GSUB : HB_OT_TAG_GPOS;
        if (hb_ot_layout_table_find_script(face, tableTag, script, &scriptIndex)) {
            if (hb_ot_layout_script_find_language(face, tableTag, scriptIndex, language, &langIndex) || language == 0) {
                rval += hb_ot_layout_language_get_feature_tags(face, tableTag, scriptIndex, langIndex, 0, NULL, NULL);
            }
        }
    }

    return rval;
}

hb_tag_t
getIndFeature(XeTeXFont font, hb_tag_t script, hb_tag_t language, unsigned int index)
{
    hb_tag_t rval = 0;

    hb_face_t* face = hb_font_get_face(XeTeXFontInst_getHbFont((XeTeXFontInst*)font));

    for (int i = 0; i < 2; ++i) {
        unsigned int scriptIndex, langIndex = 0;
        hb_tag_t tableTag = i == 0 ? HB_OT_TAG_GSUB : HB_OT_TAG_GPOS;
        if (hb_ot_layout_table_find_script(face, tableTag, script, &scriptIndex)) {
            if (hb_ot_layout_script_find_language(face, tableTag, scriptIndex, language, &langIndex) || language == 0) {
                unsigned int featCount = hb_ot_layout_language_get_feature_tags(face, tableTag, scriptIndex, langIndex, 0, NULL, NULL);
                hb_tag_t* featList = (hb_tag_t*) xcalloc(featCount, sizeof(hb_tag_t*));
                hb_ot_layout_language_get_feature_tags(face, tableTag, scriptIndex, langIndex, 0, &featCount, featList);

                if (index < featCount) {
                    rval = featList[index];
                    break;
                }

                index -= featCount;
            }
        }
    }

    return rval;
}

uint32_t
countGraphiteFeatures(XeTeXLayoutEngine engine)
{
    uint32_t rval = 0;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL)
        rval = gr_face_n_fref(grFace);

    return rval;
}

uint32_t
getGraphiteFeatureCode(XeTeXLayoutEngine engine, uint32_t index)
{
    uint32_t rval = 0;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_fref(grFace, index);
        rval = gr_fref_id(feature);
    }

    return rval;
}

uint32_t
countGraphiteFeatureSettings(XeTeXLayoutEngine engine, uint32_t featureID)
{
    uint32_t rval = 0;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_find_fref(grFace, featureID);
        rval = gr_fref_n_values(feature);
    }

    return rval;
}

uint32_t
getGraphiteFeatureSettingCode(XeTeXLayoutEngine engine, uint32_t featureID, uint32_t index)
{
    uint32_t rval = 0;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_find_fref(grFace, featureID);
        rval = gr_fref_value(feature, index);
    }

    return rval;
}

#define tag_from_lang(x) hb_tag_from_string(hb_language_to_string(x), strlen(hb_language_to_string(x)))

uint32_t
getGraphiteFeatureDefaultSetting(XeTeXLayoutEngine engine, uint32_t featureID)
{
    uint32_t rval = 0;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_find_fref(grFace, featureID);
        gr_feature_val *featureValues = gr_face_featureval_for_lang (grFace, tag_from_lang(engine->language));

        rval = gr_fref_feature_value(feature, featureValues);
    }

    return rval;
}

char *
getGraphiteFeatureLabel(XeTeXLayoutEngine engine, uint32_t featureID)
{
    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_find_fref(grFace, featureID);
        uint32_t len = 0;
        uint16_t langID = 0x409;

        return (char *) gr_fref_label(feature, &langID, gr_utf8, &len);
    }

    return NULL;
}

char *
getGraphiteFeatureSettingLabel(XeTeXLayoutEngine engine, uint32_t featureID, uint32_t settingID)
{
    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_find_fref(grFace, featureID);
        for (int i = 0; i < gr_fref_n_values(feature); i++) {
            if ((int) settingID == gr_fref_value(feature, i)) {
                uint32_t len = 0;
                uint16_t langID = 0x409;

                return (char *) gr_fref_value_label(feature, i, &langID, gr_utf8, &len);
            }
        }
    }

    return NULL;
}

bool
findGraphiteFeature(XeTeXLayoutEngine engine, const char* s, const char* e, hb_tag_t* f, int* v)
    /* s...e is a "feature=setting" string; look for this in the font */
{
    long tmp;

    *f = 0;
    *v = 0;
    while (*s == ' ' || *s == '\t')
        ++s;
    const char* cp = s;
    while (cp < e && *cp != '=')
        ++cp;

    tmp = findGraphiteFeatureNamed(engine, s, cp - s);
    *f = tmp;
    if (tmp == -1)
        return false;

    ++cp;
    while (cp < e && (*cp == ' ' || *cp == '\t'))
        ++cp;

    if (cp == e)
        /* no setting was specified */
        return false;

    *v = findGraphiteFeatureSettingNamed(engine, *f, cp, e - cp);
    if (*v == -1)
        return false;

    return true;
}

long
findGraphiteFeatureNamed(XeTeXLayoutEngine engine, const char* name, int namelength)
{
    long rval = -1;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        for (int i = 0; i < gr_face_n_fref(grFace); i++) {
            const gr_feature_ref* feature = gr_face_fref(grFace, i);
            uint32_t len = 0;
            uint16_t langID = 0x409;

            // the first call is to get the length of the string
            gr_fref_label(feature, &langID, gr_utf8, &len);
            char* label = (char*) xmalloc(len);
            label = (char*) gr_fref_label(feature, &langID, gr_utf8, &len);

            if (strncmp(label, name, namelength) == 0) {
                rval = gr_fref_id(feature);
                gr_label_destroy(label);
                break;
            }

            gr_label_destroy(label);
        }
    }

    return rval;
}

long
findGraphiteFeatureSettingNamed(XeTeXLayoutEngine engine, uint32_t id, const char* name, int namelength)
{
    long rval = -1;

    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);

    if (grFace != NULL) {
        const gr_feature_ref* feature = gr_face_find_fref(grFace, id);
        for (int i = 0; i < gr_fref_n_values(feature); i++) {
            uint32_t len = 0;
            uint16_t langID = 0x409;

            // the first call is to get the length of the string
            gr_fref_value_label(feature, i, &langID, gr_utf8, &len);
            char* label = (char*) xmalloc(len);
            label = (char*) gr_fref_value_label(feature, i, &langID, gr_utf8, &len);

            if (strncmp(label, name, namelength) == 0) {
                rval = gr_fref_value(feature, i);
                gr_label_destroy(label);
                break;
            }

            gr_label_destroy(label);
        }
    }

    return rval;
}

float
getGlyphWidth(XeTeXFont font, uint32_t gid)
{
    return XeTeXFontInst_getGlyphWidth((XeTeXFontInst*)font, gid);
}

unsigned int
countGlyphs(XeTeXFont font)
{
    return XeTeXFontInst_getNumGlyphs((XeTeXFontInst*)font);
}

XeTeXFont
getFont(XeTeXLayoutEngine engine)
{
    return (XeTeXFont)(engine->font);
}

float
getExtendFactor(XeTeXLayoutEngine engine)
{
    return engine->extend;
}

float
getSlantFactor(XeTeXLayoutEngine engine)
{
    return engine->slant;
}

float
getEmboldenFactor(XeTeXLayoutEngine engine)
{
    return engine->embolden;
}

XeTeXLayoutEngine_rec* XeTeXLayoutEngine_create() {
	return malloc(sizeof(XeTeXLayoutEngine_rec));	
}

void XeTeXLayoutEngine_delete(XeTeXLayoutEngine_rec* engine) {
	free(engine);
}

XeTeXLayoutEngine
createLayoutEngine(PlatformFontRef fontRef, XeTeXFont font, hb_tag_t script, char *language,
                    hb_feature_t* features, int nFeatures, char **shapers, uint32_t rgbValue,
                    float extend, float slant, float embolden)
{
    XeTeXLayoutEngine result = XeTeXLayoutEngine_create();
    result->fontRef = fontRef;
    result->font = (XeTeXFontInst*)font;
    result->script = script;
    result->features = features;
    result->ShaperList = shapers;
    result->shaper = NULL;
    result->nFeatures = nFeatures;
    result->rgbValue = rgbValue;
    result->extend = extend;
    result->slant = slant;
    result->embolden = embolden;
    result->hbBuffer = hb_buffer_create();

    // For Graphite fonts treat the language as BCP 47 tag, for OpenType we
    // treat it as a OT language tag for backward compatibility with pre-0.9999
    // XeTeX.
    if (getReqEngine() == 'G')
        result->language = hb_language_from_string(language, -1);
    else
        result->language = hb_ot_tag_to_language(hb_tag_from_string(language, -1));

    free(language);

    return result;
}

void
deleteLayoutEngine(XeTeXLayoutEngine engine)
{
    hb_buffer_destroy(engine->hbBuffer);
    XeTeXFontInst_delete(engine->font);
    free(engine->shaper);
    XeTeXLayoutEngine_delete(engine);
}

static unsigned int
_decompose_compat(hb_unicode_funcs_t* ufuncs,
                  hb_codepoint_t      u,
                  hb_codepoint_t*     decomposed,
                  void*               user_data)
{
    return 0;
}

static hb_unicode_funcs_t*
_get_unicode_funcs(void)
{
    static hb_unicode_funcs_t* ufuncs = NULL;
	if (!ufuncs)
		ufuncs = hb_unicode_funcs_create(hb_icu_get_unicode_funcs());
    hb_unicode_funcs_set_decompose_compatibility_func(ufuncs, _decompose_compat, NULL, NULL);
    return ufuncs;
}

static hb_unicode_funcs_t* hbUnicodeFuncs = NULL;

int
layoutChars(XeTeXLayoutEngine engine, uint16_t chars[], int32_t offset, int32_t count, int32_t max,
                        bool rightToLeft)
{
    bool res;
    hb_script_t script = HB_SCRIPT_INVALID;
    hb_direction_t direction = HB_DIRECTION_LTR;
    hb_segment_properties_t segment_props;
    hb_shape_plan_t *shape_plan;
    hb_font_t* hbFont = XeTeXFontInst_getHbFont(engine->font);
    hb_face_t* hbFace = hb_font_get_face(hbFont);

    if (XeTeXFontInst_getLayoutDirVertical(engine->font))
        direction = HB_DIRECTION_TTB;
    else if (rightToLeft)
        direction = HB_DIRECTION_RTL;

    script = hb_ot_tag_to_script (engine->script);

    if (hbUnicodeFuncs == NULL)
        hbUnicodeFuncs = _get_unicode_funcs();

    hb_buffer_reset(engine->hbBuffer);
    hb_buffer_set_unicode_funcs(engine->hbBuffer, hbUnicodeFuncs);
    hb_buffer_add_utf16(engine->hbBuffer, chars, max, offset, count);
    hb_buffer_set_direction(engine->hbBuffer, direction);
    hb_buffer_set_script(engine->hbBuffer, script);
    hb_buffer_set_language(engine->hbBuffer, engine->language);

    hb_buffer_guess_segment_properties(engine->hbBuffer);
    hb_buffer_get_segment_properties(engine->hbBuffer, &segment_props);

    if (engine->ShaperList == NULL) {
        // HarfBuzz gives graphite2 shaper a priority, so that for hybrid
        // Graphite/OpenType fonts, Graphite will be used. However, pre-0.9999
        // XeTeX preferred OpenType over Graphite, so we are doing the same
        // here for sake of backward compatibility. Since "ot" shaper never
        // fails, we set the shaper list to just include it.
        engine->ShaperList = (char**) xcalloc(2, sizeof(char*));
        engine->ShaperList[0] = (char*) "ot";
        engine->ShaperList[1] = NULL;
    }

    shape_plan = hb_shape_plan_create_cached(hbFace, &segment_props, engine->features, engine->nFeatures, (const char * const*)engine->ShaperList);
    res = hb_shape_plan_execute(shape_plan, hbFont, engine->hbBuffer, engine->features, engine->nFeatures);

    if (engine->shaper != NULL) {
        free(engine->shaper);
        engine->shaper = NULL;
    }

    if (res) {
        engine->shaper = strdup(hb_shape_plan_get_shaper(shape_plan));
        hb_buffer_set_content_type(engine->hbBuffer, HB_BUFFER_CONTENT_TYPE_GLYPHS);
    } else {
        // all selected shapers failed, retrying with default
        // we don't use _cached here as the cached plain will always fail.
        hb_shape_plan_destroy(shape_plan);
        shape_plan = hb_shape_plan_create(hbFace, &segment_props, engine->features, engine->nFeatures, NULL);
        res = hb_shape_plan_execute(shape_plan, hbFont, engine->hbBuffer, engine->features, engine->nFeatures);

        if (res) {
            engine->shaper = strdup(hb_shape_plan_get_shaper(shape_plan));
            hb_buffer_set_content_type(engine->hbBuffer, HB_BUFFER_CONTENT_TYPE_GLYPHS);
        } else {
            _tt_abort("all shapers failed");
        }
    }

    hb_shape_plan_destroy(shape_plan);

    int glyphCount = hb_buffer_get_length(engine->hbBuffer);

#ifdef DEBUG
    char buf[1024];
    unsigned int consumed;

    printf ("shaper: %s\n", engine->shaper);

    hb_buffer_serialize_flags_t flags = HB_BUFFER_SERIALIZE_FLAGS_DEFAULT;
    hb_buffer_serialize_format_t format = HB_BUFFER_SERIALIZE_FORMAT_JSON;

    hb_buffer_serialize_glyphs (engine->hbBuffer, 0, glyphCount, buf, sizeof(buf), &consumed, hbFont, format, flags);
    if (consumed)
        printf ("buffer glyphs: %s\n", buf);
#endif

    return glyphCount;
}

void
getGlyphs(XeTeXLayoutEngine engine, uint32_t glyphs[])
{
    int glyphCount = hb_buffer_get_length(engine->hbBuffer);
    hb_glyph_info_t *hbGlyphs = hb_buffer_get_glyph_infos(engine->hbBuffer, NULL);

    for (int i = 0; i < glyphCount; i++)
        glyphs[i] = hbGlyphs[i].codepoint;
}

void
getGlyphAdvances(XeTeXLayoutEngine engine, float advances[])
{
    int glyphCount = hb_buffer_get_length(engine->hbBuffer);
    hb_glyph_position_t *hbPositions = hb_buffer_get_glyph_positions(engine->hbBuffer, NULL);

    for (int i = 0; i < glyphCount; i++) {
        if (XeTeXFontInst_getLayoutDirVertical(engine->font))
            advances[i] = XeTeXFontInst_unitsToPoints(engine->font, hbPositions[i].y_advance);
        else
            advances[i] = XeTeXFontInst_unitsToPoints(engine->font, hbPositions[i].x_advance);
    }
}

void
getGlyphPositions(XeTeXLayoutEngine engine, FloatPoint positions[])
{
    int glyphCount = hb_buffer_get_length(engine->hbBuffer);
    hb_glyph_position_t *hbPositions = hb_buffer_get_glyph_positions(engine->hbBuffer, NULL);

    float x = 0, y = 0;

    if (XeTeXFontInst_getLayoutDirVertical(engine->font)) {
        for (int i = 0; i < glyphCount; i++) {
            positions[i].x = -XeTeXFontInst_unitsToPoints(engine->font, x + hbPositions[i].y_offset); /* negative is forwards */
            positions[i].y =  XeTeXFontInst_unitsToPoints(engine->font, y - hbPositions[i].x_offset);
            x += hbPositions[i].y_advance;
            y += hbPositions[i].x_advance;
        }
        positions[glyphCount].x = -XeTeXFontInst_unitsToPoints(engine->font, x);
        positions[glyphCount].y =  XeTeXFontInst_unitsToPoints(engine->font, y);
    } else {
        for (int i = 0; i < glyphCount; i++) {
            positions[i].x =  XeTeXFontInst_unitsToPoints(engine->font, x + hbPositions[i].x_offset);
            positions[i].y = -XeTeXFontInst_unitsToPoints(engine->font, y + hbPositions[i].y_offset); /* negative is upwards */
            x += hbPositions[i].x_advance;
            y += hbPositions[i].y_advance;
        }
        positions[glyphCount].x =  XeTeXFontInst_unitsToPoints(engine->font, x);
        positions[glyphCount].y = -XeTeXFontInst_unitsToPoints(engine->font, y);
    }

    if (engine->extend != 1.0 || engine->slant != 0.0)
        for (int i = 0; i <= glyphCount; ++i)
            positions[i].x = positions[i].x * engine->extend - positions[i].y * engine->slant;
}

float
getPointSize(XeTeXLayoutEngine engine)
{
    return XeTeXFontInst_getPointSize(engine->font);
}

void
getAscentAndDescent(XeTeXLayoutEngine engine, float* ascent, float* descent)
{
    *ascent = XeTeXFontInst_getAscent(engine->font);
    *descent = XeTeXFontInst_getDescent(engine->font);
}

void
getCapAndXHeight(XeTeXLayoutEngine engine, float* capheight, float* xheight)
{
    *capheight = XeTeXFontInst_getCapHeight(engine->font);
    *xheight = XeTeXFontInst_getXHeight(engine->font);
}

int
getDefaultDirection(XeTeXLayoutEngine engine)
{
    hb_script_t script = hb_buffer_get_script(engine->hbBuffer);
    if (hb_script_get_horizontal_direction (script) == HB_DIRECTION_RTL)
        return UBIDI_DEFAULT_RTL;
    else
        return UBIDI_DEFAULT_LTR;
}

uint32_t
getRgbValue(XeTeXLayoutEngine engine)
{
    return engine->rgbValue;
}

void
getGlyphBounds(XeTeXLayoutEngine engine, uint32_t glyphID, GlyphBBox* bbox)
{
    XeTeXFontInst_getGlyphBounds(engine->font, glyphID, bbox);
    if (engine->extend != 0.0) {
        bbox->xMin *= engine->extend;
        bbox->xMax *= engine->extend;
    }
}

float
getGlyphWidthFromEngine(XeTeXLayoutEngine engine, uint32_t glyphID)
{
    return engine->extend * XeTeXFontInst_getGlyphWidth(engine->font, glyphID);
}

void
getGlyphHeightDepth(XeTeXLayoutEngine engine, uint32_t glyphID, float* height, float* depth)
{
    XeTeXFontInst_getGlyphHeightDepth(engine->font, glyphID, height, depth);
}

void
getGlyphSidebearings(XeTeXLayoutEngine engine, uint32_t glyphID, float* lsb, float* rsb)
{
    XeTeXFontInst_getGlyphSidebearings(engine->font, glyphID, lsb, rsb);
    if (engine->extend != 0.0) {
        *lsb *= engine->extend;
        *rsb *= engine->extend;
    }
}

float
getGlyphItalCorr(XeTeXLayoutEngine engine, uint32_t glyphID)
{
    return engine->extend * XeTeXFontInst_getGlyphItalCorr(engine->font, glyphID);
}

uint32_t
mapCharToGlyph(XeTeXLayoutEngine engine, uint32_t charCode)
{
    return XeTeXFontInst_mapCharToGlyph(engine->font, charCode);
}

int
getFontCharRange(XeTeXLayoutEngine engine, int reqFirst)
{
    if (reqFirst)
        return XeTeXFontInst_getFirstCharCode(engine->font);
    else
        return XeTeXFontInst_getLastCharCode(engine->font);
}

const char*
getGlyphName(XeTeXFont font, uint16_t gid, int* len)
{
    return XeTeXFontInst_getGlyphName((XeTeXFontInst*)font, gid, len);
}

int
mapGlyphToIndex(XeTeXLayoutEngine engine, const char* glyphName)
{
    return XeTeXFontInst_mapGlyphToIndex(engine->font, glyphName);
}

static gr_segment* grSegment = NULL;
static const gr_slot* grPrevSlot = NULL;
static int grTextLen;

bool
initGraphiteBreaking(XeTeXLayoutEngine engine, const uint16_t* txtPtr, int txtLen)
{
    hb_face_t* hbFace = hb_font_get_face(XeTeXFontInst_getHbFont(engine->font));
    gr_face* grFace = hb_graphite2_face_get_gr_face(hbFace);
    gr_font* grFont = hb_graphite2_font_get_gr_font(XeTeXFontInst_getHbFont(engine->font));
    if (grFace != NULL && grFont != NULL) {
        if (grSegment != NULL) {
            gr_seg_destroy(grSegment);
            grSegment = NULL;
            grPrevSlot = NULL;
        }

        gr_feature_val *grFeatureValues = gr_face_featureval_for_lang (grFace, tag_from_lang(engine->language));

        int nFeatures = engine->nFeatures;
        hb_feature_t *features =  engine->features;
        while (nFeatures--) {
            const gr_feature_ref *fref = gr_face_find_fref (grFace, features->tag);
            if (fref)
                gr_fref_set_feature_value (fref, features->value, grFeatureValues);
            features++;
        }

        grSegment = gr_make_seg(grFont, grFace, engine->script, grFeatureValues, gr_utf16, txtPtr, txtLen, 0);
        grPrevSlot = gr_seg_first_slot(grSegment);
        grTextLen = txtLen;

        return true;
    }

    return false;
}

int
findNextGraphiteBreak(void)
{
    int ret = -1;

    if (grSegment != NULL) {
        if (grPrevSlot && grPrevSlot != gr_seg_last_slot(grSegment)) {
            for (const gr_slot* s = gr_slot_next_in_segment(grPrevSlot); s != NULL; s = gr_slot_next_in_segment(s)) {
                const gr_char_info* ci = NULL;
                int bw;

                ci = gr_seg_cinfo(grSegment, gr_slot_index(s));
                bw = gr_cinfo_break_weight(ci);
                if (bw < gr_breakNone && bw >= gr_breakBeforeWord) {
                    grPrevSlot = s;
                    ret = gr_cinfo_base(ci);
                } else if (bw > gr_breakNone && bw <= gr_breakWord) {
                    grPrevSlot = gr_slot_next_in_segment(s);
                    ret = gr_cinfo_base(ci) + 1;
                }

                if (ret != -1)
                    break;
            }

            if (ret == -1) {
                grPrevSlot = gr_seg_last_slot(grSegment);
                ret = grTextLen;
            }
        }
    }

    return ret;
}

bool
usingGraphite(XeTeXLayoutEngine engine)
{
    if (engine->shaper != NULL && (strcmp("graphite2", engine->shaper) == 0))
        return true;
    else
        return false;
}

bool
usingOpenType(XeTeXLayoutEngine engine)
{
    if (engine->shaper == NULL || (strcmp("ot", engine->shaper) == 0))
        return true;
    else
        return false;
}

bool
isOpenTypeMathFont(XeTeXLayoutEngine engine)
{
    return hb_ot_math_has_data(hb_font_get_face(XeTeXFontInst_getHbFont(engine->font)));
}
