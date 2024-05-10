use std::path::Path;
use timestone_tsx_parser::display;
use timestone_tsx_parser::file_info;

// Instruction:
// create a `test.js`,
// run `cargo run -p oxc_parser --example visitor`
// or `cargo watch -x "run -p oxc_parser --example visitor"`

fn main() -> std::io::Result<()> {
    let file_path = Path::new("./test.tsx");
    let mut file_info = file_info::get(&file_path)?;

    display::print_file_info(&mut file_info);

    let file_info_str = serde_json::to_string_pretty(&file_info).unwrap();
    println!("{}", file_info_str);

    // // Convert file_info to JSON and save to file
    // let json = to_string_pretty(&file_info).unwrap();
    // let mut file = File::create("file_info.json")?;
    // file.write_all(json.as_bytes())?;

    // // Example usage of the function to extract and print a section of the source code
    // let extracted_section = extract_code_section(&source_text, 1112, 1624);

    // println!("Extracted Code Section: {}", extracted_section);

    Ok(())
}
