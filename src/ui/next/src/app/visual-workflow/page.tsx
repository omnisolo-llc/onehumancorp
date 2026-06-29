"use client";

import { useState } from "react";
import { useWalkthrough } from "../../components/help";
import { WalkthroughTarget } from "../../components/Walkthrough";

export default function VisualWorkflowPage() {
  const { startWalkthrough } = useWalkthrough();
  const [nodes, setNodes] = useState<{ id: string; type: string; data: any }[]>([]);
  const [edges, setEdges] = useState<{ id: string; source: string; target: string; condition?: string }[]>([]);
  const [result, setResult] = useState<string | null>(null);
  const [inputValue, setInputValue] = useState<string>("Hello world");

  const addNode = (type: string) => {
    const id = `node-${nodes.length + 1}`;
    let data = {};
    if (type === "Llm") data = { prompt_template: "Translate to French: {{input_var}}" };
    if (type === "Input") data = { name: "input_var" };
    if (type === "Output") data = {};
    if (type === "HumanInLoop") data = { prompt_template: "Please approve this text: {{input_var}}" };

    setNodes([...nodes, { id, type, data }]);
  };

  const addEdge = (source: string, target: string) => {
    setEdges([...edges, { id: `edge-${edges.length + 1}`, source, target }]);
  };

  const runWorkflow = async () => {
    try {
      // Transform nodes to backend format
      const formattedNodes = nodes.map(n => ({
        id: n.id,
        node_type: { type: n.type, ...n.data }
      }));

      const formattedEdges = edges.map(e => ({
        source: e.source,
        target: e.target
      }));

      const res = await fetch("/api/workflow/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          graph: { nodes: formattedNodes, edges: formattedEdges },
          inputs: { input_var: inputValue }
        })
      });
      const data = await res.json();
      setResult(JSON.stringify(data, null, 2));
    } catch (e: any) {
      setResult(`Error: ${e.message}`);
    }
  };

  return (
    <div className="p-4 md:p-8 max-w-6xl mx-auto min-h-screen bg-gray-50/30">
      <h1 className="text-3xl font-bold mb-2 text-[#1D1D1F] tracking-tight">Visual Workflow Orchestrator</h1>
      <p className="mb-8 text-gray-600 text-lg">AutoGPT Unique Harness Innovations: Block-based visual workflow construction</p>

      <div className="flex flex-col gap-6 mb-8">
        <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4">
          <label className="font-semibold text-gray-700 sm:w-32 text-sm sm:text-base">Input Value:</label>
          <input
            type="text"
            className="border border-gray-300 rounded-lg px-4 py-2.5 flex-grow focus:outline-none focus:ring-2 focus:ring-[#0071E3] transition-all min-h-[44px]"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
          />
        </div>
        <div className="flex flex-wrap gap-3">
          <WalkthroughTarget id="vw-add-node">
            <button
              className="bg-[#0071E3]/90 backdrop-blur-md hover:bg-[#005bb5]/90 text-white px-4 py-2.5 rounded-lg shadow-sm transition-all min-h-[44px] font-medium"
              onClick={() => addNode("Input")}
            >
              + Add Input Node
            </button>
          </WalkthroughTarget>
          <button
            className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2.5 rounded-lg shadow-sm transition-all min-h-[44px] font-medium"
            onClick={() => addNode("Llm")}
          >
            + Add LLM Node
          </button>
          <button
            className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2.5 rounded-lg shadow-sm transition-all min-h-[44px] font-medium"
            onClick={() => addNode("Output")}
          >
            + Add Output Node
          </button>
          <button
            className="bg-orange-500 hover:bg-orange-600 text-white px-4 py-2.5 rounded-lg shadow-sm transition-all min-h-[44px] font-medium"
            onClick={() => addNode("HumanInLoop")}
          >
            + Add Human-In-Loop Node
          </button>

          <button
            className="bg-[#34C759] hover:bg-[#2db34f] text-white px-4 py-2.5 rounded-lg shadow-sm transition-all min-h-[44px] sm:ml-auto w-full sm:w-auto font-medium"
            onClick={runWorkflow}
          >
            ▶ Run Workflow
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 md:gap-8">
        <div className="border border-white/20 rounded-2xl p-6 min-h-[400px] bg-white/40 backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)]">
          <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F]">Workspace Canvas</h2>

          {nodes.length === 0 && (
            <div className="flex items-center justify-center h-48 text-gray-400 border-2 border-dashed border-gray-300 rounded-xl bg-white/30">
              Add nodes to start building
            </div>
          )}

          <div className="space-y-4">
            {nodes.map((n, i) => (
              <div key={n.id} className="bg-white/80 backdrop-blur-md p-4 shadow-sm border border-gray-200/60 rounded-xl flex items-center justify-between transition-all hover:shadow-md">
                <div className="flex items-center">
                  <span className="inline-flex items-center justify-center px-2.5 py-1 bg-gray-100/80 text-xs font-mono rounded-md mr-3 text-gray-600 min-w-[3rem]">{n.id}</span>
                  <span className="font-semibold text-gray-800">{n.type}</span>
                </div>

                {i > 0 && (
                  <button
                    className="text-sm font-medium text-[#0071E3] hover:text-[#005bb5] transition-colors p-2 -mr-2 min-h-[44px] min-w-[44px]"
                    onClick={() => addEdge(nodes[i-1].id, n.id)}
                  >
                    Connect from previous
                  </button>
                )}
              </div>
            ))}
          </div>

          {edges.length > 0 && (
            <div className="mt-8 pt-6 border-t border-gray-200/50">
              <h3 className="text-xs font-bold text-gray-500 uppercase tracking-wider mb-3">Connections</h3>
              <div className="flex flex-wrap gap-2">
                {edges.map(e => (
                  <span key={e.id} className="bg-[#0071E3]/10 text-[#0071E3] border border-[#0071E3]/20 text-xs px-3 py-1.5 rounded-full font-medium">
                    {e.source} → {e.target}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>

        <div className="border border-white/10 rounded-2xl p-6 bg-[#16161A]/60 backdrop-blur-3xl saturate-[180%] text-[#F5F5F7] shadow-xl overflow-auto h-[400px] lg:h-[500px]">
          <h2 className="text-xl font-semibold mb-4 text-white">Execution Result</h2>
          {result ? (
            <pre className="text-sm font-mono whitespace-pre-wrap break-all text-gray-300 bg-black/20 p-4 rounded-xl border border-white/5">{result}</pre>
          ) : (
            <div className="flex h-3/4 items-center justify-center text-gray-500 italic text-sm">Waiting for execution...</div>
          )}
        </div>
      </div>
    </div>
  );
}
