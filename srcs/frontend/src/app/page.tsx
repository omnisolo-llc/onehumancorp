'use client';
import { useEffect, useState } from 'react';

export default function Home() {
  const [plugins, setPlugins] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/api/mcp/tools')
      .then((res) => {
        if (!res.ok) throw new Error('Failed to fetch capabilities');
        return res.json();
      })
      .then((data) => {
        setPlugins(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  return (
    <main className="min-h-screen p-8">
      <h1 className="text-3xl font-bold mb-8">Capabilities Dashboard</h1>
      <div
        className="p-8 rounded-xl"
        style={{
          backdropFilter: 'blur(15px) saturate(180%)',
          background: 'rgba(255, 255, 255, 0.05)',
          border: '1px solid rgba(255, 255, 255, 0.1)'
        }}
      >
        <h2 className="text-xl font-semibold mb-4">Plugin Mesh Integration</h2>
        {loading && <p>Loading plugins...</p>}
        {error && <p className="text-red-400">{error}</p>}
        {!loading && !error && plugins.length === 0 && <p>No capabilities active.</p>}
        {!loading && !error && plugins.length > 0 && (
          <ul className="space-y-4">
            {plugins.map((plugin, index) => (
              <li key={index} className="flex justify-between border-b border-gray-700 pb-2">
                <span>{plugin.name || plugin.Name}</span>
                <span className="text-gray-400">{plugin.status || plugin.Status}</span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </main>
  );
}
