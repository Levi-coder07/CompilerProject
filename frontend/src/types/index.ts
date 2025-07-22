export interface TokenInfo {
  token_type: string;
  raw_value: string;
  position: number;
}

export interface TokenizeResponse {
  tokens: TokenInfo[];
  success: boolean;
  error: string | null;
}

export interface ASTNode {
  // This mirrors the Rust ASTNode structure
  [key: string]: any;
}

export interface ParseResponse {
  ast: ASTNode | null;
  success: boolean;
  error: string | null;
}

export interface NodeData {
  id: string;
  label: string;
  node_type: string;
  color: string;
}

export interface EdgeData {
  from: string;
  to: string;
}

export interface VisualizationResponse {
  dot_content: string;
  nodes: NodeData[];
  edges: EdgeData[];
  success: boolean;
  error: string | null;
}

export interface Example {
  name: string;
  code: string;
  description: string;
  category: string;
}

export interface SemanticStep {
  step_number: number;
  description: string;
  node_type: string;
  action: string;
  symbol_added: string | null;
  type_check: string | null;
  error: string | null;
}

export interface SymbolInfo {
  name: string;
  symbol_type: string;
  data_type: string;
  scope: string;
  line: number;
}

export interface TypeCheck {
  expression: string;
  expected_type: string;
  actual_type: string;
  is_valid: boolean;
  error_message: string | null;
}

export interface SemanticAnalysisResponse {
  steps: SemanticStep[];
  symbol_table: SymbolInfo[];
  type_checks: TypeCheck[];
  success: boolean;
  error: string | null;
}

export interface ExampleResponse {
  examples: Example[];
} 