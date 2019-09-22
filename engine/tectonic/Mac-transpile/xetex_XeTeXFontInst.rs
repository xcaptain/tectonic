#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, linkage)]
extern crate libc;
extern "C" {
    pub type hb_font_funcs_t;
    pub type hb_blob_t;
    /* ************************************************************************/
  /* ************************************************************************/
  /*                                                                       */
  /*                     O B J E C T   C L A S S E S                       */
  /*                                                                       */
  /* ************************************************************************/
  /* ************************************************************************/
    /* *************************************************************************
   *
   * @type:
   *   FT_Library
   *
   * @description:
   *   A handle to a FreeType library instance.  Each 'library' is completely
   *   independent from the others; it is the 'root' of a set of objects like
   *   fonts, faces, sizes, etc.
   *
   *   It also embeds a memory manager (see @FT_Memory), as well as a
   *   scan-line converter object (see @FT_Raster).
   *
   *   [Since 2.5.6] In multi-threaded applications it is easiest to use one
   *   `FT_Library` object per thread.  In case this is too cumbersome, a
   *   single `FT_Library` object across threads is possible also, as long as
   *   a mutex lock is used around @FT_New_Face and @FT_Done_Face.
   *
   * @note:
   *   Library objects are normally created by @FT_Init_FreeType, and
   *   destroyed with @FT_Done_FreeType.  If you need reference-counting
   *   (cf. @FT_Reference_Library), use @FT_New_Library and @FT_Done_Library.
   */
    pub type FT_LibraryRec_;
    /* *************************************************************************
   *
   * @section:
   *   module_management
   *
   */
    /* *************************************************************************
   *
   * @type:
   *   FT_Module
   *
   * @description:
   *   A handle to a given FreeType module object.  A module can be a font
   *   driver, a renderer, or anything else that provides services to the
   *   former.
   */
    pub type FT_ModuleRec_;
    /* *************************************************************************
   *
   * @type:
   *   FT_Driver
   *
   * @description:
   *   A handle to a given FreeType font driver object.  A font driver is a
   *   module capable of creating faces from font files.
   */
    pub type FT_DriverRec_;
    pub type FT_Face_InternalRec_;
    pub type FT_Size_InternalRec_;
    pub type FT_Slot_InternalRec_;
    pub type FT_SubGlyphRec_;
    pub type hb_face_t;
    pub type hb_font_t;
    pub type FT_Glyph_Class_;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(path: *const libc::c_char,
                         format: tt_input_format_type, is_gz: libc::c_int)
     -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut libc::c_char,
                         len: size_t) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    /* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
    #[no_mangle]
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    /* *************************************************************************
   *
   * @function:
   *   FT_New_Memory_Face
   *
   * @description:
   *   Call @FT_Open_Face to open a font that has been loaded into memory.
   *
   * @inout:
   *   library ::
   *     A handle to the library resource.
   *
   * @input:
   *   file_base ::
   *     A pointer to the beginning of the font data.
   *
   *   file_size ::
   *     The size of the memory chunk used by the font data.
   *
   *   face_index ::
   *     See @FT_Open_Face for a detailed description of this parameter.
   *
   * @output:
   *   aface ::
   *     A handle to a new face object.  If `face_index` is greater than or
   *     equal to zero, it must be non-`NULL`.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   You must not deallocate the memory before calling @FT_Done_Face.
   */
    #[no_mangle]
    fn FT_New_Memory_Face(library: FT_Library, file_base: *const FT_Byte,
                          file_size: FT_Long, face_index: FT_Long,
                          aface: *mut FT_Face) -> FT_Error;
    /* *************************************************************************
   *
   * @function:
   *   FT_Attach_Stream
   *
   * @description:
   *   'Attach' data to a face object.  Normally, this is used to read
   *   additional information for the face object.  For example, you can
   *   attach an AFM file that comes with a Type~1 font to get the kerning
   *   values and other metrics.
   *
   * @inout:
   *   face ::
   *     The target face object.
   *
   * @input:
   *   parameters ::
   *     A pointer to @FT_Open_Args that must be filled by the caller.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   The meaning of the 'attach' (i.e., what really happens when the new
   *   file is read) is not fixed by FreeType itself.  It really depends on
   *   the font format (and thus the font driver).
   *
   *   Client applications are expected to know what they are doing when
   *   invoking this function.  Most drivers simply do not implement file or
   *   stream attachments.
   */
    #[no_mangle]
    fn FT_Attach_Stream(face: FT_Face, parameters: *mut FT_Open_Args)
     -> FT_Error;
    /* *************************************************************************
   *
   * @function:
   *   FT_Done_Face
   *
   * @description:
   *   Discard a given face object, as well as all of its child slots and
   *   sizes.
   *
   * @input:
   *   face ::
   *     A handle to a target face object.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   See the discussion of reference counters in the description of
   *   @FT_Reference_Face.
   */
    #[no_mangle]
    fn FT_Done_Face(face: FT_Face) -> FT_Error;
    /* *************************************************************************
   *
   * @function:
   *   FT_Load_Glyph
   *
   * @description:
   *   Load a glyph into the glyph slot of a face object.
   *
   * @inout:
   *   face ::
   *     A handle to the target face object where the glyph is loaded.
   *
   * @input:
   *   glyph_index ::
   *     The index of the glyph in the font file.  For CID-keyed fonts
   *     (either in PS or in CFF format) this argument specifies the CID
   *     value.
   *
   *   load_flags ::
   *     A flag indicating what to load for this glyph.  The @FT_LOAD_XXX
   *     constants can be used to control the glyph loading process (e.g.,
   *     whether the outline should be scaled, whether to load bitmaps or
   *     not, whether to hint the outline, etc).
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   The loaded glyph may be transformed.  See @FT_Set_Transform for the
   *   details.
   *
   *   For subsetted CID-keyed fonts, `FT_Err_Invalid_Argument` is returned
   *   for invalid CID values (this is, for CID values that don't have a
   *   corresponding glyph in the font).  See the discussion of the
   *   @FT_FACE_FLAG_CID_KEYED flag for more details.
   *
   *   If you receive `FT_Err_Glyph_Too_Big`, try getting the glyph outline
   *   at EM size, then scale it manually and fill it as a graphics
   *   operation.
   */
    #[no_mangle]
    fn FT_Load_Glyph(face: FT_Face, glyph_index: FT_UInt,
                     load_flags: FT_Int32) -> FT_Error;
    /* these constants are deprecated; use the corresponding */
  /* `FT_Kerning_Mode` values instead                      */
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Kerning
   *
   * @description:
   *   Return the kerning vector between two glyphs of the same face.
   *
   * @input:
   *   face ::
   *     A handle to a source face object.
   *
   *   left_glyph ::
   *     The index of the left glyph in the kern pair.
   *
   *   right_glyph ::
   *     The index of the right glyph in the kern pair.
   *
   *   kern_mode ::
   *     See @FT_Kerning_Mode for more information.  Determines the scale and
   *     dimension of the returned kerning vector.
   *
   * @output:
   *   akerning ::
   *     The kerning vector.  This is either in font units, fractional pixels
   *     (26.6 format), or pixels for scalable formats, and in pixels for
   *     fixed-sizes formats.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   Only horizontal layouts (left-to-right & right-to-left) are supported
   *   by this method.  Other layouts, or more sophisticated kernings, are
   *   out of the scope of this API function -- they can be implemented
   *   through format-specific interfaces.
   *
   *   Kerning for OpenType fonts implemented in a 'GPOS' table is not
   *   supported; use @FT_HAS_KERNING to find out whether a font has data
   *   that can be extracted with `FT_Get_Kerning`.
   */
    #[no_mangle]
    fn FT_Get_Kerning(face: FT_Face, left_glyph: FT_UInt,
                      right_glyph: FT_UInt, kern_mode: FT_UInt,
                      akerning: *mut FT_Vector) -> FT_Error;
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Glyph_Name
   *
   * @description:
   *   Retrieve the ASCII name of a given glyph in a face.  This only works
   *   for those faces where @FT_HAS_GLYPH_NAMES(face) returns~1.
   *
   * @input:
   *   face ::
   *     A handle to a source face object.
   *
   *   glyph_index ::
   *     The glyph index.
   *
   *   buffer_max ::
   *     The maximum number of bytes available in the buffer.
   *
   * @output:
   *   buffer ::
   *     A pointer to a target buffer where the name is copied to.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   An error is returned if the face doesn't provide glyph names or if the
   *   glyph index is invalid.  In all cases of failure, the first byte of
   *   `buffer` is set to~0 to indicate an empty name.
   *
   *   The glyph name is truncated to fit within the buffer if it is too
   *   long.  The returned string is always zero-terminated.
   *
   *   Be aware that FreeType reorders glyph indices internally so that glyph
   *   index~0 always corresponds to the 'missing glyph' (called '.notdef').
   *
   *   This function always returns an error if the config macro
   *   `FT_CONFIG_OPTION_NO_GLYPH_NAMES` is not defined in `ftoption.h`.
   */
    #[no_mangle]
    fn FT_Get_Glyph_Name(face: FT_Face, glyph_index: FT_UInt,
                         buffer: FT_Pointer, buffer_max: FT_UInt) -> FT_Error;
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Char_Index
   *
   * @description:
   *   Return the glyph index of a given character code.  This function uses
   *   the currently selected charmap to do the mapping.
   *
   * @input:
   *   face ::
   *     A handle to the source face object.
   *
   *   charcode ::
   *     The character code.
   *
   * @return:
   *   The glyph index.  0~means 'undefined character code'.
   *
   * @note:
   *   If you use FreeType to manipulate the contents of font files directly,
   *   be aware that the glyph index returned by this function doesn't always
   *   correspond to the internal indices used within the file.  This is done
   *   to ensure that value~0 always corresponds to the 'missing glyph'.  If
   *   the first glyph is not named '.notdef', then for Type~1 and Type~42
   *   fonts, '.notdef' will be moved into the glyph ID~0 position, and
   *   whatever was there will be moved to the position '.notdef' had.  For
   *   Type~1 fonts, if there is no '.notdef' glyph at all, then one will be
   *   created at index~0 and whatever was there will be moved to the last
   *   index -- Type~42 fonts are considered invalid under this condition.
   */
    #[no_mangle]
    fn FT_Get_Char_Index(face: FT_Face, charcode: FT_ULong) -> FT_UInt;
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_First_Char
   *
   * @description:
   *   Return the first character code in the current charmap of a given
   *   face, together with its corresponding glyph index.
   *
   * @input:
   *   face ::
   *     A handle to the source face object.
   *
   * @output:
   *   agindex ::
   *     Glyph index of first character code.  0~if charmap is empty.
   *
   * @return:
   *   The charmap's first character code.
   *
   * @note:
   *   You should use this function together with @FT_Get_Next_Char to parse
   *   all character codes available in a given charmap.  The code should
   *   look like this:
   *
   *   ```
   *     FT_ULong  charcode;
   *     FT_UInt   gindex;
   *
   *
   *     charcode = FT_Get_First_Char( face, &gindex );
   *     while ( gindex != 0 )
   *     {
   *       ... do something with (charcode,gindex) pair ...
   *
   *       charcode = FT_Get_Next_Char( face, charcode, &gindex );
   *     }
   *   ```
   *
   *   Be aware that character codes can have values up to 0xFFFFFFFF; this
   *   might happen for non-Unicode or malformed cmaps.  However, even with
   *   regular Unicode encoding, so-called 'last resort fonts' (using SFNT
   *   cmap format 13, see function @FT_Get_CMap_Format) normally have
   *   entries for all Unicode characters up to 0x1FFFFF, which can cause *a
   *   lot* of iterations.
   *
   *   Note that `*agindex` is set to~0 if the charmap is empty.  The result
   *   itself can be~0 in two cases: if the charmap is empty or if the
   *   value~0 is the first valid character code.
   */
    #[no_mangle]
    fn FT_Get_First_Char(face: FT_Face, agindex: *mut FT_UInt) -> FT_ULong;
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Next_Char
   *
   * @description:
   *   Return the next character code in the current charmap of a given face
   *   following the value `char_code`, as well as the corresponding glyph
   *   index.
   *
   * @input:
   *   face ::
   *     A handle to the source face object.
   *
   *   char_code ::
   *     The starting character code.
   *
   * @output:
   *   agindex ::
   *     Glyph index of next character code.  0~if charmap is empty.
   *
   * @return:
   *   The charmap's next character code.
   *
   * @note:
   *   You should use this function with @FT_Get_First_Char to walk over all
   *   character codes available in a given charmap.  See the note for that
   *   function for a simple code example.
   *
   *   Note that `*agindex` is set to~0 when there are no more codes in the
   *   charmap.
   */
    #[no_mangle]
    fn FT_Get_Next_Char(face: FT_Face, char_code: FT_ULong,
                        agindex: *mut FT_UInt) -> FT_ULong;
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Name_Index
   *
   * @description:
   *   Return the glyph index of a given glyph name.
   *
   * @input:
   *   face ::
   *     A handle to the source face object.
   *
   *   glyph_name ::
   *     The glyph name.
   *
   * @return:
   *   The glyph index.  0~means 'undefined character code'.
   */
    #[no_mangle]
    fn FT_Get_Name_Index(face: FT_Face, glyph_name: *const FT_String)
     -> FT_UInt;
    /* *************************************************************************
   *
   * @section:
   *   glyph_variants
   *
   * @title:
   *   Unicode Variation Sequences
   *
   * @abstract:
   *   The FreeType~2 interface to Unicode Variation Sequences (UVS), using
   *   the SFNT cmap format~14.
   *
   * @description:
   *   Many characters, especially for CJK scripts, have variant forms.  They
   *   are a sort of grey area somewhere between being totally irrelevant and
   *   semantically distinct; for this reason, the Unicode consortium decided
   *   to introduce Variation Sequences (VS), consisting of a Unicode base
   *   character and a variation selector instead of further extending the
   *   already huge number of characters.
   *
   *   Unicode maintains two different sets, namely 'Standardized Variation
   *   Sequences' and registered 'Ideographic Variation Sequences' (IVS),
   *   collected in the 'Ideographic Variation Database' (IVD).
   *
   *     https://unicode.org/Public/UCD/latest/ucd/StandardizedVariants.txt
   *     https://unicode.org/reports/tr37/ https://unicode.org/ivd/
   *
   *   To date (January 2017), the character with the most ideographic
   *   variations is U+9089, having 32 such IVS.
   *
   *   Three Mongolian Variation Selectors have the values U+180B-U+180D; 256
   *   generic Variation Selectors are encoded in the ranges U+FE00-U+FE0F
   *   and U+E0100-U+E01EF.  IVS currently use Variation Selectors from the
   *   range U+E0100-U+E01EF only.
   *
   *   A VS consists of the base character value followed by a single
   *   Variation Selector.  For example, to get the first variation of
   *   U+9089, you have to write the character sequence `U+9089 U+E0100`.
   *
   *   Adobe and MS decided to support both standardized and ideographic VS
   *   with a new cmap subtable (format~14).  It is an odd subtable because
   *   it is not a mapping of input code points to glyphs, but contains lists
   *   of all variations supported by the font.
   *
   *   A variation may be either 'default' or 'non-default' for a given font.
   *   A default variation is the one you will get for that code point if you
   *   look it up in the standard Unicode cmap.  A non-default variation is a
   *   different glyph.
   *
   */
    /* *************************************************************************
   *
   * @function:
   *   FT_Face_GetCharVariantIndex
   *
   * @description:
   *   Return the glyph index of a given character code as modified by the
   *   variation selector.
   *
   * @input:
   *   face ::
   *     A handle to the source face object.
   *
   *   charcode ::
   *     The character code point in Unicode.
   *
   *   variantSelector ::
   *     The Unicode code point of the variation selector.
   *
   * @return:
   *   The glyph index.  0~means either 'undefined character code', or
   *   'undefined selector code', or 'no variation selector cmap subtable',
   *   or 'current CharMap is not Unicode'.
   *
   * @note:
   *   If you use FreeType to manipulate the contents of font files directly,
   *   be aware that the glyph index returned by this function doesn't always
   *   correspond to the internal indices used within the file.  This is done
   *   to ensure that value~0 always corresponds to the 'missing glyph'.
   *
   *   This function is only meaningful if
   *     a) the font has a variation selector cmap sub table, and
   *     b) the current charmap has a Unicode encoding.
   *
   * @since:
   *   2.3.6
   */
    #[no_mangle]
    fn FT_Face_GetCharVariantIndex(face: FT_Face, charcode: FT_ULong,
                                   variantSelector: FT_ULong) -> FT_UInt;
    /* these constants are deprecated; use the corresponding `FT_Sfnt_Tag` */
  /* values instead                                                      */
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Sfnt_Table
   *
   * @description:
   *   Return a pointer to a given SFNT table stored within a face.
   *
   * @input:
   *   face ::
   *     A handle to the source.
   *
   *   tag ::
   *     The index of the SFNT table.
   *
   * @return:
   *   A type-less pointer to the table.  This will be `NULL` in case of
   *   error, or if the corresponding table was not found **OR** loaded from
   *   the file.
   *
   *   Use a typecast according to `tag` to access the structure elements.
   *
   * @note:
   *   The table is owned by the face object and disappears with it.
   *
   *   This function is only useful to access SFNT tables that are loaded by
   *   the sfnt, truetype, and opentype drivers.  See @FT_Sfnt_Tag for a
   *   list.
   *
   * @example:
   *   Here is an example demonstrating access to the 'vhea' table.
   *
   *   ```
   *     TT_VertHeader*  vert_header;
   *
   *
   *     vert_header =
   *       (TT_VertHeader*)FT_Get_Sfnt_Table( face, FT_SFNT_VHEA );
   *   ```
   */
    #[no_mangle]
    fn FT_Get_Sfnt_Table(face: FT_Face, tag: FT_Sfnt_Tag)
     -> *mut libc::c_void;
    /* *************************************************************************
   *
   * @function:
   *   FT_Load_Sfnt_Table
   *
   * @description:
   *   Load any SFNT font table into client memory.
   *
   * @input:
   *   face ::
   *     A handle to the source face.
   *
   *   tag ::
   *     The four-byte tag of the table to load.  Use value~0 if you want to
   *     access the whole font file.  Otherwise, you can use one of the
   *     definitions found in the @FT_TRUETYPE_TAGS_H file, or forge a new
   *     one with @FT_MAKE_TAG.
   *
   *   offset ::
   *     The starting offset in the table (or file if tag~==~0).
   *
   * @output:
   *   buffer ::
   *     The target buffer address.  The client must ensure that the memory
   *     array is big enough to hold the data.
   *
   * @inout:
   *   length ::
   *     If the `length` parameter is `NULL`, try to load the whole table.
   *     Return an error code if it fails.
   *
   *     Else, if `*length` is~0, exit immediately while returning the
   *     table's (or file) full size in it.
   *
   *     Else the number of bytes to read from the table or file, from the
   *     starting offset.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   If you need to determine the table's length you should first call this
   *   function with `*length` set to~0, as in the following example:
   *
   *   ```
   *     FT_ULong  length = 0;
   *
   *
   *     error = FT_Load_Sfnt_Table( face, tag, 0, NULL, &length );
   *     if ( error ) { ... table does not exist ... }
   *
   *     buffer = malloc( length );
   *     if ( buffer == NULL ) { ... not enough memory ... }
   *
   *     error = FT_Load_Sfnt_Table( face, tag, 0, buffer, &length );
   *     if ( error ) { ... could not load table ... }
   *   ```
   *
   *   Note that structures like @TT_Header or @TT_OS2 can't be used with
   *   this function; they are limited to @FT_Get_Sfnt_Table.  Reason is that
   *   those structures depend on the processor architecture, with varying
   *   size (e.g. 32bit vs. 64bit) or order (big endian vs. little endian).
   *
   */
    #[no_mangle]
    fn FT_Load_Sfnt_Table(face: FT_Face, tag: FT_ULong, offset: FT_Long,
                          buffer: *mut FT_Byte, length: *mut FT_ULong)
     -> FT_Error;
    #[no_mangle]
    fn hb_blob_create(data: *const libc::c_char, length: libc::c_uint,
                      mode: hb_memory_mode_t, user_data: *mut libc::c_void,
                      destroy: hb_destroy_func_t) -> *mut hb_blob_t;
    #[no_mangle]
    fn hb_face_create_for_tables(reference_table_func:
                                     hb_reference_table_func_t,
                                 user_data: *mut libc::c_void,
                                 destroy: hb_destroy_func_t)
     -> *mut hb_face_t;
    #[no_mangle]
    fn hb_face_destroy(face: *mut hb_face_t);
    #[no_mangle]
    fn hb_face_set_index(face: *mut hb_face_t, index: libc::c_uint);
    #[no_mangle]
    fn hb_face_set_upem(face: *mut hb_face_t, upem: libc::c_uint);
    #[no_mangle]
    fn hb_font_funcs_create() -> *mut hb_font_funcs_t;
    #[no_mangle]
    fn hb_font_funcs_set_glyph_h_advance_func(ffuncs: *mut hb_font_funcs_t,
                                              func:
                                                  hb_font_get_glyph_h_advance_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_v_advance_func(ffuncs: *mut hb_font_funcs_t,
                                              func:
                                                  hb_font_get_glyph_v_advance_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_h_origin_func(ffuncs: *mut hb_font_funcs_t,
                                             func:
                                                 hb_font_get_glyph_h_origin_func_t,
                                             user_data: *mut libc::c_void,
                                             destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_v_origin_func(ffuncs: *mut hb_font_funcs_t,
                                             func:
                                                 hb_font_get_glyph_v_origin_func_t,
                                             user_data: *mut libc::c_void,
                                             destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_h_kerning_func(ffuncs: *mut hb_font_funcs_t,
                                              func:
                                                  hb_font_get_glyph_h_kerning_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_extents_func(ffuncs: *mut hb_font_funcs_t,
                                            func:
                                                hb_font_get_glyph_extents_func_t,
                                            user_data: *mut libc::c_void,
                                            destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_contour_point_func(ffuncs:
                                                      *mut hb_font_funcs_t,
                                                  func:
                                                      hb_font_get_glyph_contour_point_func_t,
                                                  user_data:
                                                      *mut libc::c_void,
                                                  destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_name_func(ffuncs: *mut hb_font_funcs_t,
                                         func: hb_font_get_glyph_name_func_t,
                                         user_data: *mut libc::c_void,
                                         destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_create(face: *mut hb_face_t) -> *mut hb_font_t;
    #[no_mangle]
    fn hb_font_destroy(font: *mut hb_font_t);
    #[no_mangle]
    fn hb_font_set_funcs(font: *mut hb_font_t, klass: *mut hb_font_funcs_t,
                         font_data: *mut libc::c_void,
                         destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_set_scale(font: *mut hb_font_t, x_scale: libc::c_int,
                         y_scale: libc::c_int);
    #[no_mangle]
    fn hb_font_set_ppem(font: *mut hb_font_t, x_ppem: libc::c_uint,
                        y_ppem: libc::c_uint);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_func(ffuncs: *mut hb_font_funcs_t,
                                    func: hb_font_get_glyph_func_t,
                                    user_data: *mut libc::c_void,
                                    destroy: hb_destroy_func_t);
    #[no_mangle]
    fn hb_font_funcs_set_glyph_v_kerning_func(ffuncs: *mut hb_font_funcs_t,
                                              func:
                                                  hb_font_get_glyph_v_kerning_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    #[no_mangle]
    fn xpc_debugger_api_misuse_info() -> *const libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    #[no_mangle]
    fn FT_Init_FreeType(alibrary: *mut FT_Library) -> FT_Error;
    #[no_mangle]
    fn XeTeXFontInst_unitsToPoints(self_0: *const XeTeXFontInst,
                                   units: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Fix2D(f: Fixed) -> libc::c_double;
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Glyph
   *
   * @description:
   *   A function used to extract a glyph image from a slot.  Note that the
   *   created @FT_Glyph object must be released with @FT_Done_Glyph.
   *
   * @input:
   *   slot ::
   *     A handle to the source glyph slot.
   *
   * @output:
   *   aglyph ::
   *     A handle to the glyph object.
   *
   * @return:
   *   FreeType error code.  0~means success.
   *
   * @note:
   *   Because `*aglyph->advance.x` and `*aglyph->advance.y` are 16.16
   *   fixed-point numbers, `slot->advance.x` and `slot->advance.y` (which
   *   are in 26.6 fixed-point format) must be in the range ]-32768;32768[.
   */
    #[no_mangle]
    fn FT_Get_Glyph(slot: FT_GlyphSlot, aglyph: *mut FT_Glyph) -> FT_Error;
    /* these constants are deprecated; use the corresponding */
  /* `FT_Glyph_BBox_Mode` values instead                   */
    /* *************************************************************************
   *
   * @function:
   *   FT_Glyph_Get_CBox
   *
   * @description:
   *   Return a glyph's 'control box'.  The control box encloses all the
   *   outline's points, including Bezier control points.  Though it
   *   coincides with the exact bounding box for most glyphs, it can be
   *   slightly larger in some situations (like when rotating an outline that
   *   contains Bezier outside arcs).
   *
   *   Computing the control box is very fast, while getting the bounding box
   *   can take much more time as it needs to walk over all segments and arcs
   *   in the outline.  To get the latter, you can use the 'ftbbox'
   *   component, which is dedicated to this single task.
   *
   * @input:
   *   glyph ::
   *     A handle to the source glyph object.
   *
   *   mode ::
   *     The mode that indicates how to interpret the returned bounding box
   *     values.
   *
   * @output:
   *   acbox ::
   *     The glyph coordinate bounding box.  Coordinates are expressed in
   *     1/64th of pixels if it is grid-fitted.
   *
   * @note:
   *   Coordinates are relative to the glyph origin, using the y~upwards
   *   convention.
   *
   *   If the glyph has been loaded with @FT_LOAD_NO_SCALE, `bbox_mode` must
   *   be set to @FT_GLYPH_BBOX_UNSCALED to get unscaled font units in 26.6
   *   pixel format.  The value @FT_GLYPH_BBOX_SUBPIXELS is another name for
   *   this constant.
   *
   *   If the font is tricky and the glyph has been loaded with
   *   @FT_LOAD_NO_SCALE, the resulting CBox is meaningless.  To get
   *   reasonable values for the CBox it is necessary to load the glyph at a
   *   large ppem value (so that the hinting instructions can properly shift
   *   and scale the subglyphs), then extracting the CBox, which can be
   *   eventually converted back to font units.
   *
   *   Note that the maximum coordinates are exclusive, which means that one
   *   can compute the width and height of the glyph image (be it in integer
   *   or 26.6 pixels) as:
   *
   *   ```
   *     width  = bbox.xMax - bbox.xMin;
   *     height = bbox.yMax - bbox.yMin;
   *   ```
   *
   *   Note also that for 26.6 coordinates, if `bbox_mode` is set to
   *   @FT_GLYPH_BBOX_GRIDFIT, the coordinates will also be grid-fitted,
   *   which corresponds to:
   *
   *   ```
   *     bbox.xMin = FLOOR(bbox.xMin);
   *     bbox.yMin = FLOOR(bbox.yMin);
   *     bbox.xMax = CEILING(bbox.xMax);
   *     bbox.yMax = CEILING(bbox.yMax);
   *   ```
   *
   *   To get the bbox in pixel coordinates, set `bbox_mode` to
   *   @FT_GLYPH_BBOX_TRUNCATE.
   *
   *   To get the bbox in grid-fitted pixel coordinates, set `bbox_mode` to
   *   @FT_GLYPH_BBOX_PIXELS.
   */
    #[no_mangle]
    fn FT_Glyph_Get_CBox(glyph: FT_Glyph, bbox_mode: FT_UInt,
                         acbox: *mut FT_BBox);
    /* *************************************************************************
   *
   * @function:
   *   FT_Done_Glyph
   *
   * @description:
   *   Destroy a given glyph.
   *
   * @input:
   *   glyph ::
   *     A handle to the target glyph object.
   */
    #[no_mangle]
    fn FT_Done_Glyph(glyph: FT_Glyph);
    /* ***************************************************************************
 *
 * ftadvanc.h
 *
 *   Quick computation of advance widths (specification only).
 *
 * Copyright (C) 2008-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
    /* *************************************************************************
   *
   * @section:
   *   quick_advance
   *
   * @title:
   *   Quick retrieval of advance values
   *
   * @abstract:
   *   Retrieve horizontal and vertical advance values without processing
   *   glyph outlines, if possible.
   *
   * @description:
   *   This section contains functions to quickly extract advance values
   *   without handling glyph outlines, if possible.
   *
   * @order:
   *   FT_Get_Advance
   *   FT_Get_Advances
   *
   */
    /* *************************************************************************
   *
   * @enum:
   *   FT_ADVANCE_FLAG_FAST_ONLY
   *
   * @description:
   *   A bit-flag to be OR-ed with the `flags` parameter of the
   *   @FT_Get_Advance and @FT_Get_Advances functions.
   *
   *   If set, it indicates that you want these functions to fail if the
   *   corresponding hinting mode or font driver doesn't allow for very quick
   *   advance computation.
   *
   *   Typically, glyphs that are either unscaled, unhinted, bitmapped, or
   *   light-hinted can have their advance width computed very quickly.
   *
   *   Normal and bytecode hinted modes that require loading, scaling, and
   *   hinting of the glyph outline, are extremely slow by comparison.
   */
    /* *************************************************************************
   *
   * @function:
   *   FT_Get_Advance
   *
   * @description:
   *   Retrieve the advance value of a given glyph outline in an @FT_Face.
   *
   * @input:
   *   face ::
   *     The source @FT_Face handle.
   *
   *   gindex ::
   *     The glyph index.
   *
   *   load_flags ::
   *     A set of bit flags similar to those used when calling
   *     @FT_Load_Glyph, used to determine what kind of advances you need.
   * @output:
   *   padvance ::
   *     The advance value.  If scaling is performed (based on the value of
   *     `load_flags`), the advance value is in 16.16 format.  Otherwise, it
   *     is in font units.
   *
   *     If @FT_LOAD_VERTICAL_LAYOUT is set, this is the vertical advance
   *     corresponding to a vertical layout.  Otherwise, it is the horizontal
   *     advance in a horizontal layout.
   *
   * @return:
   *   FreeType error code.  0 means success.
   *
   * @note:
   *   This function may fail if you use @FT_ADVANCE_FLAG_FAST_ONLY and if
   *   the corresponding font backend doesn't have a quick way to retrieve
   *   the advances.
   *
   *   A scaled advance is returned in 16.16 format but isn't transformed by
   *   the affine transformation specified by @FT_Set_Transform.
   */
    #[no_mangle]
    fn FT_Get_Advance(face: FT_Face, gindex: FT_UInt, load_flags: FT_Int32,
                      padvance: *mut FT_Fixed) -> FT_Error;
}
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type size_t = __darwin_size_t;
pub type int32_t = libc::c_int;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type ssize_t = __darwin_ssize_t;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
/* quasi-hack to get the primary input */
/* ***************************************************************************
 *
 * fttypes.h
 *
 *   FreeType simple types definitions (specification only).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
   *
   * @section:
   *   basic_types
   *
   * @title:
   *   Basic Data Types
   *
   * @abstract:
   *   The basic data types defined by the library.
   *
   * @description:
   *   This section contains the basic data types defined by FreeType~2,
   *   ranging from simple scalar types to bitmap descriptors.  More
   *   font-specific structures are defined in a different section.
   *
   * @order:
   *   FT_Byte
   *   FT_Bytes
   *   FT_Char
   *   FT_Int
   *   FT_UInt
   *   FT_Int16
   *   FT_UInt16
   *   FT_Int32
   *   FT_UInt32
   *   FT_Int64
   *   FT_UInt64
   *   FT_Short
   *   FT_UShort
   *   FT_Long
   *   FT_ULong
   *   FT_Bool
   *   FT_Offset
   *   FT_PtrDist
   *   FT_String
   *   FT_Tag
   *   FT_Error
   *   FT_Fixed
   *   FT_Pointer
   *   FT_Pos
   *   FT_Vector
   *   FT_BBox
   *   FT_Matrix
   *   FT_FWord
   *   FT_UFWord
   *   FT_F2Dot14
   *   FT_UnitVector
   *   FT_F26Dot6
   *   FT_Data
   *
   *   FT_MAKE_TAG
   *
   *   FT_Generic
   *   FT_Generic_Finalizer
   *
   *   FT_Bitmap
   *   FT_Pixel_Mode
   *   FT_Palette_Mode
   *   FT_Glyph_Format
   *   FT_IMAGE_TAG
   *
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Bool
   *
   * @description:
   *   A typedef of unsigned char, used for simple booleans.  As usual,
   *   values 1 and~0 represent true and false, respectively.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_FWord
   *
   * @description:
   *   A signed 16-bit integer used to store a distance in original font
   *   units.
   */
/* distance in FUnits */
/* *************************************************************************
   *
   * @type:
   *   FT_UFWord
   *
   * @description:
   *   An unsigned 16-bit integer used to store a distance in original font
   *   units.
   */
/* unsigned distance */
/* *************************************************************************
   *
   * @type:
   *   FT_Char
   *
   * @description:
   *   A simple typedef for the _signed_ char type.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Byte
   *
   * @description:
   *   A simple typedef for the _unsigned_ char type.
   */
pub type FT_Byte = libc::c_uchar;
pub type hb_destroy_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
/* ***************************************************************************
 *
 * tttables.h
 *
 *   Basic SFNT/TrueType tables definitions and interface
 *   (specification only).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
   *
   * @section:
   *   truetype_tables
   *
   * @title:
   *   TrueType Tables
   *
   * @abstract:
   *   TrueType-specific table types and functions.
   *
   * @description:
   *   This section contains definitions of some basic tables specific to
   *   TrueType and OpenType as well as some routines used to access and
   *   process them.
   *
   * @order:
   *   TT_Header
   *   TT_HoriHeader
   *   TT_VertHeader
   *   TT_OS2
   *   TT_Postscript
   *   TT_PCLT
   *   TT_MaxProfile
   *
   *   FT_Sfnt_Tag
   *   FT_Get_Sfnt_Table
   *   FT_Load_Sfnt_Table
   *   FT_Sfnt_Table_Info
   *
   *   FT_Get_CMap_Language_ID
   *   FT_Get_CMap_Format
   *
   *   FT_PARAM_TAG_UNPATENTED_HINTING
   *
   */
/* *************************************************************************
   *
   * @struct:
   *   TT_Header
   *
   * @description:
   *   A structure to model a TrueType font header table.  All fields follow
   *   the OpenType specification.  The 64-bit timestamps are stored in
   *   two-element arrays `Created` and `Modified`, first the upper then
   *   the lower 32~bits.
   */
/* *************************************************************************
   *
   * @struct:
   *   TT_HoriHeader
   *
   * @description:
   *   A structure to model a TrueType horizontal header, the 'hhea' table,
   *   as well as the corresponding horizontal metrics table, 'hmtx'.
   *
   * @fields:
   *   Version ::
   *     The table version.
   *
   *   Ascender ::
   *     The font's ascender, i.e., the distance from the baseline to the
   *     top-most of all glyph points found in the font.
   *
   *     This value is invalid in many fonts, as it is usually set by the
   *     font designer, and often reflects only a portion of the glyphs found
   *     in the font (maybe ASCII).
   *
   *     You should use the `sTypoAscender` field of the 'OS/2' table instead
   *     if you want the correct one.
   *
   *   Descender ::
   *     The font's descender, i.e., the distance from the baseline to the
   *     bottom-most of all glyph points found in the font.  It is negative.
   *
   *     This value is invalid in many fonts, as it is usually set by the
   *     font designer, and often reflects only a portion of the glyphs found
   *     in the font (maybe ASCII).
   *
   *     You should use the `sTypoDescender` field of the 'OS/2' table
   *     instead if you want the correct one.
   *
   *   Line_Gap ::
   *     The font's line gap, i.e., the distance to add to the ascender and
   *     descender to get the BTB, i.e., the baseline-to-baseline distance
   *     for the font.
   *
   *   advance_Width_Max ::
   *     This field is the maximum of all advance widths found in the font.
   *     It can be used to compute the maximum width of an arbitrary string
   *     of text.
   *
   *   min_Left_Side_Bearing ::
   *     The minimum left side bearing of all glyphs within the font.
   *
   *   min_Right_Side_Bearing ::
   *     The minimum right side bearing of all glyphs within the font.
   *
   *   xMax_Extent ::
   *     The maximum horizontal extent (i.e., the 'width' of a glyph's
   *     bounding box) for all glyphs in the font.
   *
   *   caret_Slope_Rise ::
   *     The rise coefficient of the cursor's slope of the cursor
   *     (slope=rise/run).
   *
   *   caret_Slope_Run ::
   *     The run coefficient of the cursor's slope.
   *
   *   caret_Offset ::
   *     The cursor's offset for slanted fonts.
   *
   *   Reserved ::
   *     8~reserved bytes.
   *
   *   metric_Data_Format ::
   *     Always~0.
   *
   *   number_Of_HMetrics ::
   *     Number of HMetrics entries in the 'hmtx' table -- this value can be
   *     smaller than the total number of glyphs in the font.
   *
   *   long_metrics ::
   *     A pointer into the 'hmtx' table.
   *
   *   short_metrics ::
   *     A pointer into the 'hmtx' table.
   *
   * @note:
   *   For an OpenType variation font, the values of the following fields can
   *   change after a call to @FT_Set_Var_Design_Coordinates (and friends) if
   *   the font contains an 'MVAR' table: `caret_Slope_Rise`,
   *   `caret_Slope_Run`, and `caret_Offset`.
   */
/* advance width maximum */
/* minimum left-sb       */
/* minimum right-sb      */
/* xmax extents          */
/* The following fields are not defined by the OpenType specification */
    /* but they are used to connect the metrics header to the relevant    */
    /* 'hmtx' table.                                                      */
/* *************************************************************************
   *
   * @struct:
   *   TT_VertHeader
   *
   * @description:
   *   A structure used to model a TrueType vertical header, the 'vhea'
   *   table, as well as the corresponding vertical metrics table, 'vmtx'.
   *
   * @fields:
   *   Version ::
   *     The table version.
   *
   *   Ascender ::
   *     The font's ascender, i.e., the distance from the baseline to the
   *     top-most of all glyph points found in the font.
   *
   *     This value is invalid in many fonts, as it is usually set by the
   *     font designer, and often reflects only a portion of the glyphs found
   *     in the font (maybe ASCII).
   *
   *     You should use the `sTypoAscender` field of the 'OS/2' table instead
   *     if you want the correct one.
   *
   *   Descender ::
   *     The font's descender, i.e., the distance from the baseline to the
   *     bottom-most of all glyph points found in the font.  It is negative.
   *
   *     This value is invalid in many fonts, as it is usually set by the
   *     font designer, and often reflects only a portion of the glyphs found
   *     in the font (maybe ASCII).
   *
   *     You should use the `sTypoDescender` field of the 'OS/2' table
   *     instead if you want the correct one.
   *
   *   Line_Gap ::
   *     The font's line gap, i.e., the distance to add to the ascender and
   *     descender to get the BTB, i.e., the baseline-to-baseline distance
   *     for the font.
   *
   *   advance_Height_Max ::
   *     This field is the maximum of all advance heights found in the font.
   *     It can be used to compute the maximum height of an arbitrary string
   *     of text.
   *
   *   min_Top_Side_Bearing ::
   *     The minimum top side bearing of all glyphs within the font.
   *
   *   min_Bottom_Side_Bearing ::
   *     The minimum bottom side bearing of all glyphs within the font.
   *
   *   yMax_Extent ::
   *     The maximum vertical extent (i.e., the 'height' of a glyph's
   *     bounding box) for all glyphs in the font.
   *
   *   caret_Slope_Rise ::
   *     The rise coefficient of the cursor's slope of the cursor
   *     (slope=rise/run).
   *
   *   caret_Slope_Run ::
   *     The run coefficient of the cursor's slope.
   *
   *   caret_Offset ::
   *     The cursor's offset for slanted fonts.
   *
   *   Reserved ::
   *     8~reserved bytes.
   *
   *   metric_Data_Format ::
   *     Always~0.
   *
   *   number_Of_VMetrics ::
   *     Number of VMetrics entries in the 'vmtx' table -- this value can be
   *     smaller than the total number of glyphs in the font.
   *
   *   long_metrics ::
   *     A pointer into the 'vmtx' table.
   *
   *   short_metrics ::
   *     A pointer into the 'vmtx' table.
   *
   * @note:
   *   For an OpenType variation font, the values of the following fields can
   *   change after a call to @FT_Set_Var_Design_Coordinates (and friends) if
   *   the font contains an 'MVAR' table: `Ascender`, `Descender`,
   *   `Line_Gap`, `caret_Slope_Rise`, `caret_Slope_Run`, and `caret_Offset`.
   */
/* advance height maximum */
/* minimum top-sb          */
/* minimum bottom-sb       */
/* ymax extents            */
/* The following fields are not defined by the OpenType specification */
    /* but they are used to connect the metrics header to the relevant    */
    /* 'vmtx' table.                                                      */
/* *************************************************************************
   *
   * @struct:
   *   TT_OS2
   *
   * @description:
   *   A structure to model a TrueType 'OS/2' table.  All fields comply to
   *   the OpenType specification.
   *
   *   Note that we now support old Mac fonts that do not include an 'OS/2'
   *   table.  In this case, the `version` field is always set to 0xFFFF.
   *
   * @note:
   *   For an OpenType variation font, the values of the following fields can
   *   change after a call to @FT_Set_Var_Design_Coordinates (and friends) if
   *   the font contains an 'MVAR' table: `sCapHeight`, `sTypoAscender`,
   *   `sTypoDescender`, `sTypoLineGap`, `sxHeight`, `usWinAscent`,
   *   `usWinDescent`, `yStrikeoutPosition`, `yStrikeoutSize`,
   *   `ySubscriptXOffset`, `ySubScriptXSize`, `ySubscriptYOffset`,
   *   `ySubscriptYSize`, `ySuperscriptXOffset`, `ySuperscriptXSize`,
   *   `ySuperscriptYOffset`, and `ySuperscriptYSize`.
   *
   *   Possible values for bits in the `ulUnicodeRangeX` fields are given by
   *   the @TT_UCR_XXX macros.
   */
/* 0x0001 - more or 0xFFFF */
/* Bits 0-31   */
/* Bits 32-63  */
/* Bits 64-95  */
/* Bits 96-127 */
/* only version 1 and higher: */
/* Bits 0-31   */
/* Bits 32-63  */
/* only version 2 and higher: */
/* only version 5 and higher: */
/* in twips (1/20th points) */
/* in twips (1/20th points) */
/* *************************************************************************
   *
   * @struct:
   *   TT_Postscript
   *
   * @description:
   *   A structure to model a TrueType 'post' table.  All fields comply to
   *   the OpenType specification.  This structure does not reference a
   *   font's PostScript glyph names; use @FT_Get_Glyph_Name to retrieve
   *   them.
   *
   * @note:
   *   For an OpenType variation font, the values of the following fields can
   *   change after a call to @FT_Set_Var_Design_Coordinates (and friends) if
   *   the font contains an 'MVAR' table: `underlinePosition` and
   *   `underlineThickness`.
   */
pub type TT_Postscript = TT_Postscript_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TT_Postscript_ {
    pub FormatType: FT_Fixed,
    pub italicAngle: FT_Fixed,
    pub underlinePosition: FT_Short,
    pub underlineThickness: FT_Short,
    pub isFixedPitch: FT_ULong,
    pub minMemType42: FT_ULong,
    pub maxMemType42: FT_ULong,
    pub minMemType1: FT_ULong,
    pub maxMemType1: FT_ULong,
}
/* Glyph names follow in the 'post' table, but we don't */
    /* load them by default.                                */
/* *************************************************************************
   *
   * @type:
   *   FT_Bytes
   *
   * @description:
   *   A typedef for constant memory areas.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Tag
   *
   * @description:
   *   A typedef for 32-bit tags (as used in the SFNT format).
   */
/* *************************************************************************
   *
   * @type:
   *   FT_String
   *
   * @description:
   *   A simple typedef for the char type, usually used for strings.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Short
   *
   * @description:
   *   A typedef for signed short.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_UShort
   *
   * @description:
   *   A typedef for unsigned short.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Int
   *
   * @description:
   *   A typedef for the int type.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_UInt
   *
   * @description:
   *   A typedef for the unsigned int type.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Long
   *
   * @description:
   *   A typedef for signed long.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_ULong
   *
   * @description:
   *   A typedef for unsigned long.
   */
pub type FT_ULong = libc::c_ulong;
pub type FT_Short = libc::c_short;
/* *************************************************************************
   *
   * @type:
   *   FT_F2Dot14
   *
   * @description:
   *   A signed 2.14 fixed-point type used for unit vectors.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_F26Dot6
   *
   * @description:
   *   A signed 26.6 fixed-point type used for vectorial pixel coordinates.
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Fixed
   *
   * @description:
   *   This type is used to store 16.16 fixed-point values, like scaling
   *   values or matrix coefficients.
   */
pub type FT_Fixed = libc::c_long;
pub type UniChar = UInt16;
pub type UInt16 = libc::c_ushort;
pub type Boolean = libc::c_uchar;
/* *
 * Define UChar32 as a type for single Unicode code points.
 * UChar32 is a signed 32-bit integer (same as int32_t).
 *
 * The Unicode code point range is 0..0x10ffff.
 * All other values (negative or >=0x110000) are illegal as Unicode code points.
 * They may be used as sentinel values to indicate "done", "error"
 * or similar non-code point conditions.
 *
 * Before ICU 2.4 (Jitterbug 2146), UChar32 was defined
 * to be wchar_t if that is 32 bits wide (wchar_t may be signed or unsigned)
 * or else to be uint32_t.
 * That is, the definition of UChar32 was platform-dependent.
 *
 * @see U_SENTINEL
 * @stable ICU 2.4
 */
pub type UChar32 = int32_t;
/* */
/* this #if 0 ... #endif clause is for documentation purposes */
pub type FT_Int32 = libc::c_int;
/* ***************************************************************************
 *
 * ftsystem.h
 *
 *   FreeType low-level system interface definition (specification).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
   *
   * @section:
   *  system_interface
   *
   * @title:
   *  System Interface
   *
   * @abstract:
   *  How FreeType manages memory and i/o.
   *
   * @description:
   *  This section contains various definitions related to memory management
   *  and i/o access.  You need to understand this information if you want to
   *  use a custom memory manager or you own i/o streams.
   *
   */
/* *************************************************************************
   *
   *                 M E M O R Y   M A N A G E M E N T
   *
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Memory
   *
   * @description:
   *   A handle to a given memory manager object, defined with an
   *   @FT_MemoryRec structure.
   *
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_MemoryRec_ {
    pub user: *mut libc::c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}
pub type FT_Realloc_Func
    =
    Option<unsafe extern "C" fn(_: FT_Memory, _: libc::c_long,
                                _: libc::c_long, _: *mut libc::c_void)
               -> *mut libc::c_void>;
pub type FT_Memory = *mut FT_MemoryRec_;
pub type FT_Free_Func
    =
    Option<unsafe extern "C" fn(_: FT_Memory, _: *mut libc::c_void) -> ()>;
pub type FT_Alloc_Func
    =
    Option<unsafe extern "C" fn(_: FT_Memory, _: libc::c_long)
               -> *mut libc::c_void>;
/* *************************************************************************
   *
   *                      I / O   M A N A G E M E N T
   *
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Stream
   *
   * @description:
   *   A handle to an input stream.
   *
   * @also:
   *   See @FT_StreamRec for the publicly accessible fields of a given stream
   *   object.
   *
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_StreamRec_ {
    pub base: *mut libc::c_uchar,
    pub size: libc::c_ulong,
    pub pos: libc::c_ulong,
    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,
    pub memory: FT_Memory,
    pub cursor: *mut libc::c_uchar,
    pub limit: *mut libc::c_uchar,
}
pub type FT_Stream_CloseFunc
    =
    Option<unsafe extern "C" fn(_: FT_Stream) -> ()>;
pub type FT_Stream = *mut FT_StreamRec_;
pub type FT_Stream_IoFunc
    =
    Option<unsafe extern "C" fn(_: FT_Stream, _: libc::c_ulong,
                                _: *mut libc::c_uchar, _: libc::c_ulong)
               -> libc::c_ulong>;
pub type FT_StreamDesc = FT_StreamDesc_;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union FT_StreamDesc_ {
    pub value: libc::c_long,
    pub pointer: *mut libc::c_void,
}
/* ***************************************************************************
 *
 * ftimage.h
 *
 *   FreeType glyph image formats and default raster interface
 *   (specification).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
   *
   * Note: A 'raster' is simply a scan-line converter, used to render
   *       FT_Outlines into FT_Bitmaps.
   *
   */
/* STANDALONE_ is from ftgrays.c */
/* *************************************************************************
   *
   * @section:
   *   basic_types
   *
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Pos
   *
   * @description:
   *   The type FT_Pos is used to store vectorial coordinates.  Depending on
   *   the context, these can represent distances in integer font units, or
   *   16.16, or 26.6 fixed-point pixel coordinates.
   */
pub type FT_Pos = libc::c_long;
/* *************************************************************************
   *
   * @struct:
   *   FT_Vector
   *
   * @description:
   *   A simple structure used to store a 2D vector; coordinates are of the
   *   FT_Pos type.
   *
   * @fields:
   *   x ::
   *     The horizontal coordinate.
   *   y ::
   *     The vertical coordinate.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Vector_ {
    pub x: FT_Pos,
    pub y: FT_Pos,
}
pub type FT_Vector = FT_Vector_;
/* *************************************************************************
   *
   * @struct:
   *   FT_BBox
   *
   * @description:
   *   A structure used to hold an outline's bounding box, i.e., the
   *   coordinates of its extrema in the horizontal and vertical directions.
   *
   * @fields:
   *   xMin ::
   *     The horizontal minimum (left-most).
   *
   *   yMin ::
   *     The vertical minimum (bottom-most).
   *
   *   xMax ::
   *     The horizontal maximum (right-most).
   *
   *   yMax ::
   *     The vertical maximum (top-most).
   *
   * @note:
   *   The bounding box is specified with the coordinates of the lower left
   *   and the upper right corner.  In PostScript, those values are often
   *   called (llx,lly) and (urx,ury), respectively.
   *
   *   If `yMin` is negative, this value gives the glyph's descender.
   *   Otherwise, the glyph doesn't descend below the baseline.  Similarly,
   *   if `ymax` is positive, this value gives the glyph's ascender.
   *
   *   `xMin` gives the horizontal distance from the glyph's origin to the
   *   left edge of the glyph's bounding box.  If `xMin` is negative, the
   *   glyph extends to the left of the origin.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_BBox_ {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}
pub type FT_BBox = FT_BBox_;
/* these constants are deprecated; use the corresponding `FT_Pixel_Mode` */
  /* values instead.                                                       */
/* *************************************************************************
   *
   * @struct:
   *   FT_Bitmap
   *
   * @description:
   *   A structure used to describe a bitmap or pixmap to the raster.  Note
   *   that we now manage pixmaps of various depths through the `pixel_mode`
   *   field.
   *
   * @fields:
   *   rows ::
   *     The number of bitmap rows.
   *
   *   width ::
   *     The number of pixels in bitmap row.
   *
   *   pitch ::
   *     The pitch's absolute value is the number of bytes taken by one
   *     bitmap row, including padding.  However, the pitch is positive when
   *     the bitmap has a 'down' flow, and negative when it has an 'up' flow.
   *     In all cases, the pitch is an offset to add to a bitmap pointer in
   *     order to go down one row.
   *
   *     Note that 'padding' means the alignment of a bitmap to a byte
   *     border, and FreeType functions normally align to the smallest
   *     possible integer value.
   *
   *     For the B/W rasterizer, `pitch` is always an even number.
   *
   *     To change the pitch of a bitmap (say, to make it a multiple of 4),
   *     use @FT_Bitmap_Convert.  Alternatively, you might use callback
   *     functions to directly render to the application's surface; see the
   *     file `example2.cpp` in the tutorial for a demonstration.
   *
   *   buffer ::
   *     A typeless pointer to the bitmap buffer.  This value should be
   *     aligned on 32-bit boundaries in most cases.
   *
   *   num_grays ::
   *     This field is only used with @FT_PIXEL_MODE_GRAY; it gives the
   *     number of gray levels used in the bitmap.
   *
   *   pixel_mode ::
   *     The pixel mode, i.e., how pixel bits are stored.  See @FT_Pixel_Mode
   *     for possible values.
   *
   *   palette_mode ::
   *     This field is intended for paletted pixel modes; it indicates how
   *     the palette is stored.  Not used currently.
   *
   *   palette ::
   *     A typeless pointer to the bitmap palette; this field is intended for
   *     paletted pixel modes.  Not used currently.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Bitmap_ {
    pub rows: libc::c_uint,
    pub width: libc::c_uint,
    pub pitch: libc::c_int,
    pub buffer: *mut libc::c_uchar,
    pub num_grays: libc::c_ushort,
    pub pixel_mode: libc::c_uchar,
    pub palette_mode: libc::c_uchar,
    pub palette: *mut libc::c_void,
}
pub type FT_Bitmap = FT_Bitmap_;
/* *************************************************************************
   *
   * @section:
   *   outline_processing
   *
   */
/* *************************************************************************
   *
   * @struct:
   *   FT_Outline
   *
   * @description:
   *   This structure is used to describe an outline to the scan-line
   *   converter.
   *
   * @fields:
   *   n_contours ::
   *     The number of contours in the outline.
   *
   *   n_points ::
   *     The number of points in the outline.
   *
   *   points ::
   *     A pointer to an array of `n_points` @FT_Vector elements, giving the
   *     outline's point coordinates.
   *
   *   tags ::
   *     A pointer to an array of `n_points` chars, giving each outline
   *     point's type.
   *
   *     If bit~0 is unset, the point is 'off' the curve, i.e., a Bezier
   *     control point, while it is 'on' if set.
   *
   *     Bit~1 is meaningful for 'off' points only.  If set, it indicates a
   *     third-order Bezier arc control point; and a second-order control
   *     point if unset.
   *
   *     If bit~2 is set, bits 5-7 contain the drop-out mode (as defined in
   *     the OpenType specification; the value is the same as the argument to
   *     the 'SCANMODE' instruction).
   *
   *     Bits 3 and~4 are reserved for internal purposes.
   *
   *   contours ::
   *     An array of `n_contours` shorts, giving the end point of each
   *     contour within the outline.  For example, the first contour is
   *     defined by the points '0' to `contours[0]`, the second one is
   *     defined by the points `contours[0]+1` to `contours[1]`, etc.
   *
   *   flags ::
   *     A set of bit flags used to characterize the outline and give hints
   *     to the scan-converter and hinter on how to convert/grid-fit it.  See
   *     @FT_OUTLINE_XXX.
   *
   * @note:
   *   The B/W rasterizer only checks bit~2 in the `tags` array for the first
   *   point of each contour.  The drop-out mode as given with
   *   @FT_OUTLINE_IGNORE_DROPOUTS, @FT_OUTLINE_SMART_DROPOUTS, and
   *   @FT_OUTLINE_INCLUDE_STUBS in `flags` is then overridden.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Outline_ {
    pub n_contours: libc::c_short,
    pub n_points: libc::c_short,
    pub points: *mut FT_Vector,
    pub tags: *mut libc::c_char,
    pub contours: *mut libc::c_short,
    pub flags: libc::c_int,
    /* outline masks                      */
}
pub type FT_Outline = FT_Outline_;
/* *************************************************************************
   *
   * @section:
   *   basic_types
   *
   */
/* *************************************************************************
   *
   * @macro:
   *   FT_IMAGE_TAG
   *
   * @description:
   *   This macro converts four-letter tags to an unsigned long type.
   *
   * @note:
   *   Since many 16-bit compilers don't like 32-bit enumerations, you should
   *   redefine this macro in case of problems to something like this:
   *
   *   ```
   *     #define FT_IMAGE_TAG( value, _x1, _x2, _x3, _x4 )  value
   *   ```
   *
   *   to get a simple enumeration without assigning special numbers.
   */
/* FT_IMAGE_TAG */
/* *************************************************************************
   *
   * @enum:
   *   FT_Glyph_Format
   *
   * @description:
   *   An enumeration type used to describe the format of a given glyph
   *   image.  Note that this version of FreeType only supports two image
   *   formats, even though future font drivers will be able to register
   *   their own format.
   *
   * @values:
   *   FT_GLYPH_FORMAT_NONE ::
   *     The value~0 is reserved.
   *
   *   FT_GLYPH_FORMAT_COMPOSITE ::
   *     The glyph image is a composite of several other images.  This format
   *     is _only_ used with @FT_LOAD_NO_RECURSE, and is used to report
   *     compound glyphs (like accented characters).
   *
   *   FT_GLYPH_FORMAT_BITMAP ::
   *     The glyph image is a bitmap, and can be described as an @FT_Bitmap.
   *     You generally need to access the `bitmap` field of the
   *     @FT_GlyphSlotRec structure to read it.
   *
   *   FT_GLYPH_FORMAT_OUTLINE ::
   *     The glyph image is a vectorial outline made of line segments and
   *     Bezier arcs; it can be described as an @FT_Outline; you generally
   *     want to access the `outline` field of the @FT_GlyphSlotRec structure
   *     to read it.
   *
   *   FT_GLYPH_FORMAT_PLOTTER ::
   *     The glyph image is a vectorial path with no inside and outside
   *     contours.  Some Type~1 fonts, like those in the Hershey family,
   *     contain glyphs in this format.  These are described as @FT_Outline,
   *     but FreeType isn't currently capable of rendering them correctly.
   */
pub type FT_Glyph_Format_ = libc::c_uint;
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format_ = 1886154612;
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format_ = 1869968492;
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format_ = 1651078259;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format_ = 1668246896;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format_ = 0;
pub type FT_Glyph_Format = FT_Glyph_Format_;
pub type FT_Char = libc::c_schar;
pub type FT_String = libc::c_char;
pub type FT_UShort = libc::c_ushort;
pub type FT_Int = libc::c_int;
pub type FT_UInt = libc::c_uint;
pub type FT_Long = libc::c_long;
/* *************************************************************************
   *
   * @type:
   *   FT_Error
   *
   * @description:
   *   The FreeType error code type.  A value of~0 is always interpreted as a
   *   successful operation.
   */
pub type FT_Error = libc::c_int;
/* *************************************************************************
   *
   * @type:
   *   FT_Pointer
   *
   * @description:
   *   A simple typedef for a typeless pointer.
   */
pub type FT_Pointer = *mut libc::c_void;
/* *************************************************************************
   *
   * @functype:
   *   FT_Generic_Finalizer
   *
   * @description:
   *   Describe a function used to destroy the 'client' data of any FreeType
   *   object.  See the description of the @FT_Generic type for details of
   *   usage.
   *
   * @input:
   *   The address of the FreeType object that is under finalization.  Its
   *   client data is accessed through its `generic` field.
   */
pub type FT_Generic_Finalizer
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
/* *************************************************************************
   *
   * @struct:
   *   FT_Generic
   *
   * @description:
   *   Client applications often need to associate their own data to a
   *   variety of FreeType core objects.  For example, a text layout API
   *   might want to associate a glyph cache to a given size object.
   *
   *   Some FreeType object contains a `generic` field, of type `FT_Generic`,
   *   which usage is left to client applications and font servers.
   *
   *   It can be used to store a pointer to client-specific data, as well as
   *   the address of a 'finalizer' function, which will be called by
   *   FreeType when the object is destroyed (for example, the previous
   *   client example would put the address of the glyph cache destructor in
   *   the `finalizer` field).
   *
   * @fields:
   *   data ::
   *     A typeless pointer to any client-specified data. This field is
   *     completely ignored by the FreeType library.
   *
   *   finalizer ::
   *     A pointer to a 'generic finalizer' function, which will be called
   *     when the object is destroyed.  If this field is set to `NULL`, no
   *     code will be called.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Generic_ {
    pub data: *mut libc::c_void,
    pub finalizer: FT_Generic_Finalizer,
}
pub type FT_Generic = FT_Generic_;
/* *************************************************************************
   *
   * @macro:
   *   FT_MAKE_TAG
   *
   * @description:
   *   This macro converts four-letter tags that are used to label TrueType
   *   tables into an unsigned long, to be used within FreeType.
   *
   * @note:
   *   The produced values **must** be 32-bit integers.  Don't redefine this
   *   macro.
   */
/* ************************************************************************/
  /* ************************************************************************/
  /*                                                                       */
  /*                    L I S T   M A N A G E M E N T                      */
  /*                                                                       */
  /* ************************************************************************/
  /* ************************************************************************/
/* *************************************************************************
   *
   * @section:
   *   list_processing
   *
   */
/* *************************************************************************
   *
   * @type:
   *   FT_ListNode
   *
   * @description:
   *    Many elements and objects in FreeType are listed through an @FT_List
   *    record (see @FT_ListRec).  As its name suggests, an FT_ListNode is a
   *    handle to a single list element.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_ListNodeRec_ {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *mut libc::c_void,
}
pub type FT_ListNode = *mut FT_ListNodeRec_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_ListRec_ {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}
pub type FT_ListRec = FT_ListRec_;
/* ***************************************************************************
 *
 * freetype.h
 *
 *   FreeType high-level API and common types (specification only).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
   *
   * @section:
   *   header_inclusion
   *
   * @title:
   *   FreeType's header inclusion scheme
   *
   * @abstract:
   *   How client applications should include FreeType header files.
   *
   * @description:
   *   To be as flexible as possible (and for historical reasons), FreeType
   *   uses a very special inclusion scheme to load header files, for example
   *
   *   ```
   *     #include <ft2build.h>
   *
   *     #include FT_FREETYPE_H
   *     #include FT_OUTLINE_H
   *   ```
   *
   *   A compiler and its preprocessor only needs an include path to find the
   *   file `ft2build.h`; the exact locations and names of the other FreeType
   *   header files are hidden by @header_file_macros, loaded by
   *   `ft2build.h`.  The API documentation always gives the header macro
   *   name needed for a particular function.
   *
   */
/* *************************************************************************
   *
   * @section:
   *   user_allocation
   *
   * @title:
   *   User allocation
   *
   * @abstract:
   *   How client applications should allocate FreeType data structures.
   *
   * @description:
   *   FreeType assumes that structures allocated by the user and passed as
   *   arguments are zeroed out except for the actual data.  In other words,
   *   it is recommended to use `calloc` (or variants of it) instead of
   *   `malloc` for allocation.
   *
   */
/* ************************************************************************/
  /* ************************************************************************/
  /*                                                                       */
  /*                        B A S I C   T Y P E S                          */
  /*                                                                       */
  /* ************************************************************************/
  /* ************************************************************************/
/* *************************************************************************
   *
   * @section:
   *   base_interface
   *
   * @title:
   *   Base Interface
   *
   * @abstract:
   *   The FreeType~2 base font interface.
   *
   * @description:
   *   This section describes the most important public high-level API
   *   functions of FreeType~2.
   *
   * @order:
   *   FT_Library
   *   FT_Face
   *   FT_Size
   *   FT_GlyphSlot
   *   FT_CharMap
   *   FT_Encoding
   *   FT_ENC_TAG
   *
   *   FT_FaceRec
   *
   *   FT_FACE_FLAG_SCALABLE
   *   FT_FACE_FLAG_FIXED_SIZES
   *   FT_FACE_FLAG_FIXED_WIDTH
   *   FT_FACE_FLAG_HORIZONTAL
   *   FT_FACE_FLAG_VERTICAL
   *   FT_FACE_FLAG_COLOR
   *   FT_FACE_FLAG_SFNT
   *   FT_FACE_FLAG_CID_KEYED
   *   FT_FACE_FLAG_TRICKY
   *   FT_FACE_FLAG_KERNING
   *   FT_FACE_FLAG_MULTIPLE_MASTERS
   *   FT_FACE_FLAG_VARIATION
   *   FT_FACE_FLAG_GLYPH_NAMES
   *   FT_FACE_FLAG_EXTERNAL_STREAM
   *   FT_FACE_FLAG_HINTER
   *
   *   FT_HAS_HORIZONTAL
   *   FT_HAS_VERTICAL
   *   FT_HAS_KERNING
   *   FT_HAS_FIXED_SIZES
   *   FT_HAS_GLYPH_NAMES
   *   FT_HAS_COLOR
   *   FT_HAS_MULTIPLE_MASTERS
   *
   *   FT_IS_SFNT
   *   FT_IS_SCALABLE
   *   FT_IS_FIXED_WIDTH
   *   FT_IS_CID_KEYED
   *   FT_IS_TRICKY
   *   FT_IS_NAMED_INSTANCE
   *   FT_IS_VARIATION
   *
   *   FT_STYLE_FLAG_BOLD
   *   FT_STYLE_FLAG_ITALIC
   *
   *   FT_SizeRec
   *   FT_Size_Metrics
   *
   *   FT_GlyphSlotRec
   *   FT_Glyph_Metrics
   *   FT_SubGlyph
   *
   *   FT_Bitmap_Size
   *
   *   FT_Init_FreeType
   *   FT_Done_FreeType
   *
   *   FT_New_Face
   *   FT_Done_Face
   *   FT_Reference_Face
   *   FT_New_Memory_Face
   *   FT_Face_Properties
   *   FT_Open_Face
   *   FT_Open_Args
   *   FT_Parameter
   *   FT_Attach_File
   *   FT_Attach_Stream
   *
   *   FT_Set_Char_Size
   *   FT_Set_Pixel_Sizes
   *   FT_Request_Size
   *   FT_Select_Size
   *   FT_Size_Request_Type
   *   FT_Size_RequestRec
   *   FT_Size_Request
   *   FT_Set_Transform
   *   FT_Load_Glyph
   *   FT_Get_Char_Index
   *   FT_Get_First_Char
   *   FT_Get_Next_Char
   *   FT_Get_Name_Index
   *   FT_Load_Char
   *
   *   FT_OPEN_MEMORY
   *   FT_OPEN_STREAM
   *   FT_OPEN_PATHNAME
   *   FT_OPEN_DRIVER
   *   FT_OPEN_PARAMS
   *
   *   FT_LOAD_DEFAULT
   *   FT_LOAD_RENDER
   *   FT_LOAD_MONOCHROME
   *   FT_LOAD_LINEAR_DESIGN
   *   FT_LOAD_NO_SCALE
   *   FT_LOAD_NO_HINTING
   *   FT_LOAD_NO_BITMAP
   *   FT_LOAD_NO_AUTOHINT
   *   FT_LOAD_COLOR
   *
   *   FT_LOAD_VERTICAL_LAYOUT
   *   FT_LOAD_IGNORE_TRANSFORM
   *   FT_LOAD_FORCE_AUTOHINT
   *   FT_LOAD_NO_RECURSE
   *   FT_LOAD_PEDANTIC
   *
   *   FT_LOAD_TARGET_NORMAL
   *   FT_LOAD_TARGET_LIGHT
   *   FT_LOAD_TARGET_MONO
   *   FT_LOAD_TARGET_LCD
   *   FT_LOAD_TARGET_LCD_V
   *
   *   FT_LOAD_TARGET_MODE
   *
   *   FT_Render_Glyph
   *   FT_Render_Mode
   *   FT_Get_Kerning
   *   FT_Kerning_Mode
   *   FT_Get_Track_Kerning
   *   FT_Get_Glyph_Name
   *   FT_Get_Postscript_Name
   *
   *   FT_CharMapRec
   *   FT_Select_Charmap
   *   FT_Set_Charmap
   *   FT_Get_Charmap_Index
   *
   *   FT_Get_FSType_Flags
   *   FT_Get_SubGlyph_Info
   *
   *   FT_Face_Internal
   *   FT_Size_Internal
   *   FT_Slot_Internal
   *
   *   FT_FACE_FLAG_XXX
   *   FT_STYLE_FLAG_XXX
   *   FT_OPEN_XXX
   *   FT_LOAD_XXX
   *   FT_LOAD_TARGET_XXX
   *   FT_SUBGLYPH_FLAG_XXX
   *   FT_FSTYPE_XXX
   *
   *   FT_HAS_FAST_GLYPHS
   *
   */
/* *************************************************************************
   *
   * @struct:
   *   FT_Glyph_Metrics
   *
   * @description:
   *   A structure to model the metrics of a single glyph.  The values are
   *   expressed in 26.6 fractional pixel format; if the flag
   *   @FT_LOAD_NO_SCALE has been used while loading the glyph, values are
   *   expressed in font units instead.
   *
   * @fields:
   *   width ::
   *     The glyph's width.
   *
   *   height ::
   *     The glyph's height.
   *
   *   horiBearingX ::
   *     Left side bearing for horizontal layout.
   *
   *   horiBearingY ::
   *     Top side bearing for horizontal layout.
   *
   *   horiAdvance ::
   *     Advance width for horizontal layout.
   *
   *   vertBearingX ::
   *     Left side bearing for vertical layout.
   *
   *   vertBearingY ::
   *     Top side bearing for vertical layout.  Larger positive values mean
   *     further below the vertical glyph origin.
   *
   *   vertAdvance ::
   *     Advance height for vertical layout.  Positive values mean the glyph
   *     has a positive advance downward.
   *
   * @note:
   *   If not disabled with @FT_LOAD_NO_HINTING, the values represent
   *   dimensions of the hinted glyph (in case hinting is applicable).
   *
   *   Stroking a glyph with an outside border does not increase
   *   `horiAdvance` or `vertAdvance`; you have to manually adjust these
   *   values to account for the added width and height.
   *
   *   FreeType doesn't use the 'VORG' table data for CFF fonts because it
   *   doesn't have an interface to quickly retrieve the glyph height.  The
   *   y~coordinate of the vertical origin can be simply computed as
   *   `vertBearingY + height` after loading a glyph.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Glyph_Metrics_ {
    pub width: FT_Pos,
    pub height: FT_Pos,
    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,
    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}
pub type FT_Glyph_Metrics = FT_Glyph_Metrics_;
/* *************************************************************************
   *
   * @struct:
   *   FT_Bitmap_Size
   *
   * @description:
   *   This structure models the metrics of a bitmap strike (i.e., a set of
   *   glyphs for a given point size and resolution) in a bitmap font.  It is
   *   used for the `available_sizes` field of @FT_Face.
   *
   * @fields:
   *   height ::
   *     The vertical distance, in pixels, between two consecutive baselines.
   *     It is always positive.
   *
   *   width ::
   *     The average width, in pixels, of all glyphs in the strike.
   *
   *   size ::
   *     The nominal size of the strike in 26.6 fractional points.  This
   *     field is not very useful.
   *
   *   x_ppem ::
   *     The horizontal ppem (nominal width) in 26.6 fractional pixels.
   *
   *   y_ppem ::
   *     The vertical ppem (nominal height) in 26.6 fractional pixels.
   *
   * @note:
   *   Windows FNT:
   *     The nominal size given in a FNT font is not reliable.  If the driver
   *     finds it incorrect, it sets `size` to some calculated values, and
   *     `x_ppem` and `y_ppem` to the pixel width and height given in the
   *     font, respectively.
   *
   *   TrueType embedded bitmaps:
   *     `size`, `width`, and `height` values are not contained in the bitmap
   *     strike itself.  They are computed from the global font parameters.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Bitmap_Size_ {
    pub height: FT_Short,
    pub width: FT_Short,
    pub size: FT_Pos,
    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}
pub type FT_Bitmap_Size = FT_Bitmap_Size_;
pub type FT_Library = *mut FT_LibraryRec_;
pub type FT_Module = *mut FT_ModuleRec_;
pub type FT_Driver = *mut FT_DriverRec_;
/* *************************************************************************
   *
   * @section:
   *   base_interface
   *
   */
/* *************************************************************************
   *
   * @type:
   *   FT_Face
   *
   * @description:
   *   A handle to a typographic face object.  A face object models a given
   *   typeface, in a given style.
   *
   * @note:
   *   A face object also owns a single @FT_GlyphSlot object, as well as one
   *   or more @FT_Size objects.
   *
   *   Use @FT_New_Face or @FT_Open_Face to create a new face object from a
   *   given filepath or a custom input stream.
   *
   *   Use @FT_Done_Face to destroy it (along with its slot and sizes).
   *
   *   An `FT_Face` object can only be safely used from one thread at a time.
   *   Similarly, creation and destruction of `FT_Face` with the same
   *   @FT_Library object can only be done from one thread at a time.  On the
   *   other hand, functions like @FT_Load_Glyph and its siblings are
   *   thread-safe and do not need the lock to be held as long as the same
   *   `FT_Face` object is not used from multiple threads at the same time.
   *
   * @also:
   *   See @FT_FaceRec for the publicly accessible fields of a given face
   *   object.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_FaceRec_ {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,
    pub face_flags: FT_Long,
    pub style_flags: FT_Long,
    pub num_glyphs: FT_Long,
    pub family_name: *mut FT_String,
    pub style_name: *mut FT_String,
    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *mut FT_Bitmap_Size,
    pub num_charmaps: FT_Int,
    pub charmaps: *mut FT_CharMap,
    pub generic: FT_Generic,
    pub bbox: FT_BBox,
    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,
    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,
    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,
    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,
    pub sizes_list: FT_ListRec,
    pub autohint: FT_Generic,
    pub extensions: *mut libc::c_void,
    pub internal: FT_Face_Internal,
}
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
pub type FT_CharMap = *mut FT_CharMapRec_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_CharMapRec_ {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}
pub type FT_Encoding = FT_Encoding_;
pub type FT_Encoding_ = libc::c_uint;
pub const FT_ENCODING_APPLE_ROMAN: FT_Encoding_ = 1634889070;
pub const FT_ENCODING_OLD_LATIN_2: FT_Encoding_ = 1818326066;
pub const FT_ENCODING_ADOBE_LATIN_1: FT_Encoding_ = 1818326065;
pub const FT_ENCODING_ADOBE_CUSTOM: FT_Encoding_ = 1094992451;
pub const FT_ENCODING_ADOBE_EXPERT: FT_Encoding_ = 1094992453;
pub const FT_ENCODING_ADOBE_STANDARD: FT_Encoding_ = 1094995778;
pub const FT_ENCODING_MS_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_MS_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_MS_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_MS_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_MS_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_PRC: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_UNICODE: FT_Encoding_ = 1970170211;
pub const FT_ENCODING_MS_SYMBOL: FT_Encoding_ = 1937337698;
pub const FT_ENCODING_NONE: FT_Encoding_ = 0;
pub type FT_Face = *mut FT_FaceRec_;
pub type FT_Size = *mut FT_SizeRec_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_SizeRec_ {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}
pub type FT_Size_Internal = *mut FT_Size_InternalRec_;
pub type FT_Size_Metrics = FT_Size_Metrics_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Size_Metrics_ {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,
    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,
    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos,
}
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_GlyphSlotRec_ {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub glyph_index: FT_UInt,
    pub generic: FT_Generic,
    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,
    pub format: FT_Glyph_Format,
    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,
    pub outline: FT_Outline,
    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,
    pub control_data: *mut libc::c_void,
    pub control_len: libc::c_long,
    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,
    pub other: *mut libc::c_void,
    pub internal: FT_Slot_Internal,
}
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec_;
pub type FT_SubGlyph = *mut FT_SubGlyphRec_;
/* *************************************************************************
   *
   * @enum:
   *   FT_OPEN_XXX
   *
   * @description:
   *   A list of bit field constants used within the `flags` field of the
   *   @FT_Open_Args structure.
   *
   * @values:
   *   FT_OPEN_MEMORY ::
   *     This is a memory-based stream.
   *
   *   FT_OPEN_STREAM ::
   *     Copy the stream from the `stream` field.
   *
   *   FT_OPEN_PATHNAME ::
   *     Create a new input stream from a C~path name.
   *
   *   FT_OPEN_DRIVER ::
   *     Use the `driver` field.
   *
   *   FT_OPEN_PARAMS ::
   *     Use the `num_params` and `params` fields.
   *
   * @note:
   *   The `FT_OPEN_MEMORY`, `FT_OPEN_STREAM`, and `FT_OPEN_PATHNAME` flags
   *   are mutually exclusive.
   */
/* these constants are deprecated; use the corresponding `FT_OPEN_XXX` */
  /* values instead                                                      */
/* *************************************************************************
   *
   * @struct:
   *   FT_Parameter
   *
   * @description:
   *   A simple structure to pass more or less generic parameters to
   *   @FT_Open_Face and @FT_Face_Properties.
   *
   * @fields:
   *   tag ::
   *     A four-byte identification tag.
   *
   *   data ::
   *     A pointer to the parameter data.
   *
   * @note:
   *   The ID and function of parameters are driver-specific.  See section
   *   @parameter_tags for more information.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Parameter_ {
    pub tag: FT_ULong,
    pub data: FT_Pointer,
}
pub type FT_Parameter = FT_Parameter_;
/* *************************************************************************
   *
   * @struct:
   *   FT_Open_Args
   *
   * @description:
   *   A structure to indicate how to open a new font file or stream.  A
   *   pointer to such a structure can be used as a parameter for the
   *   functions @FT_Open_Face and @FT_Attach_Stream.
   *
   * @fields:
   *   flags ::
   *     A set of bit flags indicating how to use the structure.
   *
   *   memory_base ::
   *     The first byte of the file in memory.
   *
   *   memory_size ::
   *     The size in bytes of the file in memory.
   *
   *   pathname ::
   *     A pointer to an 8-bit file pathname.
   *
   *   stream ::
   *     A handle to a source stream object.
   *
   *   driver ::
   *     This field is exclusively used by @FT_Open_Face; it simply specifies
   *     the font driver to use for opening the face.  If set to `NULL`,
   *     FreeType tries to load the face with each one of the drivers in its
   *     list.
   *
   *   num_params ::
   *     The number of extra parameters.
   *
   *   params ::
   *     Extra parameters passed to the font driver when opening a new face.
   *
   * @note:
   *   The stream type is determined by the contents of `flags` that are
   *   tested in the following order by @FT_Open_Face:
   *
   *   If the @FT_OPEN_MEMORY bit is set, assume that this is a memory file
   *   of `memory_size` bytes, located at `memory_address`.  The data are not
   *   copied, and the client is responsible for releasing and destroying
   *   them _after_ the corresponding call to @FT_Done_Face.
   *
   *   Otherwise, if the @FT_OPEN_STREAM bit is set, assume that a custom
   *   input stream `stream` is used.
   *
   *   Otherwise, if the @FT_OPEN_PATHNAME bit is set, assume that this is a
   *   normal file and use `pathname` to open it.
   *
   *   If the @FT_OPEN_DRIVER bit is set, @FT_Open_Face only tries to open
   *   the file with the driver whose handler is in `driver`.
   *
   *   If the @FT_OPEN_PARAMS bit is set, the parameters given by
   *   `num_params` and `params` is used.  They are ignored otherwise.
   *
   *   Ideally, both the `pathname` and `params` fields should be tagged as
   *   'const'; this is missing for API backward compatibility.  In other
   *   words, applications should treat them as read-only.
   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_Open_Args_ {
    pub flags: FT_UInt,
    pub memory_base: *const FT_Byte,
    pub memory_size: FT_Long,
    pub pathname: *mut FT_String,
    pub stream: FT_Stream,
    pub driver: FT_Module,
    pub num_params: FT_Int,
    pub params: *mut FT_Parameter,
}
pub type FT_Open_Args = FT_Open_Args_;
/* *************************************************************************
   *
   * @enum:
   *   FT_Kerning_Mode
   *
   * @description:
   *   An enumeration to specify the format of kerning values returned by
   *   @FT_Get_Kerning.
   *
   * @values:
   *   FT_KERNING_DEFAULT ::
   *     Return grid-fitted kerning distances in 26.6 fractional pixels.
   *
   *   FT_KERNING_UNFITTED ::
   *     Return un-grid-fitted kerning distances in 26.6 fractional pixels.
   *
   *   FT_KERNING_UNSCALED ::
   *     Return the kerning vector in original font units.
   *
   * @note:
   *   `FT_KERNING_DEFAULT` returns full pixel values; it also makes FreeType
   *   heuristically scale down kerning distances at small ppem values so
   *   that they don't become too big.
   *
   *   Both `FT_KERNING_DEFAULT` and `FT_KERNING_UNFITTED` use the current
   *   horizontal scaling factor (as set e.g. with @FT_Set_Char_Size) to
   *   convert font units to pixels.
   */
pub type FT_Kerning_Mode_ = libc::c_uint;
pub const FT_KERNING_UNSCALED: FT_Kerning_Mode_ = 2;
pub const FT_KERNING_UNFITTED: FT_Kerning_Mode_ = 1;
pub const FT_KERNING_DEFAULT: FT_Kerning_Mode_ = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TT_OS2_ {
    pub version: FT_UShort,
    pub xAvgCharWidth: FT_Short,
    pub usWeightClass: FT_UShort,
    pub usWidthClass: FT_UShort,
    pub fsType: FT_UShort,
    pub ySubscriptXSize: FT_Short,
    pub ySubscriptYSize: FT_Short,
    pub ySubscriptXOffset: FT_Short,
    pub ySubscriptYOffset: FT_Short,
    pub ySuperscriptXSize: FT_Short,
    pub ySuperscriptYSize: FT_Short,
    pub ySuperscriptXOffset: FT_Short,
    pub ySuperscriptYOffset: FT_Short,
    pub yStrikeoutSize: FT_Short,
    pub yStrikeoutPosition: FT_Short,
    pub sFamilyClass: FT_Short,
    pub panose: [FT_Byte; 10],
    pub ulUnicodeRange1: FT_ULong,
    pub ulUnicodeRange2: FT_ULong,
    pub ulUnicodeRange3: FT_ULong,
    pub ulUnicodeRange4: FT_ULong,
    pub achVendID: [FT_Char; 4],
    pub fsSelection: FT_UShort,
    pub usFirstCharIndex: FT_UShort,
    pub usLastCharIndex: FT_UShort,
    pub sTypoAscender: FT_Short,
    pub sTypoDescender: FT_Short,
    pub sTypoLineGap: FT_Short,
    pub usWinAscent: FT_UShort,
    pub usWinDescent: FT_UShort,
    pub ulCodePageRange1: FT_ULong,
    pub ulCodePageRange2: FT_ULong,
    pub sxHeight: FT_Short,
    pub sCapHeight: FT_Short,
    pub usDefaultChar: FT_UShort,
    pub usBreakChar: FT_UShort,
    pub usMaxContext: FT_UShort,
    pub usLowerOpticalPointSize: FT_UShort,
    pub usUpperOpticalPointSize: FT_UShort,
}
pub type TT_OS2 = TT_OS2_;
/* *************************************************************************
   *
   * @enum:
   *   FT_Sfnt_Tag
   *
   * @description:
   *   An enumeration to specify indices of SFNT tables loaded and parsed by
   *   FreeType during initialization of an SFNT font.  Used in the
   *   @FT_Get_Sfnt_Table API function.
   *
   * @values:
   *   FT_SFNT_HEAD ::
   *     To access the font's @TT_Header structure.
   *
   *   FT_SFNT_MAXP ::
   *     To access the font's @TT_MaxProfile structure.
   *
   *   FT_SFNT_OS2 ::
   *     To access the font's @TT_OS2 structure.
   *
   *   FT_SFNT_HHEA ::
   *     To access the font's @TT_HoriHeader structure.
   *
   *   FT_SFNT_VHEA ::
   *     To access the font's @TT_VertHeader structure.
   *
   *   FT_SFNT_POST ::
   *     To access the font's @TT_Postscript structure.
   *
   *   FT_SFNT_PCLT ::
   *     To access the font's @TT_PCLT structure.
   */
pub type FT_Sfnt_Tag_ = libc::c_uint;
pub const FT_SFNT_MAX: FT_Sfnt_Tag_ = 7;
pub const FT_SFNT_PCLT: FT_Sfnt_Tag_ = 6;
pub const FT_SFNT_POST: FT_Sfnt_Tag_ = 5;
pub const FT_SFNT_VHEA: FT_Sfnt_Tag_ = 4;
pub const FT_SFNT_HHEA: FT_Sfnt_Tag_ = 3;
pub const FT_SFNT_OS2: FT_Sfnt_Tag_ = 2;
pub const FT_SFNT_MAXP: FT_Sfnt_Tag_ = 1;
pub const FT_SFNT_HEAD: FT_Sfnt_Tag_ = 0;
pub type FT_Sfnt_Tag = FT_Sfnt_Tag_;
pub type hb_bool_t = libc::c_int;
pub type hb_codepoint_t = uint32_t;
pub type hb_position_t = int32_t;
pub type hb_tag_t = uint32_t;
pub type hb_memory_mode_t = libc::c_uint;
pub const HB_MEMORY_MODE_READONLY_MAY_MAKE_WRITABLE: hb_memory_mode_t = 3;
pub const HB_MEMORY_MODE_WRITABLE: hb_memory_mode_t = 2;
pub const HB_MEMORY_MODE_READONLY: hb_memory_mode_t = 1;
pub const HB_MEMORY_MODE_DUPLICATE: hb_memory_mode_t = 0;
pub type hb_reference_table_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_face_t, _: hb_tag_t,
                                _: *mut libc::c_void) -> *mut hb_blob_t>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct hb_glyph_extents_t {
    pub x_bearing: hb_position_t,
    pub y_bearing: hb_position_t,
    pub width: hb_position_t,
    pub height: hb_position_t,
}
pub type hb_font_get_glyph_advance_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: *mut libc::c_void)
               -> hb_position_t>;
pub type hb_font_get_glyph_h_advance_func_t
    =
    hb_font_get_glyph_advance_func_t;
pub type hb_font_get_glyph_v_advance_func_t
    =
    hb_font_get_glyph_advance_func_t;
pub type hb_font_get_glyph_origin_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: *mut hb_position_t,
                                _: *mut hb_position_t, _: *mut libc::c_void)
               -> hb_bool_t>;
pub type hb_font_get_glyph_h_origin_func_t = hb_font_get_glyph_origin_func_t;
pub type hb_font_get_glyph_v_origin_func_t = hb_font_get_glyph_origin_func_t;
pub type hb_font_get_glyph_kerning_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: hb_codepoint_t,
                                _: *mut libc::c_void) -> hb_position_t>;
pub type hb_font_get_glyph_h_kerning_func_t
    =
    hb_font_get_glyph_kerning_func_t;
pub type hb_font_get_glyph_extents_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: *mut hb_glyph_extents_t,
                                _: *mut libc::c_void) -> hb_bool_t>;
pub type hb_font_get_glyph_contour_point_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: libc::c_uint,
                                _: *mut hb_position_t, _: *mut hb_position_t,
                                _: *mut libc::c_void) -> hb_bool_t>;
pub type hb_font_get_glyph_name_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: *mut libc::c_char,
                                _: libc::c_uint, _: *mut libc::c_void)
               -> hb_bool_t>;
pub type hb_font_get_glyph_func_t
    =
    Option<unsafe extern "C" fn(_: *mut hb_font_t, _: *mut libc::c_void,
                                _: hb_codepoint_t, _: hb_codepoint_t,
                                _: *mut hb_codepoint_t, _: *mut libc::c_void)
               -> hb_bool_t>;
pub type hb_font_get_glyph_v_kerning_func_t
    =
    hb_font_get_glyph_kerning_func_t;
pub type OTTag = uint32_t;
pub type GlyphID = uint16_t;
pub type SInt32 = libc::c_int;
pub type Fixed = SInt32;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct GlyphBBox {
    pub xMin: libc::c_float,
    pub yMin: libc::c_float,
    pub xMax: libc::c_float,
    pub yMax: libc::c_float,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XeTeXFontInst {
    pub m_unitsPerEM: libc::c_ushort,
    pub m_pointSize: libc::c_float,
    pub m_ascent: libc::c_float,
    pub m_descent: libc::c_float,
    pub m_capHeight: libc::c_float,
    pub m_xHeight: libc::c_float,
    pub m_italicAngle: libc::c_float,
    pub m_vertical: bool,
    pub m_filename: *mut libc::c_char,
    pub m_index: uint32_t,
    pub m_ftFace: FT_Face,
    pub m_backingData: *mut FT_Byte,
    pub m_backingData2: *mut FT_Byte,
    pub m_hbFont: *mut hb_font_t,
    pub m_subdtor: Option<unsafe extern "C" fn(_: *mut XeTeXFontInst) -> ()>,
}
/* *************************************************************************
   *
   * @type:
   *   FT_Glyph
   *
   * @description:
   *   Handle to an object used to model generic glyph images.  It is a
   *   pointer to the @FT_GlyphRec structure and can contain a glyph bitmap
   *   or pointer.
   *
   * @note:
   *   Glyph objects are not owned by the library.  You must thus release
   *   them manually (through @FT_Done_Glyph) _before_ calling
   *   @FT_Done_FreeType.
   */
pub type FT_Glyph = *mut FT_GlyphRec_;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FT_GlyphRec_ {
    pub library: FT_Library,
    pub clazz: *const FT_Glyph_Class,
    pub format: FT_Glyph_Format,
    pub advance: FT_Vector,
}
/* ***************************************************************************
 *
 * ftglyph.h
 *
 *   FreeType convenience functions to handle glyphs (specification).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
   *
   * This file contains the definition of several convenience functions that
   * can be used by client applications to easily retrieve glyph bitmaps and
   * outlines from a given face.
   *
   * These functions should be optional if you are writing a font server or
   * text layout engine on top of FreeType.  However, they are pretty handy
   * for many other simple uses of the library.
   *
   */
/* *************************************************************************
   *
   * @section:
   *   glyph_management
   *
   * @title:
   *   Glyph Management
   *
   * @abstract:
   *   Generic interface to manage individual glyph data.
   *
   * @description:
   *   This section contains definitions used to manage glyph data through
   *   generic @FT_Glyph objects.  Each of them can contain a bitmap,
   *   a vector outline, or even images in other formats.  These objects are
   *   detached from @FT_Face, contrary to @FT_GlyphSlot.
   *
   */
/* forward declaration to a private type */
pub type FT_Glyph_Class = FT_Glyph_Class_;
pub const FT_GLYPH_BBOX_UNSCALED: FT_Glyph_BBox_Mode_ = 0;
pub type FT_Glyph_BBox_Mode_ = libc::c_uint;
pub const FT_GLYPH_BBOX_PIXELS: FT_Glyph_BBox_Mode_ = 3;
pub const FT_GLYPH_BBOX_TRUNCATE: FT_Glyph_BBox_Mode_ = 2;
pub const FT_GLYPH_BBOX_GRIDFIT: FT_Glyph_BBox_Mode_ = 1;
pub const FT_GLYPH_BBOX_SUBPIXELS: FT_Glyph_BBox_Mode_ = 0;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
/* ***************************************************************************\
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
/* Return NAME with any leading path stripped off.  This returns a
   pointer into NAME.  For example, `basename ("/foo/bar.baz")'
   returns "bar.baz".  */
unsafe extern "C" fn xbasename(mut name: *const libc::c_char)
 -> *const libc::c_char {
    let mut base: *const libc::c_char = name;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = base;
    while *p != 0 {
        if *p as libc::c_int == '/' as i32 { base = p.offset(1) }
        p = p.offset(1)
    }
    return base;
}
#[no_mangle]
pub static mut gFreeTypeLibrary: FT_Library =
    0 as *const FT_LibraryRec_ as FT_Library;
static mut hbFontFuncs: *mut hb_font_funcs_t =
    0 as *const hb_font_funcs_t as *mut hb_font_funcs_t;
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_base_ctor(mut self_0:
                                                     *mut XeTeXFontInst,
                                                 mut pathname:
                                                     *const libc::c_char,
                                                 mut index: libc::c_int,
                                                 mut pointSize: libc::c_float,
                                                 mut status:
                                                     *mut libc::c_int) {
    (*self_0).m_unitsPerEM = 0i32 as libc::c_ushort;
    (*self_0).m_pointSize = pointSize;
    (*self_0).m_ascent = 0i32 as libc::c_float;
    (*self_0).m_descent = 0i32 as libc::c_float;
    (*self_0).m_capHeight = 0i32 as libc::c_float;
    (*self_0).m_xHeight = 0i32 as libc::c_float;
    (*self_0).m_italicAngle = 0i32 as libc::c_float;
    (*self_0).m_vertical = 0i32 != 0;
    (*self_0).m_filename = 0 as *mut libc::c_char;
    (*self_0).m_index = 0i32 as uint32_t;
    (*self_0).m_ftFace = 0 as FT_Face;
    (*self_0).m_backingData = 0 as *mut FT_Byte;
    (*self_0).m_backingData2 = 0 as *mut FT_Byte;
    (*self_0).m_hbFont = 0 as *mut hb_font_t;
    (*self_0).m_subdtor = None;
    if !pathname.is_null() {
        XeTeXFontInst_initialize(self_0, pathname, index, status);
    };
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_create(mut pathname:
                                                  *const libc::c_char,
                                              mut index: libc::c_int,
                                              mut pointSize: libc::c_float,
                                              mut status: *mut libc::c_int)
 -> *mut XeTeXFontInst {
    let mut self_0: *mut XeTeXFontInst =
        malloc(::std::mem::size_of::<XeTeXFontInst>() as libc::c_ulong) as
            *mut XeTeXFontInst;
    XeTeXFontInst_base_ctor(self_0, pathname, index, pointSize, status);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_delete(mut self_0:
                                                  *mut XeTeXFontInst) {
    if self_0.is_null() { return }
    if (*self_0).m_subdtor.is_some() {
        (*self_0).m_subdtor.expect("non-null function pointer")(self_0);
    }
    if !(*self_0).m_ftFace.is_null() {
        FT_Done_Face((*self_0).m_ftFace);
        (*self_0).m_ftFace = 0 as FT_Face
    }
    hb_font_destroy((*self_0).m_hbFont);
    free((*self_0).m_backingData as *mut libc::c_void);
    free((*self_0).m_backingData2 as *mut libc::c_void);
    free((*self_0).m_filename as *mut libc::c_void);
    free(self_0 as *mut libc::c_void);
}
/* HarfBuzz font functions */
unsafe extern "C" fn _get_glyph(mut _hbf: *mut hb_font_t,
                                mut font_data: *mut libc::c_void,
                                mut ch: hb_codepoint_t,
                                mut vs: hb_codepoint_t,
                                mut gid: *mut hb_codepoint_t,
                                mut _p: *mut libc::c_void) -> hb_bool_t {
    let mut face: FT_Face = font_data as FT_Face;
    *gid = 0i32 as hb_codepoint_t;
    if vs != 0 {
        *gid =
            FT_Face_GetCharVariantIndex(face, ch as FT_ULong, vs as FT_ULong)
    }
    if *gid == 0i32 as libc::c_uint {
        *gid = FT_Get_Char_Index(face, ch as FT_ULong)
    }
    return (*gid != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn _get_glyph_advance(mut face: FT_Face, mut gid: FT_UInt,
                                        mut vertical: bool) -> FT_Fixed {
    let mut error: FT_Error = 0;
    let mut advance: FT_Fixed = 0;
    let mut flags: libc::c_int = (1i64 << 0i32) as libc::c_int;
    if vertical {
        flags = (flags as libc::c_long | 1i64 << 4i32) as libc::c_int
    }
    error = FT_Get_Advance(face, gid, flags, &mut advance);
    if error != 0 { advance = 0i32 as FT_Fixed }
    /* FreeType's vertical metrics grows downward */
    if vertical { advance = -advance }
    return advance;
}
unsafe extern "C" fn _get_glyph_h_advance(mut _hbf: *mut hb_font_t,
                                          mut font_data: *mut libc::c_void,
                                          mut gid: hb_codepoint_t,
                                          mut _p: *mut libc::c_void)
 -> hb_position_t {
    return _get_glyph_advance(font_data as FT_Face, gid, 0i32 != 0) as
               hb_position_t;
}
unsafe extern "C" fn _get_glyph_v_advance(mut _hbf: *mut hb_font_t,
                                          mut font_data: *mut libc::c_void,
                                          mut gid: hb_codepoint_t,
                                          mut _p: *mut libc::c_void)
 -> hb_position_t {
    return _get_glyph_advance(font_data as FT_Face, gid, 1i32 != 0) as
               hb_position_t;
}
unsafe extern "C" fn _get_glyph_h_origin(mut _hbf: *mut hb_font_t,
                                         mut font_data: *mut libc::c_void,
                                         mut gid: hb_codepoint_t,
                                         mut x: *mut hb_position_t,
                                         mut y: *mut hb_position_t,
                                         mut _p: *mut libc::c_void)
 -> hb_bool_t {
    // horizontal origin is (0, 0)
    return 1i32;
}
unsafe extern "C" fn _get_glyph_v_origin(mut _hbf: *mut hb_font_t,
                                         mut font_data: *mut libc::c_void,
                                         mut gid: hb_codepoint_t,
                                         mut x: *mut hb_position_t,
                                         mut y: *mut hb_position_t,
                                         mut _p: *mut libc::c_void)
 -> hb_bool_t {
    // vertical origin is (0, 0) for now
    return 1i32;
}
unsafe extern "C" fn _get_glyph_h_kerning(mut _hbf: *mut hb_font_t,
                                          mut font_data: *mut libc::c_void,
                                          mut gid1: hb_codepoint_t,
                                          mut gid2: hb_codepoint_t,
                                          mut _p: *mut libc::c_void)
 -> hb_position_t {
    let mut face: FT_Face = font_data as FT_Face;
    let mut error: FT_Error = 0;
    let mut kerning: FT_Vector = FT_Vector{x: 0, y: 0,};
    let mut ret: hb_position_t = 0;
    error =
        FT_Get_Kerning(face, gid1, gid2,
                       FT_KERNING_UNSCALED as libc::c_int as FT_UInt,
                       &mut kerning);
    if error != 0 { ret = 0i32 } else { ret = kerning.x as hb_position_t }
    return ret;
}
unsafe extern "C" fn _get_glyph_v_kerning(mut _hbf: *mut hb_font_t,
                                          mut font_data: *mut libc::c_void,
                                          mut gid1: hb_codepoint_t,
                                          mut gid2: hb_codepoint_t,
                                          mut _p: *mut libc::c_void)
 -> hb_position_t {
    /* FreeType does not support vertical kerning */
    return 0i32;
}
unsafe extern "C" fn _get_glyph_extents(mut _hbf: *mut hb_font_t,
                                        mut font_data: *mut libc::c_void,
                                        mut gid: hb_codepoint_t,
                                        mut extents: *mut hb_glyph_extents_t,
                                        mut _p: *mut libc::c_void)
 -> hb_bool_t {
    let mut face: FT_Face = font_data as FT_Face;
    let mut error: FT_Error = 0;
    error = FT_Load_Glyph(face, gid, (1i64 << 0i32) as FT_Int32);
    if error == 0 {
        (*extents).x_bearing =
            (*(*face).glyph).metrics.horiBearingX as hb_position_t;
        (*extents).y_bearing =
            (*(*face).glyph).metrics.horiBearingY as hb_position_t;
        (*extents).width = (*(*face).glyph).metrics.width as hb_position_t;
        (*extents).height = -(*(*face).glyph).metrics.height as hb_position_t
    }
    return (error == 0) as libc::c_int;
}
unsafe extern "C" fn _get_glyph_contour_point(mut _hbf: *mut hb_font_t,
                                              mut font_data:
                                                  *mut libc::c_void,
                                              mut gid: hb_codepoint_t,
                                              mut point_index: libc::c_uint,
                                              mut x: *mut hb_position_t,
                                              mut y: *mut hb_position_t,
                                              mut _p: *mut libc::c_void)
 -> hb_bool_t {
    let mut face: FT_Face = font_data as FT_Face;
    let mut error: FT_Error = 0;
    let mut ret: bool = 0i32 != 0;
    error = FT_Load_Glyph(face, gid, (1i64 << 0i32) as FT_Int32);
    if error == 0 {
        if (*(*face).glyph).format as libc::c_uint ==
               FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint {
            if point_index < (*(*face).glyph).outline.n_points as libc::c_uint
               {
                *x =
                    (*(*(*face).glyph).outline.points.offset(point_index as
                                                                 isize)).x as
                        hb_position_t;
                *y =
                    (*(*(*face).glyph).outline.points.offset(point_index as
                                                                 isize)).y as
                        hb_position_t;
                ret = 1i32 != 0
            }
        }
    }
    return ret as hb_bool_t;
}
unsafe extern "C" fn _get_glyph_name(mut _hbf: *mut hb_font_t,
                                     mut font_data: *mut libc::c_void,
                                     mut gid: hb_codepoint_t,
                                     mut name: *mut libc::c_char,
                                     mut size: libc::c_uint,
                                     mut _p: *mut libc::c_void) -> hb_bool_t {
    let mut face: FT_Face = font_data as FT_Face;
    let mut ret: bool = 0i32 != 0;
    ret = FT_Get_Glyph_Name(face, gid, name as FT_Pointer, size) == 0;
    if ret as libc::c_int != 0 && (size != 0 && *name == 0) {
        ret = 0i32 != 0
    }
    return ret as hb_bool_t;
}
unsafe extern "C" fn _get_font_funcs() -> *mut hb_font_funcs_t {
    static mut funcs: *mut hb_font_funcs_t =
        0 as *const hb_font_funcs_t as *mut hb_font_funcs_t;
    if funcs.is_null() { funcs = hb_font_funcs_create() }
    hb_font_funcs_set_glyph_func(funcs,
                                 Some(_get_glyph as
                                          unsafe extern "C" fn(_:
                                                                   *mut hb_font_t,
                                                               _:
                                                                   *mut libc::c_void,
                                                               _:
                                                                   hb_codepoint_t,
                                                               _:
                                                                   hb_codepoint_t,
                                                               _:
                                                                   *mut hb_codepoint_t,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> hb_bool_t),
                                 0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_h_advance_func(funcs,
                                           Some(_get_glyph_h_advance as
                                                    unsafe extern "C" fn(_:
                                                                             *mut hb_font_t,
                                                                         _:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             hb_codepoint_t,
                                                                         _:
                                                                             *mut libc::c_void)
                                                        -> hb_position_t),
                                           0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_v_advance_func(funcs,
                                           Some(_get_glyph_v_advance as
                                                    unsafe extern "C" fn(_:
                                                                             *mut hb_font_t,
                                                                         _:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             hb_codepoint_t,
                                                                         _:
                                                                             *mut libc::c_void)
                                                        -> hb_position_t),
                                           0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_h_origin_func(funcs,
                                          Some(_get_glyph_h_origin as
                                                   unsafe extern "C" fn(_:
                                                                            *mut hb_font_t,
                                                                        _:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            hb_codepoint_t,
                                                                        _:
                                                                            *mut hb_position_t,
                                                                        _:
                                                                            *mut hb_position_t,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> hb_bool_t),
                                          0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_v_origin_func(funcs,
                                          Some(_get_glyph_v_origin as
                                                   unsafe extern "C" fn(_:
                                                                            *mut hb_font_t,
                                                                        _:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            hb_codepoint_t,
                                                                        _:
                                                                            *mut hb_position_t,
                                                                        _:
                                                                            *mut hb_position_t,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> hb_bool_t),
                                          0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_h_kerning_func(funcs,
                                           Some(_get_glyph_h_kerning as
                                                    unsafe extern "C" fn(_:
                                                                             *mut hb_font_t,
                                                                         _:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             hb_codepoint_t,
                                                                         _:
                                                                             hb_codepoint_t,
                                                                         _:
                                                                             *mut libc::c_void)
                                                        -> hb_position_t),
                                           0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_v_kerning_func(funcs,
                                           Some(_get_glyph_v_kerning as
                                                    unsafe extern "C" fn(_:
                                                                             *mut hb_font_t,
                                                                         _:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             hb_codepoint_t,
                                                                         _:
                                                                             hb_codepoint_t,
                                                                         _:
                                                                             *mut libc::c_void)
                                                        -> hb_position_t),
                                           0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_extents_func(funcs,
                                         Some(_get_glyph_extents as
                                                  unsafe extern "C" fn(_:
                                                                           *mut hb_font_t,
                                                                       _:
                                                                           *mut libc::c_void,
                                                                       _:
                                                                           hb_codepoint_t,
                                                                       _:
                                                                           *mut hb_glyph_extents_t,
                                                                       _:
                                                                           *mut libc::c_void)
                                                      -> hb_bool_t),
                                         0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_contour_point_func(funcs,
                                               Some(_get_glyph_contour_point
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut hb_font_t,
                                                                             _:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 hb_codepoint_t,
                                                                             _:
                                                                                 libc::c_uint,
                                                                             _:
                                                                                 *mut hb_position_t,
                                                                             _:
                                                                                 *mut hb_position_t,
                                                                             _:
                                                                                 *mut libc::c_void)
                                                            -> hb_bool_t),
                                               0 as *mut libc::c_void, None);
    hb_font_funcs_set_glyph_name_func(funcs,
                                      Some(_get_glyph_name as
                                               unsafe extern "C" fn(_:
                                                                        *mut hb_font_t,
                                                                    _:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        hb_codepoint_t,
                                                                    _:
                                                                        *mut libc::c_char,
                                                                    _:
                                                                        libc::c_uint,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> hb_bool_t),
                                      0 as *mut libc::c_void, None);
    return funcs;
}
unsafe extern "C" fn _get_table(mut _hfc: *mut hb_face_t, mut tag: hb_tag_t,
                                mut user_data: *mut libc::c_void)
 -> *mut hb_blob_t {
    let mut face: FT_Face = user_data as FT_Face;
    let mut length: FT_ULong = 0i32 as FT_ULong;
    let mut table: *mut FT_Byte = 0 as *mut FT_Byte;
    let mut error: FT_Error = 0;
    let mut blob: *mut hb_blob_t = 0 as *mut hb_blob_t;
    error =
        FT_Load_Sfnt_Table(face, tag as FT_ULong, 0i32 as FT_Long,
                           0 as *mut FT_Byte, &mut length);
    if error == 0 {
        table =
            xmalloc(length.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)) as
                *mut FT_Byte;
        if !table.is_null() {
            error =
                FT_Load_Sfnt_Table(face, tag as FT_ULong, 0i32 as FT_Long,
                                   table, &mut length);
            if error == 0 {
                blob =
                    hb_blob_create(table as *const libc::c_char,
                                   length as libc::c_uint,
                                   HB_MEMORY_MODE_WRITABLE,
                                   table as *mut libc::c_void,
                                   Some(free as
                                            unsafe extern "C" fn(_:
                                                                     *mut libc::c_void)
                                                -> ()))
            } else { free(table as *mut libc::c_void); }
        }
    }
    return blob;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_initialize(mut self_0:
                                                      *mut XeTeXFontInst,
                                                  mut pathname:
                                                      *const libc::c_char,
                                                  mut index: libc::c_int,
                                                  mut status:
                                                      *mut libc::c_int) {
    let mut postTable: *mut TT_Postscript = 0 as *mut TT_Postscript;
    let mut os2Table: *mut TT_OS2 = 0 as *mut TT_OS2;
    let mut error: FT_Error = 0;
    let mut hbFace: *mut hb_face_t = 0 as *mut hb_face_t;
    if gFreeTypeLibrary.is_null() {
        error = FT_Init_FreeType(&mut gFreeTypeLibrary);
        if error != 0 {
            _tt_abort(b"FreeType initialization failed, error %d\x00" as
                          *const u8 as *const libc::c_char, error);
        }
    }
    // Here we emulate some logic that was originally in find_native_font();
    let mut handle: rust_input_handle_t =
        ttstub_input_open(pathname, TTIF_OPENTYPE, 0i32);
    if handle.is_null() {
        handle = ttstub_input_open(pathname, TTIF_TRUETYPE, 0i32)
    }
    if handle.is_null() {
        handle = ttstub_input_open(pathname, TTIF_TYPE1, 0i32)
    }
    if handle.is_null() { *status = 1i32; return }
    let mut sz: size_t = ttstub_input_get_size(handle);
    (*self_0).m_backingData = xmalloc(sz) as *mut FT_Byte;
    let mut r: ssize_t =
        ttstub_input_read(handle,
                          (*self_0).m_backingData as *mut libc::c_char, sz);
    if r < 0i32 as libc::c_long || r as size_t != sz {
        _tt_abort(b"failed to read font file\x00" as *const u8 as
                      *const libc::c_char);
    }
    ttstub_input_close(handle);
    error =
        FT_New_Memory_Face(gFreeTypeLibrary, (*self_0).m_backingData,
                           sz as FT_Long, index as FT_Long,
                           &mut (*self_0).m_ftFace);
    if (*(*self_0).m_ftFace).face_flags & 1i64 << 0i32 == 0 {
        *status = 1i32;
        return
    }
    /* for non-sfnt-packaged fonts (presumably Type 1), see if there is an AFM file we can attach */
    if index == 0i32 && (*(*self_0).m_ftFace).face_flags & 1i64 << 3i32 == 0 {
        // Tectonic: this code used to use kpse_find_file and FT_Attach_File
        // to try to find metrics for this font. Thanks to the existence of
        // FT_Attach_Stream we can emulate this behavior while going through
        // the Rust I/O layer.
        let mut afm: *mut libc::c_char = xstrdup(xbasename(pathname));
        let mut p: *mut libc::c_char = strrchr(afm, '.' as i32);
        if !p.is_null() && strlen(p) == 4i32 as libc::c_ulong &&
               tolower(*p.offset(1) as libc::c_int) == 'p' as i32 &&
               tolower(*p.offset(2) as libc::c_int) == 'f' as i32 {
            strcpy(p, b".afm\x00" as *const u8 as *const libc::c_char);
        }
        let mut afm_handle: rust_input_handle_t =
            ttstub_input_open(afm, TTIF_AFM, 0i32);
        free(afm as *mut libc::c_void);
        if !afm_handle.is_null() {
            sz = ttstub_input_get_size(afm_handle);
            (*self_0).m_backingData2 = xmalloc(sz) as *mut FT_Byte;
            r =
                ttstub_input_read(afm_handle,
                                  (*self_0).m_backingData2 as
                                      *mut libc::c_char, sz);
            if r < 0i32 as libc::c_long || r as size_t != sz {
                _tt_abort(b"failed to read AFM file\x00" as *const u8 as
                              *const libc::c_char);
            }
            ttstub_input_close(afm_handle);
            let mut open_args: FT_Open_Args =
                FT_Open_Args{flags: 0,
                             memory_base: 0 as *const FT_Byte,
                             memory_size: 0,
                             pathname: 0 as *mut FT_String,
                             stream: 0 as *mut FT_StreamRec_,
                             driver: 0 as *mut FT_ModuleRec_,
                             num_params: 0,
                             params: 0 as *mut FT_Parameter,};
            open_args.flags = 0x1i32 as FT_UInt;
            open_args.memory_base = (*self_0).m_backingData2;
            open_args.memory_size = sz as FT_Long;
            FT_Attach_Stream((*self_0).m_ftFace, &mut open_args);
        }
    }
    (*self_0).m_filename = xstrdup(pathname);
    (*self_0).m_index = index as uint32_t;
    (*self_0).m_unitsPerEM = (*(*self_0).m_ftFace).units_per_EM;
    (*self_0).m_ascent =
        XeTeXFontInst_unitsToPoints(self_0,
                                    (*(*self_0).m_ftFace).ascender as
                                        libc::c_float);
    (*self_0).m_descent =
        XeTeXFontInst_unitsToPoints(self_0,
                                    (*(*self_0).m_ftFace).descender as
                                        libc::c_float);
    postTable =
        XeTeXFontInst_getFontTableFT(self_0, FT_SFNT_POST) as
            *mut TT_Postscript;
    if !postTable.is_null() {
        (*self_0).m_italicAngle =
            Fix2D((*postTable).italicAngle as Fixed) as libc::c_float
    }
    os2Table =
        XeTeXFontInst_getFontTableFT(self_0, FT_SFNT_OS2) as *mut TT_OS2;
    if !os2Table.is_null() {
        (*self_0).m_capHeight =
            XeTeXFontInst_unitsToPoints(self_0,
                                        (*os2Table).sCapHeight as
                                            libc::c_float);
        (*self_0).m_xHeight =
            XeTeXFontInst_unitsToPoints(self_0,
                                        (*os2Table).sxHeight as libc::c_float)
    }
    // Set up HarfBuzz font
    hbFace =
        hb_face_create_for_tables(Some(_get_table as
                                           unsafe extern "C" fn(_:
                                                                    *mut hb_face_t,
                                                                _: hb_tag_t,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> *mut hb_blob_t),
                                  (*self_0).m_ftFace as *mut libc::c_void,
                                  None);
    hb_face_set_index(hbFace, index as libc::c_uint);
    hb_face_set_upem(hbFace, (*self_0).m_unitsPerEM as libc::c_uint);
    (*self_0).m_hbFont = hb_font_create(hbFace);
    hb_face_destroy(hbFace);
    if hbFontFuncs.is_null() { hbFontFuncs = _get_font_funcs() }
    hb_font_set_funcs((*self_0).m_hbFont, hbFontFuncs,
                      (*self_0).m_ftFace as *mut libc::c_void, None);
    hb_font_set_scale((*self_0).m_hbFont,
                      (*self_0).m_unitsPerEM as libc::c_int,
                      (*self_0).m_unitsPerEM as libc::c_int);
    // We don’t want device tables adjustments
    hb_font_set_ppem((*self_0).m_hbFont, 0i32 as libc::c_uint,
                     0i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_setLayoutDirVertical(mut self_0:
                                                                *mut XeTeXFontInst,
                                                            mut vertical:
                                                                bool) {
    (*self_0).m_vertical = vertical;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getFontTable(mut self_0:
                                                        *const XeTeXFontInst,
                                                    mut tag: OTTag)
 -> *mut libc::c_void {
    let mut tmpLength: FT_ULong = 0i32 as FT_ULong;
    let mut error: FT_Error =
        FT_Load_Sfnt_Table((*self_0).m_ftFace, tag as FT_ULong,
                           0i32 as FT_Long, 0 as *mut FT_Byte,
                           &mut tmpLength);
    if error != 0 { return 0 as *mut libc::c_void }
    let mut table: *mut libc::c_void =
        xmalloc(tmpLength.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                           as libc::c_ulong));
    if !table.is_null() {
        error =
            FT_Load_Sfnt_Table((*self_0).m_ftFace, tag as FT_ULong,
                               0i32 as FT_Long, table as *mut FT_Byte,
                               &mut tmpLength);
        if error != 0 { free(table); return 0 as *mut libc::c_void }
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getFontTableFT(mut self_0:
                                                          *const XeTeXFontInst,
                                                      mut tag: FT_Sfnt_Tag)
 -> *mut libc::c_void {
    return FT_Get_Sfnt_Table((*self_0).m_ftFace, tag);
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getGlyphBounds(mut self_0:
                                                          *mut XeTeXFontInst,
                                                      mut gid: GlyphID,
                                                      mut bbox:
                                                          *mut GlyphBBox) {
    (*bbox).yMax = 0.0f64 as libc::c_float;
    (*bbox).xMax = (*bbox).yMax;
    (*bbox).yMin = (*bbox).xMax;
    (*bbox).xMin = (*bbox).yMin;
    let mut error: FT_Error =
        FT_Load_Glyph((*self_0).m_ftFace, gid as FT_UInt,
                      (1i64 << 0i32) as FT_Int32);
    if error != 0 { return }
    let mut glyph: FT_Glyph = 0 as *mut FT_GlyphRec_;
    error = FT_Get_Glyph((*(*self_0).m_ftFace).glyph, &mut glyph);
    if error == 0i32 {
        let mut ft_bbox: FT_BBox =
            FT_BBox{xMin: 0, yMin: 0, xMax: 0, yMax: 0,};
        FT_Glyph_Get_CBox(glyph,
                          FT_GLYPH_BBOX_UNSCALED as libc::c_int as FT_UInt,
                          &mut ft_bbox);
        (*bbox).xMin =
            XeTeXFontInst_unitsToPoints(self_0,
                                        ft_bbox.xMin as libc::c_float);
        (*bbox).yMin =
            XeTeXFontInst_unitsToPoints(self_0,
                                        ft_bbox.yMin as libc::c_float);
        (*bbox).xMax =
            XeTeXFontInst_unitsToPoints(self_0,
                                        ft_bbox.xMax as libc::c_float);
        (*bbox).yMax =
            XeTeXFontInst_unitsToPoints(self_0,
                                        ft_bbox.yMax as libc::c_float);
        FT_Done_Glyph(glyph);
    };
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_mapCharToGlyph(mut self_0:
                                                          *const XeTeXFontInst,
                                                      mut ch: UChar32)
 -> GlyphID {
    return FT_Get_Char_Index((*self_0).m_ftFace, ch as FT_ULong) as GlyphID;
}
/* ***************************************************************************\
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
/*
 *   file name:  XeTeXFontInst.h
 *
 *   created on: 2005-10-22
 *   created by: Jonathan Kew
 *
 *  originally based on PortableFontInstance.h from ICU
 */
// create specific subclasses for each supported platform
// false = horizontal, true = vertical
// font filename
// face index
/*
class XeTeXFontInst
{
protected:

public:
    XeTeXFontInst(float pointSize, int &status);
    XeTeXFontInst(const char* filename, int index, float pointSize, int &status);

    virtual ~XeTeXFontInst();

    void initialize(const char* pathname, int index, int &status);

    void *getFontTable(OTTag tableTag) const;
    void *getFontTable(FT_Sfnt_Tag tableTag) const;

    hb_font_t *getHbFont() const { return m_hbFont; }
    void setLayoutDirVertical(bool vertical);
    bool getLayoutDirVertical() const { return m_vertical; }

    GlyphID mapCharToGlyph(UChar32 ch) const;
    GlyphID mapGlyphToIndex(const char* glyphName) const;

    uint16_t getNumGlyphs() const;

    void getGlyphBounds(GlyphID glyph, GlyphBBox* bbox);

    float getGlyphWidth(GlyphID glyph);
    void getGlyphHeightDepth(GlyphID glyph, float *ht, float* dp);
    void getGlyphSidebearings(GlyphID glyph, float* lsb, float* rsb);
    float getGlyphItalCorr(GlyphID glyph);

    const char* getGlyphName(GlyphID gid, int& nameLen);

    UChar32 getFirstCharCode();
    UChar32 getLastCharCode();

    float unitsToPoints(float units) const
    {
        return (units * m_pointSize) / (float) m_unitsPerEM;
    }

    float pointsToUnits(float points) const
    {
        return (points * (float) m_unitsPerEM) / m_pointSize;
    }
};
*/
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getNumGlyphs(mut self_0:
                                                        *const XeTeXFontInst)
 -> uint16_t {
    return (*(*self_0).m_ftFace).num_glyphs as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getGlyphWidth(mut self_0:
                                                         *mut XeTeXFontInst,
                                                     mut gid: GlyphID)
 -> libc::c_float {
    return XeTeXFontInst_unitsToPoints(self_0,
                                       _get_glyph_advance((*self_0).m_ftFace,
                                                          gid as FT_UInt,
                                                          0i32 != 0) as
                                           libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getGlyphHeightDepth(mut self_0:
                                                               *mut XeTeXFontInst,
                                                           mut gid: GlyphID,
                                                           mut ht:
                                                               *mut libc::c_float,
                                                           mut dp:
                                                               *mut libc::c_float) {
    let mut bbox: GlyphBBox =
        GlyphBBox{xMin: 0., yMin: 0., xMax: 0., yMax: 0.,};
    XeTeXFontInst_getGlyphBounds(self_0, gid, &mut bbox);
    if !ht.is_null() { *ht = bbox.yMax }
    if !dp.is_null() { *dp = -bbox.yMin };
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getGlyphSidebearings(mut self_0:
                                                                *mut XeTeXFontInst,
                                                            mut gid: GlyphID,
                                                            mut lsb:
                                                                *mut libc::c_float,
                                                            mut rsb:
                                                                *mut libc::c_float) {
    let mut width: libc::c_float = XeTeXFontInst_getGlyphWidth(self_0, gid);
    let mut bbox: GlyphBBox =
        GlyphBBox{xMin: 0., yMin: 0., xMax: 0., yMax: 0.,};
    XeTeXFontInst_getGlyphBounds(self_0, gid, &mut bbox);
    if !lsb.is_null() { *lsb = bbox.xMin }
    if !rsb.is_null() { *rsb = width - bbox.xMax };
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getGlyphItalCorr(mut self_0:
                                                            *mut XeTeXFontInst,
                                                        mut gid: GlyphID)
 -> libc::c_float {
    let mut rval: libc::c_float = 0.0f64 as libc::c_float;
    let mut width: libc::c_float = XeTeXFontInst_getGlyphWidth(self_0, gid);
    let mut bbox: GlyphBBox =
        GlyphBBox{xMin: 0., yMin: 0., xMax: 0., yMax: 0.,};
    XeTeXFontInst_getGlyphBounds(self_0, gid, &mut bbox);
    if bbox.xMax > width { rval = bbox.xMax - width }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_mapGlyphToIndex(mut self_0:
                                                           *const XeTeXFontInst,
                                                       mut glyphName:
                                                           *const libc::c_char)
 -> GlyphID {
    return FT_Get_Name_Index((*self_0).m_ftFace,
                             glyphName as *mut libc::c_char) as GlyphID;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getGlyphName(mut self_0:
                                                        *mut XeTeXFontInst,
                                                    mut gid: GlyphID,
                                                    mut nameLen:
                                                        *mut libc::c_int)
 -> *const libc::c_char {
    if (*(*self_0).m_ftFace).face_flags & 1i64 << 9i32 != 0 {
        static mut buffer: [libc::c_char; 256] = [0; 256];
        FT_Get_Glyph_Name((*self_0).m_ftFace, gid as FT_UInt,
                          buffer.as_mut_ptr() as FT_Pointer,
                          256i32 as FT_UInt);
        *nameLen = strlen(buffer.as_mut_ptr()) as libc::c_int;
        return &mut *buffer.as_mut_ptr().offset(0) as *mut libc::c_char
    } else { *nameLen = 0i32; return 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getFirstCharCode(mut self_0:
                                                            *mut XeTeXFontInst)
 -> UChar32 {
    let mut gindex: FT_UInt = 0;
    return FT_Get_First_Char((*self_0).m_ftFace, &mut gindex) as UChar32;
}
#[no_mangle]
pub unsafe extern "C" fn XeTeXFontInst_getLastCharCode(mut self_0:
                                                           *mut XeTeXFontInst)
 -> UChar32 {
    let mut gindex: FT_UInt = 0;
    let mut ch: UChar32 =
        FT_Get_First_Char((*self_0).m_ftFace, &mut gindex) as UChar32;
    let mut prev: UChar32 = ch;
    while gindex != 0i32 as libc::c_uint {
        prev = ch;
        ch =
            FT_Get_Next_Char((*self_0).m_ftFace, ch as FT_ULong, &mut gindex)
                as UChar32
    }
    return prev;
}
