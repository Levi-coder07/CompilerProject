use crate::ast::ASTNode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct GraphvizRenderer {
    node_counter: usize,
    node_ids: HashMap<*const ASTNode, usize>,
}

impl GraphvizRenderer {
    pub fn new() -> Self {
        GraphvizRenderer {
            node_counter: 0,
            node_ids: HashMap::new(),
        }
    }
    
    pub fn render_to_file(&mut self, ast: &ASTNode, filename: &str) -> Result<(), std::io::Error> {
        let dot_content = self.render_to_dot(ast);
        let mut file = File::create(filename)?;
        file.write_all(dot_content.as_bytes())?;
        println!("AST visualization saved to {}", filename);
        Ok(())
    }
    
    pub fn render_to_dot(&mut self, ast: &ASTNode) -> String {
        let mut dot = String::new();
        dot.push_str("digraph AST {\n");
        dot.push_str("  node [shape=rectangle, style=\"rounded,filled\", fillcolor=lightblue];\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("\n");
        
        self.render_node(ast, &mut dot);
        
        dot.push_str("}\n");
        dot
    }
    
    fn get_node_id(&mut self, node: &ASTNode) -> usize {
        let node_ptr = node as *const ASTNode;
        if let Some(&id) = self.node_ids.get(&node_ptr) {
            id
        } else {
            let id = self.node_counter;
            self.node_counter += 1;
            self.node_ids.insert(node_ptr, id);
            id
        }
    }
    
    fn render_node(&mut self, node: &ASTNode, dot: &mut String) -> usize {
        let node_id = self.get_node_id(node);
        
        // Create the node with appropriate styling
        let (label, color) = self.get_node_style(node);
        dot.push_str(&format!(
            "  node_{} [label=\"{}\", fillcolor=\"{}\"];\n",
            node_id, label, color
        ));
        
        // Handle children and edges
        match node {
            ASTNode::BinaryOp { left, operator: _, right } => {
                let left_id = self.render_node(left, dot);
                let right_id = self.render_node(right, dot);
                dot.push_str(&format!("  node_{} -> node_{} [label=\"left\"];\n", node_id, left_id));
                dot.push_str(&format!("  node_{} -> node_{} [label=\"right\"];\n", node_id, right_id));
            },
            ASTNode::UnaryOp { operator: _, operand } => {
                let operand_id = self.render_node(operand, dot);
                dot.push_str(&format!("  node_{} -> node_{} [label=\"operand\"];\n", node_id, operand_id));
            },
            ASTNode::Assignment { left, right } => {
                let left_id = self.render_node(left, dot);
                let right_id = self.render_node(right, dot);
                dot.push_str(&format!("  node_{} -> node_{} [label=\"left\"];\n", node_id, left_id));
                dot.push_str(&format!("  node_{} -> node_{} [label=\"right\"];\n", node_id, right_id));
            },
            ASTNode::FunctionCall { name: _, arguments } => {
                for (i, arg) in arguments.iter().enumerate() {
                    let arg_id = self.render_node(arg, dot);
                    dot.push_str(&format!("  node_{} -> node_{} [label=\"arg{}\"];\n", node_id, arg_id, i));
                }
            },
            ASTNode::Parenthesized { expression } => {
                let expr_id = self.render_node(expression, dot);
                dot.push_str(&format!("  node_{} -> node_{} [label=\"expr\"];\n", node_id, expr_id));
            },
            ASTNode::Program { statements } => {
                for (i, stmt) in statements.iter().enumerate() {
                    let stmt_id = self.render_node(stmt, dot);
                    dot.push_str(&format!("  node_{} -> node_{} [label=\"stmt{}\"];\n", node_id, stmt_id, i));
                }
            },
            ASTNode::ExpressionStatement { expression } => {
                let expr_id = self.render_node(expression, dot);
                dot.push_str(&format!("  node_{} -> node_{} [label=\"expr\"];\n", node_id, expr_id));
            },
            // Leaf nodes (literals, identifiers) don't have children
            ASTNode::Number { .. } | ASTNode::String { .. } | ASTNode::Boolean { .. } | ASTNode::Identifier { .. } => {},
        }
        
        node_id
    }
    
    fn get_node_style(&self, node: &ASTNode) -> (String, &'static str) {
        let label = self.escape_label(&node.label());
        match node {
            ASTNode::Number { .. } => (label, "lightgreen"),
            ASTNode::String { .. } => (label, "lightyellow"),
            ASTNode::Boolean { .. } => (label, "lightblue"),
            ASTNode::Identifier { .. } => (label, "lightcyan"),
            ASTNode::BinaryOp { .. } => (label, "lightcoral"),
            ASTNode::UnaryOp { .. } => (label, "lightpink"),
            ASTNode::Assignment { .. } => (label, "orange"),
            ASTNode::FunctionCall { .. } => (label, "lightsteelblue"),
            ASTNode::Parenthesized { .. } => (label, "lavender"),
            ASTNode::Program { .. } => (label, "lightgray"),
            ASTNode::ExpressionStatement { .. } => (label, "wheat"),
        }
    }
    
    fn escape_label(&self, label: &str) -> String {
        // First escape backslashes, then quotes, then newlines and tabs
        label.replace('\\', "\\\\")
             .replace('\"', "\\\"")
             .replace('\n', "\\n")
             .replace('\t', "\\t")
    }
}

impl Default for GraphvizRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to generate and render AST to PNG (requires Graphviz to be installed)
pub fn render_ast_to_png(ast: &ASTNode, base_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = GraphvizRenderer::new();
    let dot_filename = format!("{}.dot", base_filename);
    let png_filename = format!("{}.png", base_filename);
    
    // Save DOT file
    renderer.render_to_file(ast, &dot_filename)?;
    
    // Try to generate PNG using dot command
    match std::process::Command::new("dot")
        .args(&["-Tpng", &dot_filename, "-o", &png_filename])
        .output() {
        Ok(output) => {
            if output.status.success() {
                println!("PNG visualization saved to {}", png_filename);
                Ok(())
            } else {
                eprintln!("Warning: Could not generate PNG. Graphviz may not be installed.");
                eprintln!("You can install Graphviz and then run: dot -Tpng {} -o {}", dot_filename, png_filename);
                Ok(())
            }
        },
        Err(_) => {
            eprintln!("Warning: Could not generate PNG. Graphviz may not be installed.");
            eprintln!("You can install Graphviz and then run: dot -Tpng {} -o {}", dot_filename, png_filename);
            Ok(())
        }
    }
} 