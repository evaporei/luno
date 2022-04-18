use sol_lang::parser::parse;
use std::ffi::OsStr;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Parse source code into AST tests");
    let paths = fs::read_dir("./examples").unwrap();
    let sol_extension = OsStr::new("sol");

    for file in paths {
        let path = file?.path();

        if matches!(path.extension(), Some(ext) if ext == sol_extension) {
            println!("Running parse test for {:?}", path);
            let no_extension = path.file_stem().unwrap().to_str().unwrap();
            let ast_ext = format!("./examples/{no_extension}.ast");

            let source_code = fs::read_to_string(path.clone())?;

            let ast = parse(&source_code)?;
            let ast_dbg = format!("{:?}", ast);
            let expected_ast = fs::read_to_string(ast_ext)?;

            assert_eq!(ast_dbg, expected_ast.trim());
        }
    }

    println!("All tests passed!");

    Ok(())
}
