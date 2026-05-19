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
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
          <div className="p-8 flex flex-col flex-1 justify-center">
            <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Welcome to OHC Smart Builder</h1>
            <p className="text-gray-500 text-sm mb-8 leading-relaxed">
              Tell us about your business in a few words, and we'll magically generate your storefront in seconds.
            </p>

            <label className="text-sm font-semibold text-gray-700 mb-2 block">Your Business</label>
            <textarea
              className="w-full border border-gray-300 p-4 rounded-xl mb-6 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all resize-none text-gray-800"
              value={bio}
              onChange={(e) => setBio(e.target.value)}
              placeholder="e.g. I run a mobile dog grooming service in Portland"
              rows={4}
            />

            <button
              className={`w-full p-4 rounded-xl font-bold font-outfit text-lg transition-all ${
                bio.trim().length > 5
                  ? "bg-gray-900 text-white hover:bg-gray-800 shadow-md active:scale-[0.98]"
                  : "bg-gray-100 text-gray-400 cursor-not-allowed"
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
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col items-center justify-center border-x border-gray-200">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mb-6"></div>
          <p className="text-gray-600 font-medium animate-pulse text-center px-8">
            The Promoter is picking colors and building your menu...
          </p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col items-center justify-center border-x border-gray-200 text-center px-6 relative overflow-hidden">
          <div className="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-green-50 to-white -z-10" />
          <div className="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-6 shadow-sm">
            <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h1>
          <p className="text-gray-500 mb-8">Your automated storefront is successfully published and ready for customers.</p>

          <div className="w-full bg-gray-50 p-4 rounded-xl border border-gray-100 mb-8 flex items-center justify-between">
            <span className="text-sm text-gray-700 truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline">Copy</button>
          </div>

          <button
            className="w-full bg-gray-900 text-white font-bold p-4 rounded-xl shadow-md active:scale-[0.98]"
            onClick={() => setStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden">

        {/* Draft Preview Header */}
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <SmartBlock key={i} {...b} />
          ))}
        </div>

        {/* Bottom Action Bar */}
        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50">
          <div className="flex gap-3 mb-2">
            <button className="flex-1 py-2 text-sm font-medium text-gray-600 bg-gray-100 rounded-lg">Change Vibe</button>
            <button className="flex-1 py-2 text-sm font-medium text-gray-600 bg-gray-100 rounded-lg">Edit Text</button>
          </div>
          <button
            className="w-full bg-blue-600 text-white p-4 rounded-xl font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
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
        .glassmorphism { background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px) saturate(200%); -webkit-backdrop-filter: blur(20px) saturate(200%); border: 1px solid rgba(255, 255, 255, 0.2); }
      `}} />
    </div>
  );
}
