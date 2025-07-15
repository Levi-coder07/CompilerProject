import React from 'react';
import { TokenInfo } from '../types';

interface TokenDisplayProps {
  tokens: TokenInfo[];
  loading: boolean;
  error: string | null;
}

const TokenDisplay: React.FC<TokenDisplayProps> = ({ tokens, loading, error }) => {
  if (loading) {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">Tokens</h3>
        <div className="flex items-center justify-center py-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">Tokens</h3>
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <strong>Error:</strong> {error}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white p-4 rounded-lg shadow">
      <h3 className="text-lg font-semibold mb-3">Tokens ({tokens.length})</h3>
      <div className="space-y-2 max-h-96 overflow-y-auto">
        {tokens.map((token, index) => (
          <div key={index} className="flex items-center justify-between p-2 bg-gray-50 rounded">
            <div className="flex items-center space-x-3">
              <span className="text-sm font-mono bg-blue-100 text-blue-800 px-2 py-1 rounded">
                {token.position}
              </span>
              <span className="font-semibold text-gray-700">{token.token_type}</span>
            </div>
            <div className="text-sm text-gray-600 font-mono max-w-xs truncate">
              {token.raw_value}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default TokenDisplay; 