'use client';
import React, { useState, useEffect } from 'react';

export default function AgentTerminalPage() {
  const [backend, setBackend] = useState('local');
  const [command, setCommand] = useState('');
  const [output, setOutput] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetch('/api/terminal/backend')
      .then((res) => res.json())
      .then((data) => {
        if (data.backend) {
          setBackend(data.backend);
        }
      });
  }, []);

  const handleBackendChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newBackend = e.target.value;
    setBackend(newBackend);
    await fetch('/api/terminal/backend', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ backend: newBackend }),
    });
    setOutput((prev) => [...prev, `[System] Switched to ${newBackend} backend.`]);
  };

  const handleCommandSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!command.trim()) return;

    setOutput((prev) => [...prev, `$ ${command}`]);
    setCommand('');
    setLoading(true);

    try {
      const res = await fetch('/api/terminal/session/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ command, backend }),
      });

      if (!res.ok) {
        throw new Error(`Failed to start session: ${res.statusText}`);
      }

      const data = await res.json();
      setOutput((prev) => [...prev, data.output]);
    } catch (err: any) {
      setOutput((prev) => [...prev, `Error: ${err.message}`]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-4 md:p-8 max-w-5xl mx-auto min-h-screen">
      <div className="mb-6">
        <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Assistant-First Shell</h1>
        <p className="text-gray-600 dark:text-gray-400 mt-2">
          Agent Command CLI / Multi-backend terminal.
        </p>
      </div>

      <div className="backdrop-blur-xl bg-white/40 dark:bg-black/40 border border-white/20 dark:border-white/10 rounded-2xl p-6 shadow-2xl mb-6">
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-4 gap-4">
          <label className="font-semibold text-gray-700 dark:text-gray-200">
            Terminal Backend:
            <select
              value={backend}
              onChange={handleBackendChange}
              className="ml-3 p-2 border rounded-md bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-100 focus:ring-[#0066FF] focus:border-[#0066FF]"
            >
              <option value="local">Local</option>
              <option value="docker">Docker</option>
            </select>
          </label>
        </div>

        <div className="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-xl h-96 overflow-y-auto shadow-inner mb-4 flex flex-col backdrop-blur-md border border-white/10">
          {output.length === 0 ? (
            <div className="text-gray-500 italic">Welcome to the Multi-Backend Agent Terminal.</div>
          ) : (
            output.map((line, idx) => (
              <div key={idx} className="whitespace-pre-wrap">{line}</div>
            ))
          )}
          {loading && <div className="text-gray-400 mt-2 flex items-center gap-2"><span>Agent is executing</span><span className="animate-pulse">...</span></div>}
        </div>

        <form onSubmit={handleCommandSubmit} className="flex gap-3">
          <input
            type="text"
            value={command}
            onChange={(e) => setCommand(e.target.value)}
            disabled={loading}
            placeholder="Enter command (e.g. echo hello)..."
            className="flex-1 p-3 border border-white/20 dark:border-white/10 rounded-lg bg-white/50 dark:bg-black/50 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-[#0066FF] backdrop-blur-md transition-all"
          />
          <button
            type="submit"
            disabled={loading || !command.trim()}
            className="px-6 py-3 bg-[#0071E3] text-white font-semibold rounded-lg hover:bg-blue-600 disabled:opacity-50 transition-colors shadow-sm"
          >
            Submit
          </button>
        </form>
      </div>
    </div>
  );
}
