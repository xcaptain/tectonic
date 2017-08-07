// build.rs -- build helper script for Tectonic.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.
//
// TODO: this surely needs to become much smarter and more flexible.

extern crate gcc;
extern crate pkg_config;
extern crate regex;
extern crate sha2;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};
use std::path::{Path, PathBuf};

use sha2::Digest;

// MacOS platform specifics:

#[cfg(target_os = "macos")]
const LIBS: &'static str = "harfbuzz harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

#[cfg(target_os = "macos")]
fn c_platform_specifics(cfg: &mut gcc::Config) {
    cfg.define("XETEX_MAC", Some("1"));
    cfg.file("tectonic/XeTeX_mac.c");

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=CoreText");
    println!("cargo:rustc-link-lib=framework=AppKit");
}

#[cfg(target_os = "macos")]
fn cpp_platform_specifics(cfg: &mut gcc::Config) {
    cfg.define("XETEX_MAC", Some("1"));
    cfg.file("tectonic/XeTeXFontInst_Mac.cpp");
    cfg.file("tectonic/XeTeXFontMgr_Mac.mm");
}


// Not-MacOS:

#[cfg(not(target_os = "macos"))]
const LIBS: &'static str = "fontconfig harfbuzz harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

#[cfg(not(target_os = "macos"))]
fn c_platform_specifics(_: &mut gcc::Build) {
}

#[cfg(not(target_os = "macos"))]
fn cpp_platform_specifics(cfg: &mut gcc::Build) {
    cfg.file("tectonic/XeTeXFontMgr_FC.cpp");
}


// String pool:

pub fn emit_stringpool(listfile: &Path, outstem: &Path) -> Result<()> {
    let listing = BufReader::new(File::open(listfile)?);

    let c_escapes = regex::Regex::new(r#"([\\"])"#).unwrap();
    // Note: we intentionally mask out "Z"
    let bad_for_identifier = regex::Regex::new(r#"([^_a-zA-Y0-9])"#).unwrap();

    let mut source_path = outstem.to_path_buf();
    let mut header_path = source_path.clone();
    source_path.set_extension("c");
    header_path.set_extension("h");

    let mut source = File::create(source_path)?;
    let mut header = File::create(header_path)?;

    writeln!(source, "/* Automatically generated by emit_tex_constants. */")?;
    writeln!(header, "/* Automatically generated by emit_tex_constants. */")?;

    let mut i = 0;
    let mut counts = HashMap::new();
    let mut digest = sha2::Sha256::default();

    for line in listing.lines() {
        let line = line?;

        // Converting the string to a C literal is pretty easy. Fortunately
        // only a few characters that need escaping ever appear. If things get
        // tricky we could always deploy \000 escapes.

        let str_lit = c_escapes.replace_all(&line, r"\$1");

        // Converting it to an identifier for the header file is harder. We
        // need to strip characters that don't work for C identifiers. We need
        // to disambiguate things that become the same after this stripping.
        // And it's nice to truncate relatively long strings. We ensure that
        // the disambiguation can't blow up on us by treating "Z" as a
        // non-letter in bad_for_identifier. "Z" does not occur in the XeTeX
        // string pool.

        let mut def_id = bad_for_identifier.replace_all(&line, "_").into_owned();
        def_id.truncate(28);

        let count = counts.entry(def_id.clone()).or_insert(0);
        let suffix = if *count == 0 {
            "".to_owned()
        } else {
            format!("_Z{0}", *count)
        };

        digest.input(&line.as_bytes());
        digest.input(&[0u8]);
        writeln!(source, r#""{0}","#, str_lit)?;
        writeln!(header, r#"#define S__{0}{1} {2} /* "{3}" */"#, def_id, suffix, i, str_lit)?;
        *count += 1;
        i += 1;
    }

    // TeX format files include a checksum of the string pool file
    // ("TEX.POOL") used to generate the string constants. We are no longer
    // compatible with TeX(Live) format files so we are free to compute the
    // digest as we please. We just take the first 31 bits (almost). I'm on a
    // plane and not able to search for the sensible way to do this, so we do
    // it in a silly way.

    let buf = digest.result();
    let hash: u32 = ((buf[0] as u32) & 0x7F) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32);
    writeln!(header, r#"#define STRING_POOL_CHECKSUM {}"#, hash)?;

    Ok(())
}

fn main() {
    // We (have to) rerun the search again below to emit the metadata at the right time.

    let deps = pkg_config::Config::new().cargo_metadata(false).probe(LIBS).unwrap();

    // First, emit the string pool C code. Sigh.

    let out_dir = env::var("OUT_DIR").unwrap();

    {
        let listfile = PathBuf::from("tectonic/strings.txt");
        let mut outstem = PathBuf::from(&out_dir);
        outstem.push("stringpool_generated");
        emit_stringpool(&listfile, &outstem)
            .expect("failed to generate \"string pool\" C source code");
    }

    // Actually I'm not 100% sure that I can't compile the C and C++ code
    // into one library, but who cares?

    let mut ccfg = gcc::Build::new();
    let mut cppcfg = gcc::Build::new();
    let cflags = [
        "-Wall",
        "-Wcast-qual",
        "-Wdate-time",
        "-Wendif-labels",
        "-Wextra",
        "-Wextra-semi",
        "-Wformat=2",
        "-Winit-self",
        "-Wlogical-op",
        "-Wmissing-declarations",
        "-Wmissing-include-dirs",
        "-Wmissing-prototypes",
        "-Wmissing-variable-declarations",
        "-Wnested-externs",
        "-Wold-style-definition",
        "-Wpointer-arith",
        "-Wredundant-decls",
        "-Wstrict-prototypes",
        "-Wsuggest-attribute=pure",
        "-Wsuggest-attribute=const",
        "-Wsuggest-attribute=format",
        "-Wswitch-bool",
        "-Wundef",
        "-Wwrite-strings",

        // TODO: Fix existing warnings before enabling these:
        // "-Wbad-function-cast",
        // "-Wcast-align",
        // "-Wconversion",
        // "-Wdouble-promotion",
        // "-Wshadow",
        // "-Wsuggest-attribute=noreturn",
        // "-Wunreachable-code-aggresive",

        "-Wno-unused-parameter",
        "-Wno-implicit-fallthrough",
        "-Wno-sign-compare",
    ];

    for flag in &cflags {
        ccfg.flag_if_supported(flag);
    }

    ccfg
        .file("tectonic/bibtex.c")
        .file("tectonic/core-bridge.c")
        .file("tectonic/core-kpathutil.c")
        .file("tectonic/dpx-agl.c")
        .file("tectonic/dpx-bmpimage.c")
        .file("tectonic/dpx-cff.c")
        .file("tectonic/dpx-cff_dict.c")
        .file("tectonic/dpx-cid.c")
        .file("tectonic/dpx-cidtype0.c")
        .file("tectonic/dpx-cidtype2.c")
        .file("tectonic/dpx-cmap.c")
        .file("tectonic/dpx-cmap_read.c")
        .file("tectonic/dpx-cmap_write.c")
        .file("tectonic/dpx-cs_type2.c")
        .file("tectonic/dpx-dpxconf.c")
        .file("tectonic/dpx-dpxcrypt.c")
        .file("tectonic/dpx-dpxfile.c")
        .file("tectonic/dpx-dpxutil.c")
        .file("tectonic/dpx-dvi.c")
        .file("tectonic/dpx-dvipdfmx.c")
        .file("tectonic/dpx-epdf.c")
        .file("tectonic/dpx-error.c")
        .file("tectonic/dpx-fontmap.c")
        .file("tectonic/dpx-jp2image.c")
        .file("tectonic/dpx-jpegimage.c")
        .file("tectonic/dpx-mem.c")
        .file("tectonic/dpx-mfileio.c")
        .file("tectonic/dpx-mpost.c")
        .file("tectonic/dpx-numbers.c")
        .file("tectonic/dpx-otl_conf.c")
        .file("tectonic/dpx-otl_opt.c")
        .file("tectonic/dpx-pdfcolor.c")
        .file("tectonic/dpx-pdfdev.c")
        .file("tectonic/dpx-pdfdoc.c")
        .file("tectonic/dpx-pdfdraw.c")
        .file("tectonic/dpx-pdfencoding.c")
        .file("tectonic/dpx-pdfencrypt.c")
        .file("tectonic/dpx-pdffont.c")
        .file("tectonic/dpx-pdfnames.c")
        .file("tectonic/dpx-pdfobj.c")
        .file("tectonic/dpx-pdfparse.c")
        .file("tectonic/dpx-pdfresource.c")
        .file("tectonic/dpx-pdfximage.c")
        .file("tectonic/dpx-pkfont.c")
        .file("tectonic/dpx-pngimage.c")
        .file("tectonic/dpx-pst.c")
        .file("tectonic/dpx-pst_obj.c")
        .file("tectonic/dpx-sfnt.c")
        .file("tectonic/dpx-spc_color.c")
        .file("tectonic/dpx-spc_dvipdfmx.c")
        .file("tectonic/dpx-spc_dvips.c")
        .file("tectonic/dpx-spc_html.c")
        .file("tectonic/dpx-spc_misc.c")
        .file("tectonic/dpx-spc_pdfm.c")
        .file("tectonic/dpx-spc_tpic.c")
        .file("tectonic/dpx-spc_util.c")
        .file("tectonic/dpx-spc_xtx.c")
        .file("tectonic/dpx-specials.c")
        .file("tectonic/dpx-subfont.c")
        .file("tectonic/dpx-t1_char.c")
        .file("tectonic/dpx-t1_load.c")
        .file("tectonic/dpx-tfm.c")
        .file("tectonic/dpx-truetype.c")
        .file("tectonic/dpx-tt_aux.c")
        .file("tectonic/dpx-tt_cmap.c")
        .file("tectonic/dpx-tt_glyf.c")
        .file("tectonic/dpx-tt_gsub.c")
        .file("tectonic/dpx-tt_post.c")
        .file("tectonic/dpx-tt_table.c")
        .file("tectonic/dpx-type0.c")
        .file("tectonic/dpx-type1.c")
        .file("tectonic/dpx-type1c.c")
        .file("tectonic/dpx-unicode.c")
        .file("tectonic/dpx-vf.c")
        .file("tectonic/engine-interface.c")
        .file("tectonic/errors.c")
        .file("tectonic/inimisc.c")
        .file("tectonic/io.c")
        .file("tectonic/mathutil.c")
        .file("tectonic/output.c")
        .file("tectonic/stringpool.c")
        .file("tectonic/synctex.c")
        .file("tectonic/texmfmp.c")
        .file("tectonic/xetex0.c")
        .file("tectonic/XeTeX_ext.c")
        .file("tectonic/xetexini.c")
        .file("tectonic/XeTeX_pic.c")
        .define("HAVE_GETENV", "1")
        .define("HAVE_INTTYPES_H", "1")
        .define("HAVE_LIBPNG", "1")
        .define("HAVE_MKSTEMP", "1")
        .define("HAVE_STDINT_H", "1")
        .define("HAVE_TM_GMTOFF", "1")
        .define("HAVE_ZLIB", "1")
        .define("HAVE_ZLIB_COMPRESS2", "1")
        .define("ZLIB_CONST", "1")
        .include(".")
        .include(&out_dir);

    let cppflags = [
        "-std=c++14",
        "-Wall",
        "-Wdate-time",
        "-Wendif-labels",
        "-Wextra",
        "-Wformat=2",
        "-Wlogical-op",
        "-Wmissing-declarations",
        "-Wmissing-include-dirs",
        "-Wpointer-arith",
        "-Wredundant-decls",
        "-Wsuggest-attribute=pure",
        "-Wsuggest-attribute=const",
        "-Wsuggest-attribute=noreturn",
        "-Wsuggest-attribute=format",
        "-Wshadow",
        "-Wswitch-bool",
        "-Wundef",

        // TODO: Fix existing warnings before enabling these:
        // "-Wdouble-promotion",
        // "-Wcast-align",
        // "-Wconversion",
        // "-Wextra-semi",
        // "-Wmissing-variable-declarations",
        // "-Wunreachable-code-aggresive",

        "-Wno-unused-parameter",
        "-Wno-implicit-fallthrough",
    ];

    for flag in &cppflags {
        cppcfg.flag_if_supported(flag);
    }

    cppcfg
        .cpp(true)
        .flag("-Wall")
        .file("tectonic/Engine.cpp")
        .file("tectonic/XeTeXFontInst.cpp")
        .file("tectonic/XeTeXFontMgr.cpp")
        .file("tectonic/XeTeXLayoutInterface.cpp")
        .file("tectonic/XeTeXOTMath.cpp")
        .include(".")
        .include(&out_dir);

    for p in deps.include_paths {
        ccfg.include(&p);
        cppcfg.include(&p);
    }

    c_platform_specifics(&mut ccfg);
    cpp_platform_specifics(&mut cppcfg);

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");

    // Now that we've emitted the info for our own libraries, we can emit the
    // info for their dependents.

    pkg_config::Config::new().cargo_metadata(true).probe(LIBS).unwrap();

    // Tell cargo to rerun build.rs only if files in the tectonic/ directory have changed.
    for file in PathBuf::from("tectonic").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
