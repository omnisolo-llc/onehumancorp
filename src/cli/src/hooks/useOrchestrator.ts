import { useState, useEffect } from 'react';
import { ToolItem } from '../components/ToolProgress';
import { ApprovalRequest } from '../components/ApprovalGate';

export interface OrchestratorState {
  status: string;
  tools: ToolItem[];
  error: string | null;
  pendingApproval: ApprovalRequest | null;
}

export const useOrchestrator = (): OrchestratorState => {
  const [status, setStatus] = useState('Initializing Agent...');
  const [error] = useState<string | null>(null);
  const [tools, setTools] = useState<ToolItem[]>([
    { name: 'ls -la', status: 'success' },
    { name: 'read_file', status: 'pending' }
  ]);
  const [pendingApproval] = useState<ApprovalRequest | null>(null);

  useEffect(() => {
    // In the future, this will connect to the OHC local orchestrator via IPC or WebSocket.
    const timer = setTimeout(() => {
      setStatus('Analyzing Codebase...');
      setTools([
        { name: 'ls -la', status: 'success' },
        { name: 'read_file', status: 'success' },
        { name: 'set_plan', status: 'pending' }
      ]);


    }, 2000);
    return () => clearTimeout(timer);
  }, []);

  return { status, tools, error, pendingApproval };
};
