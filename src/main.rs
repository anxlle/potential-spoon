/*
изначально использовалась markdownify для перевода в markdown но были замечены лишние символы
может пойдет избавиться от всех лишних символов (**, |) и оставить все по строчкам
и просто найти слово и следующая строчка будет его значением, а-ля сделать json
{ "Наименование проекта": "Ярмарка ЯП"; "Результаты": "Результаты"; } и сделать запрос sql типа ключ значение туда куда надо
сейчас я изучал именно просто docx парсеры отдельно от markdownify в надежде что я могу без всяких избавлений от лишних символов получить структуру xml и так уже значения назначать, но что-то пока не получается :(
список pdf парсеров на пробу:
1. pdf-extract (https://crates.io/crates/pdf-extract)
2. pdf? (https://github.com/pdf-rs/pdf)
3. lopdf
4. oxidize_pdf
5. nompdf
6. safe-pdf
7. Ferrules
*/

use std::fs;
//use markdownify::{pdf, docx};
use std::path::Path;
use docx_rs::{read_docx};
//use markdownify::docx;
//use std::fs::File;
use serde::Serialize;

#[derive(Serialize)]
struct Cell {
    text: String,
    row: usize,
    column: usize,
}

#[derive(Serialize)]
struct Table {
    rows: usize,
    columns: usize,
    cells: Vec<Cell>,
}

#[derive(Serialize)]
struct DocumentTables {
    file_path: String,
    table_count: usize,
    tables: Vec<Table>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let md_dir = Path::new("./docs/parsed");
    fs::create_dir_all(md_dir)?;

    for f in fs::read_dir("./docs")? {
        let f = f?;
        let p = f.path();
        if p.is_file() {
            if p.extension().map_or(false, |ext| ext == "docx")
            || p.extension().map_or(false, |ext| ext == "doc") {
                let file = fs::read(&p)?;
                let doc = match read_docx(&file) {
                    Ok(doc) => doc,
                    Err(e) => {
                        println!("error reading {:?}: {}", p.file_name(), e);
                        continue
                    }
                };

                for child in doc.document.children.iter() {
                    match child {
                        docx_rs::DocumentChild::Table(table) => {
                            
                        }
                    }
                };
            }
        }
    }
    Ok(())
}



/*use std::fs;
//use std::fs::File;
//use markdownify::{pdf, docx};
use std::path::Path;
//use serde::{Serialize, Deserialize};
//use serde_json;
// docx_rust::{Docx, DocxFile};
use docx_rs::{read_docx};
use docx_rs::DocumentChild::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let md_dir = Path::new("./docs/md");
    fs::create_dir_all(md_dir)?;
    for f in fs::read_dir("./docs")? { // parse docx and pdf to md
        let f = f?;
        let p = f.path();
        if p.is_file() {
            if p.extension().map_or(false, |ext| ext == "docx")
            || p.extension().map_or(false, |ext| ext == "doc") {
                let file = fs::read(&p)?;
                let doc = match read_docx(&file) {
                    Ok(doc) => doc,
                    Err(e) => {
                        println!("error reading {:?}: {}", p.file_name(), e);
                        continue;
                    }
                };
                
                let document = &doc.document;

                for (_i, document_child) in document.children.iter().enumerate() {
                    let _variant = match document_child {
                        Table(_) => {
                            println!("look for {:?}", p.file_name())

                        },
                        _ => continue
                    };
                }

                /*let md = docx::docx_convert(&p)?;
                let stem = p.file_stem().unwrap().to_str().unwrap();
                let outpp = Path::new("./docs/md/").join(format!("{}.md", stem));
                fs::write(&outpp, md)?;
                println!("markdowned docx to {:?}", outpp);*/
             } else if p.extension().map_or(false, |ext| ext == "pdf") {
                /*let md = pdf::pdf_convert(&p, None)?;
                let stem = p.file_stem().unwrap().to_str().unwrap();
                let outpp = Path::new("./docs/md/").join(format!("{}.md", stem));
                fs::write(&outpp, md)?;
                println!("markdowned pdf to {:?}", outpp);*/
                println!("parsing pdf but no function here so yeah")
             } else {
                println!("failed to parse file {:?} as it is neither doc(x) nor pdf", p.file_name())
             }
        }
    }
    
    Ok(())
}*/