import axios from 'axios';
import { TokenizeResponse, ParseResponse, VisualizationResponse, ExampleResponse, SemanticAnalysisResponse } from '../types';

const API_BASE_URL = 'http://localhost:3000';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const compilerApi = {
  tokenize: async (code: string): Promise<TokenizeResponse> => {
    const response = await apiClient.post('/api/tokenize', { code });
    return response.data;
  },

  parse: async (code: string): Promise<ParseResponse> => {
    const response = await apiClient.post('/api/parse', { code });
    return response.data;
  },

  visualize: async (code: string): Promise<VisualizationResponse> => {
    const response = await apiClient.post('/api/visualize', { code });
    return response.data;
  },

  getExamples: async (): Promise<ExampleResponse> => {
    const response = await apiClient.get('/api/examples');
    return response.data;
  },

  semanticAnalysis: async (code: string): Promise<SemanticAnalysisResponse> => {
    const response = await apiClient.post('/api/semantic-analysis', { code });
    return response.data;
  },

  healthCheck: async (): Promise<string> => {
    const response = await apiClient.get('/');
    return response.data;
  },
}; 