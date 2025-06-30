#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    // Literals
    Number { value: String, is_float: bool },
    String { value: String },
    Identifier { name: String },
    
    // Binary operations
    BinaryOp {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    
    // Unary operations
    UnaryOp {
        operator: String,
        operand: Box<ASTNode>,
    },
    
    // Assignment
    Assignment {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    
    // Function call
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },
    
    // Parenthesized expression
    Parenthesized {
        expression: Box<ASTNode>,
    },
    
    // Program (root node)
    Program {
        statements: Vec<ASTNode>,
    },
    
    // Expression statement
    ExpressionStatement {
        expression: Box<ASTNode>,
    },
}

impl ASTNode {
    pub fn node_type(&self) -> &'static str {
        match self {
            ASTNode::Number { .. } => "Number",
            ASTNode::String { .. } => "String",
            ASTNode::Identifier { .. } => "Identifier",
            ASTNode::BinaryOp { .. } => "BinaryOp",
            ASTNode::UnaryOp { .. } => "UnaryOp",
            ASTNode::Assignment { .. } => "Assignment",
            ASTNode::FunctionCall { .. } => "FunctionCall",
            ASTNode::Parenthesized { .. } => "Parenthesized",
            ASTNode::Program { .. } => "Program",
            ASTNode::ExpressionStatement { .. } => "ExpressionStatement",
        }
    }
    
    pub fn label(&self) -> String {
        match self {
            ASTNode::Number { value, is_float } => {
                format!("Number\n{} ({})", value, if *is_float { "float" } else { "int" })
            },
            ASTNode::String { value } => format!("String\n\"{}\"", value),
            ASTNode::Identifier { name } => format!("Identifier\n{}", name),
            ASTNode::BinaryOp { operator, .. } => format!("BinaryOp\n{}", operator),
            ASTNode::UnaryOp { operator, .. } => format!("UnaryOp\n{}", operator),
            ASTNode::Assignment { .. } => "Assignment\n=".to_string(),
            ASTNode::FunctionCall { name, .. } => format!("FunctionCall\n{}", name),
            ASTNode::Parenthesized { .. } => "Parenthesized\n( )".to_string(),
            ASTNode::Program { .. } => "Program".to_string(),
            ASTNode::ExpressionStatement { .. } => "ExpressionStatement".to_string(),
        }
    }
} 