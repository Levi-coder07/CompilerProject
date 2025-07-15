import React, { useState, useEffect } from 'react';
import CodeEditor from './components/CodeEditor';
import TokenDisplay from './components/TokenDisplay';
import ASTVisualization from './components/ASTVisualization';
import { compilerApi } from './services/api';
import { TokenInfo, NodeData, EdgeData, Example } from './types';

function App() {
  const [code, setCode] = useState('x = 5 + 3 * 2');
  const [tokens, setTokens] = useState<TokenInfo[]>([]);
  const [astNodes, setAstNodes] = useState<NodeData[]>([]);
  const [astEdges, setAstEdges] = useState<EdgeData[]>([]);
  const [examples, setExamples] = useState<Example[]>([]);
  const [loading, setLoading] = useState({
    tokens: false,
    ast: false,
    examples: false
  });
  const [error, setError] = useState({
    tokens: null as string | null,
    ast: null as string | null,
    examples: null as string | null
  });

  // Load examples on component mount
  useEffect(() => {
    loadExamples();
  }, []);

  // Auto-compile when code changes
  useEffect(() => {
    if (code.trim()) {
      const timeoutId = setTimeout(() => {
        compileCode();
      }, 500); // Debounce
      return () => clearTimeout(timeoutId);
    }
  }, [code]);

  const loadExamples = async () => {
    setLoading(prev => ({ ...prev, examples: true }));
    try {
      const response = await compilerApi.getExamples();
      setExamples(response.examples);
      setError(prev => ({ ...prev, examples: null }));
    } catch (err) {
      setError(prev => ({ ...prev, examples: 'Failed to load examples' }));
    } finally {
      setLoading(prev => ({ ...prev, examples: false }));
    }
  };

  const compileCode = async () => {
    // Tokenize
    setLoading(prev => ({ ...prev, tokens: true }));
    try {
      const tokensResponse = await compilerApi.tokenize(code);
      if (tokensResponse.success) {
        setTokens(tokensResponse.tokens);
        setError(prev => ({ ...prev, tokens: null }));
      } else {
        setError(prev => ({ ...prev, tokens: tokensResponse.error }));
      }
    } catch (err) {
      setError(prev => ({ ...prev, tokens: 'Failed to tokenize code' }));
    } finally {
      setLoading(prev => ({ ...prev, tokens: false }));
    }

    // Parse and visualize
    setLoading(prev => ({ ...prev, ast: true }));
    try {
      const vizResponse = await compilerApi.visualize(code);
      if (vizResponse.success) {
        setAstNodes(vizResponse.nodes);
        setAstEdges(vizResponse.edges);
        setError(prev => ({ ...prev, ast: null }));
      } else {
        setError(prev => ({ ...prev, ast: vizResponse.error }));
      }
    } catch (err) {
      setError(prev => ({ ...prev, ast: 'Failed to parse code' }));
    } finally {
      setLoading(prev => ({ ...prev, ast: false }));
    }
  };

  const handleExampleSelect = (example: Example) => {
    setCode(example.code);
  };

  return (
    <div className="min-h-screen bg-gray-100 p-4">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            🧠 Compiler Visualizer
          </h1>
          <p className="text-gray-600">
            Educational tool to understand how compilers work through lexical analysis, parsing, and AST visualization
          </p>
        </div>

        {/* Examples */}
        <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
          <h2 className="text-xl font-semibold mb-4">📚 Examples</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {examples.map((example, index) => (
              <button
                key={index}
                onClick={() => handleExampleSelect(example)}
                className="p-4 border border-gray-200 rounded-lg hover:bg-gray-50 text-left transition-colors"
              >
                <div className="font-semibold text-gray-900 mb-1">{example.name}</div>
                <div className="text-sm text-gray-600 mb-2">{example.description}</div>
                <div className="text-xs font-mono bg-gray-100 p-2 rounded text-gray-800">
                  {example.code}
                </div>
              </button>
            ))}
          </div>
        </div>

        {/* Code Editor */}
        <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
          <h2 className="text-xl font-semibold mb-4">✏️ Code Editor</h2>
          <CodeEditor
            value={code}
            onChange={setCode}
            height="150px"
          />
          <div className="mt-4 flex gap-2">
            <button
              onClick={compileCode}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
              🔄 Compile
            </button>
            <button
              onClick={() => setCode('')}
              className="px-4 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition-colors"
            >
              🗑️ Clear
            </button>
          </div>
        </div>

        {/* Results */}
        <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
          {/* Tokens */}
          <TokenDisplay
            tokens={tokens}
            loading={loading.tokens}
            error={error.tokens}
          />

          {/* AST Visualization */}
          <ASTVisualization
            nodes={astNodes}
            edges={astEdges}
            loading={loading.ast}
            error={error.ast}
          />
        </div>
      </div>
    </div>
  );
}

export default App; 