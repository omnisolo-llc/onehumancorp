"use client";

import React from 'react';
import Link from 'next/link';

export default function MissionBlockedPage() {
  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter items-center p-4">
      <div className="w-full max-w-[375px] bg-[#F5F5F7] min-h-[600px] shadow-xl relative flex flex-col items-center justify-center rounded-3xl overflow-hidden p-6 text-center">
        {/* Background embellishment */}
        <div className="absolute top-[-50px] right-[-50px] w-48 h-48 bg-orange-100 rounded-full blur-3xl -z-10"></div>
        <div className="absolute bottom-[-50px] left-[-50px] w-48 h-48 bg-red-50 rounded-full blur-3xl -z-10"></div>

        <div className="w-20 h-20 rounded-2xl flex items-center justify-center text-4xl shadow-inner mb-6" style={{ background: 'rgba(255, 149, 0, 0.1)', border: '1px solid rgba(255, 149, 0, 0.2)' }}>
          ⚠️
        </div>

        <h1 className="text-3xl font-extrabold font-outfit text-gray-900 tracking-tight mb-2">Setup Required</h1>

        <p className="text-gray-600 mb-8 text-sm leading-relaxed px-2">
          Your AI helpers are ready to get to work, but they need a place to securely save their progress. Please connect your storage in the settings to continue.
        </p>

        <div className="bg-white/80 backdrop-blur-[30px] saturate-[210%] border border-white/60 shadow-sm p-5 rounded-2xl w-full mb-8 text-left relative overflow-hidden" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <div className="absolute top-0 left-0 w-1 h-full bg-[#FF9500]"></div>
            <h3 className="font-outfit font-bold text-gray-900 text-lg mb-1">Mission Paused</h3>
            <p className="text-xs text-gray-500 mb-3">Pending storage connection</p>
            <div className="w-full bg-gray-100 rounded-full h-2 overflow-hidden">
              <div className="bg-[#FF9500] h-2 rounded-full w-1/3"></div>
            </div>
        </div>

        <Link href="/dashboard" className="w-full py-3.5 bg-gray-900 text-white rounded-xl font-bold transition-all shadow-md hover:bg-black hover:shadow-lg active:scale-[0.98] flex items-center justify-center gap-2">
          Return to Dashboard
        </Link>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
