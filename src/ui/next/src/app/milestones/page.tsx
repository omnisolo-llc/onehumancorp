"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function MilestonesPage() {
  const router = useRouter();
  const [selectedMilestone, setSelectedMilestone] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('my-store');
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);

      const plan = localStorage.getItem('ohc_plan');
      if (plan === 'Pro' || plan === 'Business') {
        setHasPro(true);
      }
    }
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    } else {
      setRemoveBranding(e.target.checked);
    }
  };

  const milestones = [
    {
      id: "first-order",
      title: "First Order! 🎉",
      description: "You've officially made your first sale on OHC.",
      date: "Oct 12, 2023",
      unlocked: true,
      icon: "🎉"
    },
    {
      id: "10th-order",
      title: "10th Order Milestone",
      description: "Double digits! Your business is gaining momentum.",
      date: "Nov 05, 2023",
      unlocked: true,
      icon: "🚀"
    },
    {
      id: "100th-customer",
      title: "100th Customer",
      description: "A century of happy customers.",
      date: "Jan 22, 2024",
      unlocked: true,
      icon: "💯"
    },
    {
      id: "1000-revenue",
      title: "$1,000 Revenue",
      description: "Four figures in revenue! Keep up the great work.",
      date: null,
      unlocked: false,
      icon: "💰"
    }
  ];

  const shareText = `I just hit a huge business milestone using OHC! Launch your own store today: https://ohc.store/join?ref=${tenant}`;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Success Milestones 🏆</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Milestones List */}
        <section className="w-full md:w-1/2 flex flex-col gap-4">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Your Achievements</h2>
            <div className="flex flex-col gap-4">
                {milestones.map((m) => (
                    <div
                        key={m.id}
                        onClick={() => m.unlocked && setSelectedMilestone(m.id)}
                        className={`p-4 rounded-2xl transition-all ${
                            m.unlocked
                            ? 'glassmorphism hover:border-indigo-300 hover:shadow-md cursor-pointer'
                            : 'glassmorphism opacity-60 cursor-not-allowed'
                        } ${selectedMilestone === m.id ? 'ring-2 ring-indigo-500 shadow-md' : ''}`}
                    >
                        <div className="flex items-center gap-4">
                            <div className={`w-12 h-12 rounded-full flex items-center justify-center text-2xl ${m.unlocked ? 'bg-indigo-50' : 'bg-gray-200 grayscale'}`}>
                                {m.icon}
                            </div>
                            <div className="flex-1">
                                <h3 className="text-lg font-bold font-outfit text-gray-900">{m.title}</h3>
                                <p className="text-sm text-gray-600">{m.description}</p>
                                {m.unlocked && <p className="text-xs text-gray-400 mt-1">Achieved on {m.date}</p>}
                                {!m.unlocked && <p className="text-xs text-gray-400 mt-1 text-center font-semibold uppercase tracking-widest pt-1 flex gap-2"><span className="text-gray-400">🔒</span>Locked</p>}
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            <div className="mt-4 p-4 rounded-2xl glassmorphism border border-white/40">
              <h3 className="text-md font-semibold text-gray-800 mb-2">Branding</h3>
              <div className="flex items-center gap-2">
                <input
                    type="checkbox"
                    id="removeBranding"
                    checked={removeBranding}
                    onChange={handleRemoveBranding}
                    className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2">
                    Remove "Powered by OHC" Badge
                    {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                </label>
              </div>
            </div>
        </section>

        {/* Milestone Share Card */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
             <div className="sticky top-24">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Share Your Success</h2>

                {selectedMilestone ? (() => {
                    const activeM = milestones.find(m => m.id === selectedMilestone)!;
                    return (
                        <div className="flex flex-col gap-6">
                            {/* Preview Card */}
                            <div className="w-full aspect-square md:aspect-[4/3] rounded-3xl shadow-xl overflow-hidden relative flex flex-col items-center justify-center text-center p-8 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 text-white">
                                <div className="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full blur-3xl translate-x-1/4 -translate-y-1/4 pointer-events-none"></div>
                                <div className="absolute bottom-0 left-0 w-64 h-64 bg-black/10 rounded-full blur-3xl -translate-x-1/4 translate-y-1/4 pointer-events-none"></div>

                                <div className="z-10 flex flex-col items-center gap-4">
                                    <div className="w-24 h-24 bg-white/20 rounded-full flex items-center justify-center text-5xl backdrop-blur-md border border-white/30 shadow-inner">
                                        {activeM.icon}
                                    </div>
                                    <h3 className="text-4xl font-bold font-outfit mb-2 drop-shadow-md">{activeM.title}</h3>
                                    <p className="text-lg font-medium opacity-90 drop-shadow-sm max-w-[280px]">{activeM.description}</p>
                                </div>

                                {!removeBranding && (
                                  <div className="absolute bottom-6 flex flex-col items-center gap-1 opacity-90">
                                    <PoweredByOHC tenantId={tenant} className="mt-0 pb-0 opacity-80 mix-blend-overlay text-white border-white" />
                                  </div>
                                )}
                            </div>

                            {/* Share Buttons */}
                            <div className="flex flex-col gap-3">
                                <button
                                    onClick={() => {
                                        navigator.clipboard.writeText(shareText);
                                        setCopied(true);
                                        setTimeout(() => setCopied(false), 2000);
                                    }}
                                    className={`w-full py-3 rounded-xl text-sm font-bold transition-all shadow-sm ${copied ? 'bg-green-100 text-green-700' : 'glassmorphism text-gray-800 hover:brightness-95'}`}
                                >
                                    {copied ? 'Copied Message!' : 'Copy Share Message'}
                                </button>
                                <a
                                    href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="w-full flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-bold text-sm shadow-md hover:bg-gray-800 transition-all hover:-translate-y-0.5"
                                >
                                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                                    Share on X
                                </a>
                            </div>
                        </div>
                    );
                })() : (
                    <div className="w-full aspect-square md:aspect-[4/3] rounded-3xl border-2 border-dashed border-gray-300 flex flex-col items-center justify-center text-gray-400 glassmorphism">
                        <span className="text-4xl mb-4">🏆</span>
                        <p className="font-medium text-sm">Select an unlocked milestone</p>
                    </div>
                )}
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold">
              PRO
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Remove Branding</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make the milestones 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
