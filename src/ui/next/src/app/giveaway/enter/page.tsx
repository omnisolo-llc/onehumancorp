"use client";

import React, { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function GiveawayEnterContent() {
  const searchParams = useSearchParams();
  const tenant = searchParams.get('tenant') || 'my-store';
  const title = searchParams.get('title') || 'Enter to Win!';
  const description = searchParams.get('description') || 'Enter your email below for a chance to win. Plus, get bonus entries when you share with friends!';
  const showBranding = searchParams.get('branding') !== 'false';

  const [email, setEmail] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isEntered, setIsEntered] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    // Simulate API call
    setTimeout(() => {
      setIsSubmitting(false);
      setIsEntered(true);
    }, 1000);
  };

  const shareLink = typeof window !== 'undefined' ? `${window.location.origin}/giveaway/enter?tenant=${encodeURIComponent(tenant)}&title=${encodeURIComponent(title)}&ref=user123` : '';
  const shareText = `I just entered to win: ${title}. You should enter too! ${shareLink}`;

  return (
    <div className="min-h-screen font-inter flex flex-col" style={{ backgroundColor: '#F5F5F7' }}>
      <main className="flex-1 flex flex-col items-center justify-center p-4">
        <div className="w-full max-w-md bg-white rounded-3xl shadow-2xl overflow-hidden relative border border-gray-200 flex flex-col items-center">
            <div className="w-full h-32 bg-gradient-to-r from-purple-500 to-pink-500 relative flex items-center justify-center">
                <span className="text-5xl drop-shadow-md">🎁</span>
            </div>

            <div className="w-full p-8 flex flex-col items-center text-center">
                <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                    {title}
                </h1>

                {!isEntered ? (
                  <>
                    <p className="text-sm text-gray-600 mb-6 leading-relaxed">
                        {description}
                    </p>
                    <form onSubmit={handleSubmit} className="w-full space-y-4">
                        <input
                          type="email"
                          required
                          value={email}
                          onChange={(e) => setEmail(e.target.value)}
                          placeholder="Enter your email"
                          className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-purple-500"
                        />
                        <button
                          type="submit"
                          disabled={isSubmitting || !email}
                          className={`w-full py-3 text-white font-bold rounded-xl shadow-md transition-all ${isSubmitting || !email ? 'bg-purple-400 cursor-not-allowed' : 'bg-purple-600 hover:bg-purple-700 hover:-translate-y-0.5'}`}
                        >
                          {isSubmitting ? 'Entering...' : 'Enter Giveaway'}
                        </button>
                    </form>
                  </>
                ) : (
                  <div className="w-full animate-fade-in-up">
                    <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center text-3xl mx-auto mb-4">
                      🎉
                    </div>
                    <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">You're Entered!</h2>
                    <p className="text-sm text-gray-600 mb-6">
                      Want to increase your chances? Get <strong className="text-gray-900">3 bonus entries</strong> for every friend who enters using your unique link below!
                    </p>

                    <div className="flex flex-col gap-3">
                      <div className="flex gap-2">
                        <input
                          type="text"
                          readOnly
                          value={shareLink}
                          className="flex-1 px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm text-gray-700 focus:outline-none"
                        />
                        <button
                          onClick={() => {
                            navigator.clipboard.writeText(shareLink);
                            setCopied(true);
                            setTimeout(() => setCopied(false), 2000);
                          }}
                          className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                        >
                          {copied ? 'Copied!' : 'Copy'}
                        </button>
                      </div>

                      <div className="relative py-2">
                        <div className="absolute inset-0 flex items-center"><div className="w-full border-t border-gray-200"></div></div>
                        <div className="relative flex justify-center"><span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold">Or share via</span></div>
                      </div>

                      <div className="flex gap-2">
                         <a
                            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="flex-1 flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-bold text-sm shadow-sm hover:bg-gray-800 transition-all"
                         >
                            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                            X
                         </a>
                         <a
                            href={`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(shareLink)}&quote=${encodeURIComponent(shareText)}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="flex-1 flex items-center justify-center gap-2 bg-[#1877F2]/80 text-white py-3 rounded-xl font-bold text-sm shadow-sm hover:bg-[#166fe5] transition-all"
                         >
                            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
                            Facebook
                         </a>
                      </div>
                    </div>
                  </div>
                )}

                {showBranding && (
                <div className="mt-8">
                  <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="text-xs font-semibold text-gray-400 uppercase tracking-widest hover:text-gray-600 transition-colors">⚡ Powered by OHC</a>
                </div>
                )}
            </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fade-in-up {
          0% { opacity: 0; transform: translateY(10px); }
          100% { opacity: 1; transform: translateY(0); }
        }
        .animate-fade-in-up { animation: fade-in-up 0.4s ease-out forwards; }
      `}} />
    </div>
  );
}

export default function GiveawayEnterPage() {
  return (
    <Suspense fallback={<div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center">Loading...</div>}>
      <GiveawayEnterContent />
    </Suspense>
  );
}
