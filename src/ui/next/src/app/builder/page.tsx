"use client";

import { useState } from "react";
import { SmartBlock } from "./components";

export default function BuilderPage() {
  const [bio, setBio] = useState("");
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [liveUrl, setLiveUrl] = useState("");

  const handleGenerate = async () => {
    setStatus("generating");

    try {
      const response = await fetch('/builder/api', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ bio })
      });

      const data = await response.json();
      setBlocks(data.blocks);
      setStatus("draft");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      setStatus("idle");
    }
  };

  const handleLaunch = () => {
    // Simulate background provisioning of subdomain and SSL
    setTimeout(() => {
      setStatus("live");
      const subdomain = bio.toLowerCase().replace(/[^a-z0-9]/g, '').substring(0, 10);
      setLiveUrl(`https://${subdomain || 'myshop'}.ohc.store`);
    }, 1500);
  };

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900 font-inter p-4 transition-colors duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
        <div className="w-full max-w-[375px] h-[812px] glassmorphism-container shadow-2xl flex flex-col relative overflow-hidden transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
          <div className="p-8 flex flex-col flex-1 justify-center animate-fade-in">
            <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#F5F5F7] mb-2">Welcome to OHC Smart Builder</h1>
            <p className="text-gray-500 dark:text-gray-400 text-sm mb-8 leading-relaxed">
              Tell us about your business in a few words, and we'll magically generate your storefront in seconds.
            </p>

            <label className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2 block">Your Business</label>
            <textarea
              className="w-full glassmorphism-input p-4 mb-6 focus:ring-2 focus:ring-[#0071E3] outline-none transition-all resize-none text-gray-800 dark:text-[#F5F5F7]"
              value={bio}
              onChange={(e) => setBio(e.target.value)}
              placeholder="e.g. I run a mobile dog grooming service in Portland"
              rows={4}
            />

            <button
              className={`w-full p-4 glassmorphism-button font-bold font-outfit text-lg transition-all ${
                bio.trim().length > 5
                  ? "bg-[#0071E3] text-white hover:bg-[#0066FF] shadow-md active:scale-[0.98]"
                  : "bg-gray-100 dark:bg-gray-800 text-gray-400 dark:text-gray-500 cursor-not-allowed"
              }`}
              onClick={handleGenerate}
              disabled={bio.trim().length <= 5}
            >
              Build My Storefront
            </button>
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900 font-inter p-4 transition-colors duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
        <div className="w-full max-w-[375px] h-[812px] glassmorphism-container shadow-2xl flex flex-col items-center justify-center transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 dark:border-[#F5F5F7] mb-6"></div>
          <p className="text-gray-600 dark:text-gray-300 font-medium animate-pulse text-center px-8">
            The Promoter is picking colors and building your menu...
          </p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900 font-inter p-4 transition-colors duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
        <div className="w-full max-w-[375px] h-[812px] glassmorphism-container shadow-2xl flex flex-col items-center justify-center text-center px-6 relative overflow-hidden transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] animate-fade-in">
          <div className="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-[#34C759]/20 to-transparent -z-10" />
          <div className="w-20 h-20 bg-[#34C759]/10 text-[#34C759] rounded-full flex items-center justify-center mb-6 shadow-sm">
            <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#F5F5F7] mb-2">You're Live!</h1>
          <p className="text-gray-500 dark:text-gray-400 mb-8">Your automated storefront is successfully published and ready for customers.</p>

          <div className="w-full glassmorphism-input p-4 mb-8 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-gray-300 truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-[#0071E3] font-semibold text-sm hover:underline">Copy</button>
          </div>

          <button
            className="w-full bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#1D1D1F] font-bold p-4 glassmorphism-button shadow-md active:scale-[0.98]"
            onClick={() => setStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900 font-inter p-4 transition-colors duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
      <div className="w-full max-w-[375px] h-[812px] glassmorphism-container shadow-2xl flex flex-col relative overflow-hidden transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] animate-fade-in">

        {/* Draft Preview Header */}
        <div className="absolute top-0 left-0 w-full glassmorphism-header text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span className="text-[#1D1D1F] dark:text-[#F5F5F7]">Preview Mode</span>
          <span className="bg-black/10 dark:bg-white/20 px-2 py-0.5 rounded-md text-[#1D1D1F] dark:text-[#F5F5F7]">375px</span>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto pb-24 pt-10 hide-scrollbar bg-white dark:bg-[#16161A]">
          {blocks.map((b, i) => (
            <SmartBlock key={i} {...b} />
          ))}
        </div>

        {/* Bottom Action Bar */}
        <div className="absolute bottom-0 w-full p-4 glassmorphism-footer z-50">
          <div className="flex gap-3 mb-2">
            <button className="flex-1 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 glassmorphism-input hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors">Change Vibe</button>
            <button className="flex-1 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 glassmorphism-input hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors">Edit Text</button>
          </div>
          <button
            className="w-full bg-[#0071E3] text-white p-4 glassmorphism-button font-bold shadow-lg hover:bg-[#0066FF] active:scale-[0.98] transition-all flex justify-center items-center gap-2"
            onClick={handleLaunch}
          >
            <span>1-Tap Launch</span>
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </button>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }

        /* OHC Premium Glassmorphism */
        :root {
          --glass-bg-light: rgba(255, 255, 255, 0.65);
          --glass-border-light: rgba(255, 255, 255, 0.4);
          --glass-bg-dark: rgba(22, 22, 26, 0.7);
          --glass-border-dark: rgba(255, 255, 255, 0.1);
        }

        .glassmorphism-container {
          background: var(--glass-bg-light);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
          border: 1px solid var(--glass-border-light);
          border-radius: 16px;
        }

        .glassmorphism-input {
          background: rgba(255, 255, 255, 0.5);
          backdrop-filter: blur(20px);
          -webkit-backdrop-filter: blur(20px);
          border: 1px solid var(--glass-border-light);
          border-radius: 8px;
        }

        .glassmorphism-button {
          border-radius: 8px;
          border: 1px solid transparent;
        }

        .glassmorphism-header {
          background: rgba(255, 255, 255, 0.85);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
          border-bottom: 1px solid var(--glass-border-light);
        }

        .glassmorphism-footer {
          background: rgba(255, 255, 255, 0.85);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
          border-top: 1px solid var(--glass-border-light);
          border-bottom-left-radius: 16px;
          border-bottom-right-radius: 16px;
        }

        @media (prefers-color-scheme: dark) {
          .glassmorphism-container {
            background: var(--glass-bg-dark);
            border: 1px solid var(--glass-border-dark);
          }
          .glassmorphism-input {
            background: rgba(22, 22, 26, 0.5);
            border: 1px solid var(--glass-border-dark);
          }
          .glassmorphism-header {
            background: rgba(22, 22, 26, 0.85);
            border-bottom: 1px solid var(--glass-border-dark);
          }
          .glassmorphism-footer {
            background: rgba(22, 22, 26, 0.85);
            border-top: 1px solid var(--glass-border-dark);
          }
        }

        .animate-fade-in {
          animation: fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1);
        }

        @keyframes fadeIn {
          from { opacity: 0; transform: scale(0.98); }
          to { opacity: 1; transform: scale(1); }
        }
      `}} />
    </div>
  );
}
