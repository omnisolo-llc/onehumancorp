import React, { useState } from 'react';

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
    type: BlockType;
    label: string;
    next: string[];
  }
}

export function AgentWorkflowBuilder({ onSave }: { onSave: (name: string, payload: string) => Promise<void> }) {
  const [workflowName, setWorkflowName] = useState('');
  const [workflowBlocks, setWorkflowBlocks] = useState<BlockDefinition[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState('');

  const addBlock = (block: BlockDefinition) => {
    setWorkflowBlocks([...workflowBlocks, { ...block, id: `${block.id}_${Date.now()}` }]);
  };

  const removeBlock = (index: number) => {
    const newBlocks = [...workflowBlocks];
    newBlocks.splice(index, 1);
    setWorkflowBlocks(newBlocks);
  };

  const handleSave = async () => {
    if (!workflowName || workflowBlocks.length === 0) return;
    setIsSubmitting(true);
    setError('');

    // Compile visual blocks into a DAG/JSON structure
    const nodeMap: NodeMap = {};
    for (let i = 0; i < workflowBlocks.length; i++) {
      const b = workflowBlocks[i];
      nodeMap[b.id] = {
        id: b.id,
        type: b.type,
        label: b.label,
        next: i < workflowBlocks.length - 1 ? [workflowBlocks[i+1].id] : []
      };
    }

    const payloadString = JSON.stringify({
      version: '1.0',
      entrypoint: workflowBlocks[0].id,
      nodes: nodeMap
    });

    try {
      await onSave(workflowName, payloadString);
      setWorkflowName('');
      setWorkflowBlocks([]);
    } catch (err: any) {
      setError(err.message || 'Failed to save workflow.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="rounded-lg border border-zinc-200 bg-white p-4 shadow-sm" data-testid="visual-workflow-builder">
      <h3 className="mb-4 text-lg font-bold text-zinc-900">Visual Workflow Builder</h3>

      {error && (
        <div className="mb-4 rounded border border-red-200 bg-red-50 p-2 text-sm text-red-600" data-testid="builder-error">
          {error}
        </div>
      )}

      <div className="mb-4">
        <label className="block text-sm font-medium text-zinc-700 mb-1">Workflow Name</label>
        <input
          type="text"
          value={workflowName}
          onChange={(e) => setWorkflowName(e.target.value)}
          placeholder="e.g., Auto-reply to VIPs"
          className="w-full rounded-md border border-zinc-300 p-2 text-sm text-black focus:border-teal-500 focus:outline-none focus:ring-1 focus:ring-teal-500"
          id="visual-workflow-name"
        />
      </div>

      <div className="flex gap-6">
        {/* Palette */}
        <div className="w-1/3">
          <h4 className="mb-2 text-sm font-bold text-zinc-800">Block Palette</h4>
          <div className="flex flex-col gap-2">
            {AVAILABLE_BLOCKS.map(block => (
              <button
                key={block.id}
                onClick={() => addBlock(block)}
                className="flex items-center justify-between rounded-md border border-zinc-200 bg-zinc-50 p-2 text-left hover:bg-zinc-100 transition-colors"
                data-testid={`palette-block-${block.id}`}
              >
                <span className="text-sm font-medium text-zinc-800">{block.label}</span>
                <span className={`text-[10px] font-bold uppercase px-2 py-1 rounded-full
                  ${block.type === 'Trigger' ? 'bg-blue-100 text-blue-700' :
                    block.type === 'Action' ? 'bg-amber-100 text-amber-700' :
                    block.type === 'Condition' ? 'bg-purple-100 text-purple-700' :
                    'bg-green-100 text-green-700'}`}
                >
                  {block.type}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Canvas */}
        <div className="flex-1 rounded-md border-2 border-dashed border-zinc-300 bg-zinc-50/50 p-4 min-h-[300px]">
          <h4 className="mb-4 text-sm font-bold text-zinc-800">Workflow Canvas</h4>

          {workflowBlocks.length === 0 ? (
            <div className="flex h-full items-center justify-center text-sm text-zinc-400">
              Click blocks on the left to add them to your workflow
            </div>
          ) : (
            <div className="flex flex-col items-center gap-2 relative">
              {workflowBlocks.map((block, index) => (
                <React.Fragment key={block.id}>
                  <div className="w-full max-w-sm flex items-center justify-between rounded-md border-2 border-teal-500 bg-white p-3 shadow-sm" data-testid={`canvas-block-${index}`}>
                    <div>
                      <span className="text-xs font-bold text-teal-600 uppercase tracking-wider">{block.type}</span>
                      <p className="font-semibold text-zinc-900">{block.label}</p>
                    </div>
                    <button
                      onClick={() => removeBlock(index)}
                      className="text-zinc-400 hover:text-red-500 transition-colors p-1"
                      aria-label="Remove block"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    </button>
                  </div>
                  {index < workflowBlocks.length - 1 && (
                    <div className="h-6 w-0.5 bg-teal-300 relative">
                      <div className="absolute bottom-0 left-1/2 -translate-x-1/2 translate-y-1/2 w-2 h-2 border-r-2 border-b-2 border-teal-500 rotate-45"></div>
                    </div>
                  )}
                </React.Fragment>
              ))}
            </div>
          )}
        </div>
      </div>

      <div className="mt-6 flex justify-end">
        <button
          onClick={handleSave}
          disabled={workflowBlocks.length === 0 || !workflowName || isSubmitting}
          className="rounded-md bg-teal-700 px-6 py-2 text-sm font-bold text-white shadow-sm hover:bg-teal-800 disabled:opacity-50 transition-colors"
          id="btn-create-run-workflow"
        >
          {isSubmitting ? 'Compiling & Running...' : 'Create & Run Workflow'}
        </button>
      </div>
    </div>
  );
}
