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

export default function AgentDebugTracePage() {
  const [events, setEvents] = useState<AgentEvent[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchTrace() {
      try {
        const res = await fetch("/api/agent-debug-trace");
        const data = await res.json();
        setEvents(data);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    }
    fetchTrace();
  }, []);

  return (
    <div className="min-h-screen bg-gray-50/50 p-8 flex items-center justify-center font-sans">
      <div className="w-full max-w-4xl">
        <div className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-extrabold text-gray-900 tracking-tight">Agent Execution Trace</h1>
            <p className="mt-2 text-gray-500 text-lg">Real-time debug telemetry with LLM-Recoverable ToolMessage highlighting</p>
          </div>
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
                    : 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] border-white/40 shadow-sm'
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
