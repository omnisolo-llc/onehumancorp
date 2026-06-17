"use client";

import React, { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function UnlockContent() {
  const searchParams = useSearchParams();
  const tenant = searchParams.get('tenant') || 'my-store';
  const campaignTitle = searchParams.get('title') || 'Secret Deal';
  const reward = searchParams.get('reward') || '20% Off';
  const hiddenCode = searchParams.get('code') || 'SECRET';
  const shareMessage = searchParams.get('msg') || 'I just unlocked a secret discount!';
  const theme = searchParams.get('theme') || 'light';

  const [isUnlocked, setIsUnlocked] = useState(false);
  const [copied, setCopied] = useState(false);

  const encodedMessage = encodeURIComponent(`${shareMessage} https://ohc.app/unlock?tenant=${tenant}&title=${encodeURIComponent(campaignTitle)}&reward=${encodeURIComponent(reward)}&code=${encodeURIComponent(hiddenCode)}&msg=${encodeURIComponent(shareMessage)}`);

  const handleShareX = () => {
    window.open(`https://twitter.com/intent/tweet?text=${encodedMessage}`, '_blank', 'width=550,height=420');
    // Simulate verification (in a real app, this could be more robust, but window.open works for the simple viral loop)
    setTimeout(() => setIsUnlocked(true), 1500);
  };

  const handleShareWhatsApp = () => {
    window.open(`https://wa.me/?text=${encodedMessage}`, '_blank');
    setTimeout(() => setIsUnlocked(true), 1500);
  };

  const handleCopyCode = () => {
    navigator.clipboard.writeText(hiddenCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    return theme === 'light'
        ? { background: '#ffffff', color: '#1f2937', borderColor: '#e5e7eb' }
        : { background: '#111827', color: '#f9fafb', borderColor: '#374151' };
  };

  return (
    <div className="flex flex-col min-h-screen items-center justify-center font-inter p-4" style={{ backgroundColor: theme === 'light' ? '#f3f4f6' : '#000000' }}>
        <div
            className="w-full max-w-md rounded-[24px] shadow-2xl p-8 flex flex-col items-center relative overflow-hidden transition-all duration-300"
            style={getThemeStyles()}
        >
            <div className="w-16 h-16 bg-purple-100 text-purple-600 rounded-full flex items-center justify-center text-3xl mb-6 shadow-inner">
                {isUnlocked ? '🎉' : '🎁'}
            </div>

            <h1 className="text-2xl font-bold font-outfit text-center mb-2" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>
                {campaignTitle}
            </h1>

            <p className="text-center text-sm mb-8" style={{ color: theme === 'dark' ? '#9ca3af' : '#4b5563' }}>
                {isUnlocked ? 'Congratulations! Here is your reward:' : 'Unlock your special reward:'} <br/>
                <strong className="text-purple-500 text-lg mt-1 block">{reward}</strong>
            </p>

            <div className="w-full p-6 rounded-xl border-2 flex flex-col items-center justify-center mb-8 relative transition-all"
                 style={{
                     borderColor: theme === 'dark' ? (isUnlocked ? '#8b5cf6' : '#4b5563') : (isUnlocked ? '#8b5cf6' : '#d1d5db'),
                     background: theme === 'dark' ? (isUnlocked ? '#2e1065' : '#1f2937') : (isUnlocked ? '#f5f3ff' : '#f3f4f6'),
                     borderStyle: isUnlocked ? 'solid' : 'dashed'
                 }}>

                 <span className={`font-mono text-2xl tracking-widest font-bold select-none transition-all duration-1000 ${isUnlocked ? 'blur-none opacity-100' : 'filter blur-md opacity-40'}`} style={{ color: theme === 'dark' ? (isUnlocked ? '#c4b5fd' : '#fff') : (isUnlocked ? '#6d28d9' : '#000') }}>
                     {hiddenCode}
                 </span>

                 {!isUnlocked && (
                     <div className="absolute inset-0 flex items-center justify-center">
                         <span className="text-xs font-bold uppercase tracking-wider px-3 py-1 bg-gray-900 text-white rounded-full shadow-lg flex items-center gap-1">
                             <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
                             Locked
                         </span>
                     </div>
                 )}
            </div>

            {!isUnlocked ? (
                <div className="w-full space-y-3 mb-6 animate-fade-in">
                    <p className="text-center text-xs font-medium uppercase tracking-wider mb-4 opacity-70">Share to reveal code</p>
                    <button
                        onClick={handleShareX}
                        className="w-full py-3 px-4 bg-black hover:bg-gray-800 text-white font-medium rounded-xl transition-colors flex items-center justify-center gap-2"
                    >
                        Share on X
                    </button>
                    <button
                        onClick={handleShareWhatsApp}
                        className="w-full py-3 px-4 bg-[#25D366] hover:bg-[#128C7E] text-white font-medium rounded-xl transition-colors flex items-center justify-center gap-2"
                    >
                        Share on WhatsApp
                    </button>
                </div>
            ) : (
                <div className="w-full space-y-3 mb-6 animate-fade-in">
                    <button
                        onClick={handleCopyCode}
                        className="w-full py-3 px-4 bg-purple-600 hover:bg-purple-700 text-white font-bold rounded-xl transition-colors flex items-center justify-center gap-2 shadow-md"
                    >
                        {copied ? 'Code Copied!' : 'Copy Code'}
                    </button>
                </div>
            )}

            <div className="mt-4 pt-4 border-t w-full text-center" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="text-xs font-semibold tracking-wide hover:underline opacity-70 hover:opacity-100 transition-opacity" style={{ color: '#6b7280' }}>
                    ⚡ Powered by OHC
                </a>
            </div>
        </div>

        <style dangerouslySetInnerHTML={{__html: `
            @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
            .font-inter { font-family: 'Inter', sans-serif; }
            .font-outfit { font-family: 'Outfit', sans-serif; }
            @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
            .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
        `}} />
    </div>
  );
}

export default function UnlockPage() {
    return (
        <Suspense fallback={<div className="min-h-screen flex items-center justify-center font-inter">Loading...</div>}>
            <UnlockContent />
        </Suspense>
    );
}
