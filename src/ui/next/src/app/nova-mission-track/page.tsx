"use client";

import React, { useState } from 'react';
import Link from 'next/link';

type Mission = {
  id: string;
  title: string;
  status: 'active' | 'completed' | 'pending';
  progress: number;
  agent: string;
  department: string;
};

const MISSIONS: Mission[] = [
  { id: '1', title: 'Setting up your business profile', status: 'completed', progress: 100, agent: 'Business Advisor', department: 'Operations' },
  { id: '2', title: 'Applying your brand colors', status: 'completed', progress: 100, agent: 'Designer', department: 'Design' },
  { id: '3', title: 'Connecting AI helpers', status: 'active', progress: 65, agent: 'Software Engineer', department: 'Engineering' },
  { id: '4', title: 'Organizing team tasks', status: 'active', progress: 40, agent: 'Manager', department: 'Operations' },
  { id: '5', title: 'Learning your business needs', status: 'pending', progress: 0, agent: 'Researcher', department: 'Data Science' },
];

export default function MissionTrackPage() {
  const [activeTab, setActiveTab] = useState<'all' | 'active' | 'completed'>('all');

  const filteredMissions = MISSIONS.filter(m => {
    if (activeTab === 'active') return m.status === 'active';
    if (activeTab === 'completed') return m.status === 'completed';
    return true;
  });

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
      <div className="w-full max-w-[375px] bg-[#F5F5F7] min-h-screen shadow-xl relative flex flex-col">
        {/* Header */}
        <header className="px-5 pt-10 pb-4 bg-white/70 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-200">
          <div className="flex justify-between items-center mb-4">
            <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors flex items-center justify-center min-w-[44px] min-h-[44px]">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <span className="text-xs font-bold text-indigo-600 uppercase tracking-widest bg-indigo-50 px-2.5 py-1 rounded-full">Nova Track</span>
          </div>
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 tracking-tight">Mission Control</h1>
          <p className="text-sm text-gray-500 mt-1 font-medium">Tracking your team's progress.</p>
        </header>

        {/* Tabs */}
        <div className="flex px-5 pt-4 pb-2 gap-2 overflow-x-auto hide-scrollbar sticky top-[108px] z-10 bg-[#F5F5F7]/90 backdrop-blur-md">
          {['all', 'active', 'completed'].map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab as any)}
              className={`min-h-[44px] px-4 py-2 rounded-full text-sm font-semibold capitalize whitespace-nowrap transition-all ${
                activeTab === tab
                  ? 'bg-gray-900 text-white shadow-md'
                  : 'bg-white text-gray-600 border border-gray-200 hover:bg-gray-50'
              }`}
            >
              {tab}
            </button>
          ))}
        </div>

        {/* Mission List */}
        <main className="flex-1 p-5 overflow-y-auto pb-24 space-y-4">
          {filteredMissions.map(mission => (
            <div key={mission.id} className="bg-white/80 backdrop-blur-[30px] saturate-[210%] border border-white/60 shadow-sm p-4 rounded-2xl hover:shadow-md transition-all active:scale-[0.98] cursor-pointer">
              <div className="flex justify-between items-start mb-3">
                <span className={`text-[10px] font-bold px-2 py-1 rounded-md uppercase tracking-wider ${
                  mission.status === 'completed' ? 'bg-green-100 text-green-800' :
                  mission.status === 'active' ? 'bg-blue-100 text-blue-800' :
                  'bg-gray-100 text-gray-600'
                }`}>
                  {mission.status}
                </span>
                <span className="text-xs font-medium text-gray-400">{mission.department}</span>
              </div>

              <h3 className="font-outfit font-bold text-gray-900 text-lg mb-1 leading-tight">{mission.title}</h3>
              <p className="text-sm text-gray-500 mb-4 flex items-center gap-1.5">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
                {mission.agent}
              </p>

              {/* Progress Bar */}
              <div className="w-full bg-gray-100 rounded-full h-2 mb-1.5 overflow-hidden">
                <div
                  className={`h-2 rounded-full transition-all duration-1000 ${
                    mission.status === 'completed' ? 'bg-green-500' :
                    mission.status === 'active' ? 'bg-blue-500 relative overflow-hidden' :
                    'bg-gray-300'
                  }`}
                  style={{ width: `${mission.progress}%` }}
                >
                  {mission.status === 'active' && (
                    <div className="absolute top-0 left-0 right-0 bottom-0 bg-white/20 animate-pulse"></div>
                  )}
                </div>
              </div>
              <div className="text-right text-[10px] font-bold text-gray-400 uppercase tracking-wider">{mission.progress}%</div>
            </div>
          ))}
        </main>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
      `}} />
    </div>
  );
}
