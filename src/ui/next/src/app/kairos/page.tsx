"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { useSearchParams } from 'next/navigation';
import { WithTooltip } from "../../components/TooltipRegistry";
import { useWalkthrough } from "../../components/help";

export default function KairosDashboard() {
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
            <div className="ohc-hybrid-panel shadow-sm flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-gray-900">Shared Task List</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-indigo-50 text-indigo-600 rounded-md border border-indigo-100/50 uppercase tracking-wider">The Brain</span>
                </div>
                <p className="text-sm text-gray-600 leading-relaxed max-w-xl">
                    KAIROS prioritizes and assigns business tasks across your autonomous team.
                </p>
                <div className="space-y-3 mt-2">
                    {activeTasks.map(task => (
                        <div key={task.id} className="group relative flex items-center justify-between p-4 bg-white/70 backdrop-blur-md rounded-xl border border-white/60 shadow-[0_2px_10px_rgba(0,0,0,0.02)] transition-all duration-300 hover:shadow-[0_4px_20px_rgba(0,0,0,0.04)] hover:bg-white/90 overflow-hidden">
                            {/* Shimmer effect on hover */}
                            <div className="absolute inset-0 -translate-x-full bg-gradient-to-r from-transparent via-white/40 to-transparent group-hover:animate-[shimmer_1.5s_infinite] pointer-events-none"></div>

                            <div className="flex items-center gap-3 relative z-10">
                                <div className={`w-2 h-2 rounded-full shadow-[0_0_8px_rgba(0,0,0,0.2)] ${task.status === 'Completed' ? 'bg-[#34C759] shadow-[#34C759]/40' : task.status === 'In Progress' ? 'bg-[#0071E3] shadow-[#0071E3]/40' : 'bg-gray-400 shadow-gray-400/40'}`}></div>
                                <span className="text-sm font-semibold text-gray-800">{task.name}</span>
                            </div>
                            <div className="flex items-center gap-4 relative z-10">
                                <span className="text-xs font-medium text-gray-500">{task.status}</span>
                                <span className={`text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-md border ${task.priority === 'High' ? 'border-[#FF3B30]/20 text-[#FF3B30] bg-[#FF3B30]/5' : task.priority === 'Medium' ? 'border-[#FF9500]/20 text-[#FF9500] bg-[#FF9500]/5' : 'border-gray-200/50 text-gray-500 bg-gray-50/50'}`}>
                                    {task.priority}
                                </span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* 2. Teammate Mesh (The Nerves) */}
            <div id="kairos-nerves" className="ohc-hybrid-panel shadow-sm flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-gray-900">Teammate Mesh</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-green-50 text-green-600 rounded-md border border-green-100/50 uppercase tracking-wider">The Nerves</span>
                </div>
                <p className="text-sm text-gray-600 leading-relaxed max-w-xl">
                    Real-time communication and coordination layer for all active agents.
                </p>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-2">
                    {meshNodes.map(node => (
                        <div key={node.id} className="p-5 bg-white/70 backdrop-blur-md rounded-xl border border-white/60 shadow-[0_2px_10px_rgba(0,0,0,0.02)] flex flex-col gap-3 transition-all duration-300 hover:shadow-[0_4px_20px_rgba(0,0,0,0.04)] hover:bg-white/90">
                            <div className="flex justify-between items-center">
                                <span className="text-[11px] font-bold text-gray-400 uppercase tracking-widest">{node.type}</span>
                                <span className="w-2 h-2 rounded-full bg-[#34C759] shadow-[0_0_8px_rgba(52,199,89,0.4)]"></span>
                            </div>
                            <div className="text-lg font-bold text-gray-900">{node.status}</div>
                            <div className="flex items-center justify-between mt-2">
                                <span className="text-[11px] font-semibold text-gray-500 uppercase tracking-wider">Load</span>
                                <span className="text-xs font-bold text-gray-700">{node.load}</span>
                            </div>
                            <div className="w-full h-1.5 bg-gray-100/80 rounded-full overflow-hidden shadow-inner">
                                <div className="h-full bg-gradient-to-r from-[#0071E3] to-[#34C759] transition-all duration-1000 ease-out" style={{ width: node.load }}></div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </section>

        {/* 3. AutoDream (The Memory) */}
        <section id="kairos-memory" className="lg:col-span-1 space-y-6">
            <div className="ohc-hybrid-panel shadow-sm h-full flex flex-col gap-6">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-gray-900">AutoDream Memory</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-purple-50 text-purple-600 rounded-md border border-purple-100/50 uppercase tracking-wider">The Memory</span>
                </div>

                <div className="flex-1 flex flex-col items-center justify-center text-center p-6">
                    <div className="relative w-32 h-32 mb-8">
                        <div className="absolute inset-0 bg-[#0071E3]/10 rounded-full animate-[ping_3s_cubic-bezier(0,0,0.2,1)_infinite]"></div>
                        <div className="absolute inset-4 bg-[#0071E3]/20 rounded-full animate-[ping_2s_cubic-bezier(0,0,0.2,1)_infinite] delay-150"></div>
                        <div className="relative w-32 h-32 bg-white/80 backdrop-blur-xl border border-white/80 rounded-full shadow-[0_8px_32px_rgba(0,113,227,0.15)] flex items-center justify-center text-[#0071E3]">
                            <svg className="w-12 h-12 drop-shadow-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" /></svg>
                        </div>
                    </div>
                    <h3 className="text-lg font-bold text-gray-900 mb-2">Infinite Context</h3>
                    <p className="text-sm text-gray-500 leading-relaxed">
                        AutoDream stores every interaction, ensuring your team learns and grows with your business.
                    </p>
                </div>

                <div className="space-y-3">
                    <div className="p-4 bg-white/70 backdrop-blur-md rounded-xl border border-white/60 shadow-[0_2px_10px_rgba(0,0,0,0.02)] transition-all duration-300 hover:shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
                        <div className="text-[11px] font-bold text-[#0071E3] uppercase tracking-widest mb-1.5">Knowledge Density</div>
                        <div className="text-2xl font-bold text-gray-900">842.5 MB</div>
                    </div>
                    <div className="p-4 bg-white/70 backdrop-blur-md rounded-xl border border-white/60 shadow-[0_2px_10px_rgba(0,0,0,0.02)] transition-all duration-300 hover:shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
                        <div className="text-[11px] font-bold text-[#34C759] uppercase tracking-widest mb-1.5">Semantic Clusters</div>
                        <div className="text-2xl font-bold text-gray-900">12 Active</div>
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
            background: rgba(255, 255, 255, 0.65);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.4);
            border-radius: 16px;
            padding: 24px;
            box-shadow: 0 4px 24px rgba(0, 0, 0, 0.02), inset 0 0 0 1px rgba(255, 255, 255, 0.2);
        }

        @keyframes shimmer {
            100% {
                transform: translateX(100%);
            }
        }
      `}} />
    </div>
  );
}
