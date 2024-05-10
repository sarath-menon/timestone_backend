use serde::{Deserialize, Serialize};

pub enum ElementType {
    ReactComponent,
    HtmlTag,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum FileType {
    #[default]
    TypeScript,
    JavaScript,
    JSX,
    TSX,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReactComponent {
    pub name: String,
    pub class_names: String,
    pub children: Vec<ReactComponent>,
    pub code_span: CodeSpan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileImport {
    pub name: String,
    pub import_path: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileInfo {
    pub file_name: String,
    pub file_path: String,
    pub file_type: FileType,
    pub file_imports: Vec<FileImport>,
    pub component_stack: Vec<ReactComponent>,
    pub element_name_stack: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CodeSpan {
    pub start: u32,
    pub end: u32,
}
