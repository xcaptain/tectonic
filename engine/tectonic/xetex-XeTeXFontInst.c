/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew

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

/*
 *   file name:  XeTeXFontInst.cpp
 *
 *   created on: 2005-10-22
 *   created by: Jonathan Kew
 *
 *     originally based on PortableFontInstance.cpp from ICU
 */

#include "xetex-core.h"

#include "xetex-XeTeXFontInst.h"
#include "xetex-XeTeXLayoutInterface.h"
#include "xetex-ext.h"

#include <string.h>
#include FT_GLYPH_H
#include FT_ADVANCES_H


/* Return NAME with any leading path stripped off.  This returns a
   pointer into NAME.  For example, `basename ("/foo/bar.baz")'
   returns "bar.baz".  */

static const char *
xbasename (const char *name)
{
    const char *base = name;
    const char *p;

    for (p = base; *p; p++) {
        if (IS_DIR_SEP(*p))
            base = p + 1;
    }

    return base;
}


FT_Library gFreeTypeLibrary = 0;

static hb_font_funcs_t* hbFontFuncs = NULL;

void XeTeXFontInst_base_ctor(XeTeXFontInst* self, const char* pathname, int index, float pointSize, int *status) {
    self->m_unitsPerEM = 0;
    self->m_pointSize = pointSize;
    self->m_ascent = 0;
    self->m_descent = 0;
    self->m_capHeight = 0;
    self->m_xHeight = 0;
    self->m_italicAngle = 0;
    self->m_vertical = false;
    self->m_filename = (NULL);
    self->m_index = (0);
    self->m_ftFace =(0);
    self->m_backingData = (NULL);
    self->m_backingData2 = (NULL);
    self->m_hbFont = (NULL);
	self->m_subdtor = (NULL);
    if (pathname != NULL)
        XeTeXFontInst_initialize(self, pathname, index, status);	
}

XeTeXFontInst* XeTeXFontInst_create(const char* pathname, int index, float pointSize, int *status)
{
	XeTeXFontInst* self = malloc(sizeof(XeTeXFontInst));
	XeTeXFontInst_base_ctor(self, pathname, index, pointSize, status);
	return self;
}

void XeTeXFontInst_delete(XeTeXFontInst* self)
{
	if (!self)
		return;
	if (self->m_subdtor)
		(self->m_subdtor)(self);
    if (self->m_ftFace != 0) {
        FT_Done_Face(self->m_ftFace);
        self->m_ftFace = 0;
    }
    hb_font_destroy(self->m_hbFont);
    free(self->m_backingData);
    free(self->m_backingData2);
    free(self->m_filename);
	
	free(self);
}

/* HarfBuzz font functions */

static hb_bool_t
_get_glyph(hb_font_t* _hbf, void *font_data, hb_codepoint_t ch, hb_codepoint_t vs, hb_codepoint_t *gid, void* _p)
{
    FT_Face face = (FT_Face) font_data;
    *gid = 0;

    if (vs)
        *gid = FT_Face_GetCharVariantIndex (face, ch, vs);

    if (*gid == 0)
        *gid = FT_Get_Char_Index (face, ch);

    return *gid != 0;
}

static FT_Fixed
_get_glyph_advance(FT_Face face, FT_UInt gid, bool vertical)
{
    FT_Error error;
    FT_Fixed advance;
    int flags = FT_LOAD_NO_SCALE;

    if (vertical)
        flags |= FT_LOAD_VERTICAL_LAYOUT;

    error = FT_Get_Advance(face, gid, flags, &advance);
    if (error)
        advance = 0;

    /* FreeType's vertical metrics grows downward */
    if (vertical)
        advance = -advance;

    return advance;
}

static hb_position_t
_get_glyph_h_advance(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid, void* _p)
{
    return _get_glyph_advance((FT_Face) font_data, gid, false);
}

static hb_position_t
_get_glyph_v_advance(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid, void* _p)
{
    return _get_glyph_advance((FT_Face) font_data, gid, true);
}

static hb_bool_t
_get_glyph_h_origin(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid, hb_position_t *x, hb_position_t *y, void* _p)
{
    // horizontal origin is (0, 0)
    return true;
}

static hb_bool_t
_get_glyph_v_origin(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid, hb_position_t *x, hb_position_t *y, void* _p)
{
    // vertical origin is (0, 0) for now
    return true;

    // TODO
    // Keep the code below for reference, for now we want to keep vertical
    // origin at (0, 0) for compatibility with pre-0.9999.
    // Reconsider this (e.g. using BASE table) when we get around overhauling
    // the text directionality model and implementing real vertical typesetting.

    FT_Face face = (FT_Face) font_data;
    FT_Error error;

    error = FT_Load_Glyph (face, gid, FT_LOAD_NO_SCALE);
    if (!error) {
        *x = face->glyph->metrics.horiBearingX -   face->glyph->metrics.vertBearingX;
        *y = face->glyph->metrics.horiBearingY - (-face->glyph->metrics.vertBearingY);
    }

    return !error;
}

static hb_position_t
_get_glyph_h_kerning(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid1, hb_codepoint_t gid2, void* _p)
{
    FT_Face face = (FT_Face) font_data;
    FT_Error error;
    FT_Vector kerning;
    hb_position_t ret;

    error = FT_Get_Kerning (face, gid1, gid2, FT_KERNING_UNSCALED, &kerning);
    if (error)
        ret = 0;
    else
        ret = kerning.x;
    return ret;
}

static hb_position_t
_get_glyph_v_kerning(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid1, hb_codepoint_t gid2, void* _p)
{
    /* FreeType does not support vertical kerning */
    return 0;
}

static hb_bool_t
_get_glyph_extents(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid, hb_glyph_extents_t *extents, void* _p)
{
    FT_Face face = (FT_Face) font_data;
    FT_Error error;

    error = FT_Load_Glyph (face, gid, FT_LOAD_NO_SCALE);
    if (!error) {
        extents->x_bearing = face->glyph->metrics.horiBearingX;
        extents->y_bearing = face->glyph->metrics.horiBearingY;
        extents->width  =  face->glyph->metrics.width;
        extents->height = -face->glyph->metrics.height;
    }

    return !error;
}

static hb_bool_t
_get_glyph_contour_point(hb_font_t* _hbf, void *font_data, hb_codepoint_t gid, unsigned int point_index, hb_position_t *x, hb_position_t *y, void* _p)
{
    FT_Face face = (FT_Face) font_data;
    FT_Error error;
    bool ret = false;

    error = FT_Load_Glyph (face, gid, FT_LOAD_NO_SCALE);
    if (!error) {
        if (face->glyph->format == FT_GLYPH_FORMAT_OUTLINE) {
            if (point_index < (unsigned int) face->glyph->outline.n_points) {
                *x = face->glyph->outline.points[point_index].x;
                *y = face->glyph->outline.points[point_index].y;
                ret = true;
            }
        }
    }

    return ret;
}

static hb_bool_t
_get_glyph_name(hb_font_t * _hbf, void *font_data, hb_codepoint_t gid, char *name, unsigned int size, void * _p)
{
    FT_Face face = (FT_Face) font_data;
    bool ret = false;

    ret = !FT_Get_Glyph_Name (face, gid, name, size);
    if (ret && (size && !*name))
        ret = false;

    return ret;
}

static hb_font_funcs_t *
_get_font_funcs(void)
{
    static hb_font_funcs_t* funcs = NULL;
	if (!funcs)
		funcs = hb_font_funcs_create();

    hb_font_funcs_set_glyph_func                (funcs, _get_glyph, NULL, NULL);
    hb_font_funcs_set_glyph_h_advance_func      (funcs, _get_glyph_h_advance, NULL, NULL);
    hb_font_funcs_set_glyph_v_advance_func      (funcs, _get_glyph_v_advance, NULL, NULL);
    hb_font_funcs_set_glyph_h_origin_func       (funcs, _get_glyph_h_origin, NULL, NULL);
    hb_font_funcs_set_glyph_v_origin_func       (funcs, _get_glyph_v_origin, NULL, NULL);
    hb_font_funcs_set_glyph_h_kerning_func      (funcs, _get_glyph_h_kerning, NULL, NULL);
    hb_font_funcs_set_glyph_v_kerning_func      (funcs, _get_glyph_v_kerning, NULL, NULL);
    hb_font_funcs_set_glyph_extents_func        (funcs, _get_glyph_extents, NULL, NULL);
    hb_font_funcs_set_glyph_contour_point_func  (funcs, _get_glyph_contour_point, NULL, NULL);
    hb_font_funcs_set_glyph_name_func           (funcs, _get_glyph_name, NULL, NULL);

    return funcs;
}

static hb_blob_t *
_get_table(hb_face_t * _hfc, hb_tag_t tag, void *user_data)
{
    FT_Face face = (FT_Face) user_data;
    FT_ULong length = 0;
    FT_Byte *table;
    FT_Error error;
    hb_blob_t* blob = NULL;

    error = FT_Load_Sfnt_Table(face, tag, 0, NULL, &length);
    if (!error) {
        table = (FT_Byte *) xmalloc(length * sizeof(char));
        if (table != NULL) {
            error = FT_Load_Sfnt_Table(face, tag, 0, (FT_Byte*)table, &length);
            if (!error) {
                blob = hb_blob_create((const char*) table, length, HB_MEMORY_MODE_WRITABLE, table, free);
            } else {
                free(table);
            }
        }
    }

    return blob;
}

void
XeTeXFontInst_initialize(XeTeXFontInst* self, const char* pathname, int index, int *status)
{
    TT_Postscript *postTable;
    TT_OS2* os2Table;
    FT_Error error;
    hb_face_t *hbFace;

    if (!gFreeTypeLibrary) {
        error = FT_Init_FreeType(&gFreeTypeLibrary);
        if (error)
            _tt_abort("FreeType initialization failed, error %d", error);
    }

    // Here we emulate some logic that was originally in find_native_font();
    rust_input_handle_t handle = ttstub_input_open (pathname, TTIF_OPENTYPE, 0);
    if (handle == NULL)
        handle = ttstub_input_open (pathname, TTIF_TRUETYPE, 0);
    if (handle == NULL)
        handle = ttstub_input_open (pathname, TTIF_TYPE1, 0);
    if (handle == NULL) {
        *status = 1;
        return;
    }

    size_t sz = ttstub_input_get_size (handle);
    self->m_backingData = (FT_Byte *) xmalloc (sz);
    ssize_t r = ttstub_input_read (handle, (char *) self->m_backingData, sz);
    if (r < 0 || (size_t) r != sz)
        _tt_abort("failed to read font file");
    ttstub_input_close(handle);

    error = FT_New_Memory_Face(gFreeTypeLibrary, self->m_backingData, sz, index, &self->m_ftFace);

    if (!FT_IS_SCALABLE(self->m_ftFace)) {
        *status = 1;
        return;
    }

    /* for non-sfnt-packaged fonts (presumably Type 1), see if there is an AFM file we can attach */
    if (index == 0 && !FT_IS_SFNT(self->m_ftFace)) {
        // Tectonic: this code used to use kpse_find_file and FT_Attach_File
        // to try to find metrics for this font. Thanks to the existence of
        // FT_Attach_Stream we can emulate this behavior while going through
        // the Rust I/O layer.

        char *afm = xstrdup (xbasename (pathname));
        char *p = strrchr (afm, '.');
        if (p != NULL && strlen(p) == 4 && tolower(*(p+1)) == 'p' && tolower(*(p+2)) == 'f')
            strcpy(p, ".afm");

        rust_input_handle_t afm_handle = ttstub_input_open (afm, TTIF_AFM, 0);
        free (afm);

        if (afm_handle != NULL) {
            sz = ttstub_input_get_size (afm_handle);
            self->m_backingData2 = (FT_Byte *) xmalloc (sz);
            r = ttstub_input_read (afm_handle, (char *) self->m_backingData2, sz);
            if (r < 0 || (size_t) r != sz)
                _tt_abort("failed to read AFM file");
            ttstub_input_close(afm_handle);

            FT_Open_Args open_args;
            open_args.flags = FT_OPEN_MEMORY;
            open_args.memory_base = self->m_backingData2;
            open_args.memory_size = sz;

            FT_Attach_Stream(self->m_ftFace, &open_args);
        }
    }

    self->m_filename = xstrdup(pathname);
    self->m_index = index;
    self->m_unitsPerEM = self->m_ftFace->units_per_EM;
    self->m_ascent = XeTeXFontInst_unitsToPoints(self, self->m_ftFace->ascender);
    self->m_descent = XeTeXFontInst_unitsToPoints(self, self->m_ftFace->descender);

    postTable = (TT_Postscript *) XeTeXFontInst_getFontTableFT(self, ft_sfnt_post);
    if (postTable != NULL) {
        self->m_italicAngle = Fix2D(postTable->italicAngle);
    }

    os2Table = (TT_OS2*) XeTeXFontInst_getFontTableFT(self, ft_sfnt_os2);
    if (os2Table) {
        self->m_capHeight = XeTeXFontInst_unitsToPoints(self, os2Table->sCapHeight);
        self->m_xHeight = XeTeXFontInst_unitsToPoints(self, os2Table->sxHeight);
    }

    // Set up HarfBuzz font
    hbFace = hb_face_create_for_tables(_get_table, self->m_ftFace, NULL);
    hb_face_set_index(hbFace, index);
    hb_face_set_upem(hbFace, self->m_unitsPerEM);
    self->m_hbFont = hb_font_create(hbFace);
    hb_face_destroy(hbFace);

    if (hbFontFuncs == NULL)
        hbFontFuncs = _get_font_funcs();

    hb_font_set_funcs(self->m_hbFont, hbFontFuncs, self->m_ftFace, NULL);
    hb_font_set_scale(self->m_hbFont, self->m_unitsPerEM, self->m_unitsPerEM);
    // We don’t want device tables adjustments
    hb_font_set_ppem(self->m_hbFont, 0, 0);

    return;
}

void
XeTeXFontInst_setLayoutDirVertical(XeTeXFontInst* self, bool vertical)
{
    self->m_vertical = vertical;
}

void *
XeTeXFontInst_getFontTable(const XeTeXFontInst* self, OTTag tag)
{
    FT_ULong tmpLength = 0;
    FT_Error error = FT_Load_Sfnt_Table(self->m_ftFace, tag, 0, NULL, &tmpLength);
    if (error)
        return NULL;

    void* table = xmalloc(tmpLength * sizeof(char));
    if (table != NULL) {
        error = FT_Load_Sfnt_Table(self->m_ftFace, tag, 0, (FT_Byte*)table, &tmpLength);
        if (error) {
            free((void *) table);
            return NULL;
        }
    }

    return table;
}

void *
XeTeXFontInst_getFontTableFT(const XeTeXFontInst* self, FT_Sfnt_Tag tag) 
{
    return FT_Get_Sfnt_Table(self->m_ftFace, tag);
}

void
XeTeXFontInst_getGlyphBounds(XeTeXFontInst* self, GlyphID gid, GlyphBBox* bbox)
{
    bbox->xMin = bbox->yMin = bbox->xMax = bbox->yMax = 0.0;

    FT_Error error = FT_Load_Glyph(self->m_ftFace, gid, FT_LOAD_NO_SCALE);
    if (error)
        return;

    FT_Glyph glyph;
    error = FT_Get_Glyph(self->m_ftFace->glyph, &glyph);
    if (error == 0) {
        FT_BBox ft_bbox;
        FT_Glyph_Get_CBox(glyph, FT_GLYPH_BBOX_UNSCALED, &ft_bbox);
        bbox->xMin = XeTeXFontInst_unitsToPoints(self, ft_bbox.xMin);
        bbox->yMin = XeTeXFontInst_unitsToPoints(self, ft_bbox.yMin);
        bbox->xMax = XeTeXFontInst_unitsToPoints(self, ft_bbox.xMax);
        bbox->yMax = XeTeXFontInst_unitsToPoints(self, ft_bbox.yMax);
        FT_Done_Glyph(glyph);
    }
}

GlyphID
XeTeXFontInst_mapCharToGlyph(const XeTeXFontInst* self, UChar32 ch) 
{
    return FT_Get_Char_Index(self->m_ftFace, ch);
}

uint16_t
XeTeXFontInst_getNumGlyphs(const XeTeXFontInst* self) 
{
    return self->m_ftFace->num_glyphs;
}

float
XeTeXFontInst_getGlyphWidth(XeTeXFontInst* self, GlyphID gid)
{
    return XeTeXFontInst_unitsToPoints(self, _get_glyph_advance(self->m_ftFace, gid, false));
}

void
XeTeXFontInst_getGlyphHeightDepth(XeTeXFontInst* self, GlyphID gid, float* ht, float* dp)
{
    GlyphBBox bbox;
    XeTeXFontInst_getGlyphBounds(self, gid, &bbox);

    if (ht)
        *ht = bbox.yMax;
    if (dp)
        *dp = -bbox.yMin;
}

void
XeTeXFontInst_getGlyphSidebearings(XeTeXFontInst* self, GlyphID gid, float* lsb, float* rsb)
{
    float width = XeTeXFontInst_getGlyphWidth(self, gid);

    GlyphBBox bbox;
    XeTeXFontInst_getGlyphBounds(self, gid, &bbox);

    if (lsb)
        *lsb = bbox.xMin;
    if (rsb)
        *rsb = width - bbox.xMax;
}

float
XeTeXFontInst_getGlyphItalCorr(XeTeXFontInst* self, GlyphID gid)
{
    float rval = 0.0;

    float width = XeTeXFontInst_getGlyphWidth(self, gid);

    GlyphBBox bbox;
    XeTeXFontInst_getGlyphBounds(self, gid, &bbox);

    if (bbox.xMax > width)
        rval = bbox.xMax - width;

    return rval;
}

GlyphID
XeTeXFontInst_mapGlyphToIndex(const XeTeXFontInst* self, const char* glyphName)
{
    return FT_Get_Name_Index(self->m_ftFace, (char*)(glyphName));
}

const char*
XeTeXFontInst_getGlyphName(XeTeXFontInst* self, GlyphID gid, int* nameLen)
{
    if (FT_HAS_GLYPH_NAMES(self->m_ftFace)) {
        static char buffer[256];
        FT_Get_Glyph_Name(self->m_ftFace, gid, buffer, 256);
        *nameLen = strlen(buffer);
        return &buffer[0];
    }
    else {
        *nameLen = 0;
        return NULL;
    }
}

UChar32
XeTeXFontInst_getFirstCharCode(XeTeXFontInst* self)
{
    FT_UInt gindex;
    return FT_Get_First_Char(self->m_ftFace, &gindex);
}

UChar32
XeTeXFontInst_getLastCharCode(XeTeXFontInst* self)
{
    FT_UInt gindex;
    UChar32 ch = FT_Get_First_Char(self->m_ftFace, &gindex);
    UChar32 prev = ch;
    while (gindex != 0) {
        prev = ch;
        ch = FT_Get_Next_Char(self->m_ftFace, ch, &gindex);
    }
    return prev;
}
