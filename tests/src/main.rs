use luno_lang::parser::parse;
use std::ffi::OsStr;
use std::fs;
// use std::fs::{self, File};
// use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Parse source code into AST tests");
    let paths = fs::read_dir("./examples").unwrap();
    let luno_extension = OsStr::new("luno");

    for file in paths {
        let path = file?.path();

        if matches!(path.extension(), Some(ext) if ext == luno_extension) {
            println!("Running parse test for {:?}", path);
            let no_extension = path.file_stem().unwrap().to_str().unwrap();
            let ast_ext = format!("./examples/{no_extension}.ast");

            let source_code = fs::read_to_string(path.clone())?;

            let ast = parse(&source_code)?;
            let ast_dbg = format!("{:#?}", ast);
            // let mut file = File::create(ast_ext)?;
            // file.write_all(ast_dbg.as_bytes())?;
            let expected_ast = fs::read_to_string(ast_ext)?;

            assert_eq!(ast_dbg, expected_ast.trim());
        }
    }

    println!("All tests passed!");

    Ok(())
}
