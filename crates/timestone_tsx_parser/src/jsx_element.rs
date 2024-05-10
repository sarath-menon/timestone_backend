use oxc_ast::ast::{
    JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXElementName, JSXOpeningElement,
};

use crate::types::ReactComponent;

// Function to get JSX element name
pub fn get_jsx_element_name(name: &JSXElementName) -> Option<String> {
    match name {
        // If the JSX element name is an identifier, return its name
        JSXElementName::Identifier(identifier) => Some(identifier.name.to_string()),

        // If the JSX element name is a namespaced name, return its name
        JSXElementName::NamespacedName(namespaced_name) => Some(namespaced_name.to_string()),

        // If the JSX element name is a member expression, return the name of the object identifier
        JSXElementName::MemberExpression(member_expression) => {
            Some(member_expression.get_object_identifier().name.to_string())
        }
    }
}

pub fn process_jsx_attribute(name: &String, element: &JSXOpeningElement) -> Option<ReactComponent> {
    // Create a variable of type ReactComponent
    let mut react_component = ReactComponent::default();

    react_component.name = name.to_string();

    // Iterate over the attributes of the JSXOpeningElement
    for attribute in &element.attributes {
        // Check if the attribute is a JSXAttributeItem::Attribute
        if let JSXAttributeItem::Attribute(attribute) = attribute {
            // Check if the name of the attribute is an Identifier and its name is "className"
            if let JSXAttributeName::Identifier(ident) = &attribute.name {
                if ident.name == "className" {
                    // Check if the value of the attribute is a JSXAttributeValue::StringLiteral
                    if let Some(JSXAttributeValue::StringLiteral(string_literal)) = &attribute.value
                    {
                        // Set the value of the StringLiteral to the class_names field of the react_component
                        react_component.class_names = string_literal.value.to_string();
                    }
                }
            }
        }
    }

    // Return the value of the component_props
    Some(react_component)
}
