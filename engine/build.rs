// build.rs -- build helper script for Tectonic.
// Copyright 2016-2019 the Tectonic Project
// Licensed under the MIT License.

/// The Tectonic build script. Not only do we have internal C/C++ code, we
/// also depend on several external C/C++ libraries, so there's a lot to do
/// here. It would be great to streamline things.
///
/// TODO: this surely needs to become much smarter and more flexible.
use cc;
use pkg_config;
use vcpkg;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "macos"))]
const PKGCONFIG_LIBS: &'static str =
    "fontconfig harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2";

// No fontconfig on MacOS:
#[cfg(target_os = "macos")]
const PKGCONFIG_LIBS: &'static str =
    "harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2";

/// Build-script state when using pkg-config as the backend.
#[derive(Debug)]
struct PkgConfigState {
    libs: pkg_config::Library,
}

// Need a way to check that the vcpkg harfbuzz port has graphite2 and icu options enabled.
#[cfg(not(target_os = "macos"))]
const VCPKG_LIBS: &[&'static str] = &["fontconfig", "harfbuzz", "freetype", "graphite2"];

#[cfg(target_os = "macos")]
const VCPKG_LIBS: &[&'static str] = &["harfbuzz", "freetype", "graphite2"];

/// Build-script state when using vcpkg as the backend.
#[derive(Clone, Debug)]
struct VcPkgState {
    include_paths: Vec<PathBuf>,
}

/// State for discovering and managing our dependencies, which may vary
/// depending on the framework that we're using to discover them.
///
/// The basic gameplan is that we probe our dependencies to check that they're
/// available and pull out the C/C++ include directories; then we emit info
/// for building our C/C++ libraries; then we emit info for our dependencies.
/// Building stuff pretty much always requires some level of hackery, though,
/// so we don't try to be purist about the details.
#[derive(Debug)]
enum DepState {
    /// pkg-config
    PkgConfig(PkgConfigState),

    /// vcpkg
    VcPkg(VcPkgState),
}

impl DepState {
    /// Probe for our dependent libraries using pkg-config.
    fn new_pkg_config() -> Self {
        let libs = pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(PKGCONFIG_LIBS)
            .unwrap();
        DepState::PkgConfig(PkgConfigState { libs })
    }

    /// Probe for our dependent libraries using vcpkg.
    fn new_vcpkg() -> Self {
        let mut include_paths = vec![];

        for dep in VCPKG_LIBS {
            let library = vcpkg::find_package(dep)
                .expect(&format!("failed to load package {} from vcpkg", dep));
            include_paths.extend(library.include_paths.iter().cloned());
        }

        DepState::VcPkg(VcPkgState { include_paths })
    }

    /// Invoke a callback for each C/C++ include directory injected by our
    /// dependencies.
    fn foreach_include_path<F>(&self, mut f: F)
    where
        F: FnMut(&Path),
    {
        match self {
            &DepState::PkgConfig(ref s) => {
                for p in &s.libs.include_paths {
                    f(p);
                }
            }

            &DepState::VcPkg(ref s) => {
                for p in &s.include_paths {
                    f(p);
                }
            }
        }
    }

    /// This function is called after we've emitted the cargo compilation info
    /// for our own libraries. Now we can emit any special information
    /// relating to our dependencies, which may depend on the dep-finding
    /// backend or the target.
    fn emit_late_extras(&self, target: &str) {
        match self {
            &DepState::PkgConfig(_) => {
                pkg_config::Config::new()
                    .cargo_metadata(true)
                    .probe(PKGCONFIG_LIBS)
                    .unwrap();
            }

            &DepState::VcPkg(_) => {
                if target.contains("-linux-") {
                    // add icudata to the end of the list of libs as vcpkg-rs
                    // does not order individual libraries as a single pass
                    // linker requires.
                    println!("cargo:rustc-link-lib=icudata");
                }
            }
        }
    }
}

/// The default dependency-finding backend is pkg-config.
impl Default for DepState {
    fn default() -> Self {
        DepState::new_pkg_config()
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let rustflags = env::var("RUSTFLAGS").unwrap_or(String::new());

    // OK, how are we finding our dependencies?

    println!("cargo:rerun-if-env-changed=TECTONIC_DEP_BACKEND");

    let dep_state = if let Ok(dep_backend_str) = env::var("TECTONIC_DEP_BACKEND") {
        match dep_backend_str.as_ref() {
            "pkg-config" => DepState::new_pkg_config(),
            "vcpkg" => DepState::new_vcpkg(),
            "default" => DepState::default(),
            other => panic!("unrecognized TECTONIC_DEP_BACKEND setting {:?}", other),
        }
    } else {
        DepState::default()
    };

    // Actually I'm not 100% sure that I can't compile the C and C++ code
    // into one library, but who cares?

    let mut ccfg = cc::Build::new();
    let mut cppcfg = cc::Build::new();
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
        // "-Wsuggest-attribute=const",
        // "-Wsuggest-attribute=noreturn",
        // "-Wsuggest-attribute=pure",
        // "-Wunreachable-code-aggresive",
        "-Wno-unused-parameter",
        "-Wno-implicit-fallthrough",
        "-Wno-sign-compare",
        "-std=gnu11",
    ];

    for flag in &cflags {
        ccfg.flag_if_supported(flag);
    }

    ccfg.define("HAVE_ZLIB", "1")
        .define("HAVE_ZLIB_COMPRESS2", "1")
        .define("ZLIB_CONST", "1")
        .include(".");

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
        "-Wsuggest-attribute=noreturn",
        "-Wsuggest-attribute=format",
        "-Wshadow",
        "-Wswitch-bool",
        "-Wundef",
        // TODO: Fix existing warnings before enabling these:
        // "-Wdouble-promotion",
        // "-Wcast-align",
        // "-Wconversion",
        // "-Wmissing-variable-declarations",
        "-Wextra-semi",
        // "-Wsuggest-attribute=const",
        // "-Wsuggest-attribute=pure",
        // "-Wunreachable-code-aggresive",
        "-Wno-unused-parameter",
        "-Wno-implicit-fallthrough",
        "-fno-exceptions",
        "-fno-rtti",
    ];

    for flag in &cppflags {
        cppcfg.flag_if_supported(flag);
    }
    ccfg.flag("-Wall").file("tectonic/stub_icu.c").file("tectonic/stub_stdio.c").include(".");

    cppcfg
        .cpp(true)
        .flag("-Wall")
        .file("tectonic/teckit-Engine.cpp")
        .include(".");

    dep_state.foreach_include_path(|p| {
        ccfg.include(p);
        cppcfg.include(p);
    });

    // Platform-specific adjustments:

    if cfg!(target_os = "macos") {
        ccfg.define("XETEX_MAC", Some("1"));
        cppcfg.define("XETEX_MAC", Some("1"));

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    if cfg!(target_endian = "big") {
        ccfg.define("WORDS_BIGENDIAN", "1");
        cppcfg.define("WORDS_BIGENDIAN", "1");
    }

    if target.contains("-msvc") {
        ccfg.flag("/EHsc");
        cppcfg.flag("/EHsc");
        if rustflags.contains("+crt-static") {
            ccfg.define("GRAPHITE2_STATIC", None);
            cppcfg.define("GRAPHITE2_STATIC", None);
        }
    }

    // OK, back to generic build rules.
    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");

    dep_state.emit_late_extras(&target);

    // Tell cargo to rerun build.rs only if files in the tectonic/ directory have changed.
    for file in PathBuf::from("tectonic").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
