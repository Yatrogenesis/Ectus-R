// AION-R AST Parser: Real AST parsing with tree-sitter for code refactoring
// Replaces string-based transformations with proper syntax tree analysis

use std::path::Path;
use anyhow::{Result, Context, anyhow};
use tracing::{info, debug, warn};
use serde::{Serialize, Deserialize};

/// Supported programming languages for AST parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    TypeScript,
    Python,
    Go,
}

impl Language {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Language::Rust),
            "ts" | "tsx" => Some(Language::TypeScript),
            "py" => Some(Language::Python),
            "go" => Some(Language::Go),
            _ => None,
        }
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

/// Represents a parsed syntax tree
#[derive(Debug, Clone)]
pub struct AST {
    pub language: Language,
    pub root_node: ASTNode,
    pub source_code: String,
}

/// Node in the abstract syntax tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASTNode {
    pub node_type: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_position: Position,
    pub end_position: Position,
    pub children: Vec<ASTNode>,
    pub text: String,
}

/// Position in source code (line, column)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Represents a function definition in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub visibility: Option<String>,
    pub is_async: bool,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
    pub default_value: Option<String>,
}

/// Represents a variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Option<String>,
    pub value: Option<String>,
    pub is_mutable: bool,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Represents a struct/class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub methods: Vec<FunctionDefinition>,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Struct field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: String,
    pub visibility: Option<String>,
}

/// Main AST parser interface
pub struct ASTParser {
    rust_parser: RustASTParser,
    typescript_parser: TypeScriptASTParser,
    python_parser: PythonASTParser,
    go_parser: GoASTParser,
}

impl ASTParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            rust_parser: RustASTParser::new()?,
            typescript_parser: TypeScriptASTParser::new()?,
            python_parser: PythonASTParser::new()?,
            go_parser: GoASTParser::new()?,
        })
    }

    /// Parse source code into AST
    pub fn parse(&self, source_code: &str, language: Language) -> Result<AST> {
        debug!("Parsing {} code ({} bytes)", language_name(language), source_code.len());

        match language {
            Language::Rust => self.rust_parser.parse(source_code),
            Language::TypeScript => self.typescript_parser.parse(source_code),
            Language::Python => self.python_parser.parse(source_code),
            Language::Go => self.go_parser.parse(source_code),
        }
    }

    /// Extract all function definitions from AST
    pub fn extract_functions(&self, ast: &AST) -> Result<Vec<FunctionDefinition>> {
        match ast.language {
            Language::Rust => self.rust_parser.extract_functions(ast),
            Language::TypeScript => self.typescript_parser.extract_functions(ast),
            Language::Python => self.python_parser.extract_functions(ast),
            Language::Go => self.go_parser.extract_functions(ast),
        }
    }

    /// Extract all variable declarations from AST
    pub fn extract_variables(&self, ast: &AST) -> Result<Vec<VariableDeclaration>> {
        match ast.language {
            Language::Rust => self.rust_parser.extract_variables(ast),
            Language::TypeScript => self.typescript_parser.extract_variables(ast),
            Language::Python => self.python_parser.extract_variables(ast),
            Language::Go => self.go_parser.extract_variables(ast),
        }
    }

    /// Extract all struct/class definitions from AST
    pub fn extract_structs(&self, ast: &AST) -> Result<Vec<StructDefinition>> {
        match ast.language {
            Language::Rust => self.rust_parser.extract_structs(ast),
            Language::TypeScript => self.typescript_parser.extract_structs(ast),
            Language::Python => self.python_parser.extract_structs(ast),
            Language::Go => self.go_parser.extract_structs(ast),
        }
    }

    /// Find node at specific byte offset
    pub fn find_node_at_position(&self, ast: &AST, byte_offset: usize) -> Option<&ASTNode> {
        find_node_recursive(&ast.root_node, byte_offset)
    }

    /// Get source code slice for a node
    pub fn get_node_text(&self, ast: &AST, node: &ASTNode) -> &str {
        &ast.source_code[node.start_byte..node.end_byte]
    }
}

/// Rust AST Parser
pub struct RustASTParser {
    parser: tree_sitter::Parser,
}

impl RustASTParser {
    pub fn new() -> Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_rust::language())
            .context("Failed to set Rust language for tree-sitter")?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source_code: &str) -> Result<AST> {
        let tree = self.parser.parse(source_code, None)
            .ok_or_else(|| anyhow!("Failed to parse Rust code"))?;

        let root_node = convert_tree_sitter_node(tree.root_node(), source_code);

        Ok(AST {
            language: Language::Rust,
            root_node,
            source_code: source_code.to_string(),
        })
    }

    pub fn extract_functions(&self, ast: &AST) -> Result<Vec<FunctionDefinition>> {
        let mut functions = Vec::new();
        extract_functions_recursive(&ast.root_node, &ast.source_code, &mut functions, Language::Rust);
        Ok(functions)
    }

    pub fn extract_variables(&self, ast: &AST) -> Result<Vec<VariableDeclaration>> {
        let mut variables = Vec::new();
        extract_variables_recursive(&ast.root_node, &ast.source_code, &mut variables, Language::Rust);
        Ok(variables)
    }

    pub fn extract_structs(&self, ast: &AST) -> Result<Vec<StructDefinition>> {
        let mut structs = Vec::new();
        extract_structs_recursive(&ast.root_node, &ast.source_code, &mut structs, Language::Rust);
        Ok(structs)
    }
}

/// TypeScript AST Parser
pub struct TypeScriptASTParser {
    parser: tree_sitter::Parser,
}

impl TypeScriptASTParser {
    pub fn new() -> Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_typescript::language_typescript())
            .context("Failed to set TypeScript language for tree-sitter")?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source_code: &str) -> Result<AST> {
        let tree = self.parser.parse(source_code, None)
            .ok_or_else(|| anyhow!("Failed to parse TypeScript code"))?;

        let root_node = convert_tree_sitter_node(tree.root_node(), source_code);

        Ok(AST {
            language: Language::TypeScript,
            root_node,
            source_code: source_code.to_string(),
        })
    }

    pub fn extract_functions(&self, ast: &AST) -> Result<Vec<FunctionDefinition>> {
        let mut functions = Vec::new();
        extract_functions_recursive(&ast.root_node, &ast.source_code, &mut functions, Language::TypeScript);
        Ok(functions)
    }

    pub fn extract_variables(&self, ast: &AST) -> Result<Vec<VariableDeclaration>> {
        let mut variables = Vec::new();
        extract_variables_recursive(&ast.root_node, &ast.source_code, &mut variables, Language::TypeScript);
        Ok(variables)
    }

    pub fn extract_structs(&self, ast: &AST) -> Result<Vec<StructDefinition>> {
        let mut structs = Vec::new();
        extract_structs_recursive(&ast.root_node, &ast.source_code, &mut structs, Language::TypeScript);
        Ok(structs)
    }
}

/// Python AST Parser
pub struct PythonASTParser {
    parser: tree_sitter::Parser,
}

impl PythonASTParser {
    pub fn new() -> Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_python::language())
            .context("Failed to set Python language for tree-sitter")?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source_code: &str) -> Result<AST> {
        let tree = self.parser.parse(source_code, None)
            .ok_or_else(|| anyhow!("Failed to parse Python code"))?;

        let root_node = convert_tree_sitter_node(tree.root_node(), source_code);

        Ok(AST {
            language: Language::Python,
            root_node,
            source_code: source_code.to_string(),
        })
    }

    pub fn extract_functions(&self, ast: &AST) -> Result<Vec<FunctionDefinition>> {
        let mut functions = Vec::new();
        extract_functions_recursive(&ast.root_node, &ast.source_code, &mut functions, Language::Python);
        Ok(functions)
    }

    pub fn extract_variables(&self, ast: &AST) -> Result<Vec<VariableDeclaration>> {
        let mut variables = Vec::new();
        extract_variables_recursive(&ast.root_node, &ast.source_code, &mut variables, Language::Python);
        Ok(variables)
    }

    pub fn extract_structs(&self, ast: &AST) -> Result<Vec<StructDefinition>> {
        let mut structs = Vec::new();
        extract_structs_recursive(&ast.root_node, &ast.source_code, &mut structs, Language::Python);
        Ok(structs)
    }
}

/// Go AST Parser
pub struct GoASTParser {
    parser: tree_sitter::Parser,
}

impl GoASTParser {
    pub fn new() -> Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_go::language())
            .context("Failed to set Go language for tree-sitter")?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source_code: &str) -> Result<AST> {
        let tree = self.parser.parse(source_code, None)
            .ok_or_else(|| anyhow!("Failed to parse Go code"))?;

        let root_node = convert_tree_sitter_node(tree.root_node(), source_code);

        Ok(AST {
            language: Language::Go,
            root_node,
            source_code: source_code.to_string(),
        })
    }

    pub fn extract_functions(&self, ast: &AST) -> Result<Vec<FunctionDefinition>> {
        let mut functions = Vec::new();
        extract_functions_recursive(&ast.root_node, &ast.source_code, &mut functions, Language::Go);
        Ok(functions)
    }

    pub fn extract_variables(&self, ast: &AST) -> Result<Vec<VariableDeclaration>> {
        let mut variables = Vec::new();
        extract_variables_recursive(&ast.root_node, &ast.source_code, &mut variables, Language::Go);
        Ok(variables)
    }

    pub fn extract_structs(&self, ast: &AST) -> Result<Vec<StructDefinition>> {
        let mut structs = Vec::new();
        extract_structs_recursive(&ast.root_node, &ast.source_code, &mut structs, Language::Go);
        Ok(structs)
    }
}

// Helper functions

fn convert_tree_sitter_node(node: tree_sitter::Node, source_code: &str) -> ASTNode {
    let text = node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string();
    let children = (0..node.child_count())
        .filter_map(|i| node.child(i))
        .map(|child| convert_tree_sitter_node(child, source_code))
        .collect();

    ASTNode {
        node_type: node.kind().to_string(),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_position: Position {
            line: node.start_position().row,
            column: node.start_position().column,
        },
        end_position: Position {
            line: node.end_position().row,
            column: node.end_position().column,
        },
        children,
        text,
    }
}

fn find_node_recursive(node: &ASTNode, byte_offset: usize) -> Option<&ASTNode> {
    if byte_offset < node.start_byte || byte_offset > node.end_byte {
        return None;
    }

    for child in &node.children {
        if let Some(found) = find_node_recursive(child, byte_offset) {
            return Some(found);
        }
    }

    Some(node)
}

fn extract_functions_recursive(
    node: &ASTNode,
    source_code: &str,
    functions: &mut Vec<FunctionDefinition>,
    language: Language,
) {
    let function_node_types = match language {
        Language::Rust => vec!["function_item", "function_signature_item"],
        Language::TypeScript => vec!["function_declaration", "method_definition", "arrow_function"],
        Language::Python => vec!["function_definition"],
        Language::Go => vec!["function_declaration", "method_declaration"],
    };

    if function_node_types.contains(&node.node_type.as_str()) {
        if let Some(func) = parse_function_definition(node, source_code, language) {
            functions.push(func);
        }
    }

    for child in &node.children {
        extract_functions_recursive(child, source_code, functions, language);
    }
}

fn extract_variables_recursive(
    node: &ASTNode,
    source_code: &str,
    variables: &mut Vec<VariableDeclaration>,
    language: Language,
) {
    let var_node_types = match language {
        Language::Rust => vec!["let_declaration"],
        Language::TypeScript => vec!["variable_declaration", "lexical_declaration"],
        Language::Python => vec!["assignment"],
        Language::Go => vec!["var_declaration", "short_var_declaration"],
    };

    if var_node_types.contains(&node.node_type.as_str()) {
        if let Some(var) = parse_variable_declaration(node, source_code, language) {
            variables.push(var);
        }
    }

    for child in &node.children {
        extract_variables_recursive(child, source_code, variables, language);
    }
}

fn extract_structs_recursive(
    node: &ASTNode,
    source_code: &str,
    structs: &mut Vec<StructDefinition>,
    language: Language,
) {
    let struct_node_types = match language {
        Language::Rust => vec!["struct_item"],
        Language::TypeScript => vec!["class_declaration", "interface_declaration"],
        Language::Python => vec!["class_definition"],
        Language::Go => vec!["type_declaration"],
    };

    if struct_node_types.contains(&node.node_type.as_str()) {
        if let Some(s) = parse_struct_definition(node, source_code, language) {
            structs.push(s);
        }
    }

    for child in &node.children {
        extract_structs_recursive(child, source_code, structs, language);
    }
}

fn parse_function_definition(
    node: &ASTNode,
    source_code: &str,
    language: Language,
) -> Option<FunctionDefinition> {
    // Simplified parser - real implementation would be more sophisticated
    let name = extract_function_name(node, language)?;
    let parameters = extract_parameters(node, language);
    let return_type = extract_return_type(node, language);
    let body = extract_function_body(node, source_code);

    Some(FunctionDefinition {
        name,
        parameters,
        return_type,
        body,
        start_byte: node.start_byte,
        end_byte: node.end_byte,
        start_line: node.start_position.line,
        end_line: node.end_position.line,
        visibility: None,
        is_async: false,
    })
}

fn parse_variable_declaration(
    node: &ASTNode,
    source_code: &str,
    language: Language,
) -> Option<VariableDeclaration> {
    // Simplified parser - real implementation would be more sophisticated
    let name = extract_variable_name(node, language)?;

    Some(VariableDeclaration {
        name,
        var_type: None,
        value: None,
        is_mutable: false,
        start_byte: node.start_byte,
        end_byte: node.end_byte,
    })
}

fn parse_struct_definition(
    node: &ASTNode,
    source_code: &str,
    language: Language,
) -> Option<StructDefinition> {
    // Simplified parser - real implementation would be more sophisticated
    let name = extract_struct_name(node, language)?;

    Some(StructDefinition {
        name,
        fields: Vec::new(),
        methods: Vec::new(),
        start_byte: node.start_byte,
        end_byte: node.end_byte,
    })
}

fn extract_function_name(node: &ASTNode, language: Language) -> Option<String> {
    // Look for identifier child node
    for child in &node.children {
        if child.node_type == "identifier" || child.node_type == "name" {
            return Some(child.text.clone());
        }
    }
    None
}

fn extract_parameters(node: &ASTNode, language: Language) -> Vec<Parameter> {
    let mut params = Vec::new();

    // Look for parameters child node
    for child in &node.children {
        if child.node_type == "parameters" || child.node_type == "parameter_list" {
            for param_node in &child.children {
                if let Some(name) = extract_parameter_name(param_node, language) {
                    params.push(Parameter {
                        name,
                        param_type: None,
                        default_value: None,
                    });
                }
            }
        }
    }

    params
}

fn extract_parameter_name(node: &ASTNode, language: Language) -> Option<String> {
    if node.node_type == "identifier" || node.node_type == "parameter" {
        return Some(node.text.clone());
    }

    for child in &node.children {
        if child.node_type == "identifier" {
            return Some(child.text.clone());
        }
    }

    None
}

fn extract_return_type(node: &ASTNode, language: Language) -> Option<String> {
    for child in &node.children {
        if child.node_type == "return_type" || child.node_type == "type_annotation" {
            return Some(child.text.clone());
        }
    }
    None
}

fn extract_function_body(node: &ASTNode, source_code: &str) -> String {
    for child in &node.children {
        if child.node_type == "block" || child.node_type == "body" {
            return child.text.clone();
        }
    }
    "".to_string()
}

fn extract_variable_name(node: &ASTNode, language: Language) -> Option<String> {
    for child in &node.children {
        if child.node_type == "identifier" || child.node_type == "pattern" {
            return Some(child.text.clone());
        }
    }
    None
}

fn extract_struct_name(node: &ASTNode, language: Language) -> Option<String> {
    for child in &node.children {
        if child.node_type == "identifier" || child.node_type == "type_identifier" {
            return Some(child.text.clone());
        }
    }
    None
}

fn language_name(language: Language) -> &'static str {
    match language {
        Language::Rust => "Rust",
        Language::TypeScript => "TypeScript",
        Language::Python => "Python",
        Language::Go => "Go",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
        assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_extension("py"), Some(Language::Python));
        assert_eq!(Language::from_extension("go"), Some(Language::Go));
        assert_eq!(Language::from_extension("unknown"), None);
    }

    #[test]
    fn test_rust_parser() {
        let parser = RustASTParser::new().expect("Failed to create Rust parser");

        let source = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ast = parser.parse(source).expect("Failed to parse Rust code");
        assert_eq!(ast.language, Language::Rust);

        let functions = parser.extract_functions(&ast).expect("Failed to extract functions");
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "add");
    }

    #[test]
    fn test_typescript_parser() {
        let parser = TypeScriptASTParser::new().expect("Failed to create TypeScript parser");

        let source = r#"
function greet(name: string): string {
    return `Hello, ${name}`;
}
"#;

        let ast = parser.parse(source).expect("Failed to parse TypeScript code");
        assert_eq!(ast.language, Language::TypeScript);
    }

    #[test]
    fn test_python_parser() {
        let parser = PythonASTParser::new().expect("Failed to create Python parser");

        let source = r#"
def multiply(a, b):
    return a * b
"#;

        let ast = parser.parse(source).expect("Failed to parse Python code");
        assert_eq!(ast.language, Language::Python);
    }

    #[test]
    fn test_go_parser() {
        let parser = GoASTParser::new().expect("Failed to create Go parser");

        let source = r#"
func divide(a int, b int) int {
    return a / b
}
"#;

        let ast = parser.parse(source).expect("Failed to parse Go code");
        assert_eq!(ast.language, Language::Go);
    }
}
