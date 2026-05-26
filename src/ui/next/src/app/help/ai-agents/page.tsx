"use client";
import React from 'react';
import Link from 'next/link';

export default function AiAgentsHelp() {
  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter p-4">
      <div className="w-full max-w-[600px] bg-white shadow-xl rounded-2xl overflow-hidden flex flex-col">
        <header className="px-6 py-5 border-b border-gray-100 flex items-center gap-4">
          <Link href="/" className="text-blue-600 hover:bg-blue-50 p-2 rounded-full transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold text-gray-900">Your AI Helpers</h1>
        </header>
        <div className="p-6 space-y-6 flex-1 overflow-y-auto">
          <p className="text-gray-600 text-lg">Learn how to hire AI helpers and give them tasks to do.</p>
        </div>
      </div>
    </div>
  );
}
