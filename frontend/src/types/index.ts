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

export interface ExampleResponse {
  examples: Example[];
} 