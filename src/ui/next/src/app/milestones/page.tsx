"use client";

<<<<<<< HEAD
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

interface Milestone {
  id: string;
  title: string;
  description: string;
  reached: boolean;
}

=======
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

>>>>>>> 6448c02f (🛡️ Sentry: Fix SQLite & Postgres parity mismatch in schema definitions (#24827))
export default function MilestonesPage() {
  const router = useRouter();
  const [selectedMilestone, setSelectedMilestone] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
<<<<<<< HEAD
  const [milestones, setMilestones] = useState<Milestone[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [tenantId, setTenantId] = useState('DEFAULT');

  useEffect(() => {
    const tid = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'DEFAULT' : 'DEFAULT';
    setTenantId(tid);

    const fetchMilestones = async () => {
      try {
        const response = await fetch(`/api/v1/growth/milestones/check?tenant=${tid}`);
        const data = await response.json();
        if (data && data.milestones) {
          setMilestones(data.milestones);
          // Auto-select first unlocked milestone
          const firstUnlocked = data.milestones.find((m: Milestone) => m.reached);
          if (firstUnlocked) {
            setSelectedMilestone(firstUnlocked.id);
          }
        }
      } catch (e) {
        console.error("Failed to fetch milestones", e);
      } finally {
        setIsLoading(false);
      }
    };

    fetchMilestones();
  }, []);

  const getIcon = (id: string) => {
    switch (id) {
        case 'first_sale': return '🎉';
        case '10th_order': return '📈';
        case '100_visitors': return '🚀';
        case '5_referrals': return '🤝';
        case 'revenue_1k': return '💰';
        default: return '✨';
    }
  };
=======

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
>>>>>>> 6448c02f (🛡️ Sentry: Fix SQLite & Postgres parity mismatch in schema definitions (#24827))

  const shareTarget = typeof window !== 'undefined' ? `${window.location.origin}/onboarding?ref=milestone` : '/onboarding?ref=milestone';
  const shareText = `I just hit a huge business milestone using OHC! Launch your own store today: ${shareTarget}`;

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
<<<<<<< HEAD
                {isLoading ? (
                    [1,2,3].map(i => (
                        <div key={i} className="p-4 rounded-2xl animate-pulse bg-gray-200 h-24"></div>
                    ))
                ) : (
                    milestones.map((m) => (
                        <div
                            key={m.id}
                            onClick={() => m.reached && setSelectedMilestone(m.id)}
                            className={`p-4 rounded-2xl transition-all ${
                                m.reached
                                ? 'glassmorphism hover:border-indigo-300 hover:shadow-md cursor-pointer'
                                : 'glassmorphism opacity-60 cursor-not-allowed'
                            } ${selectedMilestone === m.id ? 'ring-2 ring-indigo-500 shadow-md' : ''}`}
                        >
                            <div className="flex items-center gap-4">
                                <div className={`w-12 h-12 rounded-full flex items-center justify-center text-2xl ${m.reached ? 'bg-indigo-50' : 'bg-gray-200 grayscale'}`}>
                                    {getIcon(m.id)}
                                </div>
                                <div className="flex-1">
                                    <h3 className="text-lg font-bold font-outfit text-gray-900">{m.title}</h3>
                                    <p className="text-sm text-gray-600">{m.description}</p>
                                    {!m.reached && <p className="text-xs text-gray-400 mt-1 text-center font-semibold uppercase tracking-widest pt-1 flex gap-2"><span className="text-gray-400">🔒</span>Locked</p>}
                                </div>
                            </div>
                        </div>
                    ))
                )}
=======
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
>>>>>>> 6448c02f (🛡️ Sentry: Fix SQLite & Postgres parity mismatch in schema definitions (#24827))
            </div>
        </section>

        {/* Milestone Share Card */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
             <div className="sticky top-24">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Share Your Success</h2>

                {selectedMilestone ? (() => {
                    const activeM = milestones.find(m => m.id === selectedMilestone);
                    if (!activeM) return null;
                    const cardUrl = `/api/v1/growth/milestone/card?milestone_id=${activeM.id}&tenant=${tenantId}`;

                    return (
                        <div className="flex flex-col gap-6">
                            {/* Preview Card */}
                            <div className="w-full aspect-[1200/630] rounded-3xl shadow-xl overflow-hidden relative border border-white/20">
                                <img
                                    src={cardUrl}
                                    alt={activeM.title}
                                    className="w-full h-full object-cover"
                                />
                            </div>

                            {/* Share Buttons */}
                            <div className="flex flex-col gap-3">
                                <button
<<<<<<< HEAD
                                    onClick={async () => {
                                        try {
                                            const response = await fetch(cardUrl);
                                            const blob = await response.blob();
                                            const url = window.URL.createObjectURL(blob);
                                            const a = document.createElement('a');
                                            a.href = url;
                                            a.download = `milestone-${activeM.id}.svg`;
                                            document.body.appendChild(a);
                                            a.click();
                                            window.URL.revokeObjectURL(url);
                                            document.body.removeChild(a);
                                        } catch (err) {
                                            console.error("Failed to download milestone card", err);
                                        }
                                    }}
                                    className="w-full py-3 rounded-xl text-sm font-bold transition-all shadow-sm bg-indigo-600 text-white hover:bg-indigo-700 flex items-center justify-center gap-2"
                                >
                                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                                    Download Achievement
                                </button>

                                <button
=======
>>>>>>> 6448c02f (🛡️ Sentry: Fix SQLite & Postgres parity mismatch in schema definitions (#24827))
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

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
