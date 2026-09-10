import React, { useState, useCallback, useRef } from 'react';
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
  Node
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

export type BlockType = 'Trigger' | 'Action' | 'Condition' | 'Output';

export interface BlockDefinition {
  id: string;
  type: BlockType;
  label: string;
}

export const AVAILABLE_BLOCKS: BlockDefinition[] = [
  { id: 'trigger_message', type: 'Trigger', label: 'Inbound Message' },
  { id: 'trigger_schedule', type: 'Trigger', label: 'Schedule (Daily)' },
  { id: 'action_research', type: 'Action', label: 'Web Research' },
  { id: 'action_analyze', type: 'Action', label: 'Analyze Sentiment' },
  { id: 'action_draft', type: 'Action', label: 'Draft Reply' },
  { id: 'condition_approval', type: 'Condition', label: 'Wait for Approval' },
  { id: 'output_send', type: 'Output', label: 'Send Message' },
  { id: 'output_save', type: 'Output', label: 'Save to Memory' },
];

export interface NodeMap {
  [id: string]: {
    id: string;
    type: string;
    label: string;
    next: string[];
  }
}

export function AgentWorkflowBuilder({ onSave }: { onSave: (name: string, payload: string) => Promise<void> }) {
  const [workflowName, setWorkflowName] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState('');

  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onConnect = useCallback(
    (params: Connection | Edge) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const reactFlowWrapper = useRef<HTMLDivElement>(null);

  const addBlock = (block: BlockDefinition) => {
    const newNodeId = `${block.id}_${Date.now()}`;
    const newNode: Node = {
      id: newNodeId,
      position: { x: Math.random() * 200 + 50, y: Math.random() * 200 + 50 },
      data: { label: block.label, type: block.type, originalId: block.id },
      style: {
        background: 'rgba(255,255,255,0.65)',
        border: '1px solid rgba(255,255,255,0.4)',
        borderRadius: '12px',
        padding: '12px',
        backdropFilter: 'blur(30px)',
        WebkitBackdropFilter: 'blur(30px)',
        color: '#000',
        fontWeight: 'bold'
      }
    };
    setNodes((nds) => nds.concat(newNode));
  };

  const handleSave = async () => {
    if (!workflowName || nodes.length === 0) return;
    setIsSubmitting(true);
    setError('');

    const nodeMap: NodeMap = {};
    for (const node of nodes) {
      const nextEdges = edges.filter(e => e.source === node.id);
      nodeMap[node.id] = {
        id: node.id,
        type: (node.data.type as string) || 'Action',
        label: (node.data.label as string) || '',
        next: nextEdges.map(e => e.target)
      };
    }

    // Attempt to find a trigger node as entrypoint
    const triggerNode = nodes.find(n => n.data.type === 'Trigger');
    const entrypoint = triggerNode ? triggerNode.id : nodes[0].id;

    const payloadString = JSON.stringify({
      version: '1.0',
      entrypoint: entrypoint,
      nodes: nodeMap
    });

    try {
      await onSave(workflowName, payloadString);
      setWorkflowName('');
      setNodes([]);
      setEdges([]);
    } catch (err: any) {
      setError(err.message || 'Failed to save workflow.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="border border-[rgba(255,255,255,0.4)] bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] p-4 shadow-sm dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)]" data-testid="visual-workflow-builder">
      <h3 className="mb-4 text-lg font-bold text-zinc-900 dark:text-zinc-100">Visual Workflow Builder</h3>

      {error && (
        <div className="mb-4 rounded border border-red-200 bg-red-50 p-2 text-sm text-red-600" data-testid="builder-error">
          {error}
        </div>
      )}

      <div className="mb-4">
        <label className="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1">Workflow Name</label>
        <input
          type="text"
          value={workflowName}
          onChange={(e) => setWorkflowName(e.target.value)}
          placeholder="e.g., Auto-reply to VIPs"
          className="w-full rounded-[8px] border border-zinc-300 p-2 text-sm text-black dark:text-white dark:bg-zinc-800 dark:border-zinc-700 focus:border-teal-500 focus:outline-none focus:ring-1 focus:ring-teal-500"
          id="visual-workflow-name"
        />
      </div>

      <div className="flex flex-col md:flex-row gap-6">
        {/* Palette */}
        <div className="w-full md:w-1/3">
          <h4 className="mb-2 text-sm font-bold text-zinc-800 dark:text-zinc-200">Block Palette</h4>
          <div className="flex flex-col gap-2">
            {AVAILABLE_BLOCKS.map(block => (
              <button
                key={block.id}
                onClick={() => addBlock(block)}
                className="flex items-center justify-between rounded-[8px] border border-zinc-200 bg-zinc-50 dark:bg-zinc-800 dark:border-zinc-700 p-2 text-left hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-colors"
                data-testid={`palette-block-${block.id}`}
              >
                <span className="text-sm font-medium text-zinc-800 dark:text-zinc-200">{block.label}</span>
                <span className={`text-[10px] font-bold uppercase px-2 py-1 rounded-full
                  ${block.type === 'Trigger' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400' :
                    block.type === 'Action' ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400' :
                    block.type === 'Condition' ? 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400' :
                    'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'}`}
                >
                  {block.type}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* ReactFlow Canvas */}
        <div className="flex-1 border-2 border-dashed border-zinc-300 dark:border-zinc-700 bg-zinc-50/50 dark:bg-zinc-900/50 min-h-[400px] h-[50vh] relative" ref={reactFlowWrapper}>
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            fitView
          >
            <Controls />
            <Background color="#ccc" gap={16} />
          </ReactFlow>

          {/* E2E Test Hooks mapping UI */}
          <div className="hidden">
            {nodes.map((node, index) => (
               <div key={node.id} data-testid={`canvas-block-${index}`}></div>
            ))}
          </div>
        </div>
      </div>

      <div className="mt-6 flex justify-end">
        <button
          onClick={handleSave}
          disabled={nodes.length === 0 || !workflowName || isSubmitting}
          className="rounded-[8px] bg-teal-600 hover:bg-teal-700 dark:bg-teal-700 dark:hover:bg-teal-600 px-6 py-2 text-sm font-bold text-white shadow-sm disabled:opacity-50 transition-colors"
          id="btn-create-run-workflow"
        >
          {isSubmitting ? 'Compiling & Running...' : 'Create & Run Workflow'}
        </button>
      </div>
    </div>
  );
}
