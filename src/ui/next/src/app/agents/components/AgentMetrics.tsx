'use client';

import React, { useEffect, useState } from 'react';

type Measurement = {
  agent_id: string;
  messages_processed: number;
  avg_response_time_ms: number | null;
  active_connections: number;
  memory_usage_bytes: number | null;
  last_active_at: string | null;
  error_rate: number | null;
  cost_accumulated: number;
};

export function AgentMetrics() {
  const [measurements, setMeasurements] = useState<Measurement[] | null>(null);
  const [disconnected, setDisconnected] = useState(false);
  useEffect(() => {
    const source = new EventSource('/api/v1/agents/metrics/stream');
    source.addEventListener('metrics', (event) => {
      try {
        const data: unknown = JSON.parse((event as MessageEvent).data);
        if (!Array.isArray(data) || !data.every((row) => row && typeof row.agent_id === 'string')) return;
        setMeasurements(data);
        setDisconnected(false);
      } catch { setDisconnected(true); }
    });
    source.addEventListener('error', () => setDisconnected(true));
    return () => source.close();
  }, []);
  return (
    <section aria-label="Agent performance" className="mt-6 rounded-xl border border-border p-4">
      <h2 className="text-lg font-semibold">Agent performance</h2>
      {disconnected && <p role="status">Connection interrupted. Reconnecting…</p>}
      {measurements === null ? <p>Waiting for measurements…</p> : measurements.length === 0 ? <p>No registered agents yet.</p> : (
        <div className="overflow-x-auto">
          <table className="w-full text-left text-sm">
            <thead><tr>{['Agent', 'Completed requests', 'Average response', 'Connections', 'Memory', 'Last active', 'Error rate', 'Cost'].map((name) => <th key={name} className="p-2">{name}</th>)}</tr></thead>
            <tbody>{measurements.map((row) => <tr key={row.agent_id}>
              <td className="p-2">{row.agent_id}</td>
              <td className="p-2">{row.messages_processed}</td>
              <td className="p-2">{row.avg_response_time_ms == null ? 'Unavailable' : `${Math.round(row.avg_response_time_ms)} ms`}</td>
              <td className="p-2">{row.active_connections}</td>
              <td className="p-2">{row.memory_usage_bytes == null ? 'Unavailable' : `${(row.memory_usage_bytes / 1048576).toFixed(1)} MiB`}</td>
              <td className="p-2">{row.last_active_at == null ? 'Unavailable' : new Date(row.last_active_at).toLocaleString()}</td>
              <td className="p-2">{row.error_rate == null ? 'Unavailable' : `${Math.round(row.error_rate * 100)}%`}</td>
              <td className="p-2">${row.cost_accumulated.toFixed(4)}</td>
            </tr>)}</tbody>
          </table>
        </div>
      )}
    </section>
  );
}
