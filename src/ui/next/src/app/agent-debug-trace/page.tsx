"use client";

import { useState, useEffect } from "react";

type AgentEvent = {
  type: string;
  content?: string;
  name?: string;
  args_json?: string;
  result?: string;
  iteration?: number;
  isLlmRecoverable?: boolean;
};

function normalizePointer(path: string): string {
  const trimmed = path.trim();
  if (trimmed.startsWith('/')) {
    return trimmed;
  }
  let pointer = '';
  let currentPart = '';
  for (const c of trimmed) {
    if (c === '.' || c === '[' || c === ']') {
      if (currentPart) {
        pointer += '/' + currentPart;
        currentPart = '';
      }
    } else {
      currentPart += c;
    }
  }
  if (currentPart) {
    pointer += '/' + currentPart;
  }
  if (!pointer.startsWith('/')) {
    pointer = '/' + pointer;
  }
  return pointer;
}

function getValueByPointer(obj: any, pointer: string): any {
  if (pointer === '/') return obj;
  const parts = pointer.split('/').slice(1);
  let current = obj;
  for (const part of parts) {
    if (current === null || current === undefined) return undefined;
    const key = part.replace(/~1/g, '/').replace(/~0/g, '~');
    current = current[key];
  }
  return current;
}

const DEFAULT_DEMO_OBSERVATION = JSON.stringify({
  data: {
    items: [
      { id: "item_101", name: "Premium custom cake", qty: 2, price: 85.0 },
      { id: "item_102", name: "Vegan strawberry pastry", qty: 5, price: 12.5 }
    ],
    metadata: {
      location_id: "loc_north_01",
      currency: "USD",
      active_campaign: "spring_baker_2025"
    }
  }
}, null, 2);

export default function AgentDebugTracePage() {
  const [events, setEvents] = useState<AgentEvent[]>([]);
  const [loading, setLoading] = useState(true);

  // Playground States
  const [playgroundOpen, setPlaygroundOpen] = useState(true);
  const [storedPayload, setStoredPayload] = useState(DEFAULT_DEMO_OBSERVATION);
  const [toolCallId, setToolCallId] = useState("call_spring_982");
  const [queryPath, setQueryPath] = useState("data.items[0].name");
  const [queryResult, setQueryResult] = useState<string | null>(null);
  const [queryError, setQueryError] = useState<string | null>(null);
  const [executingRecall, setExecutingRecall] = useState(false);

  useEffect(() => {
    async function fetchTrace() {
      try {
        const res = await fetch("/api/v1/agent-debug-trace");
        if (!res.ok) throw new Error("Failed to load agent trace");
        const data = await res.json();
        setEvents(Array.isArray(data) ? data : []);
      } catch {
        setEvents([]);
      } finally {
        setLoading(false);
      }
    }
    fetchTrace();
  }, []);

  const handleRecallPlayground = (e: React.FormEvent) => {
    e.preventDefault();
    setExecutingRecall(true);
    setQueryResult(null);
    setQueryError(null);

    // Simulate real-time async path recall processing
    setTimeout(() => {
      try {
        let parsed;
        try {
          parsed = JSON.parse(storedPayload);
        } catch {
          setQueryError(`recall_observation: Path-based query '${queryPath}' was requested, but the stored observation is plain text (not valid JSON).`);
          setExecutingRecall(false);
          return;
        }

        const pointer = normalizePointer(queryPath);
        const resolved = getValueByPointer(parsed, pointer);

        if (resolved === undefined) {
          setQueryError(`recall_observation: Path '${queryPath}' (normalized: '${pointer}') was not found in the stored JSON observation.`);
        } else {
          if (typeof resolved === 'string') {
            setQueryResult(resolved);
          } else {
            setQueryResult(JSON.stringify(resolved, null, 2));
          }
        }
      } catch (err: any) {
        setQueryError(err.message || "An unexpected error occurred during observation recall.");
      } finally {
        setExecutingRecall(false);
      }
    }, 400);
  };

  return (
    <div className="min-h-screen bg-gray-50/50 p-8 flex items-center justify-center font-sans">
      <div className="w-full max-w-4xl">
        <div className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-extrabold text-gray-900 tracking-tight">Agent Execution Trace</h1>
            <p className="mt-2 text-gray-500 text-lg">Real-time debug telemetry with LLM-Recoverable ToolMessage highlighting</p>
          </div>
        </div>

        {/* Observation Masking & Path-Based Recall Playground Panel */}
        <div className="mb-8 rounded-2xl border border-zinc-200 bg-white/70 backdrop-blur-[30px] shadow-md p-6">
          <div className="flex items-center justify-between border-b border-gray-150 pb-4 mb-4">
            <div className="flex items-center gap-2.5">
              <span className="flex h-8 w-8 items-center justify-center rounded-xl bg-teal-50 text-teal-700 text-sm">
                ⚙️
              </span>
              <div>
                <h2 className="text-lg font-bold text-gray-900">Recall Observation Playground</h2>
                <p className="text-xs text-gray-500 font-medium">Test the new Context Management path-based JSONPointer lookup.</p>
              </div>
            </div>
            <button
              onClick={() => setPlaygroundOpen(!playgroundOpen)}
              className="px-3 py-1.5 text-xs font-semibold text-gray-600 hover:text-gray-900 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
            >
              {playgroundOpen ? "Hide Playground" : "Show Playground"}
            </button>
          </div>

          {playgroundOpen && (
            <form onSubmit={handleRecallPlayground} className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs font-bold text-gray-500 uppercase mb-1">
                    1. Stored JSON Payload (Simulated Observation Store)
                  </label>
                  <textarea
                    value={storedPayload}
                    onChange={(e) => setStoredPayload(e.target.value)}
                    className="w-full h-44 p-3 font-mono text-xs border border-gray-200 rounded-xl bg-gray-50/50 focus:outline-none focus:ring-2 focus:ring-teal-500/20 focus:border-teal-500 transition-all resize-none"
                    placeholder="Enter valid JSON here..."
                  />
                </div>

                <div className="flex flex-col justify-between">
                  <div className="space-y-3">
                    <div>
                      <label className="block text-xs font-bold text-gray-500 uppercase mb-1">
                        2. Tool Call ID
                      </label>
                      <input
                        type="text"
                        value={toolCallId}
                        onChange={(e) => setToolCallId(e.target.value)}
                        className="w-full h-10 px-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-teal-500/20 focus:border-teal-500 transition-all"
                        placeholder="e.g. call_123"
                        required
                      />
                    </div>

                    <div>
                      <label className="block text-xs font-bold text-gray-500 uppercase mb-1">
                        3. Recall Query Path (JSON Pointer or dot-notation)
                      </label>
                      <input
                        type="text"
                        value={queryPath}
                        onChange={(e) => setQueryPath(e.target.value)}
                        className="w-full h-10 px-3 font-mono text-sm border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-teal-500/20 focus:border-teal-500 transition-all"
                        placeholder="e.g. /data/items/0/id or data.items[0].id"
                        required
                      />
                    </div>
                  </div>

                  <button
                    type="submit"
                    disabled={executingRecall}
                    className="w-full h-11 bg-teal-600 hover:bg-teal-700 text-white font-bold rounded-xl transition-all disabled:opacity-50 flex items-center justify-center gap-2 mt-4 md:mt-0 shadow-sm outline-none focus:ring-2 focus:ring-teal-500/40"
                  >
                    {executingRecall ? (
                      <>
                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                        Recalling...
                      </>
                    ) : (
                      "Recall Sub-slice"
                    )}
                  </button>
                </div>
              </div>

              {/* Recall Query Result Output */}
              {(queryResult || queryError) && (
                <div className="mt-4 border-t border-gray-100 pt-4">
                  <span className="block text-xs font-bold text-gray-500 uppercase mb-1.5">
                    Query Output (Sub-slice Resolved)
                  </span>
                  {queryResult && (
                    <pre className="p-4 bg-gray-900 text-emerald-400 font-mono text-xs rounded-xl overflow-x-auto border border-gray-800 shadow-inner">
                      {queryResult}
                    </pre>
                  )}
                  {queryError && (
                    <div className="p-4 bg-red-50 border border-red-200 text-red-800 text-xs font-mono rounded-xl leading-relaxed">
                      {queryError}
                    </div>
                  )}
                </div>
              )}
            </form>
          )}
        </div>

        {loading ? (
          <div className="flex justify-center p-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
          </div>
        ) : (
          <div className="space-y-6">
            {events.map((ev, index) => (
              <div
                key={index}
                data-testid={`event-${ev.type}`}
                className={`p-6 rounded-2xl border transition-all duration-200 shadow-sm ${
                  ev.isLlmRecoverable
                    ? 'bg-amber-50/80 border-amber-200 shadow-amber-100/50'
                    : 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border-white/40 shadow-sm'
                }`}
              >
                <div className="flex items-center justify-between mb-3">
                  <span className={`px-3 py-1 text-xs font-bold uppercase tracking-wider rounded-full ${
                    ev.isLlmRecoverable
                      ? 'bg-amber-100 text-amber-800'
                      : ev.type === 'TaskComplete'
                        ? 'bg-emerald-100 text-emerald-800'
                        : 'bg-indigo-100 text-indigo-800'
                  }`}>
                    {ev.type}
                  </span>
                  {ev.iteration !== undefined && (
                    <span className="text-gray-400 text-sm font-medium">Turn {ev.iteration}</span>
                  )}
                </div>

                {ev.content && (
                  <p className="text-gray-700 leading-relaxed">{ev.content}</p>
                )}

                {ev.name && (
                  <div className="mt-4 space-y-3">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-semibold text-gray-600">Tool:</span>
                      <code className="px-2 py-1 bg-gray-100 text-gray-800 rounded text-sm">{ev.name}</code>
                    </div>
                    <div>
                      <span className="text-sm font-semibold text-gray-600 block mb-1">Arguments:</span>
                      <pre className="p-3 bg-gray-900 text-gray-100 rounded-xl text-sm overflow-x-auto">
                        {ev.args_json}
                      </pre>
                    </div>
                    {ev.result && (
                      <div>
                        <span className={`text-sm font-semibold block mb-1 ${ev.isLlmRecoverable ? 'text-amber-700' : 'text-gray-600'}`}>
                          Result / Observation:
                        </span>
                        <div className={`p-4 rounded-xl text-sm border ${
                          ev.isLlmRecoverable
                            ? 'bg-amber-100/50 border-amber-200 text-amber-900'
                            : 'bg-gray-50 border-gray-100 text-gray-700'
                        }`}>
                          {ev.isLlmRecoverable && (
                            <div className="flex items-center gap-2 mb-2 font-bold text-amber-800" data-testid="llm-recoverable-badge">
                              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                              </svg>
                              LLM-Recoverable ToolMessage
                            </div>
                          )}
                          <p className="whitespace-pre-wrap">{ev.result}</p>
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
