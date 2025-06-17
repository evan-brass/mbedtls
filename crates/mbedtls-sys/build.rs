use std::{ffi::OsStr, fs::read_dir, io::Error};

fn main() -> Result<(), Error> {
	let mut files = Vec::new();
	let dir = read_dir("../../library")?;
	for e in dir {
		let path = e?.path();
		if path.extension() != Some(OsStr::new("c")) { continue }
		let Some(file_name) = path.file_name().and_then(|f| f.to_str()) else { continue };
		if file_name.starts_with("psa_") { continue }

		files.push(path);
	}

	let bindings = bindgen::builder()
		.raw_line("#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]")
		.clang_arg("-Iinclude")
		.clang_arg("-I../../include")
		.header("include/bindings.h")
		.default_enum_style(bindgen::EnumVariation::Consts)
		.allowlist_function("mbedtls_.+")
		.allowlist_type("mbedtls_.+")
		.generate()
		.map_err(Error::other)?;
	bindings.write_to_file("src/lib.rs")?;

	cc::Build::new()
		.include("include")
		.include("../../include")
		.files(files)
		.try_compile("mbedtls")
		.map_err(Error::other)?;

	Ok(())
}
