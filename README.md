# CompilerProject

A Rust-based compiler with lexical analysis, parsing, and AST visualization capabilities.

## Features

- **Lexer**: Tokenizes input code including:
  - Numbers (integers and floats with scientific notation)
  - Strings with escape sequences
  - Identifiers
  - Operators (`=`, `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`)
  - Punctuation (`()`, `[]`, `{}`, `,`, `;`)

- **Parser**: Builds Abstract Syntax Trees (AST) with support for:
  - Binary operations with proper operator precedence
  - Unary operations
  - Assignment expressions
  - Function calls
  - Parenthesized expressions
  - String and numeric literals

- **AST Visualization**: Generates beautiful tree visualizations using Graphviz:
  - DOT format files for all parsed expressions
  - PNG images (when Graphviz is installed)
  - Color-coded nodes by type
  - Clear tree structure showing parsing relationships

## Installation

1. Install Rust: https://rustup.rs/
2. Install Graphviz for visualization:
   ```bash
   # Windows (using winget)
   winget install Graphviz
   
   # macOS
   brew install graphviz
   
   # Ubuntu/Debian
   sudo apt install graphviz
   ```

## Usage

```bash
# Build the project
cargo build

# Run with example expressions
cargo run
```

The program will:
1. Tokenize several test expressions
2. Parse them into ASTs
3. Generate DOT files for visualization
4. Create PNG images (if Graphviz is available)

## Example Expressions

The program tests these expressions:
- `id2 = "Mi nombre es Levi"` - String assignment
- `x = 5 + 3 * 2` - Arithmetic with precedence
- `result = (a + b) * c` - Parenthesized expressions
- `func(x, y + 1)` - Function calls with arguments
- `a > b && c <= d` - Logical operations

## Output Files

After running, you'll find:
- `ast_test_N.dot` - Graphviz DOT format files
- `ast_test_N.png` - Visual tree representations (if Graphviz installed)

## Architecture

- `src/main.rs` - Main program demonstrating the pipeline
- `compiler_core/` - Core library with:
  - `lexer/` - Tokenization logic
  - `parser.rs` - Recursive descent parser
  - `ast.rs` - AST node definitions
  - `graphviz.rs` - Visualization generation