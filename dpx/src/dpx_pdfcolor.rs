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

use crate::DisplayExt;

use super::dpx_error::{dpx_message, dpx_warning};
use super::dpx_mem::{new, renew};
use super::dpx_numbers::sget_unsigned_pair;
use super::dpx_pdfdev::{pdf_dev_get_param, pdf_dev_reset_color};
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_add_stream, pdf_get_version, pdf_link_obj, pdf_new_array,
    pdf_new_name, pdf_new_number, pdf_new_stream, pdf_obj, pdf_ref_obj, pdf_release_obj,
    pdf_stream_dict,
};
use crate::mfree;
use crate::{info, warn};
use libc::{free, memcmp, memcpy, memset, strcmp, strcpy, strlen};
use md5::{Digest, Md5};
use std::slice::from_raw_parts;
use crate::shims::sprintf;

pub type size_t = u64;

use std::ffi::{CStr, CString};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ColorspaceType {
    Gray,
    Rgb,
    Cmyk,
    Spot,
}

#[derive(Clone)]
#[repr(C)]
pub struct pdf_color {
    num_components: usize,
    pub spot_color_name: Option<CString>,
    pub values: [f64; 4],
}

pub const WHITE: pdf_color = pdf_color {
    num_components: 1,
    spot_color_name: None,
    values: [1.0, 0.0, 0.0, 0.0],
};

impl pdf_color {
    pub const fn new() -> Self {
        Self {
            num_components: 0,
            spot_color_name: None,
            values: [0.; 4],
        }
    }

    pub fn colorspace_type(&self) -> ColorspaceType {
        match self.num_components {
            1 => ColorspaceType::Gray,
            2 => ColorspaceType::Spot,
            3 => ColorspaceType::Rgb,
            4 => ColorspaceType::Cmyk,
            _ => panic!("unknown number of components"),
        }
    }

    pub fn rgb(r: f64, g: f64, b: f64) -> Result<pdf_color, ()> {
        if r < 0.0 || r > 1.0 {
            warn!("Invalid color value specified: red={}", r);
            return Err(());
        }
        if g < 0.0 || g > 1.0 {
            warn!("Invalid color value specified: green={}", g);
            return Err(());
        }
        if b < 0.0 || b > 1.0 {
            warn!("Invalid color value specified: blue={}", b);
            return Err(());
        }
        Ok(pdf_color {
            num_components: 3,
            spot_color_name: None,
            values: [r, g, b, 0.0],
        })
    }

    pub fn cmyk(c: f64, m: f64, y: f64, k: f64) -> Result<Self, ()> {
        if c < 0.0 || c > 1.0 {
            warn!("Invalid color value specified: cyan={}", c);
            return Err(());
        }
        if m < 0.0 || m > 1.0 {
            warn!("Invalid color value specified: magenta={}", m);
            return Err(());
        }
        if y < 0.0f64 || y > 1.0f64 {
            warn!("Invalid color value specified: yellow={}", y);
            return Err(());
        }
        if k < 0.0 || k > 1.0 {
            warn!("Invalid color value specified: black={}", k);
            return Err(());
        }
        Ok(pdf_color {
            num_components: 4,
            spot_color_name: None,
            values: [c, m, y, k],
        })
    }

    pub fn gray(g: f64) -> Result<pdf_color, ()> {
        if g < 0.0 || g > 1.0 {
            warn!("Invalid color value specified: gray={}", g);
            return Err(());
        }
        Ok(pdf_color {
            values: [g, 0., 0., 0.],
            num_components: 1,
            spot_color_name: None,
        })
    }

    pub unsafe fn spot(name: *const i8, c: f64) -> Result<pdf_color, ()> {
        if c < 0.0 || c > 1.0 {
            warn!("Invalid color value specified: grade={}", c);
            return Err(());
        }
        Ok(pdf_color {
            values: [c, 0.0, 0.0, 0.0],
            num_components: 2,
            spot_color_name: Some(CStr::from_ptr(name).to_owned()),
        })
    }

    pub fn is_valid(&self) -> bool {
        let mut n = self.num_components;
        if let 1 | 2 | 3 | 4 = n {
            return false;
        }
        if let Some(value) = self
            .values
            .iter()
            .take(n as usize)
            .find(|&&value| value < 0.0 || value > 1.0)
        {
            warn!("Invalid color value: {}", value);
            return false;
        }
        if self.colorspace_type() == ColorspaceType::Spot {
            if self.spot_color_name.is_none()
                || self.spot_color_name.as_ref().unwrap().as_bytes()[0] == 0
            {
                warn!("Invalid spot color: empty name");
                return false;
            }
        }
        true
    }

    /* Brighten up a color. f == 0 means no change, f == 1 means white. */
    pub fn brighten(&self, f: f64) -> Self {
        if f == 1.0 {
            pdf_color::gray(1.0).unwrap()
        } else {
            let mut dst = pdf_color::new();
            dst.num_components = self.num_components;
            let mut n = dst.num_components;
            let f1 = if self.num_components == 4 { 0.0 } else { f };
            let f0 = 1.0 - f;
            loop {
                let fresh0 = n;
                n = n - 1;
                if !(fresh0 != 0) {
                    break;
                }
                dst.values[n as usize] = f0 * self.values[n as usize] + f1
            }
            dst
        }
    }

    pub fn is_white(&self) -> bool {
        let f = match self.num_components {
            1 | 3 => {
                /* Gray */
                /* RGB */
                1.0
            }
            4 => {
                /* CMYK */
                0.0
            }
            _ => return false,
        };
        self.values
            .iter()
            .take(self.num_components)
            .all(|&value| value == f)
    }

    pub unsafe fn to_string(&self, mut buffer: *mut i8, mut mask: i8) -> i32 {
        let mut len: i32 = 0i32;
        if self.colorspace_type() == ColorspaceType::Spot {
            len = sprintf(
                buffer,
                b" /%s %c%c %g %c%c\x00" as *const u8 as *const i8,
                self.spot_color_name.as_ref().unwrap().as_ptr(),
                'C' as i32 | mask as i32,
                'S' as i32 | mask as i32,
                (self.values[0] / 0.001f64 + 0.5f64).floor() * 0.001f64,
                'S' as i32 | mask as i32,
                'C' as i32 | mask as i32,
            )
        } else {
            for i in 0..self.num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\x00" as *const u8 as *const i8,
                    (self.values[i as usize] / 0.001f64 + 0.5f64).floor() * 0.001f64,
                );
            }
        }
        len
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_colorspace {
    pub ident: *mut i8,
    pub subtype: i32,
    pub resource: *mut pdf_obj,
    pub reference: *mut pdf_obj,
    pub cdata: *mut iccbased_cdata,
}
pub type iccSig = u32;
/*
 * In ICC profile stream dicrionary, there is /Range whose values must
 * "match the information in the profile". But where is those values in?
 *
 * How should I treat rendering intent?
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccbased_cdata {
    pub sig: i32,
    pub checksum: [u8; 16],
    pub colorspace: i32,
    pub alternate: i32,
    /* alternate colorspace (id), unused */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CspcCache {
    pub count: u32,
    pub capacity: u32,
    pub colorspaces: *mut pdf_colorspace,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct iccHeader {
    pub size: i32,
    pub CMMType: iccSig,
    pub version: i32,
    pub devClass: iccSig,
    pub colorSpace: iccSig,
    pub PCS: iccSig,
    pub creationDate: [i8; 12],
    pub acsp: iccSig,
    pub platform: iccSig,
    pub flags: [i8; 4],
    pub devMnfct: iccSig,
    pub devModel: iccSig,
    pub devAttr: [i8; 8],
    pub intent: i32,
    pub illuminant: iccXYZNumber,
    pub creator: iccSig,
    pub ID: [u8; 16],
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct iccXYZNumber {
    pub X: i32,
    pub Y: i32,
    pub Z: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IccVersion {
    pub major: i32,
    pub minor: i32,
}
impl IccVersion {
    const fn new(major: i32, minor: i32) -> Self {
        Self { major, minor }
    }
}
#[derive(Clone)]
#[repr(C)]
pub struct ColorStack {
    pub current: i32,
    pub stroke: [pdf_color; 128],
    pub fill: [pdf_color; 128],
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* No page independence here...
 */
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_color_set_verbose(mut level: i32) {
    verbose = level;
}

/*
 * This routine is not a real color matching.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_color_compare(color1: &pdf_color, color2: &pdf_color) -> i32 {
    let mut n = color1.num_components;
    let mut current_block_1: u64;
    match n {
        1 => {
            current_block_1 = 715039052867723359;
        }
        2 => {
            /* Spot */
            current_block_1 = 1982130065057554431;
        }
        3 => {
            current_block_1 = 1982130065057554431;
        }
        4 => {
            current_block_1 = 15718257842624222162;
        }
        _ => return -1i32,
    }
    match current_block_1 {
        1982130065057554431 =>
        /* RGB */
        {
            current_block_1 = 15718257842624222162;
        }
        _ => {}
    }
    match current_block_1 {
        15718257842624222162 =>
            /* CMYK */
            {}
        _ => {}
    }
    if n != color2.num_components {
        return -1i32;
    }
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0) {
            break;
        }
        if color1.values[n as usize] != color2.values[n as usize] {
            return -1i32;
        }
    }
    if color1.spot_color_name.is_some() && color2.spot_color_name.is_some() {
        return strcmp(
            color1.spot_color_name.as_ref().unwrap().as_ptr(),
            color2.spot_color_name.as_ref().unwrap().as_ptr(),
        );
    }
    0i32
}

/*static mut color_stack: ColorStack = ColorStack {
    current: 0,
    stroke: unsafe { core::mem::zeroed() },//[pdf_color::new(); 128],
    fill: unsafe { core::mem::zeroed() },//[pdf_color::new(); 128],
};*/
static mut color_stack: ColorStack =
    unsafe { std::mem::transmute([0u8; std::mem::size_of::<ColorStack>()]) };

#[no_mangle]
pub unsafe extern "C" fn pdf_color_clear_stack() {
    if color_stack.current > 0 {
        warn!("You\'ve mistakenly made a global color change within nested colors.");
    }
    loop {
        let fresh4 = color_stack.current;
        color_stack.current = color_stack.current - 1;
        if !(fresh4 != 0) {
            break;
        }
    }
    color_stack.current = 0;
    color_stack.stroke[0] = pdf_color::gray(0.0).unwrap();
    color_stack.fill[0] = pdf_color::gray(0.0).unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_set(sc: &pdf_color, fc: &pdf_color) {
    color_stack.stroke[color_stack.current as usize] = sc.clone();
    color_stack.fill[color_stack.current as usize] = fc.clone();
    pdf_dev_reset_color(0);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_push(sc: &mut pdf_color, fc: &pdf_color) {
    if color_stack.current >= 128 - 1 {
        warn!("Color stack overflow. Just ignore.");
    } else {
        color_stack.current += 1;
        pdf_color_set(sc, fc);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_pop() {
    if color_stack.current <= 0 {
        warn!("Color stack underflow. Just ignore.");
    } else {
        color_stack.current -= 1;
        pdf_dev_reset_color(0i32);
    };
}
/* Color special
 * See remark in spc_color.c.
 */
/* Color stack
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_color_get_current() -> (&'static mut pdf_color, &'static mut pdf_color)
{
    (
        &mut color_stack.stroke[color_stack.current as usize],
        &mut color_stack.fill[color_stack.current as usize],
    )
}
static mut nullbytes16: [u8; 16] = [0; 16];
static mut icc_versions: [IccVersion; 8] = [
    IccVersion::new(0, 0),
    IccVersion::new(0, 0),
    IccVersion::new(0, 0),
    IccVersion::new(0x2, 0x10),
    IccVersion::new(0x2, 0x20),
    IccVersion::new(0x4, 0),
    IccVersion::new(0x4, 0),
    IccVersion::new(0x4, 0x20),
];

unsafe extern "C" fn iccp_version_supported(mut major: i32, mut minor: i32) -> i32 {
    let mut pdf_ver = pdf_get_version() as i32;
    if pdf_ver < 8i32 {
        if icc_versions[pdf_ver as usize].major < major {
            return 0i32;
        } else if icc_versions[pdf_ver as usize].major == major
            && icc_versions[pdf_ver as usize].minor < minor
        {
            return 0i32;
        } else {
            return 1i32;
        }
    }
    0i32
}
unsafe extern "C" fn str2iccSig(mut s: *const libc::c_void) -> iccSig {
    let mut p = s as *const i8;
    return ((*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32) as iccSig;
}
unsafe extern "C" fn iccp_init_iccHeader(icch: &mut iccHeader) {
    icch.size = 0i32;
    icch.CMMType = 0i32 as iccSig;
    icch.version = 0xffffffi32;
    icch.devClass = 0i32 as iccSig;
    icch.colorSpace = 0i32 as iccSig;
    icch.PCS = 0i32 as iccSig;
    memset(
        icch.creationDate.as_mut_ptr() as *mut libc::c_void,
        0i32,
        12,
    );
    icch.acsp = str2iccSig(b"ascp\x00" as *const u8 as *const i8 as *const libc::c_void);
    icch.platform = 0i32 as iccSig;
    memset(icch.flags.as_mut_ptr() as *mut libc::c_void, 0i32, 4);
    icch.devMnfct = 0i32 as iccSig;
    icch.devModel = 0i32 as iccSig;
    memset(icch.devAttr.as_mut_ptr() as *mut libc::c_void, 0i32, 8);
    icch.intent = 0i32;
    icch.illuminant.X = 0i32;
    icch.illuminant.Y = 0i32;
    icch.illuminant.Z = 0i32;
    icch.creator = 0i32 as iccSig;
    memset(icch.ID.as_mut_ptr() as *mut libc::c_void, 0i32, 16);
}
unsafe extern "C" fn init_iccbased_cdata(cdata: &mut iccbased_cdata) {
    cdata.sig = i32::from_be_bytes(*b"iccb");
    memset(cdata.checksum.as_mut_ptr() as *mut libc::c_void, 0i32, 16);
    cdata.colorspace = 0i32;
    cdata.alternate = -1i32;
}
unsafe extern "C" fn release_iccbased_cdata(cdata: &mut iccbased_cdata) {
    assert!(cdata.sig == i32::from_be_bytes(*b"iccb"));
    free(cdata as *mut iccbased_cdata as *mut libc::c_void);
}
unsafe extern "C" fn get_num_components_iccbased(cdata: &iccbased_cdata) -> i32 {
    let mut num_components: i32 = 0i32;
    assert!(cdata.sig == i32::from_be_bytes(*b"iccb"));
    match (*cdata).colorspace {
        -3 => num_components = 3i32,
        -4 => num_components = 4i32,
        -1 => num_components = 1i32,
        2 => num_components = 3i32,
        _ => {}
    }
    num_components
}
unsafe extern "C" fn compare_iccbased(
    mut ident1: *const i8,
    mut cdata1: Option<&iccbased_cdata>,
    mut ident2: *const i8,
    mut cdata2: Option<&iccbased_cdata>,
) -> i32 {
    if let (Some(cdata1), Some(cdata2)) = (cdata1, cdata2) {
        assert!(cdata1.sig == i32::from_be_bytes(*b"iccb"));
        assert!(cdata2.sig == i32::from_be_bytes(*b"iccb"));
        if memcmp(
            (*cdata1).checksum.as_ptr() as *const libc::c_void,
            nullbytes16.as_mut_ptr() as *const libc::c_void,
            16,
        ) != 0
            && memcmp(
                cdata2.checksum.as_ptr() as *const libc::c_void,
                nullbytes16.as_mut_ptr() as *const libc::c_void,
                16,
            ) != 0
        {
            return memcmp(
                cdata1.checksum.as_ptr() as *const libc::c_void,
                cdata2.checksum.as_ptr() as *const libc::c_void,
                16,
            );
        }
        if cdata1.colorspace != cdata2.colorspace {
            return cdata1.colorspace - cdata2.colorspace;
        }
        /* Continue if checksum unknown and colorspace is same. */
    }
    if !ident1.is_null() && !ident2.is_null() {
        return strcmp(ident1, ident2);
    }
    /* No way to compare */
    return -1i32; /* acsp */
}
#[no_mangle]
pub unsafe extern "C" fn iccp_check_colorspace(
    mut colortype: i32,
    mut profile: *const libc::c_void,
    mut proflen: i32,
) -> i32 {
    let mut colorspace: iccSig;
    if profile.is_null() || proflen < 128i32 {
        return -1i32;
    }
    let mut p = profile as *const u8;
    colorspace = str2iccSig(p.offset(16) as *const libc::c_void);
    match colortype {
        3 | -3 => {
            if colorspace
                != str2iccSig(b"RGB \x00" as *const u8 as *const i8 as *const libc::c_void)
            {
                return -1i32;
            }
        }
        1 | -1 => {
            if colorspace
                != str2iccSig(b"GRAY\x00" as *const u8 as *const i8 as *const libc::c_void)
            {
                return -1i32;
            }
        }
        -4 => {
            if colorspace
                != str2iccSig(b"CMYK\x00" as *const u8 as *const i8 as *const libc::c_void)
            {
                return -1i32;
            }
        }
        _ => return -1i32,
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn iccp_get_rendering_intent(
    mut profile: *const libc::c_void,
    mut proflen: i32,
) -> *mut pdf_obj {
    if profile.is_null() || proflen < 128i32 {
        return 0 as *mut pdf_obj;
    }
    let mut p = profile as *const u8;
    let intent = (*p.offset(64) as i32) << 24i32
        | (*p.offset(65) as i32) << 16i32
        | (*p.offset(66) as i32) << 8i32
        | *p.offset(67) as i32;
    match intent >> 16i32 & 0xffi32 {
        2 => pdf_new_name("Saturation"),
        0 => pdf_new_name("Perceptual"),
        3 => pdf_new_name("AbsoluteColorimetric"),
        1 => pdf_new_name("RelativeColorimetric"),
        _ => {
            warn!(
                "Invalid rendering intent type: {}",
                intent >> 16i32 & 0xffi32
            );
            0 as *mut pdf_obj
        }
    }
}
unsafe extern "C" fn iccp_unpack_header(
    icch: &mut iccHeader,
    mut profile: *const libc::c_void,
    mut proflen: i32,
    mut check_size: i32,
) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    if check_size != 0 {
        if profile.is_null() || proflen < 128i32 || proflen % 4i32 != 0i32 {
            warn!("Profile size: {}", proflen);
            return -1i32;
        }
    }
    p = profile as *const u8;
    let endptr = p.offset(128);
    icch.size = (*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32;
    if check_size != 0 {
        if icch.size != proflen {
            warn!("ICC Profile size: {}(header) != {}", icch.size, proflen,);
            return -1i32;
        }
    }
    p = p.offset(4);
    icch.CMMType = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    icch.version = (*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32;
    p = p.offset(4);
    icch.devClass = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    icch.colorSpace = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    icch.PCS = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        icch.creationDate.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        12,
    );
    p = p.offset(12);
    icch.acsp = str2iccSig(p as *const libc::c_void);
    if icch.acsp != str2iccSig(b"acsp\x00" as *const u8 as *const i8 as *const libc::c_void) {
        dpx_warning(
            b"Invalid ICC profile: not \"acsp\" - %c%c%c%c \x00" as *const u8 as *const i8,
            *p.offset(0) as i32,
            *p.offset(1) as i32,
            *p.offset(2) as i32,
            *p.offset(3) as i32,
        );
        return -1i32;
    }
    p = p.offset(4);
    icch.platform = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        icch.flags.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        4,
    );
    p = p.offset(4);
    icch.devMnfct = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    icch.devModel = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        icch.devAttr.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        8,
    );
    p = p.offset(8);
    icch.intent = (*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32;
    p = p.offset(4);
    icch.illuminant.X = (*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32;
    p = p.offset(4);
    icch.illuminant.Y = (*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32;
    p = p.offset(4);
    icch.illuminant.Z = (*p.offset(0) as i32) << 24i32
        | (*p.offset(1) as i32) << 16i32
        | (*p.offset(2) as i32) << 8i32
        | *p.offset(3) as i32;
    p = p.offset(4);
    icch.creator = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        icch.ID.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        16,
    );
    p = p.offset(16);
    /* 28 bytes reserved - must be set to zeros */
    while p < endptr {
        if *p as i32 != '\u{0}' as i32 {
            warn!(
                "Reserved pad not zero: {:02x} (at offset {} in ICC profile header.)",
                *p as i32,
                128i32 - endptr.wrapping_offset_from(p) as i64 as i32,
            );
            return -1i32;
        }
        p = p.offset(1)
    }
    0i32
}
unsafe extern "C" fn iccp_get_checksum(profile: *const u8, proflen: usize) -> [u8; 16] {
    let mut md5 = Md5::new();
    md5.input(from_raw_parts(profile.offset(0), 56));
    md5.input(&[0u8; 12]);
    md5.input(from_raw_parts(profile.offset(68), 16));
    md5.input(&[0u8; 16]);
    md5.input(from_raw_parts(profile.offset(100), 28));
    /* body */
    md5.input(from_raw_parts(profile.offset(128), proflen - 128));
    md5.result().into()
}

unsafe extern "C" fn print_iccp_header(icch: &mut iccHeader, mut checksum: *mut u8) {
    info!("\n");
    info!("pdf_color>> ICC Profile Info\n");
    info!("pdf_color>> Profile Size:\t{} bytes\n", icch.size);
    if icch.CMMType == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "CMM Type",);
    } else if libc::isprint((icch.CMMType >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.CMMType >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.CMMType >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.CMMType & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "CMM Type",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"CMM Type\x00" as *const u8 as *const i8,
            icch.CMMType >> 24i32 & 0xff_u32,
            icch.CMMType >> 16i32 & 0xff_u32,
            icch.CMMType >> 8i32 & 0xff_u32,
            icch.CMMType & 0xff_u32,
        );
    }
    info!(
        "pdf_color>> Profile Version:\t{}.{:01}.{:01}\n",
        icch.version >> 24i32 & 0xffi32,
        icch.version >> 20i32 & 0xfi32,
        icch.version >> 16i32 & 0xfi32,
    );
    if icch.devClass == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Device Class");
    } else if libc::isprint((icch.devClass >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devClass >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devClass >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devClass & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Device Class",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Device Class\x00" as *const u8 as *const i8,
            icch.devClass >> 24i32 & 0xff_u32,
            icch.devClass >> 16i32 & 0xff_u32,
            icch.devClass >> 8i32 & 0xff_u32,
            icch.devClass & 0xff_u32,
        );
    }
    if icch.colorSpace == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Color Space");
    } else if libc::isprint((icch.colorSpace >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.colorSpace >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.colorSpace >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.colorSpace & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Color Space",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Color Space\x00" as *const u8 as *const i8,
            icch.colorSpace >> 24i32 & 0xff_u32,
            icch.colorSpace >> 16i32 & 0xff_u32,
            icch.colorSpace >> 8i32 & 0xff_u32,
            icch.colorSpace & 0xff_u32,
        );
    }
    if icch.PCS == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Connection Space");
    } else if libc::isprint((icch.PCS >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.PCS >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.PCS >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.PCS & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Connection Space",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Connection Space\x00" as *const u8 as *const i8,
            icch.PCS >> 24i32 & 0xff_u32,
            icch.PCS >> 16i32 & 0xff_u32,
            icch.PCS >> 8i32 & 0xff_u32,
            icch.PCS & 0xff_u32,
        );
    }
    info!("pdf_color>> Creation Date:\t");
    for i in (0..12).step_by(2) {
        if i == 0 {
            info!(
                "{:04}",
                sget_unsigned_pair(icch.creationDate.as_mut_ptr() as *mut u8) as i32,
            );
        } else {
            info!(
                ":{:02}",
                sget_unsigned_pair(
                    &mut *icch.creationDate.as_mut_ptr().offset(i as isize) as *mut i8 as *mut u8
                ) as i32,
            );
        }
    }
    info!("\n");
    if icch.platform == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Primary Platform");
    } else if libc::isprint((icch.platform >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.platform >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.platform >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.platform & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Primary Platform",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Primary Platform\x00" as *const u8 as *const i8,
            icch.platform >> 24i32 & 0xff_u32,
            icch.platform >> 16i32 & 0xff_u32,
            icch.platform >> 8i32 & 0xff_u32,
            icch.platform & 0xff_u32,
        );
    }
    info!(
        "pdf_color>> Profile Flags:\t{:02x}:{:02x}:{:02x}:{:02x}\n",
        icch.flags[0] as i32, icch.flags[1] as i32, icch.flags[2] as i32, icch.flags[3] as i32,
    );
    if icch.devMnfct == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Device Mnfct");
    } else if libc::isprint((icch.devMnfct >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devMnfct >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devMnfct >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devMnfct & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Device Mnfct",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Device Mnfct\x00" as *const u8 as *const i8,
            icch.devMnfct >> 24i32 & 0xff_u32,
            icch.devMnfct >> 16i32 & 0xff_u32,
            icch.devMnfct >> 8i32 & 0xff_u32,
            icch.devMnfct & 0xff_u32,
        );
    }
    if icch.devModel == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Device Model");
    } else if libc::isprint((icch.devModel >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devModel >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devModel >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.devModel & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Device Model",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Device Model\x00" as *const u8 as *const i8,
            icch.devModel >> 24i32 & 0xff_u32,
            icch.devModel >> 16i32 & 0xff_u32,
            icch.devModel >> 8i32 & 0xff_u32,
            icch.devModel & 0xff_u32,
        );
    }
    info!("pdf_color>> Device Attr:\t");
    for i in 0..8 {
        if i == 0 {
            info!("{:02x}", icch.devAttr[i]);
        } else {
            info!(":{:02x}", icch.devAttr[i]);
        }
    }
    info!("\n");
    info!("pdf_color>> Rendering Intent:\t");
    match icch.intent >> 16i32 & 0xffi32 {
        2 => {
            info!("Saturation");
        }
        0 => {
            info!("Perceptual");
        }
        3 => {
            info!("Absolute Colorimetric");
        }
        1 => {
            info!("Relative Colorimetric");
        }
        _ => {
            info!("(invalid)");
        }
    }
    info!("\n");
    if icch.creator == 0_u32 {
        info!("pdf_color>> {}:\t(null)\n", "Creator",);
    } else if libc::isprint((icch.creator >> 24i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.creator >> 16i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.creator >> 8i32 & 0xff_u32) as _) == 0
        || libc::isprint((icch.creator & 0xff_u32) as _) == 0
    {
        info!("pdf_color>> {}:\t(invalid)\n", "Creator",);
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const i8,
            b"Creator\x00" as *const u8 as *const i8,
            icch.creator >> 24i32 & 0xff_u32,
            icch.creator >> 16i32 & 0xff_u32,
            icch.creator >> 8i32 & 0xff_u32,
            icch.creator & 0xff_u32,
        );
    }
    info!("pdf_color>> Illuminant (XYZ):\t");
    info!(
        "{:.3} {:.3} {:.3}\n",
        icch.illuminant.X as f64 / 0x10000i32 as f64,
        icch.illuminant.Y as f64 / 0x10000i32 as f64,
        icch.illuminant.Z as f64 / 0x10000i32 as f64,
    );
    info!("pdf_color>> Checksum:\t");
    if memcmp(
        icch.ID.as_mut_ptr() as *const libc::c_void,
        nullbytes16.as_mut_ptr() as *const libc::c_void,
        16,
    ) == 0
    {
        info!("(null)");
    } else {
        for i in 0..16 {
            if i == 0 {
                info!("{:02x}", icch.ID[i]);
            } else {
                info!(":{:02x}", icch.ID[i]);
            }
        }
    }
    info!("\n");
    if !checksum.is_null() {
        info!("pdf_color>> Calculated:\t");
        for i in 0..16 {
            if i == 0 {
                info!("{:02x}", *checksum.offset(i as isize));
            } else {
                info!(":{:02x}", *checksum.offset(i as isize));
            }
        }
        info!("\n");
    };
}
unsafe extern "C" fn iccp_devClass_allowed(mut dev_class: i32) -> i32 {
    let colormode = pdf_dev_get_param(2i32);
    if dev_class as u32 != str2iccSig(b"scnr\x00" as *const u8 as *const i8 as *const libc::c_void)
        && dev_class as u32
            != str2iccSig(b"mntr\x00" as *const u8 as *const i8 as *const libc::c_void)
        && dev_class as u32
            != str2iccSig(b"prtr\x00" as *const u8 as *const i8 as *const libc::c_void)
        && dev_class as u32
            != str2iccSig(b"spac\x00" as *const u8 as *const i8 as *const libc::c_void)
    {
        return 0i32;
    }
    1i32
}
#[no_mangle]
pub unsafe extern "C" fn iccp_load_profile(
    mut ident: *const i8,
    mut profile: *const libc::c_void,
    mut proflen: i32,
) -> i32 {
    let mut cspc_id;
    let mut icch = iccHeader::default();
    let mut colorspace;
    iccp_init_iccHeader(&mut icch);
    if iccp_unpack_header(&mut icch, profile, proflen, 1i32) < 0i32 {
        /* check size */
        warn!(
            "Invalid ICC profile header in \"{}\"",
            CStr::from_ptr(ident).display()
        );
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    if iccp_version_supported(
        icch.version >> 24i32 & 0xffi32,
        icch.version >> 16i32 & 0xffi32,
    ) == 0
    {
        warn!("ICC profile format spec. version {}.{:01}.{:01} not supported in current PDF version setting.",
                    icch.version >> 24i32 & 0xffi32,
                    icch.version >> 20i32 & 0xfi32,
                    icch.version >> 16i32 & 0xfi32);
        warn!("ICC profile not embedded.");
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    if iccp_devClass_allowed(icch.devClass as i32) == 0 {
        warn!("Unsupported ICC Profile Device Class:");
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    if icch.colorSpace == str2iccSig(b"RGB \x00" as *const u8 as *const i8 as *const libc::c_void) {
        colorspace = -3i32
    } else if icch.colorSpace
        == str2iccSig(b"GRAY\x00" as *const u8 as *const i8 as *const libc::c_void)
    {
        colorspace = -1i32
    } else if icch.colorSpace
        == str2iccSig(b"CMYK\x00" as *const u8 as *const i8 as *const libc::c_void)
    {
        colorspace = -4i32
    } else {
        warn!("Unsupported input color space.");
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    let mut checksum = iccp_get_checksum(profile as *const u8, proflen as usize);
    if memcmp(
        icch.ID.as_mut_ptr() as *const libc::c_void,
        nullbytes16.as_mut_ptr() as *const libc::c_void,
        16,
    ) != 0
        && memcmp(
            icch.ID.as_mut_ptr() as *const libc::c_void,
            checksum.as_mut_ptr() as *const libc::c_void,
            16,
        ) != 0
    {
        warn!("Invalid ICC profile: Inconsistent checksum.");
        print_iccp_header(&mut icch, checksum.as_mut_ptr());
        return -1i32;
    }
    let cdata =
        &mut *(new((1_u64).wrapping_mul(::std::mem::size_of::<iccbased_cdata>() as u64) as u32)
            as *mut iccbased_cdata);
    init_iccbased_cdata(cdata);
    cdata.colorspace = colorspace;
    memcpy(
        cdata.checksum.as_mut_ptr() as *mut libc::c_void,
        checksum.as_mut_ptr() as *const libc::c_void,
        16,
    );
    cspc_id = pdf_colorspace_findresource(ident, 4i32, cdata);
    if cspc_id >= 0i32 {
        if verbose != 0 {
            info!("(ICCP:[id={}])", cspc_id);
        }
        release_iccbased_cdata(cdata);
        return cspc_id;
    }
    if verbose > 1i32 {
        print_iccp_header(&mut icch, checksum.as_mut_ptr());
    }
    let resource = pdf_new_array();
    let stream = pdf_new_stream(1i32 << 0i32);
    pdf_add_array(resource, pdf_new_name("ICCBased"));
    pdf_add_array(resource, pdf_ref_obj(stream));
    let stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        "N",
        pdf_new_number(get_num_components_iccbased(cdata) as f64),
    );
    pdf_add_stream(stream, profile, proflen);
    pdf_release_obj(stream);
    cspc_id = pdf_colorspace_defineresource(ident, 4i32, cdata, resource);
    cspc_id
}
static mut cspc_cache: CspcCache = {
    let mut init = CspcCache {
        count: 0_u32,
        capacity: 0_u32,
        colorspaces: 0 as *const pdf_colorspace as *mut pdf_colorspace,
    };
    init
};
unsafe extern "C" fn pdf_colorspace_findresource(
    mut ident: *const i8,
    mut type_0: i32,
    cdata: &iccbased_cdata,
) -> i32 {
    let mut cmp: i32 = -1i32;
    let mut cspc_id = 0i32;
    while cmp != 0 && (cspc_id as u32) < cspc_cache.count {
        let colorspace = &mut *cspc_cache.colorspaces.offset(cspc_id as isize);
        if !(colorspace.subtype != type_0) {
            match colorspace.subtype {
                4 => {
                    cmp = compare_iccbased(
                        ident,
                        Some(cdata),
                        colorspace.ident,
                        Some(&*colorspace.cdata),
                    )
                }
                _ => {}
            }
            if cmp == 0 {
                return cspc_id;
            }
        }
        cspc_id += 1
    }
    return -1i32;
    /* not found */
}
unsafe extern "C" fn pdf_init_colorspace_struct(colorspace: &mut pdf_colorspace) {
    colorspace.ident = 0 as *mut i8;
    colorspace.subtype = 0i32;
    colorspace.resource = 0 as *mut pdf_obj;
    colorspace.reference = 0 as *mut pdf_obj;
    colorspace.cdata = 0 as *mut iccbased_cdata;
}
unsafe extern "C" fn pdf_clean_colorspace_struct(colorspace: &mut pdf_colorspace) {
    free(colorspace.ident as *mut libc::c_void);
    pdf_release_obj(colorspace.resource);
    pdf_release_obj(colorspace.reference);
    colorspace.resource = 0 as *mut pdf_obj;
    colorspace.reference = 0 as *mut pdf_obj;
    if !colorspace.cdata.is_null() {
        match colorspace.subtype {
            4 => {
                release_iccbased_cdata(&mut *(colorspace.cdata as *mut iccbased_cdata));
            }
            _ => {}
        }
    }
    colorspace.cdata = 0 as *mut iccbased_cdata;
    colorspace.subtype = 0i32;
}
unsafe extern "C" fn pdf_flush_colorspace(colorspace: &mut pdf_colorspace) {
    pdf_release_obj(colorspace.resource);
    pdf_release_obj(colorspace.reference);
    colorspace.resource = 0 as *mut pdf_obj;
    colorspace.reference = 0 as *mut pdf_obj;
}
/* **************************** COLOR SPACE *****************************/
unsafe extern "C" fn pdf_colorspace_defineresource(
    mut ident: *const i8,
    mut subtype: i32,
    cdata: &mut iccbased_cdata,
    mut resource: *mut pdf_obj,
) -> i32 {
    if cspc_cache.count >= cspc_cache.capacity {
        cspc_cache.capacity = cspc_cache.capacity.wrapping_add(16_u32);
        cspc_cache.colorspaces = renew(
            cspc_cache.colorspaces as *mut libc::c_void,
            (cspc_cache.capacity as u64)
                .wrapping_mul(::std::mem::size_of::<pdf_colorspace>() as u64) as u32,
        ) as *mut pdf_colorspace
    }
    let cspc_id = cspc_cache.count as i32;
    let colorspace = &mut *cspc_cache.colorspaces.offset(cspc_id as isize);
    pdf_init_colorspace_struct(colorspace);
    if !ident.is_null() {
        (*colorspace).ident =
            new((strlen(ident).wrapping_add(1)).wrapping_mul(::std::mem::size_of::<i8>()) as _)
                as *mut i8;
        strcpy((*colorspace).ident, ident);
    }
    (*colorspace).subtype = subtype;
    (*colorspace).cdata = cdata;
    (*colorspace).resource = resource;
    if verbose != 0 {
        info!("(ColorSpace:{}", CStr::from_ptr(ident).display());
        if verbose > 1i32 {
            match subtype {
                4 => {
                    info!("[ICCBased]");
                }
                3 => {
                    info!("[CalRGB]");
                }
                1 => {
                    info!("[CalGray]");
                }
                _ => {}
            }
        }
        info!(")");
    }
    cspc_cache.count = cspc_cache.count.wrapping_add(1);
    cspc_id
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_colorspace_reference(mut cspc_id: i32) -> *mut pdf_obj {
    let mut colorspace =
        &mut *cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    if (*colorspace).reference.is_null() {
        (*colorspace).reference = pdf_ref_obj((*colorspace).resource);
        pdf_release_obj((*colorspace).resource);
        (*colorspace).resource = 0 as *mut pdf_obj
    }
    pdf_link_obj((*colorspace).reference)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_colors() {
    cspc_cache.count = 0_u32;
    cspc_cache.capacity = 0_u32;
    cspc_cache.colorspaces = 0 as *mut pdf_colorspace;
}
/* Not check size */
/* returns colorspace ID */
#[no_mangle]
pub unsafe extern "C" fn pdf_close_colors() {
    for i in 0..cspc_cache.count {
        let colorspace = &mut *cspc_cache.colorspaces.offset(i as isize);
        pdf_flush_colorspace(colorspace);
        pdf_clean_colorspace_struct(colorspace);
    }
    cspc_cache.colorspaces =
        mfree(cspc_cache.colorspaces as *mut libc::c_void) as *mut pdf_colorspace;
    cspc_cache.capacity = 0_u32;
    cspc_cache.count = cspc_cache.capacity;
}
