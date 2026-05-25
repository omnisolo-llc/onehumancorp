"use client";

import React, { useState, useEffect, Suspense } from "react";
import Link from "next/link";
import { useSearchParams } from 'next/navigation';
import { WithTooltip } from "../../components/TooltipRegistry";
import { useWalkthrough } from "../../components/help";

function KairosDashboardContent() {
  const searchParams = useSearchParams();
  const { startWalkthrough } = useWalkthrough();
  const [activeTasks, setActiveTasks] = useState([
    { id: "task-1", name: "Inventory Reorder Strategy", status: "In Progress", priority: "High" },
    { id: "task-2", name: "Customer Sentiment Analysis", status: "Queued", priority: "Medium" },
    { id: "task-3", name: "Social Media Campaign Draft", status: "Completed", priority: "Low" },
  ]);

  const [meshNodes, setMeshNodes] = useState([
    { id: "node-1", type: "Brain", status: "Online", load: "12%" },
    { id: "node-2", type: "Nerve", status: "Online", load: "45%" },
    { id: "node-3", type: "Memory", status: "Online", load: "8%" },
  ]);

  useEffect(() => {
    if (searchParams.get('walkthrough') === 'true') {
      setTimeout(() => {
        startWalkthrough([
          { targetId: "kairos-brain", message: "The Shared Task List is the 'Brain' of your business, where KAIROS manages and prioritizes all agent activities." },
          { targetId: "kairos-nerves", message: "The Teammate Mesh acts as the 'Nerves', providing lightning-fast communication between your AI workforce." },
          { targetId: "kairos-memory", message: "AutoDream is the 'Memory', storing every interaction so your agents never forget a detail about your business." }
        ]);
      }, 1000);
    }
  }, [searchParams, startWalkthrough]);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
             <Link href="/dashboard" className="text-blue-600 hover:text-blue-800 transition-colors">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
             </Link>
             <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>KAIROS Orchestration</h1>
         </div>
         <div className="flex items-center gap-2 px-3 py-1 bg-blue-50 rounded-full border border-blue-100">
            <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#0066FF' }}></div>
            <span className="text-xs font-medium text-blue-600">System Synchronized</span>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-7xl mx-auto w-full grid grid-cols-1 lg:grid-cols-3 gap-8">

        {/* 1. Shared Task List (The Brain) */}
        <section id="kairos-brain" className="lg:col-span-2 space-y-6">
            <div className="ohc-hybrid-panel shadow-lg flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-gray-900">Shared Task List</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-indigo-100 text-indigo-700 rounded-md">THE BRAIN</span>
                </div>
                <p className="text-sm text-gray-600 leading-relaxed">
                    KAIROS prioritizes and assigns business tasks across your autonomous team.
                </p>
                <div className="space-y-3">
                    {activeTasks.map(task => (
                        <div key={task.id} className="flex items-center justify-between p-4 bg-white/50 rounded-xl border border-gray-100 shadow-sm">
                            <div className="flex items-center gap-3">
                                <div className={`w-2 h-2 rounded-full ${task.status === 'Completed' ? 'bg-green-500' : task.status === 'In Progress' ? 'bg-blue-500' : 'bg-gray-400'}`}></div>
                                <span className="text-sm font-semibold text-gray-800">{task.name}</span>
                            </div>
                            <div className="flex items-center gap-4">
                                <span className="text-xs font-medium text-gray-500">{task.status}</span>
                                <span className={`text-[10px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded border ${task.priority === 'High' ? 'border-red-200 text-red-600 bg-red-50' : 'border-gray-200 text-gray-500'}`}>
                                    {task.priority}
                                </span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* 2. Teammate Mesh (The Nerves) */}
            <div id="kairos-nerves" className="ohc-hybrid-panel shadow-lg flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-gray-900">Teammate Mesh</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-green-100 text-green-700 rounded-md">THE NERVES</span>
                </div>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Real-time communication and coordination layer for all active agents.
                </p>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {meshNodes.map(node => (
                        <div key={node.id} className="p-4 bg-white/50 rounded-xl border border-gray-100 shadow-sm flex flex-col gap-2">
                            <div className="flex justify-between items-center">
                                <span className="text-xs font-bold text-gray-400 uppercase tracking-widest">{node.type}</span>
                                <span className="w-2 h-2 rounded-full bg-green-500"></span>
                            </div>
                            <div className="text-lg font-bold text-gray-900">{node.status}</div>
                            <div className="flex items-center justify-between mt-2">
                                <span className="text-xs text-gray-500">Load</span>
                                <span className="text-xs font-bold text-gray-700">{node.load}</span>
                            </div>
                            <div className="w-full h-1 bg-gray-100 rounded-full overflow-hidden">
                                <div className="h-full bg-blue-500 transition-all duration-1000" style={{ width: node.load }}></div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </section>

        {/* 3. AutoDream (The Memory) */}
        <section id="kairos-memory" className="lg:col-span-1 space-y-6">
            <div className="ohc-hybrid-panel shadow-lg h-full flex flex-col gap-6">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-gray-900">AutoDream Memory</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-purple-100 text-purple-700 rounded-md">THE MEMORY</span>
                </div>

                <div className="flex-1 flex flex-col items-center justify-center text-center p-8">
                    <div className="relative w-32 h-32 mb-6">
                        <div className="absolute inset-0 bg-purple-200 rounded-full opacity-20 animate-ping"></div>
                        <div className="relative w-32 h-32 bg-gradient-to-br from-purple-500 to-indigo-600 rounded-full shadow-xl flex items-center justify-center text-white">
                            <svg className="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" /></svg>
                        </div>
                    </div>
                    <h3 className="text-lg font-bold text-gray-900 mb-2">Infinite Context</h3>
                    <p className="text-sm text-gray-500 leading-relaxed">
                        AutoDream stores every interaction, ensuring your team learns and grows with your business.
                    </p>
                </div>

                <div className="space-y-4">
                    <div className="p-4 bg-purple-50 rounded-xl border border-purple-100">
                        <div className="text-xs font-bold text-purple-600 uppercase tracking-widest mb-1">Knowledge Density</div>
                        <div className="text-2xl font-bold text-purple-900">842.5 MB</div>
                    </div>
                    <div className="p-4 bg-indigo-50 rounded-xl border border-indigo-100">
                        <div className="text-xs font-bold text-indigo-600 uppercase tracking-widest mb-1">Semantic Clusters</div>
                        <div className="text-2xl font-bold text-indigo-900">12 Active</div>
                    </div>
                </div>
            </div>
        </section>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .ohc-hybrid-panel {
            backdrop-filter: blur(30px) saturate(210%);
            background: rgba(255, 255, 255, 0.65);
            border: 1px solid rgba(255, 255, 255, 0.4);
            border-radius: 16px;
            padding: 24px;
        }
      `}} />
    </div>
  );
}

export default function KairosDashboard() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <KairosDashboardContent />
    </Suspense>
  );
}
