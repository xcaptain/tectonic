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

use super::{spc_arg, spc_env, spc_handler};
use crate::dpx_dpxutil::parse_c_ident;
use crate::dpx_pdfparse::skip_white;
use crate::spc_warn;
use crate::streq_ptr;
use libc::{free, memcmp, strlen};

pub type size_t = u64;

/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */

unsafe fn spc_handler_null(mut _spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    (*args).curptr = (*args).endptr;
    0i32
}
static mut DVIPDFMX_HANDLERS: [spc_handler; 1] = [{
    let mut init = spc_handler {
        key: b"config\x00" as *const u8 as *const i8,
        exec: Some(spc_handler_null),
    };
    init
}];

#[no_mangle]
pub unsafe extern "C" fn spc_dvipdfmx_check_special(mut buf: *const i8, mut len: i32) -> bool {
    let mut p = buf;
    let endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p.offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const i8) as isize) <= endptr
        && memcmp(
            p as *const libc::c_void,
            b"dvipdfmx:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"dvipdfmx:\x00" as *const u8 as *const i8),
        ) == 0
    {
        return true;
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvipdfmx_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut error: i32 = -1i32;
    assert!(!sph.is_null() && !spe.is_null() && !ap.is_null());
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap)
        .curptr
        .offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const i8) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const libc::c_void,
            b"dvipdfmx:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"dvipdfmx:\x00" as *const u8 as *const i8),
        ) != 0
    {
        spc_warn!(spe, "Not dvipdfmx: special???");
        return -1i32;
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const i8) as isize);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    let q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        for i in 0..(::std::mem::size_of::<[spc_handler; 1]>() as u64)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
        {
            if streq_ptr(q, DVIPDFMX_HANDLERS[i as usize].key) {
                (*ap).command = DVIPDFMX_HANDLERS[i as usize].key;
                (*sph).key = b"dvipdfmx:\x00" as *const u8 as *const i8;
                (*sph).exec = DVIPDFMX_HANDLERS[i as usize].exec;
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                error = 0i32;
                break;
            }
        }
        free(q as *mut libc::c_void);
    }
    error
}
