"use client";

import Link from 'next/link';

export function GlobalSidebar() {
  return (
    <div className="fixed top-0 left-0 w-16 h-screen flex flex-col items-center py-4 gap-4 bg-white/60 backdrop-blur-[20px] saturate-200 border-r border-white/40 z-50 shadow-sm">
      <Link href="/dashboard" className="w-10 h-10 bg-indigo-100 text-indigo-700 rounded-xl flex items-center justify-center font-bold text-sm hover:scale-105 transition-transform" title="Dashboard">D</Link>

      <div className="w-10 h-[1px] bg-gray-200/50"></div>

      <Link href="/help" className="w-10 h-10 hover:bg-gray-100 rounded-xl flex items-center justify-center transition-colors" title="Help Center">
        <span className="text-xl">❔</span>
      </Link>
      <Link href="/help/videos" className="w-10 h-10 hover:bg-gray-100 rounded-xl flex items-center justify-center transition-colors" title="Tutorials">
        <span className="text-xl">▶️</span>
      </Link>
      <Link href="/changelog" className="w-10 h-10 hover:bg-gray-100 rounded-xl flex items-center justify-center transition-colors" title="Changelog">
        <span className="text-xl">📝</span>
      </Link>
      <Link href="/api-docs" className="w-10 h-10 hover:bg-gray-100 rounded-xl flex items-center justify-center transition-colors" title="API Docs">
        <span className="text-xl">💻</span>
      </Link>
    </div>
  );
}
