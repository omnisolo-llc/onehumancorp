"use client";
import React from 'react';
import Link from 'next/link';

export default function SettingsPage() {
  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden mac-glass-container">
        <header className="px-5 pt-8 pb-4 bg-white/80 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-100 flex items-center justify-between">
           <Link href="/dashboard" className="text-gray-400 hover:text-gray-600">
             <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
           </Link>
           <h1 className="text-3xl font-extrabold font-outfit text-gray-900">Settings</h1>
           <div className="w-6 h-6"></div>
        </header>

        <div className="p-6">
           <div className="flex items-center justify-between p-4 bg-white rounded-lg shadow-sm border border-gray-100">
             <span className="text-gray-800 font-medium">Enable Email Notifications</span>
             <div className="w-10 h-6 bg-green-500 rounded-full relative">
                <div className="w-4 h-4 bg-white rounded-full absolute top-1 right-1"></div>
             </div>
           </div>
        </div>
      </div>
    </div>
  );
}
