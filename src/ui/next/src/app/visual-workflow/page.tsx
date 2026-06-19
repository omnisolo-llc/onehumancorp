"use client";

import { useState } from "react";
import { useWalkthrough } from "../../components/help";
import { WalkthroughTarget } from "../../components/Walkthrough";

export default function VisualWorkflowPage() {
  const { startWalkthrough } = useWalkthrough();
  const [nodes, setNodes] = useState<{ id: string; type: string; data: any }[]>([]);
  const [edges, setEdges] = useState<{ id: string; source: string; target: string; condition?: string }[]>([]);
  const [result, setResult] = useState<string | null>(null);

  const addNode = (type: string) => {
    const id = `node-${nodes.length + 1}`;
    let data = {};
    if (type === "Llm") data = { prompt_template: "Translate to French: {input}" };
    if (type === "Input") data = { name: "input" };
    if (type === "Output") data = {};

    setNodes([...nodes, { id, type, data }]);
  };

  const addEdge = (source: string, target: string) => {
    setEdges([...edges, { id: `edge-${edges.length + 1}`, source, target }]);
  };

  const runWorkflow = async () => {
    try {
      // Transform nodes to backend format
      const formattedNodes: Record<string, any> = {};
      nodes.forEach(n => {
        formattedNodes[n.id] = { type: n.type, ...n.data };
      });

      const formattedEdges = edges.map(e => ({
        source: e.source,
        target: e.target,
        condition_expression: e.condition || null
      }));

      const res = await fetch("/api/workflow/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          graph: { nodes: formattedNodes, edges: formattedEdges, max_steps: 10 },
          inputs: { input: "Hello world" }
        })
      });
      const data = await res.json();
      setResult(JSON.stringify(data, null, 2));
    } catch (e: any) {
      setResult(`Error: ${e.message}`);
    }
  };

  return (
    <div className="p-8 max-w-6xl mx-auto">
      <h1 className="text-3xl font-bold mb-2 text-gray-800">Visual Workflow Orchestrator</h1>
      <p className="mb-8 text-gray-600">AutoGPT Unique Harness Innovations: Block-based visual workflow construction</p>

      <div className="flex gap-4 mb-8">
        <WalkthroughTarget id="vw-add-node">
          <button
            className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow transition"
            onClick={() => addNode("Input")}
          >
            + Add Input Node
          </button>
        </WalkthroughTarget>
        <button
          className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded shadow transition"
          onClick={() => addNode("Llm")}
        >
          + Add LLM Node
        </button>
        <button
          className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded shadow transition"
          onClick={() => addNode("Output")}
        >
          + Add Output Node
        </button>

        <button
          className="bg-emerald-600 hover:bg-emerald-700 text-white px-4 py-2 rounded shadow transition ml-auto"
          onClick={runWorkflow}
        >
          ▶ Run Workflow
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div className="border border-gray-200 rounded-xl p-6 min-h-[400px] bg-gray-50 shadow-inner">
          <h2 className="text-xl font-semibold mb-4 text-gray-700">Workspace Canvas</h2>

          {nodes.length === 0 && (
            <div className="flex items-center justify-center h-48 text-gray-400 border-2 border-dashed border-gray-300 rounded-lg">
              Add nodes to start building
            </div>
          )}

          <div className="space-y-4">
            {nodes.map((n, i) => (
              <div key={n.id} className="bg-white p-4 shadow-sm border border-gray-200 rounded-lg flex items-center justify-between">
                <div>
                  <span className="inline-block px-2 py-1 bg-gray-100 text-xs font-mono rounded mr-3 text-gray-600">{n.id}</span>
                  <span className="font-bold text-gray-800">{n.type}</span>
                </div>

                {i > 0 && (
                  <button
                    className="text-xs text-blue-600 hover:underline"
                    onClick={() => addEdge(nodes[i-1].id, n.id)}
                  >
                    Connect from previous
                  </button>
                )}
              </div>
            ))}
          </div>

          {edges.length > 0 && (
            <div className="mt-8">
              <h3 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-3">Connections</h3>
              <div className="flex flex-wrap gap-2">
                {edges.map(e => (
                  <span key={e.id} className="bg-blue-50 text-blue-700 border border-blue-200 text-xs px-2 py-1 rounded-full">
                    {e.source} → {e.target}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>

        <div className="border border-gray-200 rounded-xl p-6 bg-gray-900 text-gray-100 shadow-xl overflow-auto h-[500px]">
          <h2 className="text-xl font-semibold mb-4 text-gray-300">Execution Result</h2>
          {result ? (
            <pre className="text-sm font-mono whitespace-pre-wrap">{result}</pre>
          ) : (
            <div className="text-gray-500 italic">Waiting for execution...</div>
          )}
        </div>
      </div>
    </div>
  );
}
