import { useState, useEffect } from 'react';

export interface AgentTemplate {
  id: string;
  name: string;
  description: string;
  author: string;
  downloads: number;
}

export const useMarketplace = () => {
  const [agents, setAgents] = useState<AgentTemplate[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchAgents = async (query: string = "") => {
    setLoading(true);
    setError(null);

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
          method: 'am_search_agents',
          params: { query },
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();

      if (data.error) {
        throw new Error(data.error.message || 'JSON-RPC Error');
      }

      setAgents(data.result || []);
    } catch (err: any) {
      setError(err.message || 'Failed to fetch marketplace agents.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchAgents();
  }, []);

  return { agents, loading, error, fetchAgents };
};
