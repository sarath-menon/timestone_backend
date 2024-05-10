use std::path::Path;

use crate::types::FileInfo;
use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;

// use crate::types::FileInfo;
// use serde_json::to_string_pretty;

pub fn get(file_path: &Path) -> std::io::Result<FileInfo> {
    let source_text = std::fs::read_to_string(file_path)?;
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(file_path).unwrap();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    for error in ret.errors {
        let error = error.with_source_code(source_text.clone());
        println!("{error:?}");
    }

    // print_with_line_numbers(&source_text);

    // Extract the program from the result
    let program = ret.program;

    // Initialize a new FileInfo
    let mut file_info = FileInfo::default();

    // Visit the program with the FileInfo
    file_info.visit_program(&program);

    // Function to set the file name and path in the FileInfo
    fn set_file_name_path(file_info: &mut FileInfo, path: &Path) {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        file_info.file_name = file_name.clone();
        file_info.file_path = path.to_str().unwrap().to_string();
    }

    // Call the function to set the file name and path
    set_file_name_path(&mut file_info, &file_path);

    // // Convert file_info to JSON and save to file
    // let json = to_string_pretty(&file_info).unwrap();
    // let mut file = File::create("file_info.json")?;
    // file.write_all(json.as_bytes())?;

    // // Example usage of the function to extract and print a section of the source code
    // let extracted_section = extract_code_section(&source_text, 1112, 1624);

    // println!("Extracted Code Section: {}", extracted_section);

    Ok(file_info)
}

// // Function to extract a section of the source code based on a given span
// fn extract_code_section(source_text: &str, start: usize, end: usize) -> String {
//     source_text[start..end].to_string()
// }
// fn print_with_line_numbers(source_text: &str) {
//     for (index, line) in source_text.lines().enumerate() {
//         println!("{:4} {}", index + 1, line);
//     }
// }
