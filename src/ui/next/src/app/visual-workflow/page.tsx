"use client";

import { useState, useCallback, useRef } from "react";
import {
  ReactFlow,
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  Edge,
  Node,
  ReactFlowProvider,
  Handle,
  Position,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

// --- Custom Nodes ---

const nodeStyle = {
  background: '#fff',
  border: '1px solid #e5e7eb',
  borderRadius: '12px',
  padding: '16px',
  minWidth: '200px',
  boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
};

const headerStyle = {
  display: 'flex',
  alignItems: 'center',
  gap: '8px',
  marginBottom: '12px',
  fontWeight: 'bold',
  color: '#1f2937',
  fontSize: '14px',
};

const inputStyle = {
  width: '100%',
  padding: '8px',
  fontSize: '12px',
  border: '1px solid #d1d5db',
  borderRadius: '6px',
  marginTop: '4px',
  boxSizing: 'border-box' as const,
};

const labelStyle = {
  fontSize: '12px',
  color: '#6b7280',
  fontWeight: 600,
  display: 'block',
  marginTop: '8px',
};

function InputNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#34d399' }}>
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#d1fae5', color: '#059669', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>⚡</div>
        Input Trigger
      </div>
      <div>
        <label style={labelStyle}>Variable Name</label>
        <input style={inputStyle} defaultValue={data.name || 'input_var'} className="nodrag" />
      </div>
      <Handle type="source" position={Position.Right} id="a" />
    </div>
  );
}

function LlmNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#60a5fa' }}>
      <Handle type="target" position={Position.Left} />
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#dbeafe', color: '#2563eb', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>🧠</div>
        LLM Prompt
      </div>
      <div>
        <label style={labelStyle}>Prompt Template</label>
        <textarea style={{ ...inputStyle, minHeight: '60px' }} defaultValue={data.prompt_template || 'Analyze this: {{in}}'} className="nodrag" />
      </div>
      <Handle type="source" position={Position.Right} id="a" />
    </div>
  );
}

function ToolNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#c084fc' }}>
      <Handle type="target" position={Position.Left} />
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#f3e8ff', color: '#9333ea', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>🔧</div>
        Tool Action
      </div>
      <div>
        <label style={labelStyle}>Tool Name</label>
        <input style={inputStyle} defaultValue={data.tool_name || 'web_search'} className="nodrag" />
        <label style={labelStyle}>Arguments Template (JSON)</label>
        <textarea style={{ ...inputStyle, minHeight: '60px', fontFamily: 'monospace' }} defaultValue={data.args_template || '{"query": "{{llm1}}"}'} className="nodrag" />
      </div>
      <Handle type="source" position={Position.Right} id="a" />
    </div>
  );
}

function ConditionNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#fbbf24' }}>
      <Handle type="target" position={Position.Left} />
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#fef3c7', color: '#d97706', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>🔀</div>
        Condition
      </div>
      <div>
        <label style={labelStyle}>Expression</label>
        <input style={inputStyle} defaultValue={data.condition_expression || '{{in}} == "success"'} className="nodrag" />
      </div>
      <Handle type="source" position={Position.Right} id="true" style={{ top: '30%', background: '#10b981' }} />
      <Handle type="source" position={Position.Right} id="false" style={{ top: '70%', background: '#ef4444' }} />
      <div style={{ fontSize: '10px', color: '#6b7280', position: 'absolute', right: '-35px', top: '25%' }}>True</div>
      <div style={{ fontSize: '10px', color: '#6b7280', position: 'absolute', right: '-35px', top: '65%' }}>False</div>
    </div>
  );
}

function LoopNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#34d399' }}>
      <Handle type="target" position={Position.Left} />
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#d1fae5', color: '#059669', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>🔁</div>
        Loop
      </div>
      <div>
        <label style={labelStyle}>Max Iterations</label>
        <input style={inputStyle} type="number" defaultValue={data.max_iterations || 5} className="nodrag" />
      </div>
      <Handle type="source" position={Position.Right} id="body" style={{ top: '30%', background: '#60a5fa' }} />
      <Handle type="source" position={Position.Right} id="next" style={{ top: '70%', background: '#9ca3af' }} />
      <div style={{ fontSize: '10px', color: '#6b7280', position: 'absolute', right: '-40px', top: '25%' }}>Body</div>
      <div style={{ fontSize: '10px', color: '#6b7280', position: 'absolute', right: '-40px', top: '65%' }}>Next</div>
    </div>
  );
}

function ApprovalNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#f43f5e' }}>
      <Handle type="target" position={Position.Left} />
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#ffe4e6', color: '#e11d48', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>👤</div>
        Human Approval
      </div>
      <div>
        <label style={labelStyle}>Approval Message</label>
        <input style={inputStyle} defaultValue={data.message || 'Please review.'} className="nodrag" />
      </div>
      <Handle type="source" position={Position.Right} id="a" />
    </div>
  );
}

function OutputNode({ data }: { data: any }) {
  return (
    <div style={{ ...nodeStyle, borderColor: '#f87171' }}>
      <Handle type="target" position={Position.Left} />
      <div style={headerStyle}>
        <div style={{ width: '24px', height: '24px', borderRadius: '50%', background: '#fee2e2', color: '#dc2626', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>🏁</div>
        Output
      </div>
      <div style={{ fontSize: '12px', color: '#6b7280', marginTop: '8px' }}>Returns data to caller</div>
    </div>
  );
}

const nodeTypes = {
  Input: InputNode,
  Llm: LlmNode,
  Tool: ToolNode,
  Condition: ConditionNode,
  Loop: LoopNode,
  Approval: ApprovalNode,
  Output: OutputNode,
};

const initialNodes: Node[] = [
  { id: '1', type: 'Input', position: { x: 50, y: 200 }, data: { name: 'user_request' } },
  { id: '2', type: 'Llm', position: { x: 350, y: 200 }, data: { prompt_template: 'Process this request: {{1}}' } },
  { id: '3', type: 'Approval', position: { x: 650, y: 200 }, data: { message: 'Review generation' } },
  { id: '4', type: 'Output', position: { x: 950, y: 200 }, data: {} },
];

const initialEdges: Edge[] = [
  { id: 'e1-2', source: '1', target: '2', animated: true },
  { id: 'e2-3', source: '2', target: '3', animated: true },
  { id: 'e3-4', source: '3', target: '4', animated: true },
];

export default function VisualWorkflowBuilder() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const [reactFlowInstance, setReactFlowInstance] = useState<any>(null);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  const onDragOver = useCallback((event: any) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'move';
  }, []);

  const onDrop = useCallback(
    (event: any) => {
      event.preventDefault();

      const type = event.dataTransfer.getData('application/reactflow');

      if (typeof type === 'undefined' || !type || !reactFlowInstance) {
        return;
      }

      const position = reactFlowInstance.screenToFlowPosition({
        x: event.clientX,
        y: event.clientY,
      });

      const newNode = {
        id: `dndnode_${Date.now()}`,
        type,
        position,
        data: { label: `${type} node` },
      };

      setNodes((nds) => nds.concat(newNode));
    },
    [reactFlowInstance, setNodes],
  );

  const onDragStart = (event: any, nodeType: string) => {
    event.dataTransfer.setData('application/reactflow', nodeType);
    event.dataTransfer.effectAllowed = 'move';
  };

  const handleExport = async () => {
    const graphData = {
        nodes: nodes.map(n => ({
            id: n.id,
            node_type: n.type,
            data: n.data
        })),
        edges: edges.map(e => ({
            source: e.source,
            target: e.target,
            sourceHandle: e.sourceHandle
        }))
    };

    try {
        const response = await fetch('/api/agents/workflows', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: 'Custom Visual Workflow',
                task: JSON.stringify(graphData) // Store the graph as the task for now
            }),
        });

        if (response.ok) {
            alert('Workflow successfully submitted!');
        } else {
            alert('Failed to submit workflow');
        }
    } catch (e) {
        alert('Error submitting workflow');
    }
  };

  return (
    <div className="flex h-screen bg-gray-50 flex-col">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between shadow-sm z-10 relative">
        <div className="flex items-center gap-3">
          <h1 className="text-xl font-bold text-gray-900 font-outfit">Visual Workflow Builder</h1>
          <span className="px-2 py-1 bg-blue-100 text-blue-700 text-xs font-semibold rounded-full uppercase tracking-wider">Beta</span>
        </div>
        <div className="flex items-center gap-3">
            <button className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                Cancel
            </button>
            <button
                onClick={handleExport}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors shadow-sm">
                Deploy Workflow
            </button>
        </div>
      </header>

      <div className="flex-1 flex overflow-hidden">
        {/* Sidebar */}
        <aside className="w-72 bg-white border-r border-gray-200 p-4 flex flex-col gap-3 overflow-y-auto z-10 shadow-[4px_0_10px_-4px_rgba(0,0,0,0.05)]">
            <div className="mb-2">
                <h2 className="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Nodes</h2>
                <p className="text-xs text-gray-500">Drag and drop nodes onto the canvas.</p>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-emerald-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Input')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-emerald-50 flex items-center justify-center text-emerald-600 text-xl shadow-sm group-hover:scale-110 transition-transform">⚡</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">Trigger</div>
                    <div className="text-xs text-gray-500">Start of workflow</div>
                </div>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-blue-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Llm')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-blue-50 flex items-center justify-center text-blue-600 text-xl shadow-sm group-hover:scale-110 transition-transform">🧠</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">LLM Prompt</div>
                    <div className="text-xs text-gray-500">Run LLM processing</div>
                </div>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-purple-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Tool')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-purple-50 flex items-center justify-center text-purple-600 text-xl shadow-sm group-hover:scale-110 transition-transform">🔧</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">Tool Action</div>
                    <div className="text-xs text-gray-500">Execute a tool</div>
                </div>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-amber-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Condition')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center text-amber-600 text-xl shadow-sm group-hover:scale-110 transition-transform">🔀</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">Condition</div>
                    <div className="text-xs text-gray-500">Branching logic</div>
                </div>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-emerald-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Loop')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-emerald-50 flex items-center justify-center text-emerald-600 text-xl shadow-sm group-hover:scale-110 transition-transform">🔁</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">Loop</div>
                    <div className="text-xs text-gray-500">Iteration logic</div>
                </div>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-rose-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Approval')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-rose-50 flex items-center justify-center text-rose-600 text-xl shadow-sm group-hover:scale-110 transition-transform">👤</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">Human Approval</div>
                    <div className="text-xs text-gray-500">Checkpoint pause</div>
                </div>
            </div>

            <div
                className="bg-white border border-gray-200 rounded-xl p-3 cursor-grab hover:border-red-400 hover:shadow-md transition-all shadow-sm flex items-center gap-3 group"
                onDragStart={(event) => onDragStart(event, 'Output')} draggable
            >
                <div className="w-10 h-10 rounded-full bg-red-50 flex items-center justify-center text-red-600 text-xl shadow-sm group-hover:scale-110 transition-transform">🏁</div>
                <div>
                    <div className="text-sm font-bold text-gray-800">Output</div>
                    <div className="text-xs text-gray-500">End of workflow</div>
                </div>
            </div>
        </aside>

        {/* Canvas area */}
        <main className="flex-1 bg-slate-50 relative overflow-hidden flex flex-col" ref={reactFlowWrapper}>
            <ReactFlowProvider>
                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    onConnect={onConnect}
                    onInit={setReactFlowInstance}
                    onDrop={onDrop}
                    onDragOver={onDragOver}
                    nodeTypes={nodeTypes}
                    fitView
                    className="bg-slate-50"
                >
                    <Background color="#cbd5e1" gap={20} size={1} />
                    <Controls />
                    <MiniMap zoomable pannable nodeColor={(n) => {
                        if (n.type === 'Input') return '#34d399';
                        if (n.type === 'Llm') return '#60a5fa';
                        if (n.type === 'Tool') return '#c084fc';
                        if (n.type === 'Condition') return '#fbbf24';
                        if (n.type === 'Loop') return '#34d399';
                        if (n.type === 'Approval') return '#f43f5e';
                        if (n.type === 'Output') return '#f87171';
                        return '#eee';
                    }} />
                </ReactFlow>
            </ReactFlowProvider>
        </main>
      </div>
    </div>
  );
}
