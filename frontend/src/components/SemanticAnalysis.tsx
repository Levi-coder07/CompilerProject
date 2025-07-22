import React, { useState } from 'react';
import { SemanticStep, SymbolInfo, TypeCheck } from '../types';

interface SemanticAnalysisProps {
  steps: SemanticStep[];
  symbolTable: SymbolInfo[];
  typeChecks: TypeCheck[];
  loading: boolean;
  error: string | null;
}

const SemanticAnalysis: React.FC<SemanticAnalysisProps> = ({ 
  steps, 
  symbolTable, 
  typeChecks, 
  loading, 
  error 
}) => {
  const [activeTab, setActiveTab] = useState<'steps' | 'symbols' | 'types'>('steps');
  const [currentStep, setCurrentStep] = useState(0);

  if (loading) {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">Análisis Semántico</h3>
        <div className="flex items-center justify-center py-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">Análisis Semántico</h3>
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <strong>Error:</strong> {error}
        </div>
      </div>
    );
  }

  const currentStepData = steps[currentStep];
  const progress = steps.length > 0 ? ((currentStep + 1) / steps.length) * 100 : 0;

  return (
    <div className="bg-white p-4 rounded-lg shadow">
      <h3 className="text-lg font-semibold mb-3">Análisis Semántico</h3>
      
      {/* Tabs */}
      <div className="flex border-b mb-4">
        <button
          onClick={() => setActiveTab('steps')}
          className={`px-4 py-2 font-medium ${
            activeTab === 'steps' 
              ? 'border-b-2 border-green-500 text-green-600' 
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Pasos ({steps.length})
        </button>
        <button
          onClick={() => setActiveTab('symbols')}
          className={`px-4 py-2 font-medium ${
            activeTab === 'symbols' 
              ? 'border-b-2 border-green-500 text-green-600' 
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Tabla de Símbolos ({symbolTable.length})
        </button>
        <button
          onClick={() => setActiveTab('types')}
          className={`px-4 py-2 font-medium ${
            activeTab === 'types' 
              ? 'border-b-2 border-green-500 text-green-600' 
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Verificación de Tipos ({typeChecks.length})
        </button>
      </div>

      {/* Tab Content */}
      {activeTab === 'steps' && (
        <div>
          {/* Controls */}
          <div className="flex items-center gap-4 mb-4">
            <button
              onClick={() => setCurrentStep(Math.max(0, currentStep - 1))}
              disabled={currentStep === 0}
              className="px-3 py-1 bg-gray-500 text-white rounded disabled:opacity-50"
            >
              ⏮️ Anterior
            </button>
            
            <span className="text-sm text-gray-600">
              Paso {currentStep + 1} de {steps.length}
            </span>
            
            <button
              onClick={() => setCurrentStep(Math.min(steps.length - 1, currentStep + 1))}
              disabled={currentStep === steps.length - 1}
              className="px-3 py-1 bg-green-600 text-white rounded disabled:opacity-50"
            >
              ⏭️ Siguiente
            </button>
          </div>

          {/* Progress bar */}
          <div className="w-full bg-gray-200 rounded-full h-2 mb-4">
            <div 
              className="bg-green-600 h-2 rounded-full transition-all duration-300"
              style={{ width: `${progress}%` }}
            ></div>
          </div>

          {/* Current step details */}
          {currentStepData && (
            <div className="space-y-4">
              <div className={`p-4 rounded-lg ${
                currentStepData.error 
                  ? 'bg-red-50 border border-red-200' 
                  : 'bg-green-50 border border-green-200'
              }`}>
                <h4 className={`font-semibold mb-2 ${
                  currentStepData.error ? 'text-red-900' : 'text-green-900'
                }`}>
                  Paso {currentStepData.step_number}: {currentStepData.description}
                </h4>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <h5 className="font-medium text-gray-700 mb-1">Tipo de Nodo:</h5>
                    <div className="bg-white p-2 rounded border text-sm">
                      {currentStepData.node_type}
                    </div>
                  </div>
                  
                  <div>
                    <h5 className="font-medium text-gray-700 mb-1">Acción:</h5>
                    <div className="bg-white p-2 rounded border text-sm">
                      {currentStepData.action}
                    </div>
                  </div>
                </div>
                
                {currentStepData.symbol_added && (
                  <div className="mt-3">
                    <h5 className="font-medium text-gray-700 mb-1">Símbolo Agregado:</h5>
                    <div className="bg-blue-50 p-2 rounded border text-sm font-mono text-blue-800">
                      {currentStepData.symbol_added}
                    </div>
                  </div>
                )}
                
                {currentStepData.type_check && (
                  <div className="mt-3">
                    <h5 className="font-medium text-gray-700 mb-1">Verificación de Tipo:</h5>
                    <div className="bg-purple-50 p-2 rounded border text-sm font-mono text-purple-800">
                      {currentStepData.type_check}
                    </div>
                  </div>
                )}
                
                {currentStepData.error && (
                  <div className="mt-3">
                    <h5 className="font-medium text-red-700 mb-1">Error:</h5>
                    <div className="bg-red-100 p-2 rounded border text-sm text-red-800">
                      {currentStepData.error}
                    </div>
                  </div>
                )}
              </div>
            </div>
          )}

          {/* All steps overview */}
          <div className="mt-6">
            <h5 className="font-medium text-gray-700 mb-2">Resumen de Pasos:</h5>
            <div className="max-h-40 overflow-y-auto bg-gray-50 p-3 rounded">
              <div className="space-y-1">
                {steps.map((step, index) => (
                  <div 
                    key={index}
                    onClick={() => setCurrentStep(index)}
                    className={`text-sm p-2 rounded cursor-pointer transition-colors ${
                      index === currentStep 
                        ? 'bg-green-100 border-green-300 border' 
                        : 'bg-white hover:bg-gray-100'
                    }`}
                  >
                    <span className="font-medium">Paso {step.step_number}:</span> {step.description}
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'symbols' && (
        <div>
          <h4 className="font-semibold mb-3">Tabla de Símbolos</h4>
          {symbolTable.length > 0 ? (
            <div className="overflow-x-auto">
              <table className="min-w-full bg-white border border-gray-300">
                <thead>
                  <tr className="bg-gray-50">
                    <th className="px-4 py-2 border-b text-left">Nombre</th>
                    <th className="px-4 py-2 border-b text-left">Tipo</th>
                    <th className="px-4 py-2 border-b text-left">Tipo de Dato</th>
                    <th className="px-4 py-2 border-b text-left">Alcance</th>
                    <th className="px-4 py-2 border-b text-left">Línea</th>
                  </tr>
                </thead>
                <tbody>
                  {symbolTable.map((symbol, index) => (
                    <tr key={index} className="hover:bg-gray-50">
                      <td className="px-4 py-2 border-b font-mono">{symbol.name}</td>
                      <td className="px-4 py-2 border-b">{symbol.symbol_type}</td>
                      <td className="px-4 py-2 border-b">{symbol.data_type}</td>
                      <td className="px-4 py-2 border-b">{symbol.scope}</td>
                      <td className="px-4 py-2 border-b">{symbol.line}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ) : (
            <div className="text-gray-500 text-center py-8">
              No hay símbolos en la tabla
            </div>
          )}
        </div>
      )}

      {activeTab === 'types' && (
        <div>
          <h4 className="font-semibold mb-3">Verificación de Tipos</h4>
          {typeChecks.length > 0 ? (
            <div className="space-y-3">
              {typeChecks.map((check, index) => (
                <div key={index} className={`p-3 rounded border ${
                  check.is_valid ? 'bg-green-50 border-green-200' : 'bg-red-50 border-red-200'
                }`}>
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <div className="font-medium text-gray-900">{check.expression}</div>
                      <div className="text-sm text-gray-600 mt-1">
                        Esperado: <span className="font-mono">{check.expected_type}</span> | 
                        Actual: <span className="font-mono">{check.actual_type}</span>
                      </div>
                      {check.error_message && (
                        <div className="text-sm text-red-600 mt-1">{check.error_message}</div>
                      )}
                    </div>
                    <div className={`px-2 py-1 rounded text-xs font-medium ${
                      check.is_valid 
                        ? 'bg-green-100 text-green-800' 
                        : 'bg-red-100 text-red-800'
                    }`}>
                      {check.is_valid ? '✓ Válido' : '✗ Inválido'}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-gray-500 text-center py-8">
              No hay verificaciones de tipo
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default SemanticAnalysis; 