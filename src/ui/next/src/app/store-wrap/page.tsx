"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function StoreWrapPage() {
  const router = useRouter();
  const [currentSlide, setCurrentSlide] = useState(0);
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('my-store');
  const [metrics, setMetrics] = useState({ sales: 0, customers: 0 });

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);

      const token = localStorage.getItem('token') || 'test-token';

      fetch('/api/v1/dashboard/metrics', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
        body: JSON.stringify({ tenant_id: storedTenant })
      })
      .then(res => res.json())
      .then(data => {
        setMetrics({
          sales: data.total_sales || 0,
          customers: data.active_customers || 0
        });
      })
      .catch(e => console.error("Failed to fetch wrap-up metrics", e));
    }
  }, []);

  const referralLink = `${typeof window !== 'undefined' ? window.location.origin : ''}/onboarding?ref=${tenant}`;
  const shareText = `My business just generated $${metrics.sales.toLocaleString()} in revenue this year! 🚀 Built and scaled on OHC. Start your own business today and get a $50 credit: ${referralLink}`;

  const slides = [
    {
      title: "Your Year in Review",
      subtitle: "Let's see how much you've grown",
      bg: "linear-gradient(135deg, #FF6B6B 0%, #FF8E53 100%)",
      emoji: "🌟",
      content: "You launched your dream store and it's been an incredible journey."
    },
    {
      title: `${metrics.customers}`,
      subtitle: "Happy Customers",
      bg: "linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)",
      emoji: "📦",
      content: "That's a lot of happy customers! You shipped to many different places."
    },
    {
      title: `$${metrics.sales.toLocaleString()}`,
      subtitle: "Total Revenue",
      bg: "linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)",
      emoji: "💰",
      content: "Your hard work is paying off. You're in the top 10% of new merchants!"
    },
    {
      title: "Share Your Success",
      subtitle: "Inspire others and earn rewards",
      bg: "linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)",
      emoji: "🚀",
      content: "Invite friends to OHC. When they launch, you both get $50 credit."
    }
  ];

  const nextSlide = () => {
    if (currentSlide < slides.length - 1) {
      setCurrentSlide(currentSlide + 1);
    }
  };

  const prevSlide = () => {
    if (currentSlide > 0) {
      setCurrentSlide(currentSlide - 1);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#1D1D1F] text-white overflow-hidden relative">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between z-50 absolute w-full top-0">
        <h1 className="text-xl font-bold font-outfit tracking-tight text-white/90">Store Wrap-Up 🎁</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-white/20 hover:bg-white/30 backdrop-blur-md rounded-full text-sm font-medium transition-colors border border-white/10"
        >
          Close
        </button>
      </header>

      {/* Progress Bars */}
      <div className="absolute top-16 left-0 w-full px-6 flex gap-2 z-50">
        {slides.map((_, i) => (
          <div key={i} className="h-1.5 flex-1 bg-white/20 rounded-full overflow-hidden">
            <div
              className="h-full bg-white rounded-full transition-all duration-300"
              style={{ width: i <= currentSlide ? '100%' : '0%' }}
            />
          </div>
        ))}
      </div>

      <main className="flex-1 w-full h-screen relative flex items-center justify-center">
        {slides.map((slide, i) => (
          <div
            key={i}
            aria-hidden={i !== currentSlide}
            className={`absolute inset-0 flex flex-col items-center justify-center p-8 transition-opacity duration-500 ease-in-out ${i === currentSlide ? 'opacity-100 z-10' : 'opacity-0 z-0 pointer-events-none'}`}
            style={{ background: slide.bg }}
          >
            <div className="max-w-md w-full flex flex-col items-center text-center">
              <div className="text-8xl mb-8 drop-shadow-lg animate-bounce">{slide.emoji}</div>
              <h2 className="text-5xl md:text-6xl font-black font-outfit mb-4 leading-tight drop-shadow-md">
                {slide.title}
              </h2>
              <h3 className="text-2xl font-bold mb-6 text-white/90 drop-shadow-sm">
                {slide.subtitle}
              </h3>
              <p className="text-lg font-medium text-white/80 leading-relaxed max-w-sm">
                {slide.content}
              </p>

              {/* Share actions on last slide */}
              {i === slides.length - 1 && (
                <div className="mt-12 w-full space-y-4 animate-fade-in">
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(shareText);
                      setCopied(true);
                      setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-4 rounded-2xl text-lg font-bold transition-all shadow-xl flex items-center justify-center gap-2 ${copied ? 'bg-green-400 text-green-900' : 'bg-white text-gray-900 hover:scale-105 active:scale-95'}`}
                  >
                    {copied ? 'Link Copied!' : 'Copy Invite Link'}
                  </button>

                  <div className="grid grid-cols-2 gap-4">
                    <a
                      href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center justify-center gap-2 bg-black/80 backdrop-blur-md text-white p-4 rounded-2xl font-bold shadow-lg hover:bg-black transition-all hover:scale-105 active:scale-95"
                    >
                      <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                      Post to X
                    </a>
                    <a
                      href={`https://wa.me/?text=${encodeURIComponent(shareText)}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center justify-center gap-2 bg-[#25D366]/90 backdrop-blur-md text-white p-4 rounded-2xl font-bold shadow-lg hover:bg-[#25D366] transition-all hover:scale-105 active:scale-95"
                    >
                      <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                      WhatsApp
                    </a>
                  </div>
                </div>
              )}
            </div>

            <div className="absolute bottom-8 text-white/60 text-sm font-semibold tracking-widest uppercase">
              Powered by OHC
            </div>
          </div>
        ))}

        {/* Navigation Overlays */}
        <button
          type="button"
          aria-label="Previous wrap slide"
          data-ui-overlay="true"
          onClick={prevSlide}
          disabled={currentSlide === 0}
          className="absolute inset-y-0 left-0 w-1/3 z-20 cursor-pointer disabled:cursor-default disabled:pointer-events-none"
        />
        <button
          type="button"
          aria-label="Next wrap slide"
          data-ui-overlay="true"
          onClick={nextSlide}
          disabled={currentSlide === slides.length - 1}
          className="absolute inset-y-0 right-0 w-2/3 z-20 cursor-pointer disabled:cursor-default disabled:pointer-events-none"
        />
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
      `}} />
    </div>
  );
}
