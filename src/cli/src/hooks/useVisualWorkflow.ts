import { useState } from 'react';

export const useVisualWorkflow = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<string | null>(null);

  const runWorkflow = async (graph: any, inputs: Record<string, string>) => {
    setLoading(true);
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
          method: 'vw_run_workflow',
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

      setResult(data.result?.output || 'No output returned.');
    } catch (err: any) {
      setError(err.message || 'Failed to run visual workflow.');
    } finally {
      setLoading(false);
    }
  };

  return { result, loading, error, runWorkflow };
};
