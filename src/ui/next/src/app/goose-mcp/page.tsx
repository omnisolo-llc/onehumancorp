'use client';
import React, { useState, useEffect } from 'react';

export default function GooseMcpPage() {
  const [extensions, setExtensions] = useState<any[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [execResult, setExecResult] = useState<string | null>(null);

  const fetchExtensions = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/agents/goose');
      const data = await res.json();
      if (data.error) {
        setError(data.error);
      } else if (data.result) {
        setExtensions(data.result);
      } else {
        setError('Failed to fetch extensions');
      }
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchExtensions();
  }, []);

  const handleExecute = async (id: string) => {
    try {
      const res = await fetch('/api/agents/goose/execute', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          id,
          args: { echo: 'hello from UI' }
        }),
      });
      const data = await res.json();
      if (data.error) {
        setExecResult('Error: ' + JSON.stringify(data.error));
      } else {
        setExecResult(JSON.stringify(data.result, null, 2));
      }
    } catch (err: any) {
      setExecResult('Error: ' + err.message);
    }
  };

  return (
    <div className="max-w-6xl mx-auto p-8 font-sans">
      <h1 className="text-3xl font-bold mb-4">Goose MCP Extensions UI</h1>
      {error && (
        <div className="border border-red-200 p-4 mb-4 rounded-xl shadow-sm bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-700" id="goose-error">
          <p className="font-bold">Error:</p>
          <p>{error}</p>
        </div>
      )}
      <h2 className="text-xl font-semibold mb-2">Available Extensions</h2>
      {loading ? (
        <p>Loading...</p>
      ) : extensions.length === 0 ? (
        <p>No extensions found.</p>
      ) : (
        <ul className="mb-4">
          {extensions.map((ext, idx) => (
            <li key={idx} className="p-6 border border-white/20 dark:border-white/10 rounded-2xl shadow-xl bg-white/40 dark:bg-black/40 backdrop-blur-2xl saturate-[210%] mb-4 transition-all hover:bg-white/50 dark:hover:bg-black/50 hover:shadow-2xl">
              <h3 className="font-bold" id={`extension-${ext.id}`}>{ext.name}</h3>
              <p>{ext.description}</p>
              <button
                className="mt-4 px-4 py-2 bg-[#0071E3] text-white rounded-lg shadow-sm font-medium hover:bg-[#0071E3] transition-colors"
                onClick={() => handleExecute(ext.id)}
                id={`execute-${ext.id}`}
              >
                Execute
              </button>
            </li>
          ))}
        </ul>
      )}
      <button
        className="px-6 py-3 border border-white/20 dark:border-white/10 rounded-xl shadow-lg hover:shadow-xl hover:bg-white/60 dark:hover:bg-black/60 transition-all font-medium mb-8 bg-white/40 dark:bg-black/40 backdrop-blur-2xl saturate-[210%]"
        onClick={fetchExtensions}
      >
        Refresh List
      </button>

      <h2 className="text-xl font-semibold mb-2">Execute Extension</h2>
      <div className="p-6 border border-white/20 dark:border-white/10 rounded-3xl min-h-[150px] shadow-2xl bg-white/40 dark:bg-black/40 backdrop-blur-2xl saturate-[210%] overflow-hidden" id="exec-result">
        {execResult ? (
          <pre>{execResult}</pre>
        ) : (
          <p>Select an extension from the list to execute it.</p>
        )}
      </div>
    </div>
  );
}
