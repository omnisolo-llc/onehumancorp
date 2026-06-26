"use client";

import { useState } from "react";

export default function GatherActVerifyPage() {
  const [task, setTask] = useState("");
  const [response, setResponse] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);
    setResponse(null);

    try {
      const res = await fetch("/api/gather_act_verify", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ task }),
      });

      if (!res.ok) {
        throw new Error(`Error: ${res.statusText}`);
      }

      const data = await res.json();
      setResponse(JSON.stringify(data, null, 2));
    } catch (err: any) {
      setError(err.message || "Failed to process task");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container mx-auto p-8 max-w-4xl">
      <h1 className="text-3xl font-bold mb-6">Gather-Act-Verify Agent</h1>
      <div className="bg-white p-6 rounded-lg shadow-sm border mb-8">
        <h2 className="text-xl font-semibold mb-4">New Task</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="task" className="block text-sm font-medium text-gray-700 mb-1">
              Task Description
            </label>
            <textarea
              id="task"
              name="task"
              rows={4}
              className="w-full p-3 border rounded-md focus:ring-2 focus:ring-[#0066FF] outline-none"
              placeholder="E.g., Analyze the recent customer reviews and propose an action plan..."
              value={task}
              onChange={(e) => setTask(e.target.value)}
              required
            />
          </div>
          <button
            type="submit"
            disabled={isLoading || !task.trim()}
            className="px-6 py-2 bg-[#0071E3] text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? "Processing..." : "Run Agent"}
          </button>
        </form>
      </div>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 p-4 rounded-md mb-8">
          <h3 className="font-semibold mb-1">Error</h3>
          <p>{error}</p>
        </div>
      )}

      {response && (
        <div className="bg-gray-50 border p-6 rounded-lg">
          <h2 className="text-xl font-semibold mb-4">Agent Output</h2>
          <pre className="bg-white p-4 border rounded overflow-x-auto text-sm">
            <code>{response}</code>
          </pre>
        </div>
      )}
    </div>
  );
}
