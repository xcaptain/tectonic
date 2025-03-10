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
#![allow(
    unused_mut
)]

use crate::dpx_mfileio::tt_mfgets;
use crate::dpx_mpost::mps_scan_bbox;
use crate::dpx_pdfdev::{pdf_dev_put_image, transform_info, transform_info_clear};
use crate::dpx_pdfobj::pdf_obj;
use crate::dpx_pdfparse::skip_white;
use crate::dpx_pdfximage::pdf_ximage_findresource;
use crate::shims::sscanf;
use crate::spc_warn;
use crate::DisplayExt;
use crate::TTInputFormat;
use crate::{ttstub_input_close, ttstub_input_open};
use libc::{memcpy, strlen, strncmp};
use std::ffi::CStr;

pub type size_t = u64;

use super::{spc_arg, spc_env};

pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
use super::spc_handler;

use crate::dpx_pdfximage::load_options;

/* quasi-hack to get the primary input */
unsafe extern "C" fn spc_handler_postscriptbox(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32 {
    let mut ti = transform_info::new();
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    let mut filename: [i8; 256] = [0; 256];
    let mut buf: [i8; 512] = [0; 512];
    assert!(!spe.is_null() && !ap.is_null());
    if (*ap).curptr >= (*ap).endptr {
        spc_warn!(
            spe,
            "No width/height/filename given for postscriptbox special."
        );
        return -1i32;
    }
    /* input is not NULL terminated */
    let len = (*ap).endptr.wrapping_offset_from((*ap).curptr) as i64 as i32;
    let len = if 511i32 < len { 511i32 } else { len };
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        (*ap).curptr as *const libc::c_void,
        len as _,
    );
    buf[len as usize] = '\u{0}' as i32 as i8;
    transform_info_clear(&mut ti);
    spc_warn!(spe, "{}", CStr::from_ptr(buf.as_mut_ptr()).display());
    if sscanf(
        buf.as_mut_ptr(),
        b"{%lfpt}{%lfpt}{%255[^}]}\x00" as *const u8 as *const i8,
        &mut ti.width as *mut f64,
        &mut ti.height as *mut f64,
        filename.as_mut_ptr(),
    ) != 3i32
    {
        spc_warn!(spe, "Syntax error in postscriptbox special?");
        return -1i32;
    }
    (*ap).curptr = (*ap).endptr;
    ti.width *= 72.0f64 / 72.27f64;
    ti.height *= 72.0f64 / 72.27f64;
    let handle = ttstub_input_open(filename.as_mut_ptr(), TTInputFormat::PICT, 0i32);
    if handle.is_null() {
        spc_warn!(
            spe,
            "Could not open image file: {}",
            CStr::from_ptr(filename.as_mut_ptr()).display(),
        );
        return -1i32;
    }
    ti.flags |= 1i32 << 1i32 | 1i32 << 2i32;
    loop {
        let mut p: *const i8 = tt_mfgets(buf.as_mut_ptr(), 512i32, handle);
        if p.is_null() {
            break;
        }
        if !(mps_scan_bbox(&mut p, p.offset(strlen(p) as isize), &mut ti.bbox) >= 0i32) {
            continue;
        }
        ti.flags |= 1i32 << 0i32;
        break;
    }
    ttstub_input_close(handle);
    let form_id = pdf_ximage_findresource(filename.as_mut_ptr(), options);
    if form_id < 0i32 {
        spc_warn!(
            spe,
            "Failed to load image file: {}",
            CStr::from_ptr(filename.as_mut_ptr()).display(),
        );
        return -1i32;
    }
    pdf_dev_put_image(form_id, &mut ti, (*spe).x_user, (*spe).y_user);
    0i32
}
unsafe extern "C" fn spc_handler_null(mut _spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    (*args).curptr = (*args).endptr;
    0i32
}
static mut MISC_HANDLERS: [spc_handler; 6] = [
    {
        let mut init = spc_handler {
            key: b"postscriptbox\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_postscriptbox
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"landscape\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"papersize\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"src:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"pos:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"om:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
];

#[no_mangle]
pub unsafe extern "C" fn spc_misc_check_special(mut buffer: *const i8, mut size: i32) -> bool {
    let mut p = buffer;
    let endptr = p.offset(size as isize);
    skip_white(&mut p, endptr);
    size = endptr.wrapping_offset_from(p) as i64 as i32;
    for i in 0..(::std::mem::size_of::<[spc_handler; 6]>() as u64)
        .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
    {
        if size as usize >= strlen(MISC_HANDLERS[i as usize].key)
            && strncmp(
                p,
                MISC_HANDLERS[i as usize].key,
                strlen(MISC_HANDLERS[i as usize].key),
            ) == 0
        {
            return true;
        }
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn spc_misc_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    assert!(!handle.is_null() && !spe.is_null() && !args.is_null());
    skip_white(&mut (*args).curptr, (*args).endptr);
    let key = (*args).curptr;
    while (*args).curptr < (*args).endptr && libc::isalpha(*(*args).curptr.offset(0) as _) != 0 {
        (*args).curptr = (*args).curptr.offset(1)
    }
    if (*args).curptr < (*args).endptr && *(*args).curptr.offset(0) as i32 == ':' as i32 {
        (*args).curptr = (*args).curptr.offset(1)
    }
    let keylen = (*args).curptr.wrapping_offset_from(key) as i64 as i32;
    if keylen < 1i32 {
        return -1i32;
    }
    for i in 0..(::std::mem::size_of::<[spc_handler; 6]>() as u64)
        .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
    {
        if keylen as usize == strlen(MISC_HANDLERS[i as usize].key)
            && strncmp(key, MISC_HANDLERS[i as usize].key, keylen as _) == 0
        {
            skip_white(&mut (*args).curptr, (*args).endptr);
            (*args).command = MISC_HANDLERS[i as usize].key;
            (*handle).key = b"???:\x00" as *const u8 as *const i8;
            (*handle).exec = MISC_HANDLERS[i as usize].exec;
            return 0i32;
        }
    }
    -1i32
}
