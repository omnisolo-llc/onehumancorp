'use client';
import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

export default function Home() {
  const router = useRouter();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    const hasOnboarded = localStorage.getItem('has_onboarded');
    if (hasOnboarded) {
      router.push('/dashboard');
    }
  }, [router]);

  if (!mounted) return null;

  return (
    <div className="min-h-screen bg-gray-900 font-inter flex flex-col relative overflow-hidden" style={{ background: 'linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%)' }}>
      <title>OHC - Hybrid Agentic OS</title>

      {/* Decorative background elements */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-blue-500/20 rounded-full blur-[120px] pointer-events-none"></div>
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-purple-500/20 rounded-full blur-[120px] pointer-events-none"></div>

      <header className="px-8 py-6 flex items-center justify-between relative z-10">
         <h1 className="text-2xl font-bold font-outfit text-white tracking-tight">OneHumanCorp</h1>
         <div className="flex items-center gap-4">
             <button onClick={() => router.push('/login')} className="px-4 py-2 text-sm font-medium text-gray-300 hover:text-white transition-colors">
               Sign In
             </button>
         </div>
      </header>

      <main className="flex-1 max-w-6xl w-full mx-auto p-4 md:p-8 flex flex-col items-center justify-center relative z-10 text-center">
        <h2 className="text-4xl md:text-6xl font-bold font-outfit text-white mb-6 tracking-tight leading-tight">
          The <span className="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-400">Hybrid Agentic OS</span><br/> for Small Businesses.
        </h2>
        <p className="text-lg md:text-xl text-gray-300 mb-12 max-w-2xl leading-relaxed">
          Run your entire business with specialized AI agents. Choose between total local sovereignty or seamless cloud collaboration.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 w-full max-w-4xl">
          {/* Local-First Standalone Sovereignty */}
          <div className="text-left rounded-2xl p-8 transition-all hover:translate-y-[-4px]"
               style={{
                 backdropFilter: 'blur(20px) saturate(200%)',
                 background: 'rgba(255, 255, 255, 0.05)',
                 border: '1px solid rgba(255, 255, 255, 0.1)',
                 boxShadow: '0 8px 32px 0 rgba(0, 0, 0, 0.3)'
               }}>
            <div className="w-14 h-14 bg-blue-500/20 rounded-xl flex items-center justify-center text-blue-400 mb-6 border border-blue-500/30">
              <svg className="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8V7a4 4 0 00-8 0v4h8z" /></svg>
            </div>
            <h3 className="text-2xl font-bold font-outfit text-white mb-3">Local-First Sovereignty</h3>
            <p className="text-gray-400 mb-6 text-sm leading-relaxed">
              Complete privacy with Zero Data Leakage. All AI operations run locally via SQLite. Your data never leaves your machine. Perfect for prosumers and IP-sensitive operations.
            </p>
            <ul className="space-y-3 mb-8">
              <li className="flex items-center gap-2 text-sm text-gray-300">
                <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Zero Cloud Telemetry
              </li>
              <li className="flex items-center gap-2 text-sm text-gray-300">
                <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Air-Gapped Autonomy
              </li>
              <li className="flex items-center gap-2 text-sm text-gray-300">
                <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Local SQLite Database
              </li>
            </ul>
            <button
              onClick={() => router.push('/onboarding')}
              className="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-xl font-semibold transition-colors shadow-lg shadow-blue-900/50"
            >
              Start Local Workspace
            </button>
          </div>

          {/* Cloud Convenience */}
          <div className="text-left rounded-2xl p-8 transition-all hover:translate-y-[-4px]"
               style={{
                 backdropFilter: 'blur(20px) saturate(200%)',
                 background: 'rgba(255, 255, 255, 0.05)',
                 border: '1px solid rgba(255, 255, 255, 0.1)',
                 boxShadow: '0 8px 32px 0 rgba(0, 0, 0, 0.3)'
               }}>
            <div className="w-14 h-14 bg-purple-500/20 rounded-xl flex items-center justify-center text-purple-400 mb-6 border border-purple-500/30">
              <svg className="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" /></svg>
            </div>
            <h3 className="text-2xl font-bold font-outfit text-white mb-3">Cloud Convenience</h3>
            <p className="text-gray-400 mb-6 text-sm leading-relaxed">
              Scale your operations with multi-tenant collaboration. Invite team members instantly and access your AI workforce from anywhere with enterprise-grade infrastructure.
            </p>
            <ul className="space-y-3 mb-8">
              <li className="flex items-center gap-2 text-sm text-gray-300">
                <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Seamless Team Expansion
              </li>
              <li className="flex items-center gap-2 text-sm text-gray-300">
                <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Sync Across Devices
              </li>
              <li className="flex items-center gap-2 text-sm text-gray-300">
                <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Managed High-Availability
              </li>
            </ul>
            <button
              onClick={() => router.push('/onboarding')}
              className="w-full py-3 bg-white hover:bg-gray-100 text-gray-900 rounded-xl font-semibold transition-colors shadow-lg shadow-white/10"
            >
              Deploy to Cloud
            </button>
          </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}