import React, { useEffect, useRef } from 'react';
import { NodeData, EdgeData } from '../types';

interface ASTVisualizationProps {
  nodes: NodeData[];
  edges: EdgeData[];
  loading: boolean;
  error: string | null;
}

const ASTVisualization: React.FC<ASTVisualizationProps> = ({ nodes, edges, loading, error }) => {
  const svgRef = useRef<SVGSVGElement>(null);

  // Calculate dynamic height based on tree depth
  const calculateDynamicHeight = () => {
    if (nodes.length === 0) return 400;
    const tempPositions = calculateNodePositions(nodes, edges, Math.max(600, nodes.length * 120), 600);
    const maxY = Object.values(tempPositions).reduce((max, pos) => Math.max(max, pos.y), 0);
    return Math.max(400, maxY + 100);
  };

  const dynamicHeight = calculateDynamicHeight();

  useEffect(() => {
    if (!svgRef.current || nodes.length === 0) return;

    const svg = svgRef.current;
    const width = svg.clientWidth;
    const height = svg.clientHeight;

    // Clear previous content
    svg.innerHTML = '';

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
        svg.appendChild(line);
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

      svg.appendChild(group);
    });
  }, [nodes, edges]);

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
    <div className="bg-white p-4 rounded-lg shadow">
      <h3 className="text-lg font-semibold mb-3">AST Visualization</h3>
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