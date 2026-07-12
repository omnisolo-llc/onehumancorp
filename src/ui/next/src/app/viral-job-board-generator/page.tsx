"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import '../globals.css';

import { PoweredByOHC } from "../components/PoweredByOHC";

export default function ViralJobBoardGeneratorPage() {
  const router = useRouter();
  const [boardTitle, setBoardTitle] = useState('We are hiring!');
  const [description, setDescription] = useState('Join our team and help us build the future.');
  const [theme, setTheme] = useState('light');
  const [copied, setCopied] = useState(false);

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { backgroundColor: '#111827', color: '#fff', borderColor: '#374151' };
    }
    return { backgroundColor: '#fff', color: '#111827', borderColor: '#e5e7eb' };
  };

  const generatedLink = `https://ohc.app/jobs/${boardTitle.toLowerCase().replace(/\s+/g, '-')}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral Job Board Generator 📢</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-2xl">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Board Settings</h2>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Job Board Title</label>
              <input
                type="text"
                value={boardTitle}
                onChange={(e) => setBoardTitle(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                placeholder="e.g. We are hiring!"
              />
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Description</label>
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                rows={3}
                placeholder="e.g. Join our team and help us build the future."
              />
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
              <div className="flex gap-2 border p-1 min-h-[44px] min-w-[44px] bg-gray-50 border-gray-200">
                <button
                  onClick={() => setTheme('light')}
                  className={`flex-1 py-1 px-3 rounded text-sm font-medium transition-all ${theme === 'light' ? 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                >
                  Light
                </button>
                <button
                  onClick={() => setTheme('dark')}
                  className={`flex-1 py-1 px-3 rounded text-sm font-medium transition-all ${theme === 'dark' ? 'bg-gray-800 shadow-sm text-white' : 'text-gray-500 hover:text-gray-700'}`}
                >
                  Dark
                </button>
              </div>
            </div>
          </div>

          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-indigo-50/70 backdrop-blur-[40px] border border-indigo-100/50 rounded-2xl mt-6">
            <h3 className="font-bold text-indigo-900 mb-2 flex items-center gap-2">
              <span className="text-xl">🚀</span> Share Your Board
            </h3>
            <p className="text-sm text-indigo-800 mb-4">
              Post this link on social media. People can apply and refer friends to get a bonus!
            </p>

            <div className="flex items-center gap-2 bg-white min-h-[44px] min-w-[44px] border border-indigo-200 p-1 mb-4 overflow-hidden">
              <div className="px-2 py-1 text-xs text-gray-500 truncate flex-1 font-mono">
                {generatedLink}
              </div>
            </div>

            <button
              onClick={handleCopy}
              className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors"
            >
              {copied ? 'Copied!' : 'Copy Link'}
            </button>
          </div>
        </div>

        <div className="w-full md:w-2/3 flex flex-col">
          <div className="flex-1 shadow-[0_20px_40px_rgb(0,0,0,0.15)] overflow-hidden flex flex-col bg-white/40 backdrop-blur-[40px] saturate-[200%] border border-white/50 rounded-2xl relative">
            <div className="bg-gray-200 py-3 px-4 flex items-center gap-2 border-b border-gray-300">
              <div className="flex gap-1.5">
                <div className="w-3 h-3 rounded-full bg-red-400"></div>
                <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                <div className="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <div className="mx-auto bg-white/60 text-xs text-gray-500 px-4 py-1 rounded-full w-1/2 text-center truncate">
                Preview: Your Job Board Page
              </div>
            </div>

            <div className="flex-1 flex items-center justify-center p-8 bg-black/5 overflow-y-auto">
              <div
                className="w-full max-w-sm shadow-[0_8px_30px_rgb(0,0,0,0.1)] p-8 flex flex-col items-center relative overflow-hidden transition-all duration-300 rounded-3xl border"
                style={getThemeStyles()}
              >
                <div className="w-16 h-16 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-3xl mb-6 shadow-inner">
                  💼
                </div>
                <h2 className="text-2xl font-bold font-outfit text-center mb-2">
                  {boardTitle || 'We are hiring!'}
                </h2>
                <p className="text-center text-sm mb-8" style={{ color: theme === 'dark' ? '#9ca3af' : '#4b5563' }}>
                  {description || 'Join our team.'}
                </p>

                <div className="w-full space-y-3 mb-6">
                  <div className="p-4 border rounded-lg cursor-pointer hover:bg-gray-50/10 transition-colors" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                    <h4 className="font-bold text-sm">Senior Developer</h4>
                    <p className="text-xs mt-1 opacity-70">Remote • Full-time</p>
                  </div>
                  <div className="p-4 border rounded-lg cursor-pointer hover:bg-gray-50/10 transition-colors" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                    <h4 className="font-bold text-sm">Marketing Manager</h4>
                    <p className="text-xs mt-1 opacity-70">New York • Full-time</p>
                  </div>
                </div>

                <div className="w-full p-4 rounded-lg bg-green-50 border border-green-200 text-center mb-4">
                  <p className="text-green-800 text-sm font-bold">Know someone?</p>
                  <p className="text-green-700 text-xs mt-1">Refer a friend and get $500 if they are hired!</p>
                  <button className="mt-3 w-full py-2 bg-green-600 hover:bg-green-700 text-white text-xs font-bold rounded min-h-[44px] transition-colors">
                    Refer a Friend
                  </button>
                </div>

                <div className="mt-4 pt-4 border-t w-full text-center" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                  <PoweredByOHC tenantId="growth" />
                </div>
              </div>
            </div>
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
