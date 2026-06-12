"use client";

import { useState, useEffect } from "react";

export default function AgentBuilderPage() {
  const [graph, setGraph] = useState({ nodes: [], edges: [] });
  const [inputs, setInputs] = useState("{}");
  const [result, setResult] = useState("");
  const [loading, setLoading] = useState(false);

  const fetchDefaultGraph = () => {
    const defaultGraph = {
      nodes: [
        {
          id: "in",
          node_type: { type: "Input", name: "input_var" },
          position: { x: 50, y: 50 }
        },
        {
          id: "llm1",
          node_type: { type: "LlmChat", system_prompt: "Process this: {{in}}" },
          position: { x: 250, y: 50 }
        },
        {
          id: "out",
          node_type: { type: "Output" },
          position: { x: 450, y: 50 }
        }
      ],
      edges: [
        { source: "in", target: "llm1" },
        { source: "llm1", target: "out" }
      ]
    };
    setGraph(defaultGraph as any);
    setInputs(JSON.stringify({ "in": "Hello world" }, null, 2));
  };

  useEffect(() => {
    fetchDefaultGraph();
  }, []);

  const handleRun = async () => {
    setLoading(true);
    setResult("");
    try {
      const parsedInputs = JSON.parse(inputs);
      const res = await fetch("/api/workflow/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ graph, inputs: parsedInputs })
      });
      const data = await res.json();
      if (data.success) {
        setResult(data.result);
      } else {
        setResult("Error: " + data.error);
      }
    } catch (err: any) {
      setResult("Error: " + err.message);
    }
    setLoading(false);
  };

  const handleDragStart = (e: any, id: string) => {
    e.dataTransfer.setData("text/plain", id);
  };

  const handleDrop = (e: any) => {
    e.preventDefault();
    const id = e.dataTransfer.getData("text/plain");
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    setGraph(prev => ({
      ...prev,
      nodes: prev.nodes.map((node: any) =>
        node.id === id ? { ...node, position: { x, y } } : node
      )
    }));
  };

  const handleDragOver = (e: any) => {
    e.preventDefault();
  };

  return (
    <div className="p-8 max-w-6xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">Agent Builder (Visual Canvas)</h1>
      <p className="text-gray-600 mb-8">SOTA Harness Pattern: Visual/low-code orchestration.</p>

      <div className="grid grid-cols-3 gap-8">
        <div className="col-span-2">
          <h2 className="text-xl font-semibold mb-2">Workflow Canvas</h2>
          <div
            className="w-full h-96 border-2 border-dashed border-gray-300 rounded relative bg-gray-50 overflow-hidden"
            onDrop={handleDrop}
            onDragOver={handleDragOver}
          >
            {/* Draw edges (simple lines for now) */}
            <svg className="absolute top-0 left-0 w-full h-full pointer-events-none">
              {graph.edges.map((edge: any, i) => {
                const sourceNode: any = graph.nodes.find((n: any) => n.id === edge.source);
                const targetNode: any = graph.nodes.find((n: any) => n.id === edge.target);
                if (!sourceNode || !targetNode) return null;

                // Approximate connection points
                const sx = (sourceNode.position?.x || 0) + 75;
                const sy = (sourceNode.position?.y || 0) + 25;
                const tx = (targetNode.position?.x || 0);
                const ty = (targetNode.position?.y || 0) + 25;

                return (
                  <line
                    key={i}
                    x1={sx}
                    y1={sy}
                    x2={tx}
                    y2={ty}
                    stroke="#CBD5E1"
                    strokeWidth="2"
                    markerEnd="url(#arrowhead)"
                  />
                );
              })}
              <defs>
                <marker id="arrowhead" markerWidth="10" markerHeight="7"
                refX="9" refY="3.5" orient="auto">
                  <polygon points="0 0, 10 3.5, 0 7" fill="#CBD5E1" />
                </marker>
              </defs>
            </svg>

            {/* Render Nodes */}
            {graph.nodes.map((node: any) => (
              <div
                key={node.id}
                draggable
                onDragStart={(e) => handleDragStart(e, node.id)}
                className="absolute w-36 bg-white border border-gray-300 shadow-sm rounded p-3 text-sm cursor-move flex flex-col items-center justify-center"
                style={{
                  left: node.position?.x || 0,
                  top: node.position?.y || 0,
                }}
              >
                <span className="font-bold text-gray-800">{node.id}</span>
                <span className="text-xs text-gray-500 mt-1">{node.node_type.type || node.node_type}</span>
              </div>
            ))}
          </div>
        </div>

        <div>
          <h2 className="text-xl font-semibold mb-2">Inputs (JSON)</h2>
          <textarea
            className="w-full h-96 p-4 border rounded font-mono text-sm bg-gray-50"
            value={inputs}
            onChange={(e) => setInputs(e.target.value)}
          />
        </div>
      </div>

      <div className="mt-8">
        <button
          className="bg-blue-600 text-white px-6 py-3 rounded-lg font-bold hover:bg-blue-700 disabled:opacity-50"
          onClick={handleRun}
          disabled={loading}
        >
          {loading ? "Running..." : "Run Agent Workflow"}
        </button>
      </div>

      {result && (
        <div className="mt-8">
          <h2 className="text-xl font-semibold mb-2">Result</h2>
          <div className="p-4 border rounded bg-green-50 text-green-900 whitespace-pre-wrap">
            {result}
          </div>
        </div>
      )}
    </div>
  );
}
