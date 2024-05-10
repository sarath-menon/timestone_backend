use oxc_ast::{ast::ImportDeclarationSpecifier, AstKind, Visit};

use crate::exports::{process_export_default_declaration, process_export_named_declaration};
use crate::jsx_element::{get_jsx_element_name, process_jsx_attribute};
use crate::types::{ElementType, FileImport, FileInfo, ReactComponent};

fn determine_element_type(name: &String) -> ElementType {
    if name.starts_with(char::is_uppercase) {
        ElementType::ReactComponent
    } else {
        ElementType::HtmlTag
    }
}

fn insert_component(
    component_stack: &mut Vec<ReactComponent>,
    element: ReactComponent,
    name: String,
) {
    for component in component_stack {
        // println!("Component name: {}, Name: {}", component.name, name);

        if component.name != name {
            insert_component(&mut component.children, element.clone(), name.clone());
        } else {
            component.children.push(element.clone());
        }
    }
}

impl<'a> Visit<'a> for FileInfo {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::JSXOpeningElement(element) => {
                // Attempt to retrieve the name of the JSX element
                if let Some(name) = get_jsx_element_name(&element.name) {
                    let element_type = determine_element_type(&name);

                    // Determine action based on the element type
                    match element_type {
                        // If the element is a React component
                        ElementType::ReactComponent => {
                            // Process JSX attributes to create a React component
                            let mut react_component = process_jsx_attribute(&name, &element);

                            // Set the start and end of the code span in the react_component
                            if let Some(ref mut component) = react_component {
                                component.code_span.start = element.span.start;
                                component.code_span.end = element.span.end;
                            }

                            // If the element stack is empty, push the new component onto it
                            if self.component_stack.is_empty() {
                                self.component_stack.push(react_component.unwrap());
                            } else {
                                // Otherwise, insert the component into the existing stack
                                let name = self.element_name_stack.last().unwrap().clone();

                                insert_component(
                                    &mut self.component_stack,
                                    react_component.clone().unwrap(),
                                    name,
                                );
                            }

                            // If the element is not self-closing, add its name to the stack
                            if !element.self_closing {
                                self.element_name_stack.push(name.clone());
                            }
                        }

                        ElementType::HtmlTag => {
                            // println!("HTML Tag: {}", name);
                        }
                    }
                }
            }

            AstKind::JSXClosingElement(element) => {
                // Attempt to retrieve the name of the JSX element
                if let Some(name) = get_jsx_element_name(&element.name) {
                    // If successful, push the name onto the element stack

                    let element_type = determine_element_type(&name);

                    match element_type {
                        ElementType::ReactComponent => {
                            // Retrieve the code span of the JSXClosingElement
                            let code_span_start = element.span.start;
                            let code_span_end = element.span.end;

                            // Print the code span of the JSXClosingElement
                            println!(
                                "JSXClosingElement '{}' Code Span: {}-{}",
                                name, code_span_start, code_span_end
                            );

                            // if let Some(modified_component) = self.component_stack.last_mut() {
                            //     modified_component.code_span.end = code_span_end;
                            // }

                            self.element_name_stack.pop();

                            // // And print the name of the JSX element
                            // println!("JSX closing: {:?}", name);

                            // println!("Current Element Stack: {:?}", self.element_name_stack);
                        }
                        ElementType::HtmlTag => {
                            // println!("HTML Tag: {}", name);
                        }
                    }
                }
            }

            //For  export  declarations within the AST.
            AstKind::ExportDefaultDeclaration(decl) => {
                process_export_default_declaration(&decl);
            }

            //For  export  declarations within the AST.
            AstKind::ExportNamedDeclaration(decl) => {
                process_export_named_declaration(&decl);
            }

            // handle import statements
            AstKind::ImportDeclaration(decl) => {
                // create an empty string
                let mut import_name = String::new();

                // Process each import specifier
                if let Some(specifiers) = &decl.specifiers {
                    for specifier in specifiers {
                        match specifier {
                            ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                                // println!("Import Specifier: {:?}", spec.imported.name());
                                import_name = spec.imported.name().to_string();
                            }
                            ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                                // println!("Import Default Specifier: {:?}", spec.local.name);
                                import_name = spec.local.name.to_string();
                            }
                            ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                                // println!("Import Namespace Specifier: {:?}", spec.local.name);
                                import_name = spec.local.name.to_string();
                            }
                        }
                    }
                }

                // println!("{:?} \t{:?}", import_name, &decl.source.value);

                // Check if the import is a default import
                let is_default_import = matches!(
                    decl.specifiers.as_ref().map(|specs| specs.first()),
                    Some(Some(ImportDeclarationSpecifier::ImportDefaultSpecifier(_)))
                );

                // Create a new FileImport instance
                let file_import = FileImport {
                    name: import_name.clone(),                  // Clone the import name
                    import_path: decl.source.value.to_string(), // Convert the import path to a String
                    is_default: is_default_import,              // Set the default import flag
                };

                // Add the file import to the list of file imports
                self.file_imports.push(file_import);
            }

            _ => {}
        }
    }
}
