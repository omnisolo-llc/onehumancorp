"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function EmailSignatureGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('demo-store');
  const [hasPro, setHasPro] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    role: '',
    company: '',
    phone: '',
    website: '',
    bannerText: '',
  });
  const [generatedHtml, setGeneratedHtml] = useState('');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showUpgradeModal, setShowUpgradeModal] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setTenant(localStorage.getItem('tenant') || 'demo-store');
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleGenerate = async () => {
    try {
      const response = await fetch('/api/v1/growth/email-signature/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ...formData,
          tenantId: tenant,
          hasPro: hasPro,
          removeBranding: removeBranding && hasPro,
        }),
      });
      const data = await response.json();
      setGeneratedHtml(data.html);
    } catch (err) {
      console.error(err);
      setGeneratedHtml('<p>Error generating signature</p>');
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedHtml);
    alert('Signature copied to clipboard!');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-orange-50 via-red-50 to-pink-50">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Email Signature Generator ✉️</h1>
        <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        <div className="glassmorphism rounded-2xl p-6 md:p-8 shadow-sm">
          <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Build Your Signature</h2>
          <p className="text-gray-600 text-sm mb-6">Create a professional email signature and drive traffic to your store with every email you send.</p>

          <div className="flex flex-col gap-4">
            <input name="name" value={formData.name} onChange={handleChange} placeholder="Full Name (e.g. Maya Smith)" className="w-full p-3 rounded border" />
            <input name="role" value={formData.role} onChange={handleChange} placeholder="Role (e.g. Owner)" className="w-full p-3 rounded border" />
            <input name="company" value={formData.company} onChange={handleChange} placeholder="Company Name" className="w-full p-3 rounded border" />
            <input name="phone" value={formData.phone} onChange={handleChange} placeholder="Phone Number" className="w-full p-3 rounded border" />
            <input name="website" value={formData.website} onChange={handleChange} placeholder="Website URL (e.g. https://maya-cakes.ohc.app)" className="w-full p-3 rounded border" />
            <input name="bannerText" value={formData.bannerText} onChange={handleChange} placeholder="Banner Text (e.g. Book a consultation!)" className="w-full p-3 rounded border" />

            <div className="mt-4 flex items-center justify-between p-4 bg-gray-50 rounded-xl border border-gray-200">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={removeBranding}
                  onChange={(e) => {
                    if (!hasPro && e.target.checked) {
                      setShowUpgradeModal(true);
                      return;
                    }
                    setRemoveBranding(e.target.checked);
                  }}
                  className="w-5 h-5 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                />
                <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge</span>
              </label>
              {!hasPro && <span className="text-xs font-bold px-2 py-1 bg-gradient-to-r from-indigo-500 to-purple-500 text-white rounded-md">PRO</span>}
            </div>

            <button onClick={handleGenerate} className="mt-4 px-6 py-3 bg-black text-white font-bold rounded shadow hover:bg-gray-800 transition">
              Generate Signature
            </button>
          </div>
        </div>

        {generatedHtml && (
          <div className="glassmorphism rounded-2xl p-6 md:p-8 shadow-sm">
            <h3 className="text-lg font-bold font-outfit mb-4">Preview</h3>
            <div className="p-4 bg-white border rounded shadow-inner mb-4" dangerouslySetInnerHTML={{ __html: generatedHtml }} />

            <h3 className="text-lg font-bold font-outfit mb-2">HTML Source</h3>
            <textarea readOnly value={generatedHtml} className="w-full h-32 p-3 text-xs font-mono bg-gray-50 rounded border mb-4" />

            <button onClick={handleCopy} className="px-6 py-3 bg-indigo-600 text-white font-bold rounded shadow hover:bg-indigo-700 transition">
              Copy HTML to Clipboard
            </button>
          </div>
        )}
      </main>

      {showUpgradeModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Upgrade to Pro</h2>
            <p className="text-gray-600 text-sm mb-6">Make the Email Signature 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.</p>
            <div className="flex gap-3 justify-end">
              <button onClick={() => setShowUpgradeModal(false)} className="px-4 py-2 font-medium text-gray-600 hover:text-gray-900">Cancel</button>
              <button onClick={() => router.push('/upgrade-roi')} className="px-4 py-2 bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-bold rounded-lg shadow-md hover:shadow-lg transition-all">View Plans</button>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
        }
      `}} />
    </div>
  );
}
