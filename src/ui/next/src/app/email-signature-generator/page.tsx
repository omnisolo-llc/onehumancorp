'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function EmailSignatureGeneratorPage() {
  const router = useRouter();

  // Form State
  const [name, setName] = useState('Jane Doe');
  const [title, setTitle] = useState('Founder & CEO');
  const [company, setCompany] = useState("Jane's Bakery");
  const [phone, setPhone] = useState('+1 (555) 123-4567');
  const [email, setEmail] = useState('jane@example.com');
  const [website, setWebsite] = useState('www.example.com');

  // Settings State
  const [themeColor, setThemeColor] = useState('#0066FF');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('demo');

  React.useEffect(() => {
    const tid = localStorage.getItem('tenant_id') || localStorage.getItem('tenant');
    if (tid) setTenant(tid);
  }, []);

  const handleCopy = () => {
    const signatureHtml = document.getElementById('signature-preview')?.innerHTML;
    if (signatureHtml) {
      const blob = new Blob([signatureHtml], { type: 'text/html' });
      const data = [new ClipboardItem({ 'text/html': blob })];
      navigator.clipboard.write(data).then(() => {
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
      }).catch(err => {
        console.error('Failed to copy: ', err);
        // Fallback for some browsers
        navigator.clipboard.writeText(signatureHtml);
      });
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#121212] p-4 md:p-8 font-inter">
      <header className="max-w-6xl mx-auto mb-8">
         <h1 className="text-3xl md:text-4xl font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Free Email Signature Generator</h1>
         <p className="text-gray-600 dark:text-gray-400 mt-2">Create a professional, beautifully branded email signature in seconds.</p>
      </header>

      <main className="max-w-6xl mx-auto flex flex-col lg:flex-row gap-8">

        {/* Builder Panel */}
        <section className="w-full lg:w-1/3 flex flex-col gap-6">

            <div className="p-6 rounded-[24px] bg-white dark:bg-[#1E1E1E] border border-gray-200 dark:border-gray-800 shadow-sm">
                <h3 className="font-semibold text-gray-900 dark:text-white mb-4">Your Details</h3>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Full Name</label>
                        <input
                            type="text"
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-black text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] outline-none transition-all"
                            placeholder="e.g. Jane Doe"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Job Title</label>
                        <input
                            type="text"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-black text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] outline-none transition-all"
                            placeholder="e.g. Founder & CEO"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Company</label>
                        <input
                            type="text"
                            value={company}
                            onChange={(e) => setCompany(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-black text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] outline-none transition-all"
                            placeholder="e.g. Jane's Bakery"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Phone Number</label>
                        <input
                            type="text"
                            value={phone}
                            onChange={(e) => setPhone(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-black text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] outline-none transition-all"
                            placeholder="e.g. +1 (555) 123-4567"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email Address</label>
                        <input
                            type="email"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-black text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] outline-none transition-all"
                            placeholder="e.g. jane@example.com"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Website URL</label>
                        <input
                            type="text"
                            value={website}
                            onChange={(e) => setWebsite(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-black text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] outline-none transition-all"
                            placeholder="e.g. www.example.com"
                        />
                    </div>
                </div>
            </div>

            <div className="p-6 rounded-[24px] bg-white dark:bg-[#1E1E1E] border border-gray-200 dark:border-gray-800 shadow-sm">
                <h3 className="font-semibold text-gray-900 dark:text-white mb-4">Design</h3>
                <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Theme Color</label>
                    <div className="flex gap-3">
                        {['#0066FF', '#34C759', '#FF9500', '#FF3B30', '#8A2BE2', '#1D1D1F'].map(color => (
                            <button
                                key={color}
                                onClick={() => setThemeColor(color)}
                                className={`w-8 h-8 rounded-full border-2 transition-transform ${themeColor === color ? 'border-gray-400 scale-110' : 'border-transparent hover:scale-105'}`}
                                style={{ backgroundColor: color }}
                                aria-label={`Select color ${color}`}
                            />
                        ))}
                    </div>
                </div>

                <div className="mt-6 border-t border-gray-100 dark:border-gray-800 pt-6">
                    <label className="flex items-start gap-3 cursor-pointer group">
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
                            className="mt-1 w-4 h-4 text-[#0071E3] rounded focus:ring-[#0066FF]"
                        />
                        <div>
                            <span className="text-sm font-medium text-gray-900 dark:text-gray-200">Remove "Powered by OHC" branding</span>
                            <p className="text-xs text-gray-500 mt-1">Requires Pro plan or higher.</p>
                        </div>
                    </label>
                </div>
            </div>

        </section>

        {/* Live Preview & Action */}
        <section className="w-full lg:w-2/3 flex flex-col gap-6 items-center">

             <div className="w-full bg-white dark:bg-black rounded-[24px] overflow-hidden border border-gray-200 dark:border-gray-800 shadow-lg relative p-8">
                <h2 className="text-xl font-semibold font-outfit text-gray-900 dark:text-white mb-6 border-b border-gray-100 dark:border-gray-800 pb-4">Live Preview</h2>

                 {/* The Actual Signature */}
                 <div className="bg-white p-8 rounded-xl border border-gray-100 shadow-sm flex items-center justify-center min-h-[300px]">
                    <div id="signature-preview" style={{ fontFamily: 'Arial, sans-serif', color: '#1D1D1F', maxWidth: '500px' }}>
                        <table cellPadding="0" cellSpacing="0" border={0} style={{ margin: 0, padding: 0 }}>
                            <tbody>
                                <tr>
                                    <td style={{ paddingRight: '20px', borderRight: `2px solid ${themeColor}` }}>
                                        <div style={{ width: '80px', height: '80px', borderRadius: '50%', backgroundColor: '#f3f4f6', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '32px', color: themeColor, fontWeight: 'bold', border: `2px solid ${themeColor}` }}>
                                            {name ? name.charAt(0).toUpperCase() : 'J'}
                                        </div>
                                    </td>
                                    <td style={{ paddingLeft: '20px' }}>
                                        <h2 style={{ margin: '0 0 4px 0', fontSize: '18px', fontWeight: 'bold', color: '#1D1D1F' }}>{name || 'Jane Doe'}</h2>
                                        <p style={{ margin: '0 0 12px 0', fontSize: '14px', color: '#6b7280' }}>
                                            <span style={{ fontWeight: 600, color: themeColor }}>{title || 'Job Title'}</span>
                                            {company && <span> | {company}</span>}
                                        </p>

                                        <table cellPadding="0" cellSpacing="0" border={0}>
                                            <tbody>
                                                {phone && (
                                                    <tr>
                                                        <td style={{ paddingBottom: '4px', fontSize: '13px', color: '#4b5563' }}>
                                                            <strong style={{ color: themeColor, marginRight: '4px' }}>P:</strong> {phone}
                                                        </td>
                                                    </tr>
                                                )}
                                                {email && (
                                                    <tr>
                                                        <td style={{ paddingBottom: '4px', fontSize: '13px', color: '#4b5563' }}>
                                                            <strong style={{ color: themeColor, marginRight: '4px' }}>E:</strong> <a href={`mailto:${email}`} style={{ color: '#4b5563', textDecoration: 'none' }}>{email}</a>
                                                        </td>
                                                    </tr>
                                                )}
                                                {website && (
                                                    <tr>
                                                        <td style={{ paddingBottom: '0px', fontSize: '13px', color: '#4b5563' }}>
                                                            <strong style={{ color: themeColor, marginRight: '4px' }}>W:</strong> <a href={`https://${website}`} target="_blank" rel="noreferrer" style={{ color: '#4b5563', textDecoration: 'none' }}>{website}</a>
                                                        </td>
                                                    </tr>
                                                )}
                                            </tbody>
                                        </table>
                                    </td>
                                </tr>
                                {!removeBranding && (
                                    <tr>
                                        <td colSpan={2} style={{ paddingTop: '16px' }}>
                                            <div style={{ borderTop: '1px solid #e5e7eb', paddingTop: '12px', fontSize: '11px', color: '#9ca3af' }}>
                                                Create your own free signature with <a href={`https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noreferrer" style={{ color: '#0066FF', textDecoration: 'none', fontWeight: 600 }}>One Human Corp</a>.
                                            </div>
                                        </td>
                                    </tr>
                                )}
                            </tbody>
                        </table>
                    </div>
                 </div>

                 <div className="mt-8 flex justify-center">
                    <button
                        onClick={handleCopy}
                        className="px-8 py-4 bg-[#0066FF] hover:bg-blue-700 text-white font-medium rounded-xl transition-colors shadow-sm flex items-center justify-center gap-2 text-lg w-full max-w-md"
                    >
                        {copied ? (
                            <>
                                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                                Copied to Clipboard!
                            </>
                        ) : (
                            <>
                                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                                Copy Signature HTML
                            </>
                        )}
                    </button>
                 </div>
                 <p className="text-center text-sm text-gray-500 mt-4">
                     Paste directly into Gmail, Outlook, Apple Mail, or your favorite email client.
                 </p>
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4">
          <div className="bg-white dark:bg-[#1E1E1E] w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 dark:bg-blue-900/20 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-[#0071E3] mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 dark:text-gray-400 mb-6 text-sm leading-relaxed">
              Make the Email Signature 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-[#0071E3] hover:bg-blue-700"
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
