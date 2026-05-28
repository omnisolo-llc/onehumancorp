"use client";

import React, { Suspense, useEffect, useState } from 'react';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';
import { useWalkthrough, WalkthroughProvider } from '../../components/help';

export default function VirtualMeetingRoomPage() {
  return (
    <WalkthroughProvider>
      <Suspense fallback={<div className="min-h-screen bg-[#16161A] flex items-center justify-center text-white font-inter">Loading...</div>}>
        <VirtualMeetingContent />
      </Suspense>
    </WalkthroughProvider>
  );
}

function VirtualMeetingContent() {
  const searchParams = useSearchParams();
  const { startWalkthrough } = useWalkthrough();

  useEffect(() => {
    if (searchParams.get('walkthrough') === 'true') {
      setTimeout(() => {
        startWalkthrough([
          { targetId: "vmr-board", message: "Agents join the Virtual Meeting Room to debate and plan before executing tasks." },
          { targetId: "ultraplan-phases", message: "Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol)." }
        ]);
      }, 1000);
    }
  }, [searchParams, startWalkthrough]);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#16161A' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(22, 22, 26, 0.7)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.1)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
             <Link href="/dashboard" className="text-blue-600 hover:text-blue-800 transition-colors">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
             </Link>
             <h1 className="text-2xl font-bold font-outfit" style={{ color: '#F5F5F7', letterSpacing: '-0.02em' }}>Virtual Meeting Room</h1>
         </div>
         <div className="flex items-center gap-2 px-3 py-1 bg-blue-50 rounded-full border border-blue-100">
            <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#0066FF' }}></div>
            <span className="text-xs font-medium text-blue-600">Meeting in Progress</span>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-7xl mx-auto w-full grid grid-cols-1 lg:grid-cols-3 gap-8">

        {/* 1. Interactive Planning Board */}
        <section id="vmr-board" className="lg:col-span-2 space-y-6">
            <div className="ohc-hybrid-panel shadow-lg flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-[#F5F5F7]">Interactive Planning Board</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-indigo-100 text-indigo-700 rounded-md">VMR COLLAB</span>
                </div>
                <p className="text-sm text-gray-400 leading-relaxed">
                    Real-time visibility into agent contributions and debates.
                </p>
                <div className="space-y-4">
                    <div className="flex gap-4 items-start p-4 bg-white/5 rounded-xl border border-gray-800">
                        <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
                            <span className="text-sm font-bold text-blue-600">PM</span>
                        </div>
                        <div>
                            <p className="text-sm font-semibold text-gray-200">Product Manager Agent</p>
                            <p className="text-sm text-gray-400 mt-1">Proposed Initial Requirements for "Add User Avatars".</p>
                        </div>
                    </div>
                    <div className="flex gap-4 items-start p-4 bg-white/5 rounded-xl border border-gray-800">
                        <div className="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center flex-shrink-0">
                            <span className="text-sm font-bold text-green-600">ED</span>
                        </div>
                        <div>
                            <p className="text-sm font-semibold text-gray-200">Engineering Director Agent</p>
                            <p className="text-sm text-gray-400 mt-1">Challenging technical feasibility based on current schema.</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        {/* 2. UltraPlan Protocol Phases */}
        <section id="ultraplan-phases" className="lg:col-span-1 space-y-6">
            <div className="ohc-hybrid-panel shadow-lg h-full flex flex-col gap-6">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-[#F5F5F7]">UltraPlan Protocol</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-purple-100 text-purple-700 rounded-md">CONSENSUS</span>
                </div>
                <p className="text-sm text-gray-400 leading-relaxed">
                    Structured collaborations and outputs.
                </p>

                <div className="space-y-4">
                    <div className="p-4 bg-white/5 rounded-xl border border-gray-800 flex flex-col gap-2">
                        <div className="text-xs font-bold text-purple-400 uppercase tracking-widest">Phase 1</div>
                        <div className="text-lg font-bold text-gray-200">Brainstorming</div>
                        <p className="text-xs text-gray-400">Free-form generation of ideas and approaches.</p>
                    </div>
                    <div className="p-4 bg-white/5 rounded-xl border border-gray-800 flex flex-col gap-2">
                        <div className="text-xs font-bold text-indigo-400 uppercase tracking-widest">Phase 2</div>
                        <div className="text-lg font-bold text-gray-200">Refinement</div>
                        <p className="text-xs text-gray-400">Structured critique and debate on technical constraints.</p>
                    </div>
                    <div className="p-4 bg-white/5 rounded-xl border border-gray-800 flex flex-col gap-2">
                        <div className="text-xs font-bold text-green-400 uppercase tracking-widest">Phase 3</div>
                        <div className="text-lg font-bold text-gray-200">Consensus</div>
                        <p className="text-xs text-gray-400">Finalizing the output format as structured JSON/YAML.</p>
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
            backdrop-filter: blur(20px) saturate(200%);
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 16px;
            padding: 24px;
        }
      `}} />
    </div>
  );
}
