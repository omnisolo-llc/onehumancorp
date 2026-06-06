"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function DiscountSharePage() {
  const router = useRouter();
  const [shareUrl, setShareUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleGenerate = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/v1/growth/discount_share/generate', {
        method: 'POST',
      });
      if (response.ok) {
        const data = await response.json();
        setShareUrl(data.share_url);
      } else {
        console.error("Failed to generate discount share link");
      }
    } catch (error) {
      console.error("Error generating discount share link:", error);
    } finally {
      setLoading(false);
    }
  };

  const shareText = `Get an exclusive discount on my store using this link: ${shareUrl}\n\n⚡ Powered by OHC`;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Discount Share Promotion</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col items-center justify-center gap-8 text-center">
        <div className="mac-glass-container p-8 rounded-[24px] border border-white/40 shadow-xl w-full bg-white/10 backdrop-blur-2xl">
            <h2 className="text-2xl font-bold font-outfit mb-4 text-gray-900">Generate Your Promotional Link</h2>
            <p className="text-gray-600 mb-8 max-w-md mx-auto">Create a unique discount link to share with your audience and drive more sales to your storefront.</p>

            {!shareUrl ? (
                <button
                    onClick={handleGenerate}
                    disabled={loading}
                    className="bg-indigo-600 text-white font-semibold py-3 px-8 rounded-xl shadow-md hover:bg-indigo-700 transition-colors disabled:opacity-50"
                >
                    {loading ? 'Generating...' : 'Generate Promo Link'}
                </button>
            ) : (
                <div className="flex flex-col gap-6 w-full items-center">
                    <div className="w-full bg-white/50 border border-gray-200 p-6 rounded-xl flex flex-col gap-4 text-left">
                        <label className="text-sm font-semibold text-gray-700">Your Shareable Link & Message</label>
                        <pre className="text-sm text-gray-800 whitespace-pre-wrap font-sans bg-gray-50/50 p-4 rounded-lg border border-gray-100">
                            {shareText}
                        </pre>
                    </div>

                    <div className="flex gap-4 w-full">
                        <button
                            onClick={() => {
                                navigator.clipboard.writeText(shareText);
                                setCopied(true);
                                setTimeout(() => setCopied(false), 2000);
                            }}
                            className={`flex-1 py-3 rounded-xl text-sm font-bold transition-all shadow-sm ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-800 hover:bg-gray-200'}`}
                        >
                            {copied ? 'Copied!' : 'Copy to Clipboard'}
                        </button>
                        <a
                            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="flex-1 flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-bold text-sm shadow-md hover:bg-gray-800 transition-all"
                        >
                            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                            Share on X
                        </a>
                    </div>
                </div>
            )}
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