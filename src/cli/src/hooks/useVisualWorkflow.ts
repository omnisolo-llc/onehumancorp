import { useState } from 'react';

export interface Node {
  id: string;
  node_type: any;
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

    const agentUrl = process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789';

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
    } catch (err: any) {
      setError(err.message || 'An error occurred during execution.');
      setStatus('error');
    }
  };

  return { status, result, error, runWorkflow };
};
