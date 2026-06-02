"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import WorkflowList, { AIaaSWorkflow } from './components/WorkflowList';

export type AIAgentPersona = {
  id: string;
  name: string;
  system_prompt: string;
  capabilities: string[];
};

export default function AIInboxPage() {
  const router = useRouter();
  const [workflows, setWorkflows] = useState<AIaaSWorkflow[]>([]);
  const [personas, setPersonas] = useState<AIAgentPersona[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const [wfRes, personaRes] = await Promise.all([
        fetch('/api/agents/aiaas/workflows'),
        fetch('/api/agents/aiaas/personas')
      ]);

      if (!wfRes.ok || !personaRes.ok) {
        throw new Error("Failed to fetch AIaaS data");
      }

      const wfData = await wfRes.json();
      const personaData = await personaRes.json();

      setWorkflows(wfData.workflows || []);
      setPersonas(personaData.personas || []);
    } catch (err: any) {
      setError(err.message || "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button onClick={() => router.push('/team')} className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          </button>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">AI Inbox</h1>
            <p className="text-gray-500 text-xs mt-1">Manage AIaaS Workflows</p>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 hide-scrollbar space-y-6">

          {error && (
            <div className="p-4 bg-red-50 text-red-700 rounded-xl text-sm border border-red-100">
              {error}
              <button onClick={fetchData} className="ml-2 font-bold underline">Retry</button>
            </div>
          )}

          {loading ? (
             <div className="flex justify-center py-10">
               <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
             </div>
          ) : (
            <>
              <div>
                <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-1">Active Personas</h2>
                {personas.length === 0 ? (
                  <div className="text-sm text-gray-500 px-1">No personas configured.</div>
                ) : (
                  <div className="flex gap-3 overflow-x-auto pb-2 hide-scrollbar">
                    {personas.map(p => (
                      <div key={p.id} className="flex-shrink-0 w-48 p-4 bg-white rounded-xl shadow-sm border border-gray-100">
                        <h3 className="font-semibold text-gray-900 text-sm truncate">{p.name}</h3>
                        <p className="text-[10px] text-gray-500 mt-1 line-clamp-2">{p.system_prompt}</p>
                        <div className="mt-2 flex flex-wrap gap-1">
                          {p.capabilities.map(c => (
                            <span key={c} className="px-2 py-0.5 bg-blue-50 text-blue-700 text-[9px] rounded font-medium">{c}</span>
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>

              <div>
                <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-1">Active Workflows</h2>
                <WorkflowList workflows={workflows} />
              </div>
            </>
          )}

        </div>
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
