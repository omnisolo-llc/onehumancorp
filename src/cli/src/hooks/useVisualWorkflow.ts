import { useState } from 'react';

// Wire format matches the Rust internally tagged NodeType enum.
export type NodeType =
  | { type: 'Input'; name: string }
  | { type: 'Llm'; prompt_template: string }
  | { type: 'Output' }
  | { type: 'Tool'; tool_name: string; args_template: string }
  | { type: 'Condition'; condition_expression: string; true_target: string; false_target: string }
  | { type: 'SubAgent'; agent_name: string; task_template: string }
  | { type: 'HumanInLoop'; prompt_template: string }
  | { type: 'Merge' | 'ParallelJoin'; state_keys: string[]; output_key: string }
  | { type: 'ParallelFork'; targets: string[] };

export interface Node {
  id: string;
  node_type: NodeType;
}

export interface Edge {
  source: string;
  target: string;
}

export interface WorkflowGraph {
  nodes: Node[];
  edges: Edge[];
}

export const useVisualWorkflow = () => {
  const [status, setStatus] = useState<'idle' | 'running' | 'complete' | 'error'>('idle');
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const runWorkflow = async (graph: WorkflowGraph, inputs: Record<string, string>) => {
    setStatus('running');
    setError(null);
    setResult(null);

    const agentUrl = process.env.OMNISOLO_AGENT_URL || 'http://127.0.0.1:18789';

    try {
      const response = await fetch(`${agentUrl}/rpc`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: Math.random().toString(36).substring(7),
          method: 'execute_visual_workflow',
          params: { graph, inputs },
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();

      if (data.error) {
        throw new Error(data.error.message || 'JSON-RPC Error');
      }

      setResult(data.result?.output || 'No output received.');
      setStatus('complete');
    } catch (err: unknown) {
      setError(err instanceof Error && err.message ? err.message : 'An error occurred during execution.');
      setStatus('error');
    }
  };

  return { status, result, error, runWorkflow };
};
