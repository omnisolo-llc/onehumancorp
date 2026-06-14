"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function DigitalBusinessCardPage() {
  const router = useRouter();

  // Form State
  const [name, setName] = useState('Jane Doe');
  const [title, setTitle] = useState('Founder & CEO');
  const [company, setCompany] = useState("Jane's Bakery");
  const [phone, setPhone] = useState('+1 (555) 123-4567');
  const [email, setEmail] = useState('jane@example.com');
  const [website, setWebsite] = useState('www.example.com');
  const [bio, setBio] = useState('We make the best cakes in town! Come visit us or order online.');

  // Settings State
  const [themeColor, setThemeColor] = useState('#0066FF');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('demo');
  const [shareLink, setShareLink] = useState('');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const tid = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'demo';
      setTenant(tid);
      setShareLink(`https://ohc.app/card/${tid}`);
    }
  }, []);

  const handleCopyLink = () => {
    if (navigator.clipboard) {
      navigator.clipboard.writeText(shareLink).then(() => {
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
      });
    }
  };

  const claimTrialExtension = () => {
    const referralUrl = `https://ohc.app/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just created a beautiful digital business card for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setShowSoftPaywall(false);
    setRemoveBranding(true);
  };

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white sticky top-0 z-50 shadow-sm">
         <h1 className="text-2xl font-bold font-outfit text-gray-900">Digital Business Card</h1>
         <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-lg text-sm font-medium transition-colors text-gray-700">
           Back to Dashboard
         </button>
      </header>

      <main className="p-6 md:p-8 flex-1 w-full max-w-7xl mx-auto flex flex-col lg:flex-row gap-8">

        {/* Editor Settings */}
        <section className="w-full lg:w-1/3 flex flex-col gap-6">
            <div className="bg-white p-6 rounded-2xl border border-gray-200 shadow-sm">
                <div className="flex items-center gap-3 mb-6">
                    <div className="w-10 h-10 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center text-xl">
                        📇
                    </div>
                    <div>
                        <h2 className="text-lg font-bold font-outfit text-gray-900">Card Details</h2>
                        <p className="text-xs text-gray-500">Customize your digital business card.</p>
                    </div>
                </div>

                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Full Name</label>
                        <input type="text" value={name} onChange={e => setName(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none" />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Job Title</label>
                        <input type="text" value={title} onChange={e => setTitle(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none" />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Company</label>
                        <input type="text" value={company} onChange={e => setCompany(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none" />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Phone</label>
                        <input type="text" value={phone} onChange={e => setPhone(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none" />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Email</label>
                        <input type="text" value={email} onChange={e => setEmail(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none" />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Website</label>
                        <input type="text" value={website} onChange={e => setWebsite(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none" />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Short Bio</label>
                        <textarea value={bio} onChange={e => setBio(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none resize-none" rows={3}></textarea>
                    </div>
                </div>
            </div>

            <div className="bg-white p-6 rounded-2xl border border-gray-200 shadow-sm">
                <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">Design & Branding</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme Color</label>
                        <div className="flex gap-3">
                            {['#0066FF', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#111827'].map(color => (
                                <button
                                    key={color}
                                    onClick={() => setThemeColor(color)}
                                    className={`w-8 h-8 rounded-full border-2 transition-transform hover:scale-110 ${themeColor === color ? 'border-gray-900 scale-110 shadow-md' : 'border-transparent'}`}
                                    style={{ backgroundColor: color }}
                                />
                            ))}
                        </div>
                    </div>

                    <label className="flex items-start gap-3 mt-4 cursor-pointer group">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => {
                                if (e.target.checked) {
                                    setShowSoftPaywall(true);
                                    setRemoveBranding(false);
                                } else {
                                    setRemoveBranding(false);
                                }
                            }}
                            className="mt-1 w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
                        />
                        <div>
                            <span className="text-sm font-medium text-gray-900">Remove "Powered by OHC" branding</span>
                            <p className="text-xs text-gray-500 mt-1">Requires Pro plan or higher.</p>
                        </div>
                    </label>
                </div>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full lg:w-2/3 flex flex-col items-center">
             <div className="w-full max-w-sm bg-white rounded-[40px] overflow-hidden border-[8px] border-gray-900 shadow-2xl relative pb-8">
                {/* Mobile notch mockup */}
                <div className="absolute top-0 inset-x-0 h-6 bg-gray-900 rounded-b-3xl w-40 mx-auto z-10"></div>

                {/* Header Background */}
                <div className="h-32 w-full relative" style={{ backgroundColor: themeColor, opacity: 0.9 }}>
                    <div className="absolute inset-0 bg-black/10"></div>
                </div>

                {/* Profile Picture */}
                <div className="flex justify-center -mt-16 relative z-10">
                    <div className="w-32 h-32 rounded-full border-4 border-white bg-gray-100 flex items-center justify-center text-4xl font-bold shadow-md" style={{ color: themeColor }}>
                        {name ? name.charAt(0).toUpperCase() : 'J'}
                    </div>
                </div>

                {/* Content */}
                <div className="px-6 pt-4 pb-6 text-center">
                    <h2 className="text-2xl font-bold text-gray-900 font-outfit mb-1">{name || 'Jane Doe'}</h2>
                    <p className="text-sm font-semibold mb-1" style={{ color: themeColor }}>{title || 'Job Title'}</p>
                    <p className="text-sm text-gray-500 mb-6">{company}</p>

                    <p className="text-sm text-gray-700 mb-8 leading-relaxed px-2">
                        {bio}
                    </p>

                    <div className="flex flex-col gap-3 mb-8">
                        {phone && (
                            <a href={`tel:${phone}`} className="flex items-center gap-3 p-3 rounded-xl bg-gray-50 hover:bg-gray-100 transition-colors border border-gray-100 text-gray-700 text-sm font-medium">
                                <span className="w-8 h-8 rounded-full flex items-center justify-center bg-white shadow-sm">📞</span>
                                {phone}
                            </a>
                        )}
                        {email && (
                            <a href={`mailto:${email}`} className="flex items-center gap-3 p-3 rounded-xl bg-gray-50 hover:bg-gray-100 transition-colors border border-gray-100 text-gray-700 text-sm font-medium">
                                <span className="w-8 h-8 rounded-full flex items-center justify-center bg-white shadow-sm">✉️</span>
                                {email}
                            </a>
                        )}
                        {website && (
                            <a href={`https://${website}`} target="_blank" rel="noreferrer" className="flex items-center gap-3 p-3 rounded-xl bg-gray-50 hover:bg-gray-100 transition-colors border border-gray-100 text-gray-700 text-sm font-medium">
                                <span className="w-8 h-8 rounded-full flex items-center justify-center bg-white shadow-sm">🌐</span>
                                {website}
                            </a>
                        )}
                    </div>

                    {!removeBranding && (
                        <div className="mt-8 pt-6 border-t border-gray-100">
                            <p className="text-xs text-gray-500 mb-2">Create your own digital card for free</p>
                            <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noreferrer" className="inline-block px-4 py-2 rounded-full bg-gray-900 text-white text-xs font-bold uppercase tracking-wider hover:bg-gray-800 transition-colors">
                                ⚡ Powered by OHC
                            </a>
                        </div>
                    )}
                </div>
             </div>

             <div className="mt-8 flex flex-col sm:flex-row gap-4 justify-center w-full max-w-sm">
                <button
                    onClick={handleCopyLink}
                    className="flex-1 py-3.5 px-6 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-xl transition-all shadow-md flex items-center justify-center gap-2"
                >
                    {copied ? 'Copied!' : 'Copy Share Link'}
                </button>
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-blue-100 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-blue-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make your Digital Business Card 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg bg-blue-600 hover:bg-blue-700"
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm bg-black text-white border-2 border-black hover:bg-gray-800 flex items-center justify-center gap-2"
            >
              Share on X to get 7 Days Free
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
