use crate::types::{FileImport, FileInfo, ReactComponent};

// Print the component_stack recursively as a pretty hierarchial structure with indentation
pub fn print_component_stack(component_stack: &Vec<ReactComponent>, level: usize) {
    for component in component_stack {
        // actual printing
        println!(
            "{:indent$}{} \t\t\t\t {} \t\t\t\t Code Span: {}-{}",
            "",
            component.name,
            component.class_names,
            component.code_span.start,
            component.code_span.end,
            indent = level * 4
        );

        if !component.children.is_empty() {
            print_component_stack(&component.children, level + 1);
        }
    }
}

// Print the file imports in a pretty format
pub fn print_file_imports(file_imports: &Vec<FileImport>) {
    for file_import in file_imports {
        println!("{:indent$}Name: {}", "", file_import.name, indent = 4);
        println!(
            "{:indent$}Import Path: {}",
            "",
            file_import.import_path,
            indent = 8
        );
        println!(
            "{:indent$}Default Import: {}",
            "",
            file_import.is_default,
            indent = 8
        );
    }
}

pub fn print_file_info(file_info: &mut FileInfo) {
    println!("File Name: {}", file_info.file_name);
    println!("File Path: {}", file_info.file_path);

    print_component_stack(&file_info.component_stack, 0);
    print_file_imports(&file_info.file_imports);
}
