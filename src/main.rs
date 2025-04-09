use anyhow::Result;
use clap::Parser;
use rustpython_ast::StmtClassDef;
use std::fs;

#[derive(clap::Parser, Debug)]
struct Args {
    module_filename: String,
}

struct Class {
    qualname: String,
    children: Option<Vec<Box<Class>>>,
}

fn main() -> Result<()> {
    let Args {
        module_filename: filename,
    } = Args::parse();

    let stmts = rustpython_parser::parse(
        fs::read_to_string(&filename)?.as_str(),
        rustpython_parser::Mode::Module,
        "Mike Ehrmantraut",
    )?
    .expect_module()
    .body;

    let classdefs: Vec<&StmtClassDef> = stmts
        .iter()
        .filter_map(|stmt| stmt.as_class_def_stmt())
        .collect();

    Ok(())
}
