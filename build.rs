#[macro_use]
extern crate bart_derive;
extern crate bindgen;
extern crate cpp_build;
//extern crate curl;
//extern crate failure;
//extern crate flate2;
//extern crate hex;
//extern crate reqwest;
//extern crate sha2;
//extern crate tar;

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use fs_extra::dir::CopyOptions;

//use failure::Fallible;
//use heck::CamelCase;

//const TFLITE_VERSION: &'static str = "1.13.2";
//
//fn is_valid_tf_src<P: AsRef<Path>>(filepath: P) -> bool {
//    use sha2::{Digest, Sha256};
//
//    let mut sha256 = Sha256::new();
//    sha256.input(&fs::read(filepath).unwrap());
//    ::hex::encode(sha256.result())
//        == "abe3bf0c47845a628b7df4c57646f41a10ee70f914f1b018a5c761be75e1f1a9"
//}
//
//fn download<P: reqwest::IntoUrl, P2: AsRef<Path>>(source_url: P, target_file: P2) -> Fallible<()> {
//    eprintln!("Downloading");
//    let mut resp = reqwest::get(source_url)?;
//    let f = fs::File::create(&target_file)?;
//    let mut writer = ::std::io::BufWriter::new(f);
//    resp.copy_to(&mut writer)?;
//    Ok(())
//}
//
//fn extract<P: AsRef<Path>, P2: AsRef<Path>>(archive_path: P, extract_to: P2) {
//    use flate2::read::GzDecoder;
//    use tar::Archive;
//    eprintln!("extracting");
//
//    let file = fs::File::open(archive_path).unwrap();
//    let unzipped = GzDecoder::new(file);
//    let mut a = Archive::new(unzipped);
//    a.unpack(extract_to).unwrap();
//}

fn prepare_tensorflow_source() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tf_src_dir = out_dir.join("tensorflow");
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    eprintln!("Preparing tensorflow source in {:?}", tf_src_dir);

    let copy_dir = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 65536,
        copy_inside: false,
        depth: 0
    };

    //    fs::create_dir(&tf_src_dir).unwrap_or_default();

    //    let tf_src_name = tf_src_dir.join(format!("v{}.tar.gz", TFLITE_VERSION));
    //    if !tf_src_name.exists() || !is_valid_tf_src(&tf_src_name) {
    //        let tf_src_url = format!(
    //            "https://codeload.github.com/tensorflow/tensorflow/tar.gz/v{}",
    //            TFLITE_VERSION
    //        );
    //
    //        download(&tf_src_url, &tf_src_name).unwrap();
    //
    //        assert!(
    //            is_valid_tf_src(&tf_src_name),
    //            "{} is not valid",
    //            tf_src_name.to_str().unwrap()
    //        );
    //    }

    //    let tf_src_dir_inner = tf_src_dir.join(format!("tensorflow-{}", TFLITE_VERSION));
    if !tf_src_dir.exists() {
        std::fs::create_dir_all(&tf_src_dir)?;
        eprintln!("Copying submodules/tensorflow");
        fs_extra::dir::copy(manifest_dir.join("submodules/tensorflow/tensorflow"), &out_dir, &copy_dir)?;
        fs_extra::dir::copy(manifest_dir.join("submodules/tensorflow/third_party"), &out_dir, &copy_dir)?;
    }

    let download_dir = tf_src_dir.join("lite/tools/make/downloads");
    std::fs::remove_dir(&download_dir).ok();
    if !download_dir.exists() {
        std::fs::create_dir_all(&download_dir)?;
        eprintln!("Copying submodules/tflite-deps");

        std::fs::read_dir(manifest_dir.join("submodules/tflite-deps/"))?.try_for_each(|de| {
            let path = de?.path();
            if !path.ends_with("replacements") {
                eprintln!("Copying {:?} to {:?}", path, &download_dir);
                fs_extra::dir::copy(path, &download_dir, &copy_dir).map(|_| ())
            } else {
                Ok(())
            }
        })?;

        std::fs::read_dir(manifest_dir.join("submodules/tflite-deps/replacements"))?.try_for_each(|de| {
            let path = de?.path();
            eprintln!("Copying {:?} to {:?}", path, &download_dir);
            fs_extra::dir::copy(path, &download_dir, &copy_dir).map(|_| ())
        })?;
//        eprintln!("Copying submodules/tflite-deps/replacements");
//        fs_extra::dir::copy(
//            manifest_dir.join("submodules/tflite-deps/replacements"),
//            download_dir.as_path(),
//            &copy_inside_dir
//        )?;
    }
    //        extract(&tf_src_name, &tf_src_dir);

    //        dbg!(Command::new("tensorflow/lite/tools/make/download_dependencies.sh")
    //            .current_dir(&tf_src_dir_inner)
    //            .status()
    //            .expect("failed to download tflite dependencies."));

    // To make `NativeTable` polymorphic
//    Command::new("sed")
//        .arg("-i")
//        .arg("s/struct NativeTable {};/struct NativeTable { virtual ~NativeTable() {} };/g")
//        .arg("lite/tools/make/downloads/flatbuffers/include/flatbuffers/flatbuffers.h")
//        .current_dir(&tf_src_dir)
//        .status()
//        .expect("failed to edit flatbuffers.h.");

    let flat_h_path = download_dir.join("flatbuffers/include/flatbuffers/flatbuffers.h");
    let flat_h = std::fs::read_to_string(&flat_h_path)?;
    std::fs::write(&flat_h_path, flat_h.replace(
        "struct NativeTable {};",
        "struct NativeTable { virtual ~NativeTable() {} };"
    ))?;


    // No longer needed
    //        // To compile C files with -fPIC
    //        if env::var("CARGO_CFG_TARGET_OS").unwrap() == "linux" {
    //            dbg!(fs::copy(
    //                "data/linux_makefile.inc",
    //                tf_src_dir_inner.join("tensorflow/lite/tools/make/targets/linux_makefile.inc"),
    //            )
    //              .expect("Unable to copy linux makefile"));
    //        }
    //        // To allow for cross-compiling to aarch64
    //        if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "aarch64" {
    //            dbg!(fs::copy(
    //                "data/aarch64_makefile.inc",
    //                tf_src_dir_inner.join("tensorflow/lite/tools/make/targets/aarch64_makefile.inc"),
    //            )
    //              .expect("Unable to copy aarch64 makefile"));
    //        }

    #[cfg(feature = "debug_tflite")]
    {
        Command::new("sed")
            .arg("-i")
            .arg("54s/.*/CXXFLAGS := -O0 -g/")
            .arg("tensorflow/lite/tools/make/Makefile")
            .current_dir(&tf_src_dir_inner)
            .status()?;

        // CFLAGS is set to CXXFLAGS by default
        //            Command::new("sed")
        //                .arg("-i")
        //                .arg("58s/.*/CFLAGS := -O0 -g/")
        //                .arg("tensorflow/lite/tools/make/Makefile")
        //                .current_dir(&tf_src_dir_inner)
        //                .status()?;
    }

    Ok(tf_src_dir)
}

fn prepare_tensorflow_library<P: AsRef<Path>>(tflite: P) {
    let tf_lib_name = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libtensorflow-lite.a");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Unable to get TARGET_OS");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Unable to get TARGET_ARCH");
    if !tf_lib_name.exists() {
        let mut make = Command::new("make");
        if let Ok(prefix) = env::var("TARGET_TOOLCHAIN_PREFIX") {
            make.arg(format!("TARGET_TOOLCHAIN_PREFIX={}", prefix));
        };

        make
            .arg("-j")
            // allow parallelism to be overridden
            .arg(env::var("TFLITE_RS_MAKE_PARALLELISM").unwrap_or(env::var("NUM_JOBS").unwrap_or_else(|_|"4".to_string())))
            .arg("-f")
            .arg("tensorflow/lite/tools/make/Makefile");
            // Use cargo's cross-compilation information while building tensorflow
        if &arch == "aarch64" {
            // Now that tensorflow has an aarch64_makefile.inc use theirs
            make.arg(format!("TARGET={}", arch));
        } else {
            make.arg(format!("TARGET={}", os));
        }
        make
            .arg(format!("TARGET_ARCH={}", arch))
            .arg("micro")
            .current_dir(tflite.as_ref().parent().unwrap())
            .status()
            .expect("failed to build tensorflow");

        // find library
        let library = std::fs::read_dir(tflite.as_ref().join("lite/tools/make/gen"))
            .expect("Make gen file should exist")
            .filter_map(|de|Some(de.ok()?.path().join("lib/libtensorflow-lite.a")))
            .find(|p|p.exists())
            .expect("Unable to copy libtensorflow-lite.a");
        fs::copy(&library,&tf_lib_name)
            .expect("Unable to copy libtensorflow-lite.a to OUT_DIR");
    }
}

// This generates "tflite_types.rs" containing structs and enums which are inter-operable with Glow.
fn import_tflite_types<P: AsRef<Path>>(tflite: P) {
    use bindgen::*;

    eprintln!("Import tflite types");

    let bindings = Builder::default()
        .whitelist_recursively(true)
        .prepend_enum_name(false)
        .impl_debug(true)
        .with_codegen_config(CodegenConfig::TYPES)
        .layout_tests(false)
        .enable_cxx_namespaces()
        .derive_default(true)
        // for model APIs
        .whitelist_type("tflite::ModelT")
        .whitelist_type(".+OptionsT")
        .blacklist_type(".+_TableType")
        // for interpreter
        .whitelist_type("tflite::FlatBufferModel")
        .opaque_type("tflite::FlatBufferModel")
        .whitelist_type("tflite::InterpreterBuilder")
        .opaque_type("tflite::InterpreterBuilder")
        .whitelist_type("tflite::Interpreter")
        .opaque_type("tflite::Interpreter")
        .whitelist_type("tflite::ops::builtin::BuiltinOpResolver")
        .opaque_type("tflite::ops::builtin::BuiltinOpResolver")
        .whitelist_type("tflite::OpResolver")
        .opaque_type("tflite::OpResolver")
        .whitelist_type("TfLiteTensor")
        .opaque_type("std::string")
        .opaque_type("flatbuffers::NativeTable")
        .blacklist_type("std")
        .blacklist_type("tflite::Interpreter_TfLiteDelegatePtr")
        .blacklist_type("tflite::Interpreter_State")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_partialeq(true)
        .derive_eq(true)
        .header("csrc/tflite_wrapper.hpp")
        .clang_arg(format!("-I{}", tflite.as_ref().to_str().unwrap()))
        .clang_arg(format!(
            "-I{}/tensorflow/lite/tools/make/downloads/flatbuffers/include",
            tflite.as_ref().to_str().unwrap()
        ))
        .clang_arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        // required to get cross compilation for aarch64 to work because of an issue in flatbuffers
        .clang_arg("-fms-extensions");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tflite_types.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn build_inline_cpp<P: AsRef<Path>>(tflite: P) {
    println!("cargo:rustc-link-lib=static=tensorflow-lite");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");

    eprintln!("build inline cpp");

    cpp_build::Config::new()
        .include(tflite.as_ref())
        .include(
            tflite
                .as_ref()
                .join("tensorflow/lite/tools/make/downloads/flatbuffers/include"),
        )
        .flag("-fPIC")
        .flag("-std=c++14")
        .flag("-Wno-sign-compare")
        .define("GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK", None)
        .debug(true)
        .opt_level(if cfg!(debug_assertions) { 0 } else { 2 })
        .build("src/lib.rs");
}

fn import_stl_types() {
    use bindgen::*;

    eprintln!("import stl types");

    let bindings = Builder::default()
        .enable_cxx_namespaces()
        .whitelist_type("std::string")
        .opaque_type("std::string")
        .whitelist_type("rust::.+")
        .opaque_type("rust::.+")
        .blacklist_type("std")
        .header("csrc/stl_wrapper.hpp")
        .layout_tests(false)
        .derive_partialeq(true)
        .derive_eq(true)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14");

    let bindings = bindings
        .generate()
        .expect("Unable to generate STL bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("stl_types.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn generate_memory_impl() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("generate memory impl");
    let mut file = File::create("src/model/stl/memory_impl.rs")?;
    writeln!(
        &mut file,
        r#"
use std::{{fmt, mem}};
use std::ops::{{Deref, DerefMut}};

use crate::model::stl::memory::UniquePtr;
"#
    )?;

    #[derive(BartDisplay)]
    #[template = "data/memory_basic_impl.rs.template"]
    struct MemoryBasicImpl<'a> {
        cpp_type: &'a str,
        rust_type: &'a str,
    }

    let memory_types = vec![
        ("OperatorCodeT", "crate::model::OperatorCodeT"),
        ("TensorT", "crate::model::TensorT"),
        ("OperatorT", "crate::model::OperatorT"),
        ("SubGraphT", "crate::model::SubGraphT"),
        ("BufferT", "crate::model::BufferT"),
        (
            "QuantizationParametersT",
            "crate::model::QuantizationParametersT",
        ),
        ("ModelT", "crate::model::ModelT"),
    ];

    for (cpp_type, rust_type) in memory_types {
        writeln!(
            &mut file,
            "{}\n",
            &MemoryBasicImpl {
                cpp_type,
                rust_type,
            },
        )?;
    }
    Ok(())
}

fn generate_vector_impl() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("generate vector impl");
    let mut file = File::create("src/model/stl/vector_impl.rs")?;
    writeln!(
        &mut file,
        r#"
use std::{{fmt, mem, slice}};
use std::ops::{{Deref, DerefMut, Index, IndexMut}};

use libc::size_t;

use super::memory::UniquePtr;
use super::vector::{{VectorOfUniquePtr, VectorErase, VectorExtract, VectorInsert, VectorSlice}};
use crate::model::stl::bindings::root::rust::dummy_vector;

cpp! {{{{
    #include <vector>
}}}}
"#
    )?;

    #[derive(BartDisplay)]
    #[template = "data/vector_primitive_impl.rs.template"]
    #[allow(non_snake_case)]
    struct VectorPrimitiveImpl<'a> {
        cpp_type: &'a str,
        rust_type: &'a str,
        RustType: &'a str,
    }

    let vector_types = vec![
        ("uint8_t", "u8", "U8"),
        ("int32_t", "i32", "I32"),
        ("int64_t", "i64", "I64"),
        ("float", "f32", "F32"),
    ];

    #[allow(non_snake_case)]
    for (cpp_type, rust_type, RustType) in vector_types {
        writeln!(
            &mut file,
            "{}\n",
            &VectorPrimitiveImpl {
                cpp_type,
                rust_type,
                RustType,
            },
        )?;
    }

    #[derive(BartDisplay)]
    #[template = "data/vector_basic_impl.rs.template"]
    struct VectorBasicImpl<'a> {
        cpp_type: &'a str,
        rust_type: &'a str,
    }

    let vector_types = vec![
        (
            "std::unique_ptr<OperatorCodeT>",
            "UniquePtr<crate::model::OperatorCodeT>",
        ),
        (
            "std::unique_ptr<TensorT>",
            "UniquePtr<crate::model::TensorT>",
        ),
        (
            "std::unique_ptr<OperatorT>",
            "UniquePtr<crate::model::OperatorT>",
        ),
        (
            "std::unique_ptr<SubGraphT>",
            "UniquePtr<crate::model::SubGraphT>",
        ),
        (
            "std::unique_ptr<BufferT>",
            "UniquePtr<crate::model::BufferT>",
        ),
    ];

    for (cpp_type, rust_type) in vector_types {
        writeln!(
            &mut file,
            "{}\n",
            &VectorBasicImpl {
                cpp_type,
                rust_type,
            },
        )?;
    }
    Ok(())
}

fn generate_builtin_options_impl() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("generate builtin options impl");
    let mut file = File::create("src/model/builtin_options_impl.rs")?;
    writeln!(
        &mut file,
        r#"
use super::{{BuiltinOptions, BuiltinOptionsUnion, NativeTable}};
"#
    )?;

    #[derive(BartDisplay)]
    #[template = "data/builtin_options_impl.rs.template"]
    struct BuiltinOptionsImpl<'a> {
        name: &'a str,
    }

    let option_names = vec![
        "Conv2DOptions",
        "DepthwiseConv2DOptions",
        "ConcatEmbeddingsOptions",
        "LSHProjectionOptions",
        "Pool2DOptions",
        "SVDFOptions",
        "RNNOptions",
        "FullyConnectedOptions",
        "SoftmaxOptions",
        "ConcatenationOptions",
        "AddOptions",
        "L2NormOptions",
        "LocalResponseNormalizationOptions",
        "LSTMOptions",
        "ResizeBilinearOptions",
        "CallOptions",
        "ReshapeOptions",
        "SkipGramOptions",
        "SpaceToDepthOptions",
        "EmbeddingLookupSparseOptions",
        "MulOptions",
        "PadOptions",
        "GatherOptions",
        "BatchToSpaceNDOptions",
        "SpaceToBatchNDOptions",
        "TransposeOptions",
        "ReducerOptions",
        "SubOptions",
        "DivOptions",
        "SqueezeOptions",
        "SequenceRNNOptions",
        "StridedSliceOptions",
        "ExpOptions",
        "TopKV2Options",
        "SplitOptions",
        "LogSoftmaxOptions",
        "CastOptions",
        "DequantizeOptions",
        "MaximumMinimumOptions",
        "ArgMaxOptions",
        "LessOptions",
        "NegOptions",
        "PadV2Options",
        "GreaterOptions",
        "GreaterEqualOptions",
        "LessEqualOptions",
        "SelectOptions",
        "SliceOptions",
        "TransposeConvOptions",
        "SparseToDenseOptions",
        "TileOptions",
        "ExpandDimsOptions",
        "EqualOptions",
        "NotEqualOptions",
        "ShapeOptions",
        "PowOptions",
        "ArgMinOptions",
        "FakeQuantOptions",
        "PackOptions",
        "LogicalOrOptions",
        "OneHotOptions",
        "LogicalAndOptions",
        "LogicalNotOptions",
        "UnpackOptions",
        "FloorDivOptions",
        "SquareOptions",
        "ZerosLikeOptions",
        "FillOptions",
        "BidirectionalSequenceLSTMOptions",
        "BidirectionalSequenceRNNOptions",
        "UnidirectionalSequenceLSTMOptions",
        "FloorModOptions",
        "RangeOptions",
        "ResizeNearestNeighborOptions",
        "LeakyReluOptions",
        "SquaredDifferenceOptions",
        "MirrorPadOptions",
        "AbsOptions",
        "SplitVOptions",
    ];

    for name in option_names {
        writeln!(&mut file, "{}\n", &BuiltinOptionsImpl { name },)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    eprintln!("main");
    import_stl_types();
    if cfg!(feature = "generate_model_apis") {
        generate_memory_impl().unwrap();
        generate_vector_impl().unwrap();
        generate_builtin_options_impl().unwrap();
    }

    let tflite_src_dir = prepare_tensorflow_source()?;
    #[cfg(not(feature = "build_doc"))]
    prepare_tensorflow_library(&tflite_src_dir);
    import_tflite_types(&tflite_src_dir);
    build_inline_cpp(&tflite_src_dir);
    Ok(())
}
