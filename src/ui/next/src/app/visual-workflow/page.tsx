'use client';

import React, { useState } from 'react';

export default function VisualWorkflowPage() {
  const [nodes, setNodes] = useState([
    { id: 'node-1', type: 'Input', x: 50, y: 50, label: 'Start', details: { name: 'input_var' } },
    { id: 'node-2', type: 'LLM', x: 250, y: 50, label: 'Process Text', details: { prompt_template: 'Please process: {{input_var}}' } },
    { id: 'node-3', type: 'Output', x: 450, y: 50, label: 'End', details: {} }
  ]);

  const [edges, setEdges] = useState([
    { id: 'edge-1', source: 'node-1', target: 'node-2' },
    { id: 'edge-2', source: 'node-2', target: 'node-3' }
  ]);

  const [executionResult, setExecutionResult] = useState<string | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);

  const handleAddNode = (type: string) => {
    const newNode = {
      id: `node-${Date.now()}`,
      type,
      x: Math.random() * 300 + 50,
      y: Math.random() * 300 + 50,
      label: `New ${type}`,
      details: type === 'LLM' ? { prompt_template: 'New Prompt' } : {}
    };
    setNodes([...nodes, newNode]);
  };

  const handleRunWorkflow = async () => {
    setIsExecuting(true);
    setExecutionResult(null);

    alert('Workflow execution simulation started! (AutoGPT Block-based Visual Workflow)');

    try {
      const graphPayload = {
        nodes: nodes.map(n => ({
          id: n.id,
          node_type: n.type === 'Input' ? { Input: n.details } :
                     n.type === 'LLM' ? { Llm: n.details } :
                     n.type === 'Tool' ? { Tool: { tool_name: 'echo', args_template: '{"val": "{{llm1}}"}' } } :
                     n.type === 'Condition' ? { Condition: { condition_expression: '{{in}} == trigger', true_target: 'out', false_target: 'out' } } :
                     { Output: {} }
        })),
        edges: edges.map(e => ({ source: e.source, target: e.target }))
      };

      const response = await fetch('/api/visual-workflow/run', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(graphPayload)
      });

      const data = await response.json();

      if (response.ok) {
        setExecutionResult(data.result || 'Workflow executed successfully.');
      } else {
        setExecutionResult(`Error: ${data.error}`);
      }
    } catch (err: any) {
      setExecutionResult(`Error: ${err.message}`);
    } finally {
      setIsExecuting(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 font-outfit">
      <div className="max-w-6xl mx-auto">
        <header className="mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">Block-Based Visual Workflow</h1>
          <p className="text-xl text-gray-600">
            Build and test agents using a drag-and-drop node interface. (AutoGPT Unique Harness Innovations)
          </p>
        </header>

        <div className="bg-white p-4 rounded-t-xl border border-gray-200 shadow-sm flex gap-4">
          <button
            onClick={() => handleAddNode('Input')}
            className="px-4 py-2 bg-blue-100 text-blue-700 rounded shadow-sm hover:bg-blue-200 transition-colors"
          >
            + Add Input
          </button>
          <button
            onClick={() => handleAddNode('LLM')}
            className="px-4 py-2 bg-purple-100 text-purple-700 rounded shadow-sm hover:bg-purple-200 transition-colors"
          >
            + Add LLM Node
          </button>
          <button
            onClick={() => handleAddNode('Tool')}
            className="px-4 py-2 bg-orange-100 text-orange-700 rounded shadow-sm hover:bg-orange-200 transition-colors"
          >
            + Add Tool Node
          </button>
          <button
            onClick={() => handleAddNode('Condition')}
            className="px-4 py-2 bg-yellow-100 text-yellow-700 rounded shadow-sm hover:bg-yellow-200 transition-colors"
          >
            + Add Condition
          </button>

          <div className="flex-grow" />

          <button
            onClick={handleRunWorkflow}
            disabled={isExecuting}
            className={`px-6 py-2 text-white rounded font-bold shadow-sm transition-colors ${isExecuting ? 'bg-gray-400 cursor-not-allowed' : 'bg-green-600 hover:bg-green-700'}`}
          >
            {isExecuting ? 'Running...' : '▶ Run Workflow'}
          </button>
        </div>

        <div className="bg-gray-100 h-[600px] border border-t-0 border-gray-200 rounded-b-xl relative overflow-hidden">
          {/* Edges representation (simplified using SVG) */}
          <svg className="absolute top-0 left-0 w-full h-full pointer-events-none">
            {edges.map(edge => {
              const sourceNode = nodes.find(n => n.id === edge.source);
              const targetNode = nodes.find(n => n.id === edge.target);
              if (!sourceNode || !targetNode) return null;

              return (
                <line
                  key={edge.id}
                  x1={sourceNode.x + 150}
                  y1={sourceNode.y + 40}
                  x2={targetNode.x}
                  y2={targetNode.y + 40}
                  stroke="#9CA3AF"
                  strokeWidth="3"
                  markerEnd="url(#arrowhead)"
                />
              );
            })}
            <defs>
              <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
                <polygon points="0 0, 10 3.5, 0 7" fill="#9CA3AF" />
              </marker>
            </defs>
          </svg>

          {/* Nodes rendering */}
          {nodes.map(node => (
            <div
              key={node.id}
              className="absolute bg-white border border-gray-300 shadow-md rounded-lg w-[150px] cursor-grab hover:shadow-lg transition-shadow"
              style={{ left: node.x, top: node.y }}
              onMouseDown={(e) => {
                const el = e.currentTarget;
                const startX = e.clientX - node.x;
                const startY = e.clientY - node.y;
                const handleMouseMove = (moveEvent: MouseEvent) => {
                  setNodes(nodes => nodes.map(n => n.id === node.id ? { ...n, x: moveEvent.clientX - startX, y: moveEvent.clientY - startY } : n));
                };
                const handleMouseUp = () => {
                  document.removeEventListener('mousemove', handleMouseMove);
                  document.removeEventListener('mouseup', handleMouseUp);
                };
                document.addEventListener('mousemove', handleMouseMove);
                document.addEventListener('mouseup', handleMouseUp);
              }}
            >
              <div className="bg-gray-800 text-white text-xs font-bold px-3 py-1 rounded-t-lg uppercase tracking-wider flex justify-between">
                <span>{node.type}</span>
              </div>
              <div className="p-3">
                <p className="text-gray-800 text-sm font-medium text-center truncate">{node.label}</p>
                {node.type === 'LLM' && (
                  <p className="text-xs text-gray-500 mt-1 truncate">{(node.details as any).prompt_template}</p>
                )}
              </div>
              {node.type !== 'Input' && (
                <div className="absolute top-1/2 -left-3 w-3 h-3 bg-gray-400 rounded-full transform -translate-y-1/2 border border-white"></div>
              )}
              {node.type !== 'Output' && (
                <div className="absolute top-1/2 -right-3 w-3 h-3 bg-blue-500 rounded-full transform -translate-y-1/2 border border-white"></div>
              )}
            </div>
          ))}
        </div>

        {executionResult && (
          <div className="mt-8 bg-blue-50 p-6 rounded-xl border border-blue-200 shadow-sm">
             <h3 className="text-lg font-bold text-blue-900 mb-2">Execution Result</h3>
             <pre className="text-blue-800 whitespace-pre-wrap font-mono text-sm overflow-x-auto">
               {executionResult}
             </pre>
          </div>
        )}

        <div className="mt-8 bg-white p-6 rounded-xl border border-gray-200 shadow-sm">
           <h3 className="text-lg font-bold text-gray-900 mb-2">Backend Integration Note</h3>
           <p className="text-gray-600 mb-2">
             This UI constructs a <code>WorkflowGraph</code> JSON object representing the nodes (Llm, Tool, Condition) and edges.
           </p>
           <p className="text-gray-600">
             When "Run Workflow" is clicked, this graph is posted to the <code>/api/visual-workflow/run</code> endpoint,
             which maps directly to the Rust backend <code>WorkflowExecutor</code> for code-native execution.
           </p>
        </div>
      </div>
    </div>
  );
}
