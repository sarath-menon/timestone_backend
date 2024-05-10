use oxc_ast::ast::{
    Declaration, ExportDefaultDeclaration, ExportDefaultDeclarationKind, ExportNamedDeclaration,
};

// Function to process export default declaration
pub fn process_export_default_declaration(decl: &ExportDefaultDeclaration) {
    // Match the declaration of the export default declaration
    match &decl.declaration {
        // If the declaration is a function declaration
        ExportDefaultDeclarationKind::FunctionDeclaration(function_declaration) => {
            // Check if the function declaration has an identifier and print its name
            if let Some(id) = &function_declaration.id {
                println!("Function Declaration name: {}", id.name);
            }
        }
        // If the declaration is not a function declaration, do nothing
        _ => {}
    }
}

pub fn process_export_named_declaration(decl: &ExportNamedDeclaration) {
    // Match the declaration of the export default declaration
    match &decl.declaration {
        // If the declaration is a function declaration
        Some(Declaration::FunctionDeclaration(function_declaration)) => {
            // Check if the function declaration has an identifier and print its name
            if let Some(id) = &function_declaration.id {
                println!("Function Declaration name: {}", id.name);
            }
        }
        // If the declaration is not a function declaration, do nothing
        _ => {}
    }
}
