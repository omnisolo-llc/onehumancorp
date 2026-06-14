import { useState, useEffect } from 'react';
import { ToolItem } from '../components/ToolProgress';

export interface OrchestratorState {
  status: string;
  tools: ToolItem[];
  error: string | null;
  runAgent: (message: string) => Promise<void>;
  output: string | null;
}

export const useOrchestrator = (): OrchestratorState => {
  const [status, setStatus] = useState('Idle');
  const [error, setError] = useState<string | null>(null);
  const [tools, setTools] = useState<ToolItem[]>([]);
  const [output, setOutput] = useState<string | null>(null);

  const runAgent = async (message: string) => {
    setStatus('Executing request...');
    setError(null);
    setOutput(null);

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
          method: 'run_agent',
          params: { message },
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();

      if (data.error) {
        throw new Error(data.error.message || 'JSON-RPC Error');
      }

      setOutput(data.result?.output || 'No output received.');
      setStatus('Complete');
    } catch (err: any) {
      setError(err.message || 'An error occurred during execution.');
      setStatus('Error');
    }
  };

  return { status, tools, error, runAgent, output };
};
