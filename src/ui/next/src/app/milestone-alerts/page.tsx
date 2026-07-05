"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

interface Milestone {
  id: string;
  title: string;
  description: string;
  reached: boolean;
}

export default function MilestoneAlertsPage() {
  const router = useRouter();
  const [selectedMilestone, setSelectedMilestone] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
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

  const [shareTarget, setShareTarget] = useState('/onboarding?ref=milestone');
  useEffect(() => {
    if (typeof window !== 'undefined') {
      setShareTarget(`${window.location.origin}/onboarding?ref=milestone`);
    }
  }, []);

  const getShareText = () => {
    const activeM = milestones.find(m => m.id === selectedMilestone);
    const title = activeM ? activeM.title.replace('🎉 Milestone: ', '') : 'huge business milestone';
    return `I just hit a huge business milestone (🎉 Milestone: ${title}) using OHC! Launch your own store today: ${shareTarget} ⚡ Powered by OHC`;
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white backdrop-blur-[30px] saturate-[210%] border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Success Milestones 🏆</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8 backdrop-blur-[30px] saturate-[210%] bg-white shadow-sm border border-white/50 rounded-2xl">

        {/* Milestones List */}
        <section className="w-full md:w-1/2 flex flex-col gap-4">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Your Achievements</h2>
            <div className="flex flex-col gap-4">
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
                                    onError={(e) => { e.currentTarget.src = 'data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMjAwIiBoZWlnaHQ9IjYzMCI+PHJlY3Qgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgZmlsbD0iI2YxZjVmOSIvPjx0ZXh0IHg9IjUwJSIgeT0iNTAlIiBmb250LWZhbWlseT0ic2Fucy1zZXJpZiIgZm9udC1zaXplPSI0OCIgZmlsbD0iIzk0YTNiOSIgdGV4dC1hbmNob3I9Im1pZGRsZSI+Q291bGQgbm90IGxvYWQgbWlsZXN0b25lIGNhcmQ8L3RleHQ+PC9zdmc+'; }}
                                />
                            </div>

                            {/* Share Buttons */}
                            <div className="flex flex-col gap-3">
                                <button
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
                                    onClick={() => {
                                        navigator.clipboard.writeText(getShareText());
                                        setCopied(true);
                                        setTimeout(() => setCopied(false), 2000);
                                    }}
                                    className={`w-full py-3 rounded-xl text-sm font-bold transition-all shadow-sm ${copied ? 'bg-green-100 text-green-700' : 'glassmorphism text-gray-800 hover:brightness-95'}`}
                                >
                                    {copied ? 'Copied Message!' : 'Copy Share Message'}
                                </button>
                                <a
                                    href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(getShareText())}`}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="w-full flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-bold text-sm shadow-md hover:bg-gray-800 transition-all hover:-translate-y-0.5"
                                >
                                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                                    Share on X
                                </a>
                                <a
                                    href={`https://wa.me/?text=${encodeURIComponent(getShareText())}`}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="w-full flex items-center justify-center gap-2 bg-[#25D366]/80 text-white py-3 rounded-xl font-bold text-sm shadow-md hover:bg-[#20bd5a] transition-all hover:-translate-y-0.5"
                                >
                                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                                    Share to WhatsApp
                                </a>
                                <a
                                    href={`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(shareTarget)}&quote=${encodeURIComponent(getShareText())}`}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="w-full flex items-center justify-center gap-2 bg-[#1877F2]/80 text-white py-3 rounded-xl font-bold text-sm shadow-md hover:bg-[#166fe5] transition-all hover:-translate-y-0.5"
                                >
                                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
                                    Share on Facebook
                                </a>
                                <button
                                    onClick={() => router.push('/referrals?ref=milestone')}
                                    className="w-full py-3 rounded-xl text-sm font-bold transition-all shadow-sm bg-indigo-50 text-indigo-700 hover:bg-indigo-100 flex items-center justify-center gap-2 mt-2"
                                >
                                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
                                    Invite a friend and get a $50 credit
                                </button>
                            </div>

                            {/* Embed Generator UI */}
                            <div className="mt-6 bg-white rounded-2xl p-6 shadow-sm border border-gray-100 dark:bg-gray-800 dark:border-gray-700">
                                <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-2">Embed on your website</h3>
                                <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
                                    Show off your verified milestone directly on your storefront or blog to build customer trust.
                                </p>
                                <div className="relative">
                                    <textarea
                                        readOnly
                                        className="w-full h-32 p-3 text-sm font-mono text-gray-600 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 resize-none dark:bg-gray-900 dark:border-gray-700 dark:text-gray-400"
                                        value={`<a href="${window.location.origin}/onboarding?ref=${tenantId}&source=milestone_embed" target="_blank" rel="noopener noreferrer">
  <img src="${window.location.origin}${cardUrl}" alt="${activeM.title}" style="width: 100%; max-width: 600px; height: auto;" />
</a>`}
                                    />
                                    <button
                                        onClick={() => {
                                            const code = `<a href="${window.location.origin}/onboarding?ref=${tenantId}&source=milestone_embed" target="_blank" rel="noopener noreferrer">\n  <img src="${window.location.origin}${cardUrl}" alt="${activeM.title}" style="width: 100%; max-width: 600px; height: auto;" />\n</a>`;
                                            navigator.clipboard.writeText(code);
                                            setCopied(true);
                                            setTimeout(() => setCopied(false), 2000);
                                        }}
                                        className="absolute top-2 right-2 p-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-sm hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                                        title="Copy embed code"
                                    >
                                        <svg className="w-4 h-4 text-gray-600 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                                    </button>
                                </div>
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

      <PoweredByOHC tenantId={tenantId} />

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
