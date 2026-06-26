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
    <div className="p-8">
      <h1 className="text-2xl font-bold mb-4">Goose MCP Extensions UI</h1>
      {error && (
        <div className="border border-[#FF3B30] p-2 mb-4" id="goose-error">
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
            <li key={idx} className="mb-2 p-4 border rounded">
              <h3 className="font-bold" id={`extension-${ext.id}`}>{ext.name}</h3>
              <p>{ext.description}</p>
              <button
                className="mt-2 px-4 py-2 bg-[#0066FF] text-white rounded hover:bg-[#0071E3]"
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
        className="px-4 py-2 border rounded border-black hover:bg-gray-100 mb-8"
        onClick={fetchExtensions}
      >
        Refresh List
      </button>

      <h2 className="text-xl font-semibold mb-2">Execute Extension</h2>
      <div className="p-4 border border-dashed border-gray-400 min-h-[100px]" id="exec-result">
        {execResult ? (
          <pre>{execResult}</pre>
        ) : (
          <p>Select an extension from the list to execute it.</p>
        )}
      </div>
    </div>
  );
}
