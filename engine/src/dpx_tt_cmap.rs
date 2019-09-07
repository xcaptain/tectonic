#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value)]
extern crate libc;
extern "C" {
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    /* Here is the complete list of PDF object types */
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    pub type otl_gsub;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> libc::c_uchar;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> libc::c_ushort;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> libc::c_short;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[no_mangle]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[no_mangle]
    fn dfont_open(handle: rust_input_handle_t, index: libc::c_int) -> *mut sfnt;
    #[no_mangle]
    fn sfnt_close(sfont: *mut sfnt);
    /* table directory */
    #[no_mangle]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> libc::c_int;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const libc::c_char) -> SFNT_ULONG;
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const libc::c_char) -> SFNT_ULONG;
    #[no_mangle]
    fn agl_get_unicodes(
        glyphstr: *const libc::c_char,
        unicodes: *mut int32_t,
        max_uncodes: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    static mut CSI_IDENTITY: CIDSysInfo;
    #[no_mangle]
    static mut CSI_UNICODE: CIDSysInfo;
    #[no_mangle]
    fn CMap_set_silent(value: libc::c_int);
    #[no_mangle]
    fn CMap_add_cidchar(
        cmap: *mut CMap,
        src: *const libc::c_uchar,
        srcdim: size_t,
        dest: CID,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_release(cmap: *mut CMap);
    #[no_mangle]
    fn CMap_get_type(cmap: *mut CMap) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_bfchar(
        cmap: *mut CMap,
        src: *const libc::c_uchar,
        srcdim: size_t,
        dest: *const libc::c_uchar,
        destdim: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_new() -> *mut CMap;
    #[no_mangle]
    fn CMap_set_name(cmap: *mut CMap, name: *const libc::c_char);
    #[no_mangle]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: libc::c_int);
    #[no_mangle]
    fn CMap_set_type(cmap: *mut CMap, type_0: libc::c_int);
    #[no_mangle]
    fn CMap_set_CIDSysInfo(cmap: *mut CMap, csi: *const CIDSysInfo);
    #[no_mangle]
    fn CMap_add_codespacerange(
        cmap: *mut CMap,
        codelo: *const libc::c_uchar,
        codehi: *const libc::c_uchar,
        dim: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_decode(
        cmap: *mut CMap,
        inbuf: *mut *const libc::c_uchar,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut libc::c_uchar,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn CMap_reverse_decode(cmap: *mut CMap, cid: CID) -> libc::c_int;
    #[no_mangle]
    fn CMap_cache_get(id: libc::c_int) -> *mut CMap;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn CMap_cache_add(cmap: *mut CMap) -> libc::c_int;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn CMap_create_stream(cmap: *mut CMap) -> *mut pdf_obj;
    #[no_mangle]
    fn dpx_open_truetype_file(filename: *const libc::c_char) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_opentype_file(filename: *const libc::c_char) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_dfont_file(filename: *const libc::c_char) -> rust_input_handle_t;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_defineresource(
        category: *const libc::c_char,
        resname: *const libc::c_char,
        object: *mut pdf_obj,
        flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_findresource(category: *const libc::c_char, resname: *const libc::c_char)
        -> libc::c_int;
    #[no_mangle]
    fn pdf_get_resource_reference(res_id: libc::c_int) -> *mut pdf_obj;
    /* TTC (TrueType Collection) */
    #[no_mangle]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: libc::c_int) -> SFNT_ULONG;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn otl_gsub_set_verbose(level: libc::c_int);
    /* LookupType for GSUB */
    #[no_mangle]
    fn otl_gsub_new() -> *mut otl_gsub;
    #[no_mangle]
    fn otl_gsub_release(gsub_list: *mut otl_gsub);
    #[no_mangle]
    fn otl_gsub_select(
        gsub_list: *mut otl_gsub,
        script: *const libc::c_char,
        language: *const libc::c_char,
        feature: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_apply(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> libc::c_int;
    /* Handle a list of OTL features */
    #[no_mangle]
    fn otl_gsub_apply_chain(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_add_feat(
        gsub_list: *mut otl_gsub,
        script: *const libc::c_char,
        language: *const libc::c_char,
        feature: *const libc::c_char,
        sfont: *mut sfnt,
    ) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_add_feat_list(
        gsub_list: *mut otl_gsub,
        otl_tags: *const libc::c_char,
        sfont: *mut sfnt,
    ) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_set_chain(gsub_list: *mut otl_gsub, otl_tags: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table;
    #[no_mangle]
    fn tt_release_post_table(post: *mut tt_post_table);
    #[no_mangle]
    fn tt_get_glyphname(post: *mut tt_post_table, gid: USHORT) -> *mut libc::c_char;
    #[no_mangle]
    fn UC_UTF16BE_encode_char(
        ucv: int32_t,
        dstpp: *mut *mut libc::c_uchar,
        endptr: *mut libc::c_uchar,
    ) -> size_t;
    #[no_mangle]
    fn cff_open(
        handle: rust_input_handle_t,
        offset: libc::c_int,
        idx: libc::c_int,
    ) -> *mut cff_font;
    #[no_mangle]
    fn cff_close(cff: *mut cff_font);
    /* Charsets */
    #[no_mangle]
    fn cff_read_charsets(cff: *mut cff_font) -> libc::c_int;
    /* Return PS name of "gid" */
    #[no_mangle]
    fn cff_get_glyphname(cff: *mut cff_font, gid: card16) -> *mut libc::c_char;
    /* Returns SID or CID */
    #[no_mangle]
    fn cff_charsets_lookup_inverse(cff: *mut cff_font, gid: card16) -> card16;
    /* String */
    #[no_mangle]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut libc::c_char;
    #[no_mangle]
    fn cff_dict_get(
        dict: *mut cff_dict,
        key: *const libc::c_char,
        idx: libc::c_int,
    ) -> libc::c_double;
    #[no_mangle]
    fn cff_dict_known(dict: *mut cff_dict, key: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */
pub type BYTE = libc::c_uchar;
pub type USHORT = libc::c_ushort;
pub type SHORT = libc::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
/* 16.16-bit signed fixed-point number */
pub type FWord = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [libc::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut libc::c_char,
    /* table data */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut libc::c_char,
    pub tables: *mut sfnt_table,
}
/* sfnt resource */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_cmap {
    pub format: USHORT,
    pub platform: USHORT,
    pub encoding: USHORT,
    pub language: SFNT_ULONG,
    pub map: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut libc::c_char,
    pub ordering: *mut libc::c_char,
    pub supplement: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut libc::c_char,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: card16,
    pub num_fds: card8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: libc::c_int,
    pub index: libc::c_int,
    pub flag: libc::c_int,
    pub is_notdef_notzero: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
pub type card8 = libc::c_uchar;
pub type l_offset = uint32_t;
pub type c_offsize = libc::c_uchar;
pub type card16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: libc::c_int,
    pub count: libc::c_int,
    pub entries: *mut cff_dict_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: libc::c_int,
    pub key: *const libc::c_char,
    pub count: libc::c_int,
    pub values: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* CFF Data Types */
/* SID SID number */
/* offset(0) */
/* size offset(0) */
/* 1-byte unsigned number */
/* 2-byte unsigned number */
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
/* 1, 2, 3, or 4-byte offset */
pub type s_SID = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed_1,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
/* 2-byte string identifier  */
/* number of objects stored in INDEX */
/* Offset array element size, 1-4    */
/* Offset array, count + 1 offsets   */
/* Object data                       */
/* format major version (starting at 1) */
/* format minor version (starting at 0) */
/* Header size (bytes)                  */
/* Absolute offset (0) size             */
/* Dictionary */
/* encoded data value (as card8 or card16) */
/* opname                                 */
/* number of values                        */
/* values                                  */
/* Encoding, Charset and FDSelect */
/* SID or CID, or card8 for Encoding  */
/* no. of remaining gids/codes in this range */
/* SID or CID (card16)      */
/* card16-version of range1 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const libc::c_char,
    pub names: *mut *mut libc::c_char,
    pub count: USHORT,
    /* Number of glyph names in names[] */
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* Mapping types, MAP_IS_NAME is not supported. */
/* Lookup flags */
/* DEBUG */
/* Codespacerange */
/* Dimension of this codespacerange */
/* Lower bounds of valid input code */
/* Upper bounds of valid input code */
/* 2 for CID, variable for Code..  */
/* CID (as 16-bit BE), Code ...    */
/* Next Subtbl for LOOKUP_CONTINUE */
/* CID, Code... MEM_ALLOC_SIZE bytes  */
/* Previous mapData data segment      */
/* Position of next free data segment */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub wmode: libc::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_3,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: libc::c_int,
    pub profile: C2RustUnnamed_2,
    pub reverseMap: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut libc::c_uchar,
    pub prev: *mut mapData,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: libc::c_int,
    pub len: size_t,
    pub code: *mut libc::c_uchar,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
    pub ranges: *mut rangeDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut libc::c_uchar,
    pub codeHi: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap12 {
    pub nGroups: SFNT_ULONG,
    pub groups: *mut charGroup,
}
/* Format 8 and 10 not supported...
 *
 *  format  8: mixed 16-bit and 32-bit coverage
 *  format 10: trimmed array
 */
/*
 * format 12: segmented coverage
 *
 * startGlyphID is 32-bit long, however, GlyphID is still 16-bit long !
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charGroup {
    pub startCharCode: SFNT_ULONG,
    pub endCharCode: SFNT_ULONG,
    pub startGlyphID: SFNT_ULONG,
}
/* format 6: trimmed table mapping */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap6 {
    pub firstCode: USHORT,
    pub entryCount: USHORT,
    pub glyphIndexArray: *mut USHORT,
}
/*
 * format 4: segment mapping to delta values
 * - Microsoft standard character to glyph index mapping table
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap4 {
    pub segCountX2: USHORT,
    pub searchRange: USHORT,
    pub entrySelector: USHORT,
    pub rangeShift: USHORT,
    pub endCount: *mut USHORT,
    pub reservedPad: USHORT,
    pub startCount: *mut USHORT,
    pub idDelta: *mut USHORT,
    pub idRangeOffset: *mut USHORT,
    pub glyphIndexArray: *mut USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap2 {
    pub subHeaderKeys: [USHORT; 256],
    pub subHeaders: *mut SubHeader,
    pub glyphIndexArray: *mut USHORT,
}
/* format 2: high-byte mapping through table */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubHeader {
    pub firstCode: USHORT,
    pub entryCount: USHORT,
    pub idDelta: SHORT,
    pub idRangeOffset: USHORT,
}
/* format 0: byte encoding table */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap0 {
    pub glyphIndexArray: [BYTE; 256],
}
pub type CID = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap_plat_enc_rec {
    pub platform: libc::c_short,
    pub encoding: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_maxp_table {
    pub version: Fixed,
    pub numGlyphs: USHORT,
    pub maxPoints: USHORT,
    pub maxContours: USHORT,
    pub maxComponentPoints: USHORT,
    pub maxComponentContours: USHORT,
    pub maxZones: USHORT,
    pub maxTwilightPoints: USHORT,
    pub maxStorage: USHORT,
    pub maxFunctionDefs: USHORT,
    pub maxInstructionDefs: USHORT,
    pub maxStackElements: USHORT,
    pub maxSizeOfInstructions: USHORT,
    pub maxComponentElements: USHORT,
    pub maxComponentDepth: USHORT,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr); /* the number of subHeaders is one plus the max of subHeaderKeys */
    return 0 as *mut libc::c_void;
}
static mut verbose: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn otf_cmap_set_verbose(mut level: libc::c_int) {
    otl_gsub_set_verbose(level);
    verbose = level;
}
unsafe extern "C" fn read_cmap0(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap0 {
    let mut map: *mut cmap0 = 0 as *mut cmap0;
    let mut i: libc::c_uint = 0;
    if len < 256i32 as libc::c_uint {
        _tt_abort(b"invalid cmap subtable\x00" as *const u8 as *const libc::c_char);
    }
    map = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cmap0>() as libc::c_ulong) as uint32_t)
        as *mut cmap0;
    i = 0i32 as libc::c_uint;
    while i < 256i32 as libc::c_uint {
        (*map).glyphIndexArray[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i = i.wrapping_add(1)
    }
    return map;
}
unsafe extern "C" fn release_cmap0(mut map: *mut cmap0) {
    free(map as *mut libc::c_void);
}
unsafe extern "C" fn lookup_cmap0(mut map: *mut cmap0, mut cc: USHORT) -> USHORT {
    return (if cc as libc::c_int > 255i32 {
        0i32
    } else {
        (*map).glyphIndexArray[cc as usize] as libc::c_int
    }) as USHORT;
}
unsafe extern "C" fn read_cmap2(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap2 {
    let mut map: *mut cmap2 = 0 as *mut cmap2;
    let mut i: USHORT = 0;
    let mut n: USHORT = 0;
    if len < 512i32 as libc::c_uint {
        _tt_abort(b"invalid cmap subtable\x00" as *const u8 as *const libc::c_char);
    }
    map = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cmap2>() as libc::c_ulong) as uint32_t)
        as *mut cmap2;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < 256i32 {
        (*map).subHeaderKeys[i as usize] = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    n = 0i32 as USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < 256i32 {
        (*map).subHeaderKeys[i as usize] =
            ((*map).subHeaderKeys[i as usize] as libc::c_int / 8i32) as USHORT;
        if (n as libc::c_int) < (*map).subHeaderKeys[i as usize] as libc::c_int {
            n = (*map).subHeaderKeys[i as usize]
        }
        i = i.wrapping_add(1)
    }
    n = (n as libc::c_int + 1i32) as USHORT;
    (*map).subHeaders = new((n as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<SubHeader>() as libc::c_ulong)
        as uint32_t) as *mut SubHeader;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < n as libc::c_int {
        (*(*map).subHeaders.offset(i as isize)).firstCode = tt_get_unsigned_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).entryCount = tt_get_unsigned_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).idDelta = tt_get_signed_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).idRangeOffset =
            tt_get_unsigned_pair((*sfont).handle);
        /* It makes things easier to let the offset starts from
         * the beginning of glyphIndexArray.
         */
        if (*(*map).subHeaders.offset(i as isize)).idRangeOffset as libc::c_int != 0i32 {
            let ref mut fresh0 = (*(*map).subHeaders.offset(i as isize)).idRangeOffset;
            *fresh0 = (*fresh0 as libc::c_int
                - (2i32 + (n as libc::c_int - i as libc::c_int - 1i32) * 8i32))
                as USHORT
        }
        i = i.wrapping_add(1)
    }
    /* Caculate the length of glyphIndexArray, this is ugly,
     * there should be a better way to get this information.
     */
    n = (len
        .wrapping_sub(518i32 as libc::c_uint)
        .wrapping_sub((n as libc::c_int * 8i32) as libc::c_uint) as USHORT as libc::c_int
        / 2i32) as USHORT;
    (*map).glyphIndexArray = new((n as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < n as libc::c_int {
        *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    return map;
}
unsafe extern "C" fn release_cmap2(mut map: *mut cmap2) {
    if !map.is_null() {
        free((*map).subHeaders as *mut libc::c_void);
        free((*map).glyphIndexArray as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap2(mut map: *mut cmap2, mut cc: USHORT) -> USHORT {
    let mut idx: USHORT = 0i32 as USHORT;
    let mut idDelta: SHORT = 0;
    let mut firstCode: USHORT = 0;
    let mut entryCount: USHORT = 0;
    let mut idRangeOffset: USHORT = 0;
    let mut hi: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut i: USHORT = 0;
    hi = cc as libc::c_int >> 8i32 & 0xffi32;
    lo = cc as libc::c_int & 0xffi32;
    /* select which subHeader to use */
    i = (*map).subHeaderKeys[hi as usize];
    firstCode = (*(*map).subHeaders.offset(i as isize)).firstCode;
    entryCount = (*(*map).subHeaders.offset(i as isize)).entryCount;
    idDelta = (*(*map).subHeaders.offset(i as isize)).idDelta;
    idRangeOffset =
        ((*(*map).subHeaders.offset(i as isize)).idRangeOffset as libc::c_int / 2i32) as USHORT;
    if lo >= firstCode as libc::c_int && lo < firstCode as libc::c_int + entryCount as libc::c_int {
        idRangeOffset = (idRangeOffset as libc::c_int + (lo - firstCode as libc::c_int)) as USHORT;
        idx = *(*map).glyphIndexArray.offset(idRangeOffset as isize);
        if idx as libc::c_int != 0i32 {
            idx = (idx as libc::c_int + idDelta as libc::c_int & 0xffffi32) as USHORT
        }
    }
    return idx;
}
unsafe extern "C" fn read_cmap4(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap4 {
    let mut map: *mut cmap4 = 0 as *mut cmap4;
    let mut i: USHORT = 0;
    let mut n: USHORT = 0;
    let mut segCount: USHORT = 0;
    if len < 8i32 as libc::c_uint {
        _tt_abort(b"invalid cmap subtable\x00" as *const u8 as *const libc::c_char);
    }
    map = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cmap4>() as libc::c_ulong) as uint32_t)
        as *mut cmap4;
    segCount = tt_get_unsigned_pair((*sfont).handle);
    (*map).segCountX2 = segCount;
    (*map).searchRange = tt_get_unsigned_pair((*sfont).handle);
    (*map).entrySelector = tt_get_unsigned_pair((*sfont).handle);
    (*map).rangeShift = tt_get_unsigned_pair((*sfont).handle);
    segCount = (segCount as libc::c_int / 2i32) as USHORT;
    (*map).endCount = new((segCount as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < segCount as libc::c_int {
        *(*map).endCount.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    (*map).reservedPad = tt_get_unsigned_pair((*sfont).handle);
    (*map).startCount = new((segCount as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < segCount as libc::c_int {
        *(*map).startCount.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    (*map).idDelta = new((segCount as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < segCount as libc::c_int {
        *(*map).idDelta.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    (*map).idRangeOffset = new((segCount as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < segCount as libc::c_int {
        *(*map).idRangeOffset.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    n = len
        .wrapping_sub(16i32 as libc::c_uint)
        .wrapping_sub((8i32 * segCount as libc::c_int) as libc::c_uint)
        .wrapping_div(2i32 as libc::c_uint) as USHORT;
    if n as libc::c_int == 0i32 {
        (*map).glyphIndexArray = 0 as *mut USHORT
    } else {
        (*map).glyphIndexArray = new((n as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
            as uint32_t) as *mut USHORT;
        i = 0i32 as USHORT;
        while (i as libc::c_int) < n as libc::c_int {
            *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
            i = i.wrapping_add(1)
        }
    }
    return map;
}
unsafe extern "C" fn release_cmap4(mut map: *mut cmap4) {
    if !map.is_null() {
        free((*map).endCount as *mut libc::c_void);
        free((*map).startCount as *mut libc::c_void);
        free((*map).idDelta as *mut libc::c_void);
        free((*map).idRangeOffset as *mut libc::c_void);
        free((*map).glyphIndexArray as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap4(mut map: *mut cmap4, mut cc: USHORT) -> USHORT {
    let mut gid: USHORT = 0i32 as USHORT;
    let mut i: USHORT = 0;
    let mut j: USHORT = 0;
    let mut segCount: USHORT = 0;
    /*
     * Segments are sorted in order of increasing endCode values.
     * Last segment maps 0xffff to gid 0 (?)
     */
    segCount = ((*map).segCountX2 as libc::c_int / 2i32) as USHORT;
    i = segCount;
    loop {
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 as libc::c_int > 0i32
            && cc as libc::c_int <= *(*map).endCount.offset(i as isize) as libc::c_int)
        {
            break;
        }
        if !(cc as libc::c_int >= *(*map).startCount.offset(i as isize) as libc::c_int) {
            continue;
        }
        if *(*map).idRangeOffset.offset(i as isize) as libc::c_int == 0i32 {
            gid = (cc as libc::c_int + *(*map).idDelta.offset(i as isize) as libc::c_int
                & 0xffffi32) as USHORT
        } else if cc as libc::c_int == 0xffffi32
            && *(*map).idRangeOffset.offset(i as isize) as libc::c_int == 0xffffi32
        {
            /* this is for protection against some old broken fonts... */
            gid = 0i32 as USHORT
        } else {
            j = (*(*map).idRangeOffset.offset(i as isize) as libc::c_int
                - (segCount as libc::c_int - i as libc::c_int) * 2i32) as USHORT;
            j = (cc as libc::c_int - *(*map).startCount.offset(i as isize) as libc::c_int
                + j as libc::c_int / 2i32) as USHORT;
            gid = *(*map).glyphIndexArray.offset(j as isize);
            if gid as libc::c_int != 0i32 {
                gid = (gid as libc::c_int + *(*map).idDelta.offset(i as isize) as libc::c_int
                    & 0xffffi32) as USHORT
            }
        }
        break;
    }
    return gid;
}
unsafe extern "C" fn read_cmap6(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap6 {
    let mut map: *mut cmap6 = 0 as *mut cmap6;
    let mut i: USHORT = 0;
    if len < 4i32 as libc::c_uint {
        _tt_abort(b"invalid cmap subtable\x00" as *const u8 as *const libc::c_char);
    }
    map = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cmap6>() as libc::c_ulong) as uint32_t)
        as *mut cmap6;
    (*map).firstCode = tt_get_unsigned_pair((*sfont).handle);
    (*map).entryCount = tt_get_unsigned_pair((*sfont).handle);
    (*map).glyphIndexArray = new(((*map).entryCount as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < (*map).entryCount as libc::c_int {
        *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    return map;
}
unsafe extern "C" fn release_cmap6(mut map: *mut cmap6) {
    if !map.is_null() {
        free((*map).glyphIndexArray as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap6(mut map: *mut cmap6, mut cc: USHORT) -> USHORT {
    let mut idx: USHORT = 0;
    idx = (cc as libc::c_int - (*map).firstCode as libc::c_int) as USHORT;
    if (idx as libc::c_int) < (*map).entryCount as libc::c_int {
        return *(*map).glyphIndexArray.offset(idx as isize);
    }
    return 0i32 as USHORT;
}
/* ULONG length */
unsafe extern "C" fn read_cmap12(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap12 {
    let mut map: *mut cmap12 = 0 as *mut cmap12;
    let mut i: SFNT_ULONG = 0;
    if len < 4i32 as libc::c_uint {
        _tt_abort(b"invalid cmap subtable\x00" as *const u8 as *const libc::c_char);
    }
    map = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cmap12>() as libc::c_ulong) as uint32_t)
        as *mut cmap12;
    (*map).nGroups = tt_get_unsigned_quad((*sfont).handle);
    (*map).groups = new(((*map).nGroups as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<charGroup>() as libc::c_ulong)
        as uint32_t) as *mut charGroup;
    i = 0i32 as SFNT_ULONG;
    while i < (*map).nGroups {
        (*(*map).groups.offset(i as isize)).startCharCode = tt_get_unsigned_quad((*sfont).handle);
        (*(*map).groups.offset(i as isize)).endCharCode = tt_get_unsigned_quad((*sfont).handle);
        (*(*map).groups.offset(i as isize)).startGlyphID = tt_get_unsigned_quad((*sfont).handle);
        i = i.wrapping_add(1)
    }
    return map;
}
unsafe extern "C" fn release_cmap12(mut map: *mut cmap12) {
    if !map.is_null() {
        free((*map).groups as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap12(mut map: *mut cmap12, mut cccc: SFNT_ULONG) -> USHORT {
    let mut gid: USHORT = 0i32 as USHORT;
    let mut i: libc::c_int = 0;
    i = (*map).nGroups as libc::c_int;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 >= 0i32 && cccc <= (*(*map).groups.offset(i as isize)).endCharCode) {
            break;
        }
        if !(cccc >= (*(*map).groups.offset(i as isize)).startCharCode) {
            continue;
        }
        gid = (cccc
            .wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode)
            .wrapping_add((*(*map).groups.offset(i as isize)).startGlyphID)
            & 0xffffi32 as libc::c_uint) as USHORT;
        break;
    }
    return gid;
}
/* read cmap */
#[no_mangle]
pub unsafe extern "C" fn tt_cmap_read(
    mut sfont: *mut sfnt,
    mut platform: USHORT,
    mut encoding: USHORT,
) -> *mut tt_cmap {
    let mut cmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut offset: SFNT_ULONG = 0;
    let mut length: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut p_id: USHORT = 0;
    let mut e_id: USHORT = 0;
    let mut i: USHORT = 0;
    let mut n_subtabs: USHORT = 0;
    if !sfont.is_null() {
    } else {
        __assert_fail(
            b"sfont\x00" as *const u8 as *const libc::c_char,
            b"dpx-tt_cmap.c\x00" as *const u8 as *const libc::c_char,
            447i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"tt_cmap *tt_cmap_read(sfnt *, USHORT, USHORT)\x00",
            ))
            .as_ptr(),
        );
    }
    offset = sfnt_locate_table(sfont, b"cmap\x00" as *const u8 as *const libc::c_char);
    tt_get_unsigned_pair((*sfont).handle);
    n_subtabs = tt_get_unsigned_pair((*sfont).handle);
    i = 0i32 as USHORT;
    while (i as libc::c_int) < n_subtabs as libc::c_int {
        p_id = tt_get_unsigned_pair((*sfont).handle);
        e_id = tt_get_unsigned_pair((*sfont).handle);
        if p_id as libc::c_int != platform as libc::c_int
            || e_id as libc::c_int != encoding as libc::c_int
        {
            tt_get_unsigned_quad((*sfont).handle);
            i = i.wrapping_add(1)
        } else {
            offset = (offset as libc::c_uint).wrapping_add(tt_get_unsigned_quad((*sfont).handle))
                as SFNT_ULONG as SFNT_ULONG;
            break;
        }
    }
    if i as libc::c_int == n_subtabs as libc::c_int {
        return 0 as *mut tt_cmap;
    }
    cmap = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<tt_cmap>() as libc::c_ulong) as uint32_t)
        as *mut tt_cmap;
    (*cmap).map = 0 as *mut libc::c_void;
    (*cmap).platform = platform;
    (*cmap).encoding = encoding;
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    (*cmap).format = tt_get_unsigned_pair((*sfont).handle);
    /* Length and version (language) is ULONG for
     * format 8, 10, 12 !
     */
    if (*cmap).format as libc::c_int <= 6i32 {
        length = tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG;
        (*cmap).language = tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG
    /* language (Mac) */
    } else if tt_get_unsigned_pair((*sfont).handle) as libc::c_int != 0i32 {
        /* reverved - 0 */
        dpx_warning(b"Unrecognized cmap subtable format.\x00" as *const u8 as *const libc::c_char);
        tt_cmap_release(cmap);
        return 0 as *mut tt_cmap;
    } else {
        length = tt_get_unsigned_quad((*sfont).handle);
        (*cmap).language = tt_get_unsigned_quad((*sfont).handle)
    }
    match (*cmap).format as libc::c_int {
        0 => (*cmap).map = read_cmap0(sfont, length) as *mut libc::c_void,
        2 => (*cmap).map = read_cmap2(sfont, length) as *mut libc::c_void,
        4 => (*cmap).map = read_cmap4(sfont, length) as *mut libc::c_void,
        6 => (*cmap).map = read_cmap6(sfont, length) as *mut libc::c_void,
        12 => {
            /* dpx_warning("UCS-4 TrueType cmap table..."); */
            (*cmap).map = read_cmap12(sfont, length) as *mut libc::c_void
        }
        _ => {
            dpx_warning(
                b"Unrecognized OpenType/TrueType cmap format.\x00" as *const u8
                    as *const libc::c_char,
            );
            tt_cmap_release(cmap);
            return 0 as *mut tt_cmap;
        }
    }
    if (*cmap).map.is_null() {
        tt_cmap_release(cmap);
        cmap = 0 as *mut tt_cmap
    }
    return cmap;
}
#[no_mangle]
pub unsafe extern "C" fn tt_cmap_release(mut cmap: *mut tt_cmap) {
    if !cmap.is_null() {
        if !(*cmap).map.is_null() {
            match (*cmap).format as libc::c_int {
                0 => {
                    release_cmap0((*cmap).map as *mut cmap0);
                }
                2 => {
                    release_cmap2((*cmap).map as *mut cmap2);
                }
                4 => {
                    release_cmap4((*cmap).map as *mut cmap4);
                }
                6 => {
                    release_cmap6((*cmap).map as *mut cmap6);
                }
                12 => {
                    release_cmap12((*cmap).map as *mut cmap12);
                }
                _ => {
                    _tt_abort(
                        b"Unrecognized OpenType/TrueType cmap format.\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
        free(cmap as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tt_cmap_lookup(mut cmap: *mut tt_cmap, mut cc: SFNT_ULONG) -> USHORT {
    let mut gid: USHORT = 0i32 as USHORT;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-tt_cmap.c\x00" as *const u8 as *const libc::c_char,
            560i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"USHORT tt_cmap_lookup(tt_cmap *, SFNT_ULONG)\x00",
            ))
            .as_ptr(),
        );
    }
    if cc as libc::c_long > 0xffffi64 && ((*cmap).format as libc::c_int) < 12i32 {
        dpx_warning(
            b"Four bytes charcode not supported in OpenType/TrueType cmap format 0...6.\x00"
                as *const u8 as *const libc::c_char,
        );
        return 0i32 as USHORT;
    }
    match (*cmap).format as libc::c_int {
        0 => gid = lookup_cmap0((*cmap).map as *mut cmap0, cc as USHORT),
        2 => gid = lookup_cmap2((*cmap).map as *mut cmap2, cc as USHORT),
        4 => gid = lookup_cmap4((*cmap).map as *mut cmap4, cc as USHORT),
        6 => gid = lookup_cmap6((*cmap).map as *mut cmap6, cc as USHORT),
        12 => gid = lookup_cmap12((*cmap).map as *mut cmap12, cc),
        _ => {
            _tt_abort(
                b"Unrecognized OpenType/TrueType cmap subtable format\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return gid;
}
static mut wbuf: [libc::c_uchar; 1024] = [0; 1024];
static mut srange_min: [libc::c_uchar; 2] = [0i32 as libc::c_uchar, 0i32 as libc::c_uchar];
static mut srange_max: [libc::c_uchar; 2] = [0xffi32 as libc::c_uchar, 0xffi32 as libc::c_uchar];
static mut lrange_min: [libc::c_uchar; 4] = [
    0i32 as libc::c_uchar,
    0i32 as libc::c_uchar,
    0i32 as libc::c_uchar,
    0i32 as libc::c_uchar,
];
static mut lrange_max: [libc::c_uchar; 4] = [
    0x7fi32 as libc::c_uchar,
    0xffi32 as libc::c_uchar,
    0xffi32 as libc::c_uchar,
    0xffi32 as libc::c_uchar,
];
unsafe extern "C" fn load_cmap4(
    mut map: *mut cmap4,
    mut GIDToCIDMap: *mut libc::c_uchar,
    mut gsub_vert: *mut otl_gsub,
    mut gsub_list: *mut otl_gsub,
    mut cmap: *mut CMap,
    mut tounicode_add: *mut CMap,
) {
    let mut c0: USHORT = 0;
    let mut c1: USHORT = 0;
    let mut gid: USHORT = 0;
    let mut cid: USHORT = 0;
    let mut j: USHORT = 0;
    let mut d: USHORT = 0;
    let mut segCount: USHORT = 0;
    let mut ch: USHORT = 0;
    let mut i: libc::c_int = 0;
    segCount = ((*map).segCountX2 as libc::c_int / 2i32) as USHORT;
    i = segCount as libc::c_int - 1i32;
    while i >= 0i32 {
        c0 = *(*map).startCount.offset(i as isize);
        c1 = *(*map).endCount.offset(i as isize);
        d = (*(*map).idRangeOffset.offset(i as isize) as libc::c_int / 2i32
            - (segCount as libc::c_int - i)) as USHORT;
        j = 0i32 as USHORT;
        while j as libc::c_int <= c1 as libc::c_int - c0 as libc::c_int {
            ch = (c0 as libc::c_int + j as libc::c_int) as USHORT;
            if *(*map).idRangeOffset.offset(i as isize) as libc::c_int == 0i32 {
                gid = (ch as libc::c_int + *(*map).idDelta.offset(i as isize) as libc::c_int
                    & 0xffffi32) as USHORT
            } else if c0 as libc::c_int == 0xffffi32
                && c1 as libc::c_int == 0xffffi32
                && *(*map).idRangeOffset.offset(i as isize) as libc::c_int == 0xffffi32
            {
                /* this is for protection against some old broken fonts... */
                gid = 0i32 as USHORT
            } else {
                gid = (*(*map)
                    .glyphIndexArray
                    .offset((j as libc::c_int + d as libc::c_int) as isize)
                    as libc::c_int
                    + *(*map).idDelta.offset(i as isize) as libc::c_int
                    & 0xffffi32) as USHORT
            } /* LONG ? */
            if gid as libc::c_int != 0i32 && gid as libc::c_int != 0xffffi32 {
                if !gsub_list.is_null() {
                    otl_gsub_apply_chain(gsub_list, &mut gid);
                }
                if !gsub_vert.is_null() {
                    otl_gsub_apply(gsub_vert, &mut gid);
                }
                if !GIDToCIDMap.is_null() {
                    cid = ((*GIDToCIDMap.offset((2i32 * gid as libc::c_int) as isize)
                        as libc::c_int)
                        << 8i32
                        | *GIDToCIDMap.offset((2i32 * gid as libc::c_int + 1i32) as isize)
                            as libc::c_int) as USHORT;
                    if cid as libc::c_int == 0i32 {
                        dpx_warning(
                            b"GID %u does not have corresponding CID %u.\x00" as *const u8
                                as *const libc::c_char,
                            gid as libc::c_int,
                            cid as libc::c_int,
                        );
                    }
                } else {
                    cid = gid
                }
                wbuf[0] = 0i32 as libc::c_uchar;
                wbuf[1] = 0i32 as libc::c_uchar;
                wbuf[2] = (ch as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                wbuf[3] = (ch as libc::c_int & 0xffi32) as libc::c_uchar;
                wbuf[4] = (cid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                wbuf[5] = (cid as libc::c_int & 0xffi32) as libc::c_uchar;
                CMap_add_cidchar(cmap, wbuf.as_mut_ptr(), 4i32 as size_t, cid);
                if !tounicode_add.is_null() {
                    let mut p: *mut libc::c_uchar = wbuf.as_mut_ptr().offset(6);
                    let mut uc_len: size_t = 0;
                    uc_len = UC_UTF16BE_encode_char(
                        ch as int32_t,
                        &mut p,
                        wbuf.as_mut_ptr().offset(1024).offset(-1),
                    );
                    CMap_add_bfchar(
                        tounicode_add,
                        wbuf.as_mut_ptr().offset(4),
                        2i32 as size_t,
                        wbuf.as_mut_ptr().offset(6),
                        uc_len,
                    );
                }
            }
            j = j.wrapping_add(1)
        }
        i -= 1
    }
}
unsafe extern "C" fn load_cmap12(
    mut map: *mut cmap12,
    mut GIDToCIDMap: *mut libc::c_uchar,
    mut gsub_vert: *mut otl_gsub,
    mut gsub_list: *mut otl_gsub,
    mut cmap: *mut CMap,
    mut tounicode_add: *mut CMap,
) {
    let mut i: SFNT_ULONG = 0;
    let mut ch: SFNT_ULONG = 0;
    let mut gid: USHORT = 0;
    let mut cid: USHORT = 0;
    i = 0i32 as SFNT_ULONG;
    while i < (*map).nGroups {
        ch = (*(*map).groups.offset(i as isize)).startCharCode;
        while ch <= (*(*map).groups.offset(i as isize)).endCharCode {
            let mut d: libc::c_int =
                ch.wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode) as libc::c_int;
            gid = ((*(*map).groups.offset(i as isize))
                .startGlyphID
                .wrapping_add(d as libc::c_uint)
                & 0xffffi32 as libc::c_uint) as USHORT;
            if !gsub_list.is_null() {
                otl_gsub_apply_chain(gsub_list, &mut gid);
            }
            if !gsub_vert.is_null() {
                otl_gsub_apply(gsub_vert, &mut gid);
            }
            if !GIDToCIDMap.is_null() {
                cid = ((*GIDToCIDMap.offset((2i32 * gid as libc::c_int) as isize) as libc::c_int)
                    << 8i32
                    | *GIDToCIDMap.offset((2i32 * gid as libc::c_int + 1i32) as isize)
                        as libc::c_int) as USHORT;
                if cid as libc::c_int == 0i32 {
                    dpx_warning(
                        b"GID %u does not have corresponding CID %u.\x00" as *const u8
                            as *const libc::c_char,
                        gid as libc::c_int,
                        cid as libc::c_int,
                    );
                }
            } else {
                cid = gid
            }
            wbuf[0] = (ch >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
            wbuf[1] = (ch >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
            wbuf[2] = (ch >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
            wbuf[3] = (ch & 0xffi32 as libc::c_uint) as libc::c_uchar;
            wbuf[4] = (cid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
            wbuf[5] = (cid as libc::c_int & 0xffi32) as libc::c_uchar;
            CMap_add_cidchar(cmap, wbuf.as_mut_ptr(), 4i32 as size_t, cid);
            if !tounicode_add.is_null() {
                let mut p: *mut libc::c_uchar = wbuf.as_mut_ptr().offset(6);
                let mut uc_len: size_t = 0;
                uc_len = UC_UTF16BE_encode_char(
                    ch as int32_t,
                    &mut p,
                    wbuf.as_mut_ptr().offset(1024).offset(-1),
                );
                CMap_add_bfchar(
                    tounicode_add,
                    wbuf.as_mut_ptr().offset(4),
                    2i32 as size_t,
                    wbuf.as_mut_ptr().offset(6),
                    uc_len,
                );
            }
            ch = ch.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
}
/* OpenType CIDFont:
 *
 *  We don't use GID for them. OpenType cmap table is for
 *  charcode to GID mapping rather than to-CID mapping.
 */
unsafe extern "C" fn handle_CIDFont(
    mut sfont: *mut sfnt,
    mut GIDToCIDMap: *mut *mut libc::c_uchar,
    mut csi: *mut CIDSysInfo,
) -> libc::c_int {
    let mut cffont: *mut cff_font = 0 as *mut cff_font; /* CID... */
    let mut offset: libc::c_int = 0; /* card8 */
    let mut i: libc::c_int = 0;
    let mut num_glyphs: card16 = 0;
    let mut gid: card16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut map: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    if !csi.is_null() {
    } else {
        __assert_fail(
            b"csi\x00" as *const u8 as *const libc::c_char,
            b"dpx-tt_cmap.c\x00" as *const u8 as *const libc::c_char,
            728i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"int handle_CIDFont(sfnt *, unsigned char **, CIDSysInfo *)\x00",
            ))
            .as_ptr(),
        );
    }
    offset =
        sfnt_find_table_pos(sfont, b"CFF \x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if offset == 0i32 {
        (*csi).registry = 0 as *mut libc::c_char;
        (*csi).ordering = 0 as *mut libc::c_char;
        *GIDToCIDMap = 0 as *mut libc::c_uchar;
        return 0i32;
    }
    maxp = tt_read_maxp_table(sfont);
    num_glyphs = (*maxp).numGlyphs;
    free(maxp as *mut libc::c_void);
    if (num_glyphs as libc::c_int) < 1i32 {
        _tt_abort(b"No glyph contained in this font...\x00" as *const u8 as *const libc::c_char);
    }
    cffont = cff_open((*sfont).handle, offset, 0i32);
    if cffont.is_null() {
        _tt_abort(b"Could not open CFF font...\x00" as *const u8 as *const libc::c_char);
    }
    if (*cffont).flag & 1i32 << 0i32 == 0 {
        cff_close(cffont);
        (*csi).registry = 0 as *mut libc::c_char;
        (*csi).ordering = 0 as *mut libc::c_char;
        *GIDToCIDMap = 0 as *mut libc::c_uchar;
        return 0i32;
    }
    if cff_dict_known(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        _tt_abort(b"No CIDSystemInfo???\x00" as *const u8 as *const libc::c_char);
    } else {
        let mut reg: card16 = 0;
        let mut ord: card16 = 0;
        reg = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const libc::c_char,
            0i32,
        ) as card16;
        ord = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const libc::c_char,
            1i32,
        ) as card16;
        (*csi).registry = cff_get_string(cffont, reg);
        (*csi).ordering = cff_get_string(cffont, ord);
        (*csi).supplement = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const libc::c_char,
            2i32,
        ) as libc::c_int
    }
    cff_read_charsets(cffont);
    charset = (*cffont).charsets;
    if charset.is_null() {
        _tt_abort(b"No CFF charset data???\x00" as *const u8 as *const libc::c_char);
    }
    map = new(
        ((num_glyphs as libc::c_int * 2i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as uint32_t,
    ) as *mut libc::c_uchar;
    memset(
        map as *mut libc::c_void,
        0i32,
        (num_glyphs as libc::c_int * 2i32) as libc::c_ulong,
    );
    match (*charset).format as libc::c_int {
        0 => {
            let mut cids: *mut s_SID = 0 as *mut s_SID;
            cids = (*charset).data.glyphs;
            gid = 1i32 as card16;
            i = 0i32;
            while i < (*charset).num_entries as libc::c_int {
                *map.offset((2i32 * gid as libc::c_int) as isize) =
                    (*cids.offset(i as isize) as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                *map.offset((2i32 * gid as libc::c_int + 1i32) as isize) =
                    (*cids.offset(i as isize) as libc::c_int & 0xffi32) as libc::c_uchar;
                gid = gid.wrapping_add(1);
                i += 1
            }
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            let mut cid: card16 = 0;
            let mut count: card16 = 0;
            ranges = (*charset).data.range1;
            gid = 1i32 as card16;
            i = 0i32;
            while i < (*charset).num_entries as libc::c_int {
                cid = (*ranges.offset(i as isize)).first;
                count = ((*ranges.offset(i as isize)).n_left as libc::c_int + 1i32) as card16;
                loop {
                    let fresh3 = count;
                    count = count.wrapping_sub(1);
                    if !(fresh3 as libc::c_int > 0i32
                        && gid as libc::c_int <= num_glyphs as libc::c_int)
                    {
                        break;
                    }
                    *map.offset((2i32 * gid as libc::c_int) as isize) =
                        (cid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                    *map.offset((2i32 * gid as libc::c_int + 1i32) as isize) =
                        (cid as libc::c_int & 0xffi32) as libc::c_uchar;
                    gid = gid.wrapping_add(1);
                    cid = cid.wrapping_add(1)
                }
                i += 1
            }
        }
        2 => {
            let mut ranges_0: *mut cff_range2 = 0 as *mut cff_range2;
            let mut cid_0: card16 = 0;
            let mut count_0: card16 = 0;
            ranges_0 = (*charset).data.range2;
            if (*charset).num_entries as libc::c_int == 1i32
                && (*ranges_0.offset(0)).first as libc::c_int == 1i32
            {
                /* "Complete" CIDFont */
                map = mfree(map as *mut libc::c_void) as *mut libc::c_uchar
            } else {
                /* Not trivial mapping */
                gid = 1i32 as card16;
                i = 0i32;
                while i < (*charset).num_entries as libc::c_int {
                    cid_0 = (*ranges_0.offset(i as isize)).first;
                    count_0 =
                        ((*ranges_0.offset(i as isize)).n_left as libc::c_int + 1i32) as card16;
                    loop {
                        let fresh4 = count_0;
                        count_0 = count_0.wrapping_sub(1);
                        if !(fresh4 as libc::c_int > 0i32
                            && gid as libc::c_int <= num_glyphs as libc::c_int)
                        {
                            break;
                        }
                        *map.offset((2i32 * gid as libc::c_int) as isize) =
                            (cid_0 as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                        *map.offset((2i32 * gid as libc::c_int + 1i32) as isize) =
                            (cid_0 as libc::c_int & 0xffi32) as libc::c_uchar;
                        gid = gid.wrapping_add(1);
                        cid_0 = cid_0.wrapping_add(1)
                    }
                    i += 1
                }
            }
        }
        _ => {
            map = mfree(map as *mut libc::c_void) as *mut libc::c_uchar;
            _tt_abort(
                b"Unknown CFF charset format...: %d\x00" as *const u8 as *const libc::c_char,
                (*charset).format as libc::c_int,
            );
        }
    }
    cff_close(cffont);
    *GIDToCIDMap = map;
    return 1i32;
}
unsafe extern "C" fn is_PUA_or_presentation(mut uni: libc::c_uint) -> bool {
    /* KANGXI RADICALs are commonly double encoded. */
    return uni >= 0x2f00i32 as libc::c_uint && uni <= 0x2fd5i32 as libc::c_uint
        || uni >= 0xe000i32 as libc::c_uint && uni <= 0xf8ffi32 as libc::c_uint
        || uni >= 0xfb00i32 as libc::c_uint && uni <= 0xfb4fi32 as libc::c_uint
        || uni >= 0xf0000i32 as libc::c_uint && uni <= 0xffffdi32 as libc::c_uint
        || uni >= 0x100000i32 as libc::c_uint && uni <= 0x10fffdi32 as libc::c_uint;
}
unsafe extern "C" fn sfnt_get_glyphname(
    mut post: *mut tt_post_table,
    mut cffont: *mut cff_font,
    mut gid: USHORT,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if !post.is_null() {
        name = tt_get_glyphname(post, gid)
    }
    if name.is_null() && !cffont.is_null() {
        name = cff_get_glyphname(cffont, gid)
    }
    return name;
}
/*
 * Substituted glyphs:
 *
 *  Mapping information stored in cmap_add.
 */
unsafe extern "C" fn handle_subst_glyphs(
    mut cmap: *mut CMap,
    mut cmap_add: *mut CMap,
    mut used_glyphs: *const libc::c_char,
    mut sfont: *mut sfnt,
    mut cffont: *mut cff_font,
) -> USHORT {
    let mut count: USHORT = 0;
    let mut i: USHORT = 0;
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    if cmap_add.is_null() {
        post = tt_read_post_table(sfont)
    }
    count = 0i32 as USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < 8192i32 {
        let mut j: libc::c_uint = 0;
        let mut len: size_t = 0;
        let mut inbytesleft: size_t = 0;
        let mut outbytesleft: size_t = 0;
        let mut inbuf: *const libc::c_uchar = 0 as *const libc::c_uchar;
        let mut outbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if !(*used_glyphs.offset(i as isize) as libc::c_int == 0i32) {
            j = 0i32 as libc::c_uint;
            while j < 8i32 as libc::c_uint {
                let mut gid: USHORT =
                    ((8i32 * i as libc::c_int) as libc::c_uint).wrapping_add(j) as USHORT;
                if !(*used_glyphs.offset((gid as libc::c_int / 8i32) as isize) as libc::c_int
                    & 1i32 << 7i32 - gid as libc::c_int % 8i32
                    == 0)
                {
                    if cmap_add.is_null() {
                        /* try to look up Unicode values from the glyph name... */
                        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut unicodes: [int32_t; 16] = [0; 16];
                        let mut unicode_count: libc::c_int = -1i32;
                        name = sfnt_get_glyphname(post, cffont, gid);
                        if !name.is_null() {
                            unicode_count = agl_get_unicodes(name, unicodes.as_mut_ptr(), 16i32)
                        }
                        if unicode_count == -1i32 {
                            if !name.is_null() {
                                dpx_message(
                                    b"No Unicode mapping available: GID=%u, name=%s\n\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                    gid as libc::c_int,
                                    name,
                                );
                            } else {
                                dpx_message(
                                    b"No Unicode mapping available: GID=%u\n\x00" as *const u8
                                        as *const libc::c_char,
                                    gid as libc::c_int,
                                );
                            }
                        } else {
                            /* the Unicode characters go into wbuf[2] and following, in UTF16BE */
                            /* we rely on WBUF_SIZE being more than adequate for MAX_UNICODES  */
                            let mut p: *mut libc::c_uchar = wbuf.as_mut_ptr().offset(2);
                            let mut k: libc::c_int = 0;
                            len = 0i32 as size_t;
                            k = 0i32;
                            while k < unicode_count {
                                len = (len as libc::c_ulong).wrapping_add(UC_UTF16BE_encode_char(
                                    unicodes[k as usize],
                                    &mut p,
                                    wbuf.as_mut_ptr().offset(1024),
                                )) as size_t as size_t;
                                k += 1
                            }
                            wbuf[0] = (gid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                            wbuf[1] = (gid as libc::c_int & 0xffi32) as libc::c_uchar;
                            CMap_add_bfchar(
                                cmap,
                                wbuf.as_mut_ptr(),
                                2i32 as size_t,
                                wbuf.as_mut_ptr().offset(2),
                                len,
                            );
                        }
                        free(name as *mut libc::c_void);
                    } else {
                        wbuf[0] = (gid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                        wbuf[1] = (gid as libc::c_int & 0xffi32) as libc::c_uchar;
                        inbuf = wbuf.as_mut_ptr();
                        inbytesleft = 2i32 as size_t;
                        outbuf = wbuf.as_mut_ptr().offset(2);
                        outbytesleft = (1024i32 - 2i32) as size_t;
                        CMap_decode(
                            cmap_add,
                            &mut inbuf,
                            &mut inbytesleft,
                            &mut outbuf,
                            &mut outbytesleft,
                        );
                        if inbytesleft != 0i32 as libc::c_ulong {
                            dpx_warning(
                                b"CMap conversion failed...\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            len = ((1024i32 - 2i32) as libc::c_ulong).wrapping_sub(outbytesleft);
                            CMap_add_bfchar(
                                cmap,
                                wbuf.as_mut_ptr(),
                                2i32 as size_t,
                                wbuf.as_mut_ptr().offset(2),
                                len,
                            );
                            count = count.wrapping_add(1);
                            if verbose > 0i32 {
                                let mut _i: size_t = 0;
                                dpx_message(
                                    b"otf_cmap>> Additional ToUnicode mapping: <%04X> <\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                    gid as libc::c_int,
                                );
                                _i = 0i32 as size_t;
                                while _i < len {
                                    dpx_message(
                                        b"%02X\x00" as *const u8 as *const libc::c_char,
                                        wbuf[(2i32 as libc::c_ulong).wrapping_add(_i) as usize]
                                            as libc::c_int,
                                    );
                                    _i = _i.wrapping_add(1)
                                }
                                dpx_message(b">\n\x00" as *const u8 as *const libc::c_char);
                            }
                        }
                    }
                }
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    if !post.is_null() {
        tt_release_post_table(post);
    }
    return count;
}
unsafe extern "C" fn prepare_CIDFont_from_sfnt(mut sfont: *mut sfnt) -> *mut cff_font {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut offset: libc::c_uint = 0i32 as libc::c_uint;
    if (*sfont).type_0 != 1i32 << 2i32
        || sfnt_read_table_directory(sfont, 0i32 as SFNT_ULONG) < 0i32
        || {
            offset = sfnt_find_table_pos(sfont, b"CFF \x00" as *const u8 as *const libc::c_char);
            offset == 0i32 as libc::c_uint
        }
    {
        return 0 as *mut cff_font;
    }
    cffont = cff_open((*sfont).handle, offset as libc::c_int, 0i32);
    if cffont.is_null() {
        return 0 as *mut cff_font;
    }
    cff_read_charsets(cffont);
    return cffont;
}
unsafe extern "C" fn add_to_cmap_if_used(
    mut cmap: *mut CMap,
    mut cffont: *mut cff_font,
    mut used_chars: *mut libc::c_char,
    mut gid: USHORT,
    mut ch: SFNT_ULONG,
) -> USHORT {
    let mut count: USHORT = 0i32 as USHORT;
    let mut cid: USHORT = (if !cffont.is_null() {
        cff_charsets_lookup_inverse(cffont, gid) as libc::c_int
    } else {
        gid as libc::c_int
    }) as USHORT;
    /* Skip PUA characters and alphabetic presentation forms, allowing
     * handle_subst_glyphs() as it might find better mapping. Fixes the
     * mapping of ligatures encoded in PUA in fonts like Linux Libertine
     * and old Adobe fonts.
     */
    if *used_chars.offset((cid as libc::c_int / 8i32) as isize) as libc::c_int
        & 1i32 << 7i32 - cid as libc::c_int % 8i32
        != 0
        && !is_PUA_or_presentation(ch)
    {
        let mut len: libc::c_int = 0;
        let mut p: *mut libc::c_uchar = wbuf.as_mut_ptr().offset(2);
        count = count.wrapping_add(1);
        wbuf[0] = (cid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
        wbuf[1] = (cid as libc::c_int & 0xffi32) as libc::c_uchar;
        len = UC_UTF16BE_encode_char(ch as int32_t, &mut p, wbuf.as_mut_ptr().offset(1024))
            as libc::c_int;
        CMap_add_bfchar(
            cmap,
            wbuf.as_mut_ptr(),
            2i32 as size_t,
            wbuf.as_mut_ptr().offset(2),
            len as size_t,
        );
        /* Avoid duplicate entry
         * There are problem when two Unicode code is mapped to
         * single glyph...
         */
        let ref mut fresh5 = *used_chars.offset((cid as libc::c_int / 8i32) as isize);
        *fresh5 =
            (*fresh5 as libc::c_int & !(1i32 << 7i32 - cid as libc::c_int % 8i32)) as libc::c_char
    }
    return count;
}
unsafe extern "C" fn create_ToUnicode_cmap4(
    mut cmap: *mut CMap,
    mut map: *mut cmap4,
    mut used_chars: *mut libc::c_char,
    mut cffont: *mut cff_font,
) -> USHORT {
    let mut count: USHORT = 0i32 as USHORT;
    let mut segCount: USHORT = ((*map).segCountX2 as libc::c_int / 2i32) as USHORT;
    let mut i: USHORT = 0;
    let mut j: USHORT = 0;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < segCount as libc::c_int {
        let mut c0: USHORT = *(*map).startCount.offset(i as isize);
        let mut c1: USHORT = *(*map).endCount.offset(i as isize);
        let mut d: USHORT = (*(*map).idRangeOffset.offset(i as isize) as libc::c_int / 2i32
            - (segCount as libc::c_int - i as libc::c_int)) as USHORT;
        j = 0i32 as USHORT;
        while j as libc::c_int <= c1 as libc::c_int - c0 as libc::c_int {
            let mut ch: USHORT = (c0 as libc::c_int + j as libc::c_int) as USHORT;
            let mut gid: USHORT = 0;
            if *(*map).idRangeOffset.offset(i as isize) as libc::c_int == 0i32 {
                gid = (ch as libc::c_int + *(*map).idDelta.offset(i as isize) as libc::c_int
                    & 0xffffi32) as USHORT
            } else if c0 as libc::c_int == 0xffffi32
                && c1 as libc::c_int == 0xffffi32
                && *(*map).idRangeOffset.offset(i as isize) as libc::c_int == 0xffffi32
            {
                /* this is for protection against some old broken fonts... */
                gid = 0i32 as USHORT
            } else {
                gid = (*(*map)
                    .glyphIndexArray
                    .offset((j as libc::c_int + d as libc::c_int) as isize)
                    as libc::c_int
                    + *(*map).idDelta.offset(i as isize) as libc::c_int
                    & 0xffffi32) as USHORT
            }
            count = (count as libc::c_int
                + add_to_cmap_if_used(cmap, cffont, used_chars, gid, ch as SFNT_ULONG)
                    as libc::c_int) as USHORT;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return count;
}
unsafe extern "C" fn create_ToUnicode_cmap12(
    mut cmap: *mut CMap,
    mut map: *mut cmap12,
    mut used_chars: *mut libc::c_char,
    mut cffont: *mut cff_font,
) -> USHORT {
    let mut i: SFNT_ULONG = 0;
    let mut ch: SFNT_ULONG = 0;
    let mut count: SFNT_ULONG = 0i32 as SFNT_ULONG;
    i = 0i32 as SFNT_ULONG;
    while i < (*map).nGroups {
        ch = (*(*map).groups.offset(i as isize)).startCharCode;
        while ch <= (*(*map).groups.offset(i as isize)).endCharCode {
            let mut d: libc::c_int =
                ch.wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode) as libc::c_int;
            let mut gid: USHORT = ((*(*map).groups.offset(i as isize))
                .startGlyphID
                .wrapping_add(d as libc::c_uint)
                & 0xffffi32 as libc::c_uint) as USHORT;
            count =
                (count as libc::c_uint).wrapping_add(add_to_cmap_if_used(
                    cmap, cffont, used_chars, gid, ch,
                ) as libc::c_uint) as SFNT_ULONG as SFNT_ULONG;
            ch = ch.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return count as USHORT;
}
unsafe extern "C" fn create_ToUnicode_cmap(
    mut ttcmap: *mut tt_cmap,
    mut cmap_name: *const libc::c_char,
    mut cmap_add: *mut CMap,
    mut used_chars: *const libc::c_char,
    mut sfont: *mut sfnt,
    mut code_to_cid_cmap: *mut CMap,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut count: USHORT = 0i32 as USHORT;
    let mut cffont: *mut cff_font = prepare_CIDFont_from_sfnt(sfont);
    let mut is_cidfont: libc::c_char =
        (!cffont.is_null() && (*cffont).flag & 1i32 << 0i32 != 0) as libc::c_int as libc::c_char;
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_wmode(cmap, 0i32);
    CMap_set_type(cmap, 2i32);
    CMap_set_CIDSysInfo(cmap, &mut CSI_UNICODE);
    CMap_add_codespacerange(
        cmap,
        srange_min.as_mut_ptr(),
        srange_max.as_mut_ptr(),
        2i32 as size_t,
    );
    /* cmap_add here stores information about all unencoded glyphs which can be
     * accessed only through OT Layout GSUB table.
     */
    if !code_to_cid_cmap.is_null()
        && !cffont.is_null()
        && is_cidfont as libc::c_int != 0
        && cmap_add.is_null()
    {
        let mut i: USHORT = 0;
        i = 0i32 as USHORT;
        while (i as libc::c_int) < 8192i32 {
            let mut j: libc::c_int = 0;
            if !(*used_chars.offset(i as isize) as libc::c_int == 0i32) {
                j = 0i32;
                while j < 8i32 {
                    let mut cid: USHORT = (8i32 * i as libc::c_int + j) as USHORT;
                    let mut ch: libc::c_int = 0;
                    if !(*used_chars.offset((cid as libc::c_int / 8i32) as isize) as libc::c_int
                        & 1i32 << 7i32 - cid as libc::c_int % 8i32
                        == 0)
                    {
                        ch = CMap_reverse_decode(code_to_cid_cmap, cid);
                        if ch >= 0i32 {
                            let mut len: libc::c_int = 0;
                            let mut p: *mut libc::c_uchar = wbuf.as_mut_ptr().offset(2);
                            wbuf[0] = (cid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                            wbuf[1] = (cid as libc::c_int & 0xffi32) as libc::c_uchar;
                            len = UC_UTF16BE_encode_char(ch, &mut p, wbuf.as_mut_ptr().offset(1024))
                                as libc::c_int;
                            CMap_add_bfchar(
                                cmap,
                                wbuf.as_mut_ptr(),
                                2i32 as size_t,
                                wbuf.as_mut_ptr().offset(2),
                                len as size_t,
                            );
                            count = count.wrapping_add(1)
                        }
                    }
                    j += 1
                }
            }
            i = i.wrapping_add(1)
        }
    } else {
        let mut used_chars_copy: [libc::c_char; 8192] = [0; 8192];
        memcpy(
            used_chars_copy.as_mut_ptr() as *mut libc::c_void,
            used_chars as *const libc::c_void,
            8192i32 as libc::c_ulong,
        );
        /* For create_ToUnicode_cmap{4,12}(), cffont is for GID -> CID lookup,
         * so it is only needed for CID fonts. */
        match (*ttcmap).format as libc::c_int {
            4 => {
                count = create_ToUnicode_cmap4(
                    cmap,
                    (*ttcmap).map as *mut cmap4,
                    used_chars_copy.as_mut_ptr(),
                    if is_cidfont as libc::c_int != 0 {
                        cffont
                    } else {
                        0 as *mut cff_font
                    },
                )
            }
            12 => {
                count = create_ToUnicode_cmap12(
                    cmap,
                    (*ttcmap).map as *mut cmap12,
                    used_chars_copy.as_mut_ptr(),
                    if is_cidfont as libc::c_int != 0 {
                        cffont
                    } else {
                        0 as *mut cff_font
                    },
                )
            }
            _ => {}
        }
        /* For handle_subst_glyphs(), cffont is for GID -> glyph name lookup, so
         * it is only needed for non-CID fonts. */
        count = (count as libc::c_int
            + handle_subst_glyphs(
                cmap,
                cmap_add,
                used_chars_copy.as_mut_ptr(),
                sfont,
                if is_cidfont as libc::c_int != 0 {
                    0 as *mut cff_font
                } else {
                    cffont
                },
            ) as libc::c_int) as USHORT
    }
    if (count as libc::c_int) < 1i32 {
        stream = 0 as *mut pdf_obj
    } else {
        stream = CMap_create_stream(cmap)
    }
    CMap_release(cmap);
    if !cffont.is_null() {
        cff_close(cffont);
    }
    return stream;
}
static mut cmap_plat_encs: [cmap_plat_enc_rec; 5] = [
    {
        let mut init = cmap_plat_enc_rec {
            platform: 3i32 as libc::c_short,
            encoding: 10i32 as libc::c_short,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 0i32 as libc::c_short,
            encoding: 3i32 as libc::c_short,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 0i32 as libc::c_short,
            encoding: 0i32 as libc::c_short,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 3i32 as libc::c_short,
            encoding: 1i32 as libc::c_short,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 0i32 as libc::c_short,
            encoding: 1i32 as libc::c_short,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn otf_create_ToUnicode_stream(
    mut font_name: *const libc::c_char,
    mut ttc_index: libc::c_int,
    mut used_chars: *const libc::c_char,
    mut cmap_id: libc::c_int,
) -> *mut pdf_obj {
    let mut cmap_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut res_id: libc::c_int = 0;
    let mut cmap_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap_add: *mut CMap = 0 as *mut CMap;
    let mut code_to_cid_cmap: *mut CMap = 0 as *mut CMap;
    let mut cmap_add_id: libc::c_int = 0;
    let mut ttcmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut normalized_font_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmap_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmap_add_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut cmap_type: libc::c_int = 0;
    let mut i: size_t = 0;
    /* replace slash in map name with dash to make the output cmap name valid,
     * happens when XeTeX embeds full font path
     * https://sourceforge.net/p/xetex/bugs/52/
     */
    normalized_font_name = new(
        (strlen(font_name).wrapping_add(1i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t,
    ) as *mut libc::c_char; /* many warnings without this... */
    strcpy(normalized_font_name, font_name);
    i = 0i32 as size_t;
    while i < strlen(font_name) {
        if *normalized_font_name.offset(i as isize) as libc::c_int == '/' as i32 {
            *normalized_font_name.offset(i as isize) = '-' as i32 as libc::c_char
        }
        i = i.wrapping_add(1)
    }
    cmap_name = new((strlen(font_name)
        .wrapping_add(strlen(b"-UTF16\x00" as *const u8 as *const libc::c_char))
        .wrapping_add(5i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    sprintf(
        cmap_name,
        b"%s,%03d-UTF16\x00" as *const u8 as *const libc::c_char,
        normalized_font_name,
        ttc_index,
    );
    free(normalized_font_name as *mut libc::c_void);
    res_id = pdf_findresource(b"CMap\x00" as *const u8 as *const libc::c_char, cmap_name);
    if res_id >= 0i32 {
        free(cmap_name as *mut libc::c_void);
        cmap_ref = pdf_get_resource_reference(res_id);
        return cmap_ref;
    }
    if verbose > 0i32 {
        dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
        dpx_message(
            b"otf_cmap>> Creating ToUnicode CMap for \"%s\"...\n\x00" as *const u8
                as *const libc::c_char,
            font_name,
        );
    }
    handle = dpx_open_truetype_file(font_name);
    if !handle.is_null() || {
        handle = dpx_open_opentype_file(font_name);
        !handle.is_null()
    } {
        sfont = sfnt_open(handle)
    } else {
        handle = dpx_open_dfont_file(font_name);
        if !handle.is_null() {
            sfont = dfont_open(handle, ttc_index)
        } else {
            free(cmap_name as *mut libc::c_void);
            return 0 as *mut pdf_obj;
        }
    }
    if sfont.is_null() {
        _tt_abort(
            b"Could not open OpenType/TrueType font file \"%s\"\x00" as *const u8
                as *const libc::c_char,
            font_name,
        );
    }
    match (*sfont).type_0 {
        256 => offset = (*sfont).offset,
        16 => {
            offset = ttc_read_offset(sfont, ttc_index);
            if offset == 0i32 as libc::c_uint {
                _tt_abort(b"Invalid TTC index\x00" as *const u8 as *const libc::c_char);
            }
        }
        _ => offset = 0i32 as SFNT_ULONG,
    }
    if sfnt_read_table_directory(sfont, offset) < 0i32 {
        _tt_abort(
            b"Could not read OpenType/TrueType table directory.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    code_to_cid_cmap = CMap_cache_get(cmap_id);
    cmap_type = CMap_get_type(code_to_cid_cmap);
    if cmap_type != 1i32 {
        code_to_cid_cmap = 0 as *mut CMap
    }
    cmap_add_name = new((strlen(font_name)
        .wrapping_add(strlen(
            b",000-UCS32-Add\x00" as *const u8 as *const libc::c_char,
        ))
        .wrapping_add(1i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    sprintf(
        cmap_add_name,
        b"%s,%03d-UCS32-Add\x00" as *const u8 as *const libc::c_char,
        font_name,
        ttc_index,
    );
    cmap_add_id = CMap_cache_find(cmap_add_name);
    free(cmap_add_name as *mut libc::c_void);
    if cmap_add_id < 0i32 {
        cmap_add = 0 as *mut CMap
    } else {
        cmap_add = CMap_cache_get(cmap_add_id)
    }
    CMap_set_silent(1i32);
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[cmap_plat_enc_rec; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmap_plat_enc_rec>() as libc::c_ulong)
    {
        ttcmap = tt_cmap_read(
            sfont,
            cmap_plat_encs[i as usize].platform as USHORT,
            cmap_plat_encs[i as usize].encoding as USHORT,
        );
        if !ttcmap.is_null() {
            if (*ttcmap).format as libc::c_int == 4i32 || (*ttcmap).format as libc::c_int == 12i32 {
                cmap_obj = create_ToUnicode_cmap(
                    ttcmap,
                    cmap_name,
                    cmap_add,
                    used_chars,
                    sfont,
                    code_to_cid_cmap,
                );
                break;
            }
        }
        i = i.wrapping_add(1)
    }
    if cmap_obj.is_null() {
        dpx_warning(
            b"Unable to read OpenType/TrueType Unicode cmap table.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    tt_cmap_release(ttcmap);
    CMap_set_silent(0i32);
    if !cmap_obj.is_null() {
        res_id = pdf_defineresource(
            b"CMap\x00" as *const u8 as *const libc::c_char,
            cmap_name,
            cmap_obj,
            1i32,
        );
        cmap_ref = pdf_get_resource_reference(res_id)
    } else {
        cmap_ref = 0 as *mut pdf_obj
    }
    free(cmap_name as *mut libc::c_void);
    sfnt_close(sfont);
    if !handle.is_null() {
        ttstub_input_close(handle);
    }
    return cmap_ref;
}
unsafe extern "C" fn load_base_CMap(
    mut cmap_name: *const libc::c_char,
    mut tounicode_add: *mut CMap,
    mut wmode: libc::c_int,
    mut csi: *mut CIDSysInfo,
    mut GIDToCIDMap: *mut libc::c_uchar,
    mut gsub_vert: *mut otl_gsub,
    mut gsub_list: *mut otl_gsub,
    mut ttcmap: *mut tt_cmap,
) -> libc::c_int {
    let mut cmap_id: libc::c_int = 0;
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id < 0i32 {
        let mut cmap: *mut CMap = 0 as *mut CMap;
        cmap = CMap_new();
        CMap_set_name(cmap, cmap_name);
        CMap_set_type(cmap, 1i32);
        CMap_set_wmode(cmap, wmode);
        CMap_add_codespacerange(
            cmap,
            lrange_min.as_mut_ptr(),
            lrange_max.as_mut_ptr(),
            4i32 as size_t,
        );
        if !csi.is_null() {
            /* CID */
            CMap_set_CIDSysInfo(cmap, csi);
        } else {
            CMap_set_CIDSysInfo(cmap, &mut CSI_IDENTITY);
        }
        if (*ttcmap).format as libc::c_int == 12i32 {
            load_cmap12(
                (*ttcmap).map as *mut cmap12,
                GIDToCIDMap,
                gsub_vert,
                gsub_list,
                cmap,
                tounicode_add,
            );
        } else if (*ttcmap).format as libc::c_int == 4i32 {
            load_cmap4(
                (*ttcmap).map as *mut cmap4,
                GIDToCIDMap,
                gsub_vert,
                gsub_list,
                cmap,
                tounicode_add,
            );
        }
        cmap_id = CMap_cache_add(cmap)
    }
    return cmap_id;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* TrueType cmap table */
/* or version, only for Mac */
/* Paltform ID */
/* Platform-specific encoding ID */
/* Windows */
/* Mac */
/* Indirect reference */
/* CMap ID */
#[no_mangle]
pub unsafe extern "C" fn otf_load_Unicode_CMap(
    mut map_name: *const libc::c_char,
    mut ttc_index: libc::c_int,
    mut otl_tags: *const libc::c_char,
    mut wmode: libc::c_int,
) -> libc::c_int {
    let mut cmap_id: libc::c_int = -1i32;
    /* Additional ToUnicode mappings required by OTL GSUB substitusion */
    let mut tounicode_add_id: libc::c_int = -1i32;
    let mut tounicode_add: *mut CMap = 0 as *mut CMap;
    let mut tounicode_add_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_cidfont: libc::c_int = 0i32;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut base_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmap_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut gsub_vert: *mut otl_gsub = 0 as *mut otl_gsub;
    let mut gsub_list: *mut otl_gsub = 0 as *mut otl_gsub;
    let mut ttcmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut csi: CIDSysInfo = {
        let mut init = CIDSysInfo {
            registry: 0 as *mut libc::c_char,
            ordering: 0 as *mut libc::c_char,
            supplement: 0i32,
        };
        init
    };
    let mut GIDToCIDMap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if map_name.is_null() {
        return -1i32;
    }
    if ttc_index > 999i32 || ttc_index < 0i32 {
        return -1i32;
        /* Sorry for this... */
    }
    handle = dpx_open_truetype_file(map_name) as *mut rust_input_handle_t;
    if handle.is_null() {
        handle = dpx_open_opentype_file(map_name) as *mut rust_input_handle_t
    }
    if handle.is_null() {
        handle = dpx_open_dfont_file(map_name) as *mut rust_input_handle_t;
        if handle.is_null() {
            return -1i32;
        }
        sfont = dfont_open(handle as rust_input_handle_t, ttc_index)
    } else {
        sfont = sfnt_open(handle as rust_input_handle_t)
    }
    if sfont.is_null() {
        _tt_abort(
            b"Could not open OpenType/TrueType/dfont font file \"%s\"\x00" as *const u8
                as *const libc::c_char,
            map_name,
        );
    }
    match (*sfont).type_0 {
        16 => {
            offset = ttc_read_offset(sfont, ttc_index);
            if offset == 0i32 as libc::c_uint {
                _tt_abort(b"Invalid TTC index\x00" as *const u8 as *const libc::c_char);
            }
        }
        1 | 4 => offset = 0i32 as SFNT_ULONG,
        256 => offset = (*sfont).offset,
        _ => {
            _tt_abort(
                b"Not a OpenType/TrueType/TTC font?: %s\x00" as *const u8 as *const libc::c_char,
                map_name,
            );
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0i32 {
        _tt_abort(
            b"Could not read OpenType/TrueType table directory.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    base_name = new((strlen(map_name)
        .wrapping_add(strlen(b"-UCS4-H\x00" as *const u8 as *const libc::c_char))
        .wrapping_add(5i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    if wmode != 0 {
        sprintf(
            base_name,
            b"%s,%03d-UCS4-V\x00" as *const u8 as *const libc::c_char,
            map_name,
            ttc_index,
        );
    } else {
        sprintf(
            base_name,
            b"%s,%03d-UCS4-H\x00" as *const u8 as *const libc::c_char,
            map_name,
            ttc_index,
        );
    }
    if !otl_tags.is_null() {
        cmap_name = new((strlen(map_name)
            .wrapping_add(strlen(otl_tags))
            .wrapping_add(strlen(b"-UCS4-H\x00" as *const u8 as *const libc::c_char))
            .wrapping_add(6i32 as libc::c_ulong) as uint32_t
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_char;
        if wmode != 0 {
            sprintf(
                cmap_name,
                b"%s,%03d,%s-UCS4-V\x00" as *const u8 as *const libc::c_char,
                map_name,
                ttc_index,
                otl_tags,
            );
        } else {
            sprintf(
                cmap_name,
                b"%s,%03d,%s-UCS4-H\x00" as *const u8 as *const libc::c_char,
                map_name,
                ttc_index,
                otl_tags,
            );
        }
        /* tounicode_add here is later refered by otf_create_ToUnicode_stream()
         * for finding additional CID to Unicode mapping entries required by
         * OTL gsub substitution.
         */
        tounicode_add_name = new((strlen(map_name)
            .wrapping_add(strlen(
                b",000-UCS32-Add\x00" as *const u8 as *const libc::c_char,
            ))
            .wrapping_add(1i32 as libc::c_ulong) as uint32_t
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_char; /* Microsoft UCS4 */
        sprintf(
            tounicode_add_name,
            b"%s,%03d-UCS32-Add\x00" as *const u8 as *const libc::c_char,
            map_name,
            ttc_index,
        ); /* Microsoft UCS2 */
        tounicode_add_id = CMap_cache_find(tounicode_add_name); /* Unicode 2.0 or later */
        if tounicode_add_id >= 0i32 {
            tounicode_add = CMap_cache_get(tounicode_add_id)
        } else {
            tounicode_add = CMap_new();
            CMap_set_name(tounicode_add, tounicode_add_name);
            CMap_set_type(tounicode_add, 2i32);
            CMap_set_wmode(tounicode_add, 0i32);
            CMap_add_codespacerange(
                tounicode_add,
                srange_min.as_mut_ptr(),
                srange_max.as_mut_ptr(),
                2i32 as size_t,
            );
            CMap_set_CIDSysInfo(tounicode_add, &mut CSI_UNICODE);
            CMap_add_bfchar(
                tounicode_add,
                srange_min.as_mut_ptr(),
                2i32 as size_t,
                srange_max.as_mut_ptr(),
                2i32 as size_t,
            );
            tounicode_add_id = CMap_cache_add(tounicode_add)
        }
        free(tounicode_add_name as *mut libc::c_void);
    } else {
        cmap_name = new(
            (strlen(base_name).wrapping_add(1i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as uint32_t,
        ) as *mut libc::c_char;
        strcpy(cmap_name, base_name);
    }
    if (*sfont).type_0 == 1i32 << 2i32 {
        is_cidfont = handle_CIDFont(sfont, &mut GIDToCIDMap, &mut csi)
    } else {
        is_cidfont = 0i32
    }
    if verbose > 0i32 {
        dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
        dpx_message(
            b"otf_cmap>> Unicode charmap for font=\"%s\" layout=\"%s\"\n\x00" as *const u8
                as *const libc::c_char,
            map_name,
            if !otl_tags.is_null() {
                otl_tags
            } else {
                b"none\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0i32 {
        free(cmap_name as *mut libc::c_void);
        free(base_name as *mut libc::c_void);
        free(GIDToCIDMap as *mut libc::c_void);
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        if verbose > 0i32 {
            dpx_message(
                b"otf_cmap>> Found at cmap_id=%d.\n\x00" as *const u8 as *const libc::c_char,
                cmap_id,
            );
        }
        return cmap_id;
    }
    ttcmap = tt_cmap_read(sfont, 3i32 as USHORT, 10i32 as USHORT);
    if ttcmap.is_null() {
        ttcmap = tt_cmap_read(sfont, 3i32 as USHORT, 1i32 as USHORT);
        if ttcmap.is_null() {
            ttcmap = tt_cmap_read(sfont, 0i32 as USHORT, 3i32 as USHORT);
            if ttcmap.is_null() {
                _tt_abort(
                    b"Unable to read OpenType/TrueType Unicode cmap table.\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if wmode == 1i32 {
        gsub_vert = otl_gsub_new();
        if otl_gsub_add_feat(
            gsub_vert,
            b"*\x00" as *const u8 as *const libc::c_char,
            b"*\x00" as *const u8 as *const libc::c_char,
            b"vrt2\x00" as *const u8 as *const libc::c_char,
            sfont,
        ) < 0i32
        {
            if otl_gsub_add_feat(
                gsub_vert,
                b"*\x00" as *const u8 as *const libc::c_char,
                b"*\x00" as *const u8 as *const libc::c_char,
                b"vert\x00" as *const u8 as *const libc::c_char,
                sfont,
            ) < 0i32
            {
                dpx_warning(
                    b"GSUB feature vrt2/vert not found.\x00" as *const u8 as *const libc::c_char,
                );
                otl_gsub_release(gsub_vert);
                gsub_vert = 0 as *mut otl_gsub
            } else {
                otl_gsub_select(
                    gsub_vert,
                    b"*\x00" as *const u8 as *const libc::c_char,
                    b"*\x00" as *const u8 as *const libc::c_char,
                    b"vert\x00" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            otl_gsub_select(
                gsub_vert,
                b"*\x00" as *const u8 as *const libc::c_char,
                b"*\x00" as *const u8 as *const libc::c_char,
                b"vrt2\x00" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        gsub_vert = 0 as *mut otl_gsub
    }
    if !otl_tags.is_null() {
        gsub_list = otl_gsub_new();
        if otl_gsub_add_feat_list(gsub_list, otl_tags, sfont) < 0i32 {
            dpx_warning(
                b"Reading GSUB feature table(s) failed for \"%s\"\x00" as *const u8
                    as *const libc::c_char,
                otl_tags,
            );
        } else {
            otl_gsub_set_chain(gsub_list, otl_tags);
        }
    } else {
        gsub_list = 0 as *mut otl_gsub
    }
    cmap_id = load_base_CMap(
        cmap_name,
        tounicode_add,
        wmode,
        if is_cidfont != 0 {
            &mut csi
        } else {
            0 as *mut CIDSysInfo
        },
        GIDToCIDMap,
        gsub_vert,
        gsub_list,
        ttcmap,
    );
    if cmap_id < 0i32 {
        _tt_abort(
            b"Failed to read OpenType/TrueType cmap table.\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !gsub_vert.is_null() {
        otl_gsub_release(gsub_vert);
    }
    gsub_vert = 0 as *mut otl_gsub;
    if !gsub_list.is_null() {
        otl_gsub_release(gsub_list);
    }
    gsub_list = 0 as *mut otl_gsub;
    free(cmap_name as *mut libc::c_void);
    free(base_name as *mut libc::c_void);
    free(GIDToCIDMap as *mut libc::c_void);
    if is_cidfont != 0 {
        free(csi.registry as *mut libc::c_void);
        free(csi.ordering as *mut libc::c_void);
    }
    tt_cmap_release(ttcmap);
    sfnt_close(sfont);
    ttstub_input_close(handle as rust_input_handle_t);
    return cmap_id;
}
