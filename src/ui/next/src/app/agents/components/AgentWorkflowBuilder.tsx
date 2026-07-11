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
    <div className="border border-white/50 bg-white/60 backdrop-blur-[40px] saturate-[200%] p-8 shadow-[0_8px_32px_0_rgba(31,38,135,0.07)] rounded-3xl dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)]" data-testid="visual-workflow-builder">
      <h3 className="mb-6 text-xl font-bold text-zinc-900 dark:text-zinc-100">Visual Workflow Builder (Visual/low-code orchestration --&gt; democratizing agent construction)</h3>

      {error && (
        <div className="mb-6 rounded-xl border border-red-200 bg-red-50/80 backdrop-blur-[30px] saturate-[210%] p-4 text-sm text-red-700 shadow-sm" data-testid="builder-error">
          <h4 className="font-bold flex items-center mb-1">
            <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            Error
          </h4>
          <p>{error}</p>
        </div>
      )}

      <div className="mb-8">
        <label className="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-2">Workflow Name</label>
        <input
          type="text"
          value={workflowName}
          onChange={(e) => setWorkflowName(e.target.value)}
          placeholder="e.g., Auto-reply to VIPs"
          className="w-full rounded-xl border border-white/40 p-4 text-sm text-black dark:text-white dark:bg-zinc-800 dark:border-zinc-700 focus:border-[#0066FF] focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 bg-white/50 backdrop-blur-[20px] saturate-[150%] shadow-inner transition-all"
          id="visual-workflow-name"
        />
      </div>

      <div className="flex gap-8">
        {/* Palette */}
        <div className="w-1/3">
          <h4 className="mb-4 text-sm font-bold text-zinc-800 dark:text-zinc-200">Block Palette</h4>
          <div className="flex flex-col gap-3">
            {AVAILABLE_BLOCKS.map(block => (
              <button
                key={block.id}
                onClick={() => addBlock(block)}
                className="flex items-center justify-between rounded-xl border border-white/50 bg-white/60 backdrop-blur-[30px] saturate-[200%] shadow-sm dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)] p-3 text-left hover:bg-white/80 dark:hover:bg-zinc-700 transition-all hover:-translate-y-[1px]"
                data-testid={`palette-block-${block.id}`}
              >
                <span className="text-sm font-medium text-zinc-800 dark:text-zinc-200">{block.label}</span>
                <span className={`text-[10px] font-bold uppercase px-2 py-1 rounded-full shadow-sm
                  ${block.type === 'Trigger' ? 'bg-blue-100/80 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400 border border-blue-200/50' :
                    block.type === 'Action' ? 'bg-amber-100/80 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400 border border-amber-200/50' :
                    block.type === 'Condition' ? 'bg-purple-100/80 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400 border border-purple-200/50' :
                    'bg-green-100/80 text-green-700 dark:bg-green-900/30 dark:text-green-400 border border-green-200/50'}`}
                >
                  {block.type}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Canvas */}
        <div className="flex-1 border-2 border-dashed border-zinc-300 dark:border-zinc-700 rounded-3xl bg-zinc-50/30 dark:bg-zinc-900/50 p-6 min-h-[400px] shadow-inner relative overflow-hidden">
          <div className="absolute inset-0 bg-grid-zinc-200/50 dark:bg-grid-zinc-800/50 [mask-image:linear-gradient(0deg,white,rgba(255,255,255,0.5))] pointer-events-none"></div>

          <h4 className="mb-6 text-sm font-bold text-zinc-800 dark:text-zinc-200 relative z-10">Workflow Canvas</h4>

          {workflowBlocks.length === 0 ? (
            <div className="flex h-[300px] items-center justify-center text-sm text-zinc-400 relative z-10">
              <div className="flex flex-col items-center">
                <svg className="w-12 h-12 mb-4 text-zinc-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M12 6v6m0 0v6m0-6h6m-6 0H6" /></svg>
                <p>Click blocks on the left to add them to your workflow</p>
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center gap-3 relative z-10">
              {workflowBlocks.map((block, index) => (
                <React.Fragment key={block.id}>
                  <div className="w-full max-w-sm flex items-center justify-between border border-white/50 bg-white/70 backdrop-blur-[40px] saturate-[200%] dark:bg-[rgba(22,22,26,0.8)] dark:border-[rgba(255,255,255,0.1)] rounded-2xl p-4 shadow-[0_8px_16px_0_rgba(31,38,135,0.05)] transition-all" data-testid={`canvas-block-${index}`}>
                    <div>
                      <span className="text-xs font-bold text-[#0066FF] dark:text-blue-400 uppercase tracking-widest">{block.type}</span>
                      <p className="font-semibold text-zinc-900 dark:text-zinc-100 mt-1 text-lg">{block.label}</p>
                    </div>
                    <button
                      onClick={() => removeBlock(index)}
                      className="text-zinc-400 hover:text-[#FF3B30] hover:bg-red-50 rounded-full transition-all p-2"
                      aria-label="Remove block"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    </button>
                  </div>
                  {index < workflowBlocks.length - 1 && (
                    <div className="h-8 w-0.5 bg-[#0066FF]/30 dark:bg-[#0066FF]/50 relative">
                      <div className="absolute bottom-0 left-1/2 -translate-x-1/2 translate-y-1/2 w-2 h-2 border-r-2 border-b-2 border-[#0066FF] rotate-45"></div>
                    </div>
                  )}
                </React.Fragment>
              ))}
            </div>
          )}
        </div>
      </div>

      <div className="mt-8 flex justify-end">
        <button
          onClick={handleSave}
          disabled={workflowBlocks.length === 0 || !workflowName || isSubmitting}
          className="rounded-xl bg-[#0066FF] hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-500 px-8 py-3 text-base font-bold text-white shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] disabled:opacity-50 disabled:shadow-none transition-all hover:-translate-y-[1px]"
          id="btn-create-run-workflow"
        >
          {isSubmitting ? (
            <span className="flex items-center">
              <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle><path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
              Compiling & Running...
            </span>
          ) : 'Create & Run Workflow'}
        </button>
      </div>
    </div>
  );
}
