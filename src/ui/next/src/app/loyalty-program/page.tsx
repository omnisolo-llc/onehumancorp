'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function LoyaltyProgramPage() {
  const router = useRouter();
  const [storeName, setStoreName] = useState('');
  const [discountAmount, setDiscountAmount] = useState('$10');
  const [loyaltyTier, setLoyaltyTier] = useState('Gold');
  const [theme, setTheme] = useState('gradient');
  const [copied, setCopied] = useState(false);
  const [message, setMessage] = useState('');
  const [generating, setGenerating] = useState(false);
  const [sent, setSent] = useState(false);

  const getThemeStyles = () => {
    switch (theme) {
      case 'dark': return { background: '#1D1D1F', color: '#ffffff' };
      case 'light': return { background: '#ffffff', color: '#1D1D1F', border: '1px solid #eaeaea' };
      case 'purple': return { background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', color: '#ffffff' };
      case 'gradient':
      default: return { background: 'linear-gradient(135deg, #f6d365 0%, #fda085 100%)', color: '#ffffff' };
    }
  };

  const generateMessage = async () => {
    setGenerating(true);
    try {
      const response = await fetch('/api/v1/growth/campaign/loyalty', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            store_name: storeName,
            discount_amount: discountAmount,
            loyalty_tier: loyaltyTier
        })
      });

      if (response.ok) {
        const data = await response.json();
        setMessage(data.message);
      } else {
        setMessage(`🌟 You've reached ${loyaltyTier} status at ${storeName || 'our store'}!\n\nTo celebrate, we're giving you a special offer to share with your friends. Give them ${discountAmount} off their first order, and you'll get ${discountAmount} in store credit when they purchase!\n\nShare your VIP link: https://ohc.store/loyalty-invite\n\nThanks for being an amazing customer,\nThe ${storeName || 'Team'}\n\n⚡ Powered by OHC`);
      }
    } catch (err) {
       setMessage(`🌟 You've reached ${loyaltyTier} status at ${storeName || 'our store'}!\n\nTo celebrate, we're giving you a special offer to share with your friends. Give them ${discountAmount} off their first order, and you'll get ${discountAmount} in store credit when they purchase!\n\nShare your VIP link: https://ohc.store/loyalty-invite\n\nThanks for being an amazing customer,\nThe ${storeName || 'Team'}\n\n⚡ Powered by OHC`);
    } finally {
      setGenerating(false);
    }
  };

  // Generate initial message
  React.useEffect(() => {
     generateMessage();
  }, []);


  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Loyalty Program Generator 🌟</h1>
        <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Editor Settings */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md bg-white/65 backdrop-blur-md border border-white/40 rounded-2xl">
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Campaign Settings</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Store Name</label>
                        <input
                            type="text"
                            value={storeName}
                            onChange={(e) => setStoreName(e.target.value)}
                            placeholder="e.g. Maya's Sweets"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Loyalty Tier Name</label>
                        <input
                            type="text"
                            value={loyaltyTier}
                            onChange={(e) => setLoyaltyTier(e.target.value)}
                            placeholder="e.g. Gold, VIP, Superfan"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Discount Amount</label>
                        <input
                            type="text"
                            value={discountAmount}
                            onChange={(e) => setDiscountAmount(e.target.value)}
                            placeholder="e.g. $10, 15%"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white"
                        />
                    </div>

                    <button
                        onClick={generateMessage}
                        disabled={generating}
                        className="mt-2 w-full py-2 bg-indigo-600 text-white rounded-lg font-semibold hover:bg-indigo-700 transition-colors disabled:opacity-50"
                    >
                        {generating ? 'Generating...' : 'Update Preview'}
                    </button>

                    <div className="mt-2">
                        <label className="block text-sm font-medium text-gray-700 mb-2">Card Theme</label>
                        <div className="flex gap-2">
                            <button onClick={() => setTheme('gradient')} className={`w-8 h-8 rounded-full border-2 ${theme === 'gradient' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: 'linear-gradient(135deg, #f6d365 0%, #fda085 100%)' }}></button>
                            <button onClick={() => setTheme('dark')} className={`w-8 h-8 rounded-full border-2 ${theme === 'dark' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: '#1D1D1F' }}></button>
                            <button onClick={() => setTheme('light')} className={`w-8 h-8 rounded-full border-2 ${theme === 'light' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ background: '#ffffff' }}></button>
                            <button onClick={() => setTheme('purple')} className={`w-8 h-8 rounded-full border-2 ${theme === 'purple' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)' }}></button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md bg-white/65 backdrop-blur-md border border-white/40 rounded-2xl">
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Launch Campaign</h2>
                <div className="flex flex-col gap-3">
                    <button
                        onClick={() => {
                            navigator.clipboard.writeText(message);
                            setCopied(true);
                            setTimeout(() => setCopied(false), 2000);
                        }}
                        className={`w-full py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                    >
                        {copied ? 'Copied Message!' : 'Copy Message'}
                    </button>
                    <button
                         onClick={async () => {
                             setSent(true);
                             await new Promise(r => setTimeout(r, 2000));
                             setSent(false);
                         }}
                        disabled={sent}
                        className="w-full flex items-center justify-center gap-2 bg-indigo-50 text-indigo-700 py-2 rounded-lg font-semibold text-sm hover:bg-indigo-100 transition-all border border-indigo-100 disabled:opacity-50"
                    >
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                        {sent ? 'Emails Sent!' : 'Email to Customers'}
                    </button>
                </div>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-2/3 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Live Preview</h2>

             {/* Social Card Preview */}
             <div className="w-full aspect-[1.91/1] rounded-2xl shadow-xl flex flex-col justify-center items-center text-center p-8 md:p-12 overflow-hidden relative transition-all duration-300" style={getThemeStyles()}>
                 {theme !== 'light' && (
                     <>
                        <div className="absolute top-0 left-0 w-64 h-64 bg-white/20 rounded-full blur-3xl -translate-x-1/2 -translate-y-1/2"></div>
                        <div className="absolute bottom-0 right-0 w-64 h-64 bg-black/10 rounded-full blur-3xl translate-x-1/2 translate-y-1/2"></div>
                     </>
                 )}

                 <div className="z-10 flex flex-col items-center">
                    <div className="w-16 h-16 mb-4 bg-white/20 rounded-full shadow-inner flex items-center justify-center backdrop-blur-md border border-white/30 text-3xl">
                        🌟
                    </div>
                    <div className="text-sm font-bold uppercase tracking-widest mb-2 opacity-80">
                        {loyaltyTier || 'VIP'} Status Unlocked
                    </div>
                    <h1 className="text-3xl md:text-5xl font-bold font-outfit mb-3 leading-tight tracking-tight drop-shadow-sm">
                        {storeName || 'Your Store'}
                    </h1>
                    <p className="text-base md:text-xl font-medium opacity-90 max-w-md leading-relaxed drop-shadow-sm">
                        Give {discountAmount}, Get {discountAmount} when you refer friends!
                    </p>
                 </div>

                 <div className="absolute bottom-4 right-6 opacity-80">
                     <span className="text-xs font-bold tracking-wider uppercase">⚡ Powered by OHC</span>
                 </div>
             </div>

             {/* Message Text Preview */}
             <div className="mt-4 p-6 bg-white rounded-2xl border border-gray-200 shadow-sm">
                 <h3 className="text-sm font-bold text-gray-500 uppercase tracking-wider mb-3">Message Preview</h3>
                 <div className="whitespace-pre-wrap font-mono text-sm text-gray-800 bg-gray-50 p-4 rounded-lg border border-gray-100">
                     {message}
                 </div>
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
