import React, { useEffect, useRef, useState } from 'react';
import { NodeData, EdgeData } from '../types';

interface ASTVisualizationProps {
  nodes: NodeData[];
  edges: EdgeData[];
  loading: boolean;
  error: string | null;
}

const ASTVisualization: React.FC<ASTVisualizationProps> = ({ nodes, edges, loading, error }) => {
  const svgRef = useRef<SVGSVGElement>(null);
  const modalSvgRef = useRef<SVGSVGElement>(null);
  const [showModal, setShowModal] = useState(false);
  const [zoom, setZoom] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });

  // Calculate dynamic height based on tree depth
  const calculateDynamicHeight = () => {
    if (nodes.length === 0) return 400;
    const tempPositions = calculateNodePositions(nodes, edges, Math.max(600, nodes.length * 120), 600);
    const maxY = Object.values(tempPositions).reduce((max, pos) => Math.max(max, pos.y), 0);
    return Math.max(400, maxY + 100);
  };

  const dynamicHeight = calculateDynamicHeight();

  // Zoom and pan handlers
  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    const newZoom = Math.max(0.5, Math.min(3, zoom * delta));
    setZoom(newZoom);
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    setDragStart({ x: e.clientX - pan.x, y: e.clientY - pan.y });
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isDragging) {
      setPan({
        x: e.clientX - dragStart.x,
        y: e.clientY - dragStart.y
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const resetView = () => {
    setZoom(1);
    setPan({ x: 0, y: 0 });
  };

  const renderSVG = (svgElement: SVGSVGElement, width: number, height: number) => {
    // Clear previous content
    svgElement.innerHTML = '';

    // Create a traditional tree layout with dynamic width
    const minWidth = 600;
    const dynamicWidth = Math.max(width, minWidth, nodes.length * 120);
    const nodePositions = calculateNodePositions(nodes, edges, dynamicWidth, height);
    
    // Draw edges
    edges.forEach(edge => {
      const fromPos = nodePositions[edge.from];
      const toPos = nodePositions[edge.to];
      
      if (fromPos && toPos) {
        const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
        line.setAttribute('x1', fromPos.x.toString());
        line.setAttribute('y1', fromPos.y.toString());
        line.setAttribute('x2', toPos.x.toString());
        line.setAttribute('y2', toPos.y.toString());
        line.setAttribute('stroke', '#374151');
        line.setAttribute('stroke-width', '3');
        svgElement.appendChild(line);
      }
    });

    // Draw nodes
    nodes.forEach(node => {
      const pos = nodePositions[node.id];
      if (!pos) return;

      const group = document.createElementNS('http://www.w3.org/2000/svg', 'g');
      
      // Node circle
      const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
      circle.setAttribute('cx', pos.x.toString());
      circle.setAttribute('cy', pos.y.toString());
      circle.setAttribute('r', '30');
      circle.setAttribute('fill', node.color);
      circle.setAttribute('stroke', '#374151');
      circle.setAttribute('stroke-width', '3');
      group.appendChild(circle);

      // Node label
      const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
      text.setAttribute('x', pos.x.toString());
      text.setAttribute('y', (pos.y + 5).toString());
      text.setAttribute('text-anchor', 'middle');
      text.setAttribute('font-size', '14');
      text.setAttribute('font-family', 'monospace');
      text.setAttribute('font-weight', 'bold');
      text.setAttribute('fill', '#1f2937');
      text.textContent = node.label.length > 10 ? node.label.substring(0, 10) + '...' : node.label;
      group.appendChild(text);

      // Tooltip
      const title = document.createElementNS('http://www.w3.org/2000/svg', 'title');
      title.textContent = `${node.node_type}: ${node.label}`;
      group.appendChild(title);

      svgElement.appendChild(group);
    });
  };

  // Render main SVG
  useEffect(() => {
    if (!svgRef.current || nodes.length === 0) return;
    const svg = svgRef.current;
    const width = svg.clientWidth || Math.max(600, nodes.length * 120);
    const height = dynamicHeight;
    renderSVG(svg, width, height);
  }, [nodes, edges, dynamicHeight]);

  // Render modal SVG
  useEffect(() => {
    if (!modalSvgRef.current || nodes.length === 0 || !showModal) return;
    const svg = modalSvgRef.current;
    const width = Math.max(800, nodes.length * 150);
    const height = Math.max(600, dynamicHeight * 1.2);
    renderSVG(svg, width, height);
  }, [nodes, edges, dynamicHeight, showModal]);

  if (loading) {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">AST Visualization</h3>
        <div className="flex items-center justify-center py-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">AST Visualization</h3>
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <strong>Error:</strong> {error}
        </div>
      </div>
    );
  }

  return (
    <>
      <div className="bg-white p-4 rounded-lg shadow">
        <div className="flex justify-between items-center mb-3">
          <h3 className="text-lg font-semibold">AST Visualization</h3>
          {nodes.length > 0 && (
            <button
              onClick={() => setShowModal(true)}
              className="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 transition-colors"
            >
              🔍 Ver en Pantalla Completa
            </button>
          )}
        </div>
        <div className="text-sm text-gray-600 mb-2">
          💡 {nodes.length > 3 ? 'Usa scroll horizontal y vertical para navegar por el árbol' : 'Jerarquía del árbol sintáctico'}
        </div>
        <div className="border border-gray-300 rounded-lg overflow-auto bg-gray-50 max-h-80">
          <svg
            ref={svgRef}
            width={Math.max(600, nodes.length * 120)}
            height={dynamicHeight}
            className="bg-gray-50"
          />
        </div>
      </div>

      {/* Modal for full-screen view */}
      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-6xl h-full max-h-[90vh] flex flex-col">
            {/* Modal Header */}
            <div className="flex justify-between items-center p-4 border-b">
              <h3 className="text-xl font-semibold">AST Visualization - Pantalla Completa</h3>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setZoom(Math.max(0.5, zoom - 0.1))}
                  className="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300"
                  disabled={zoom <= 0.5}
                >
                  🔍-
                </button>
                <span className="text-sm font-mono min-w-[60px] text-center">
                  {Math.round(zoom * 100)}%
                </span>
                <button
                  onClick={() => setZoom(Math.min(3, zoom + 0.1))}
                  className="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300"
                  disabled={zoom >= 3}
                >
                  🔍+
                </button>
                <button
                  onClick={resetView}
                  className="px-2 py-1 bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  🔄 Reset
                </button>
                <button
                  onClick={() => setShowModal(false)}
                  className="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700"
                >
                  ✕ Cerrar
                </button>
              </div>
            </div>

            {/* Modal Content */}
            <div className="flex-1 overflow-hidden relative">
              <div
                className="w-full h-full overflow-auto cursor-grab active:cursor-grabbing"
                onWheel={handleWheel}
                onMouseDown={handleMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                onMouseLeave={handleMouseUp}
              >
                <div
                  style={{
                    transform: `translate(${pan.x}px, ${pan.y}px) scale(${zoom})`,
                    transformOrigin: '0 0',
                    minWidth: '100%',
                    minHeight: '100%'
                  }}
                >
                  <svg
                    ref={modalSvgRef}
                    width={Math.max(800, nodes.length * 150)}
                    height={Math.max(600, dynamicHeight * 1.2)}
                    className="block"
                    style={{ 
                      background: 'linear-gradient(45deg, #f8fafc 25%, transparent 25%), linear-gradient(-45deg, #f8fafc 25%, transparent 25%), linear-gradient(45deg, transparent 75%, #f8fafc 75%), linear-gradient(-45deg, transparent 75%, #f8fafc 75%)',
                      backgroundSize: '20px 20px',
                      backgroundPosition: '0 0, 0 10px, 10px -10px, -10px 0px'
                    }}
                  />
                </div>
              </div>
            </div>

            {/* Modal Footer */}
            <div className="p-4 border-t bg-gray-50">
              <div className="text-sm text-gray-600">
                💡 Usa la rueda del mouse para hacer zoom, arrastra para mover, o usa los controles arriba
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

// Simple hierarchical tree layout algorithm
function calculateNodePositions(nodes: NodeData[], edges: EdgeData[], width: number, height: number) {
  const positions: { [key: string]: { x: number; y: number } } = {};
  
  if (nodes.length === 0) return positions;

  // Find root node (no incoming edges)
  const nodeIds = new Set(nodes.map(n => n.id));
  const hasIncoming = new Set(edges.map(e => e.to));
  const rootId = Array.from(nodeIds).find(id => !hasIncoming.has(id)) || nodes[0].id;

  // Build tree structure
  const children: { [key: string]: string[] } = {};
  edges.forEach(edge => {
    if (!children[edge.from]) children[edge.from] = [];
    children[edge.from].push(edge.to);
  });

  // Calculate levels using BFS
  const levels: { [key: string]: number } = {};
  const queue: { id: string; level: number }[] = [{ id: rootId, level: 0 }];
  const visited = new Set<string>();
  let maxLevel = 0;

  while (queue.length > 0) {
    const { id, level } = queue.shift()!;
    if (visited.has(id)) continue;
    visited.add(id);

    levels[id] = level;
    maxLevel = Math.max(maxLevel, level);

    const nodeChildren = children[id] || [];
    nodeChildren.forEach(childId => {
      queue.push({ id: childId, level: level + 1 });
    });
  }

  // Group nodes by level
  const levelGroups: { [key: number]: string[] } = {};
  Object.entries(levels).forEach(([nodeId, level]) => {
    if (!levelGroups[level]) levelGroups[level] = [];
    levelGroups[level].push(nodeId);
  });

  // Position nodes with compact spacing
  const levelHeight = Math.max(120, height / (maxLevel + 2));
  
  for (let level = 0; level <= maxLevel; level++) {
    const levelNodes = levelGroups[level] || [];
    const minSpacing = 120; // Minimum spacing between nodes
    const levelWidth = Math.max(width, levelNodes.length * minSpacing);
    const nodeSpacing = levelWidth / (levelNodes.length + 1);
    
    levelNodes.forEach((nodeId, index) => {
      positions[nodeId] = {
        x: (index + 1) * nodeSpacing,
        y: 60 + level * levelHeight
      };
    });
  }

  // Center children under their parents
  for (let level = 0; level < maxLevel; level++) {
    const levelNodes = levelGroups[level] || [];
    
    levelNodes.forEach(parentId => {
      const parentChildren = children[parentId] || [];
      if (parentChildren.length === 0) return;
      
      const parentPos = positions[parentId];
      if (!parentPos) return;
      
      // Calculate center position for children
      const childPositions = parentChildren.map(childId => positions[childId]).filter(pos => pos);
      if (childPositions.length === 0) return;
      
      const childrenCenterX = childPositions.reduce((sum, pos) => sum + pos.x, 0) / childPositions.length;
      const offset = parentPos.x - childrenCenterX;
      
             // Adjust children positions to center them under parent
       parentChildren.forEach((childId, index) => {
         if (positions[childId]) {
           positions[childId].x += offset;
           // Add slight horizontal spacing between siblings
           if (parentChildren.length > 1) {
             const siblingOffset = (index - (parentChildren.length - 1) / 2) * 10;
             positions[childId].x += siblingOffset;
           }
         }
       });
    });
  }

  return positions;
}

export default ASTVisualization; 