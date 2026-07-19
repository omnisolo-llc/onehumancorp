"use client";

import React, { useState, useEffect } from 'react';
import { useProPlan } from '../components/useProPlan';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ProjectShowcasePage() {
  const router = useRouter();
  const [projectName, setProjectName] = useState('');
  const [customerName, setCustomerName] = useState('');
  const [description, setDescription] = useState('');
  const [beforeImage, setBeforeImage] = useState('');
  const [afterImage, setAfterImage] = useState('');
  const [ctaLink, setCtaLink] = useState('');

  const [removeBranding, setRemoveBranding] = useState(false);
  const { hasPro } = useProPlan();
  const [showPaywall, setShowPaywall] = useState(false);
  const [copied, setCopied] = useState(false);

  const [tenant, setTenant] = useState('demo');

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setTenant(localStorage.getItem('business_display_name') || 'demo');
    }
  }, []);

  const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro && e.target.checked) {
      setShowPaywall(true);
      e.preventDefault();
      return;
    }
    setRemoveBranding(e.target.checked);
  };

  const getShareLink = () => {
    const data = {
      p: projectName,
      c: customerName,
      d: description,
      b: beforeImage,
      a: afterImage,
      l: ctaLink,
      r: removeBranding ? '1' : '0',
      t: tenant
    };

    // Convert to query string
    const query = new URLSearchParams();
    Object.entries(data).forEach(([key, value]) => {
      if (value) query.append(key, value);
    });

    return `${window.location.origin}/showcase?${query.toString()}`;
  };

  const handleCopyLink = () => {
    navigator.clipboard.writeText(getShareLink());
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen bg-[#F5F5F7] text-[#1D1D1F] font-inter">
      {/* Header */}
      <header className="px-4 py-4 md:px-6 flex items-center justify-between sticky top-0 z-40 bg-[#F5F5F7]/80 backdrop-blur-[30px] saturate-[210%] border-b border-[#E5E5EA]">
        <h1 className="text-xl md:text-2xl font-bold tracking-tight">Project Showcase</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-white rounded-full text-sm font-semibold shadow-sm border border-[#E5E5EA] hover:bg-gray-50 transition-colors"
        >
          Close
        </button>
      </header>

      <main className="flex-1 p-4 md:p-6 w-full max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-6 lg:gap-8">

        {/* Editor Section */}
        <section className="flex flex-col gap-6">
          <div className="bg-white rounded-3xl p-6 shadow-sm border border-[#E5E5EA]">
            <h2 className="text-lg font-semibold mb-4">Project Details</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-[#86868B] mb-1">Project Name *</label>
                <input
                  type="text"
                  className="w-full px-4 py-3 rounded-xl bg-[#F5F5F7] border border-transparent focus:border-[#0066CC] focus:bg-white focus:outline-none transition-colors"
                  placeholder="e.g. Modern Kitchen Remodel"
                  value={projectName}
                  onChange={(e) => setProjectName(e.target.value)}
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-[#86868B] mb-1">Customer Name (Optional)</label>
                <input
                  type="text"
                  className="w-full px-4 py-3 rounded-xl bg-[#F5F5F7] border border-transparent focus:border-[#0066CC] focus:bg-white focus:outline-none transition-colors"
                  placeholder="e.g. The Smith Family"
                  value={customerName}
                  onChange={(e) => setCustomerName(e.target.value)}
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-[#86868B] mb-1">Description *</label>
                <textarea
                  className="w-full px-4 py-3 rounded-xl bg-[#F5F5F7] border border-transparent focus:border-[#0066CC] focus:bg-white focus:outline-none transition-colors min-h-[100px] resize-y"
                  placeholder="Describe the work done, the challenges, and the outcome..."
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-[#86868B] mb-1">Before Image URL</label>
                  <input
                    type="url"
                    className="w-full px-4 py-3 rounded-xl bg-[#F5F5F7] border border-transparent focus:border-[#0066CC] focus:bg-white focus:outline-none transition-colors"
                    placeholder="https://example.com/before.jpg"
                    value={beforeImage}
                    onChange={(e) => setBeforeImage(e.target.value)}
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-[#86868B] mb-1">After Image URL</label>
                  <input
                    type="url"
                    className="w-full px-4 py-3 rounded-xl bg-[#F5F5F7] border border-transparent focus:border-[#0066CC] focus:bg-white focus:outline-none transition-colors"
                    placeholder="https://example.com/after.jpg"
                    value={afterImage}
                    onChange={(e) => setAfterImage(e.target.value)}
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-[#86868B] mb-1">"Book Me" Link URL</label>
                <input
                  type="url"
                  className="w-full px-4 py-3 rounded-xl bg-[#F5F5F7] border border-transparent focus:border-[#0066CC] focus:bg-white focus:outline-none transition-colors"
                  placeholder="https://mybusiness.com/book"
                  value={ctaLink}
                  onChange={(e) => setCtaLink(e.target.value)}
                />
              </div>
            </div>
          </div>

          <div className="bg-white rounded-3xl p-6 shadow-sm border border-[#E5E5EA]">
            <h2 className="text-lg font-semibold mb-4">Sharing Options</h2>

            <div className="flex items-center justify-between p-4 bg-[#F5F5F7] rounded-xl mb-4">
              <div>
                <p className="font-medium text-sm">Remove "Powered by OHC" Badge</p>
                <p className="text-xs text-[#86868B] mt-1">Make the showcase 100% white-labeled</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  className="sr-only peer"
                  checked={removeBranding}
                  onChange={handleBrandingToggle}
                />
                <div className="w-11 h-6 bg-[#D1D1D6] peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-[#E5E5EA] after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#34C759]"></div>
              </label>
            </div>

            <button
              onClick={handleCopyLink}
              className="w-full py-3 px-4 bg-[#0066CC] text-white rounded-xl font-semibold hover:bg-[#0055B3] transition-colors flex items-center justify-center gap-2"
            >
              {copied ? '✓ Link Copied!' : 'Copy Share Link'}
            </button>
          </div>
        </section>

        {/* Preview Section */}
        <section className="flex flex-col h-full min-h-[600px] lg:min-h-0 bg-white rounded-3xl overflow-hidden shadow-xl border border-[#E5E5EA]">
          <div className="bg-[#F5F5F7] px-4 py-3 border-b border-[#E5E5EA] flex items-center gap-2">
            <div className="flex gap-1.5">
              <div className="w-3 h-3 rounded-full bg-[#FF3B30]"></div>
              <div className="w-3 h-3 rounded-full bg-[#FFCC00]"></div>
              <div className="w-3 h-3 rounded-full bg-[#34C759]"></div>
            </div>
            <div className="ml-4 text-xs text-[#86868B] font-medium tracking-wide">LIVE PREVIEW</div>
          </div>

          <div className="flex-1 overflow-y-auto bg-[#F5F5F7] p-4 md:p-8">
            <div className="max-w-2xl mx-auto bg-white rounded-2xl shadow-sm overflow-hidden border border-[#E5E5EA] flex flex-col">

              {/* Showcase Content */}
              <div className="p-6 md:p-8 flex-1">
                {projectName ? (
                  <h1 className="text-3xl font-bold tracking-tight mb-2">{projectName}</h1>
                ) : (
                  <h1 className="text-3xl font-bold tracking-tight text-[#D1D1D6] mb-2">Project Name</h1>
                )}

                {customerName && (
                  <p className="text-sm text-[#86868B] font-medium uppercase tracking-wider mb-6">For {customerName}</p>
                )}

                {(!customerName && projectName) && (
                  <div className="mb-6"></div>
                )}

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
                  <div className="space-y-2">
                    <div className="text-xs font-semibold text-[#86868B] tracking-widest uppercase">Before</div>
                    <div className="aspect-[4/3] bg-[#F5F5F7] rounded-xl overflow-hidden flex items-center justify-center border border-[#E5E5EA]">
                      {beforeImage ? (
                        <img src={beforeImage} alt="Before" className="w-full h-full object-cover" />
                      ) : (
                        <span className="text-[#86868B] text-sm">No image</span>
                      )}
                    </div>
                  </div>
                  <div className="space-y-2">
                    <div className="text-xs font-semibold text-[#86868B] tracking-widest uppercase">After</div>
                    <div className="aspect-[4/3] bg-[#F5F5F7] rounded-xl overflow-hidden flex items-center justify-center border border-[#E5E5EA]">
                      {afterImage ? (
                        <img src={afterImage} alt="After" className="w-full h-full object-cover" />
                      ) : (
                        <span className="text-[#86868B] text-sm">No image</span>
                      )}
                    </div>
                  </div>
                </div>

                <div className="prose prose-sm max-w-none text-[#1D1D1F] mb-8">
                  {description ? (
                    <p className="whitespace-pre-wrap">{description}</p>
                  ) : (
                    <p className="text-[#D1D1D6]">Project description will appear here...</p>
                  )}
                </div>

                {ctaLink && (
                  <div className="mt-8 pt-8 border-t border-[#E5E5EA]">
                    <a
                      href={ctaLink}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="block w-full py-4 px-6 bg-[#1D1D1F] text-white text-center rounded-xl font-semibold hover:bg-black transition-colors"
                    >
                      Book a Similar Project
                    </a>
                  </div>
                )}
              </div>

              {/* Powered By OHC Loop */}
              {!removeBranding && (
                <div className="bg-[#F5F5F7] py-6 flex justify-center border-t border-[#E5E5EA]">
                  <PoweredByOHC tenantId={tenant} />
                </div>
              )}
            </div>
          </div>
        </section>
      </main>

      {/* Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/40 backdrop-blur-[30px] saturate-[210%] z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-3xl p-8 max-w-md w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
            <button
              onClick={() => setShowPaywall(false)}
              className="absolute top-4 right-4 p-2 text-[#86868B] hover:text-[#1D1D1F] transition-colors"
            >
              ✕
            </button>
            <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-white text-2xl mb-6 shadow-lg">
              ✨
            </div>
            <h2 className="text-2xl font-bold mb-2">Upgrade to Pro</h2>
            <p className="text-[#86868B] mb-8">
              Make the Project Showcase 100% white-labeled. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>
            <div className="space-y-3">
              <button
                onClick={() => router.push('/pricing')}
                className="w-full py-3 bg-[#0066CC] text-white rounded-xl font-semibold hover:bg-[#0055B3] transition-colors shadow-md"
              >
                View Plans
              </button>
              <button
                onClick={() => setShowPaywall(false)}
                className="w-full py-3 bg-[#F5F5F7] text-[#1D1D1F] rounded-xl font-semibold hover:bg-[#E5E5EA] transition-colors"
              >
                Maybe Later
              </button>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
      `}} />
    </div>
  );
}
