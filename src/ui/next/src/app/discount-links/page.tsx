"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function DiscountLinksPage() {
  const router = useRouter();
  const [shareUrl, setShareUrl] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleGenerate = async () => {
    setIsGenerating(true);
    try {
      const response = await fetch('/api/v1/growth/discount_share/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        }
      });
      if (response.ok) {
        const data = await response.json();
        setShareUrl(data.share_url);
      } else {
        alert('Failed to generate discount link.');
      }
    } catch (e) {
      console.error('Error generating discount link', e);
      alert('Error generating discount link.');
    }
    setIsGenerating(false);
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(shareUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Discount Share Links 💸</h1>
        <div className="flex items-center gap-3">
          <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
            Back to Dashboard
          </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-2xl mx-auto w-full flex flex-col gap-8">
        <section className="mb-6 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Generate a New Discount Link</h2>
          <p className="text-gray-600 mb-6">Create a unique link to share with potential customers. When they buy using this link, they get a special discount and you acquire a new customer!</p>

          <button
            onClick={handleGenerate}
            disabled={isGenerating}
            className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all ${isGenerating ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
          >
            {isGenerating ? 'Generating...' : 'Generate Magic Link'}
          </button>
        </section>

        {shareUrl && (
          <section className="p-6 shadow-sm flex flex-col items-center justify-center relative overflow-hidden" style={{ background: 'linear-gradient(135deg, #e0c3fc 0%, #8ec5fc 100%)', color: '#fff', borderRadius: '16px' }}>
            <h3 className="text-2xl font-bold font-outfit mb-4 text-gray-900">Your Link is Ready!</h3>

            <div className="flex w-full gap-2 mb-6">
              <input
                type="text"
                readOnly
                value={shareUrl}
                className="flex-1 px-4 py-2 text-gray-800 rounded-lg outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
              />
              <button
                onClick={copyToClipboard}
                className="px-4 py-2 bg-gray-900 text-white rounded-lg font-semibold hover:bg-black transition-colors whitespace-nowrap"
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
            </div>

            <div className="w-full flex flex-col items-center">
              <p className="text-sm font-semibold text-gray-800 uppercase tracking-wide mb-3">Share Instantly</p>
              <div className="flex gap-4 w-full">
                <a
                  href={`https://wa.me/?text=${encodeURIComponent(`Hey! I'm giving you an exclusive discount to my store. Use this link to claim it: ${shareUrl}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex-1 flex items-center justify-center gap-2 bg-[#25D366] text-white py-3 rounded-xl font-semibold shadow hover:bg-[#20bd5a] transition-colors"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Hey! I'm giving you an exclusive discount to my store. Use this link to claim it: ${shareUrl}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex-1 flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-semibold shadow hover:bg-gray-800 transition-colors"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                  X (Twitter)
                </a>
              </div>
            </div>
          </section>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}