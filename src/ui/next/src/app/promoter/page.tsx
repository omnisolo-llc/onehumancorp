"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function PromoterPage() {
  const router = useRouter();
  const [productName, setProductName] = useState('');
  const [description, setDescription] = useState('');
  const [theme, setTheme] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedPosts, setGeneratedPosts] = useState<{instagram: string, twitter: string, email: string} | null>(null);
  const [copiedState, setCopiedState] = useState<{instagram: boolean, twitter: boolean, email: boolean}>({
    instagram: false,
    twitter: false,
    email: false
  });

  const [tenant, setTenant] = useState('my-store');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
    }
  }, []);

  const handleGenerate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!productName) return;

    setIsGenerating(true);

    try {
      const response = await fetch('/api/v1/growth/promoter/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          product_name: productName,
          description,
          theme,
          tenant
        }),
      });

      if (response.ok) {
        const data = await response.json();
        setGeneratedPosts({
          instagram: `${data.instagram}\n\n⚡ Powered by OHC`,
          twitter: `${data.twitter}\n\n⚡ Powered by OHC`,
          email: `${data.email}\n\n⚡ Powered by OHC`
        });
      }
    } catch (error) {
      console.error("Failed to generate posts", error);
    } finally {
      setIsGenerating(false);
    }
  };

  const copyToClipboard = async (text: string, platform: 'instagram' | 'twitter' | 'email') => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedState(prev => ({ ...prev, [platform]: true }));
      setTimeout(() => {
        setCopiedState(prev => ({ ...prev, [platform]: false }));
      }, 2000);
    } catch (err) {
      console.error('Failed to copy text', err);
    }
  };

  return (
    <main className="min-h-screen bg-gray-50 pb-12">
      <div className="bg-indigo-600 pb-32">
        <header className="py-10">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center">
            <h1 className="text-3xl font-bold text-white tracking-tight font-outfit">The Promoter</h1>
            <button
              onClick={() => router.push('/dashboard')}
              className="text-white bg-indigo-500 hover:bg-indigo-400 px-4 py-2 rounded-lg text-sm font-medium transition-colors"
            >
              Back to Dashboard
            </button>
          </div>
        </header>
      </div>

      <div className="-mt-32 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">

          {/* Input Form */}
          <div className="lg:col-span-1">
            <div className="bg-white rounded-[24px] shadow-sm border border-gray-100 p-6">
              <h2 className="text-xl font-bold text-gray-900 font-outfit mb-4">Promote a Product</h2>
              <p className="text-gray-600 text-sm mb-6">Let OHC's AI write engaging social media posts to drive traffic to your storefront.</p>

              <form onSubmit={handleGenerate} className="space-y-4">
                <div>
                  <label htmlFor="productName" className="block text-sm font-medium text-gray-700 mb-1">Product/Service Name *</label>
                  <input
                    id="productName"
                    type="text"
                    required
                    placeholder="e.g. Summer Floral Dress"
                    value={productName}
                    onChange={(e) => setProductName(e.target.value)}
                    className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all text-sm"
                  />
                </div>

                <div>
                  <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">Key Selling Points</label>
                  <textarea
                    id="description"
                    rows={3}
                    placeholder="e.g. Lightweight, breathable fabric. Perfect for the beach."
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all text-sm resize-none"
                  />
                </div>

                <div>
                  <label htmlFor="theme" className="block text-sm font-medium text-gray-700 mb-1">Campaign Theme (Optional)</label>
                  <input
                    id="theme"
                    type="text"
                    placeholder="e.g. Summer Sale, Back to School"
                    value={theme}
                    onChange={(e) => setTheme(e.target.value)}
                    className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all text-sm"
                  />
                </div>

                <button
                  type="submit"
                  disabled={isGenerating || !productName}
                  className="w-full mt-4 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 text-white font-bold py-3 px-4 rounded-xl shadow-md transition-all hover:shadow-lg disabled:opacity-70 flex justify-center items-center gap-2"
                >
                  {isGenerating ? (
                    <>
                      <span className="animate-spin text-lg">⏳</span> Generating...
                    </>
                  ) : (
                    <>
                      <span className="text-lg">✨</span> Generate Posts
                    </>
                  )}
                </button>
              </form>
            </div>
          </div>

          {/* Results Area */}
          <div className="lg:col-span-2 space-y-6">
            {!generatedPosts && !isGenerating && (
              <div className="bg-white rounded-[24px] shadow-sm border border-gray-100 p-12 text-center h-full flex flex-col items-center justify-center">
                <div className="w-16 h-16 bg-indigo-50 rounded-2xl flex items-center justify-center text-3xl mb-4">
                  🤖
                </div>
                <h3 className="text-lg font-bold text-gray-900 mb-2">Ready to grow your audience?</h3>
                <p className="text-gray-500 max-w-sm">Enter your product details on the left and our AI will draft high-converting social media posts for you to share instantly.</p>
              </div>
            )}

            {isGenerating && (
              <div className="bg-white backdrop-blur-[30px] saturate-[210%] rounded-[24px] shadow-sm border border-gray-100 p-12 text-center h-full flex flex-col items-center justify-center animate-pulse">
                <div className="text-4xl mb-4 animate-bounce">⚡</div>
                <h3 className="text-lg font-bold text-gray-900">Crafting your content...</h3>
              </div>
            )}

            {generatedPosts && !isGenerating && (
              <>
                {/* Instagram Card */}
                <div className="bg-white rounded-[24px] shadow-sm border border-gray-100 overflow-hidden">
                  <div className="bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-1"></div>
                  <div className="p-6">
                    <div className="flex justify-between items-center mb-4">
                      <div className="flex items-center gap-2">
                        <div className="w-8 h-8 rounded-full bg-pink-100 flex items-center justify-center text-pink-600">
                          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path fillRule="evenodd" d="M12.315 2c2.43 0 2.784.013 3.808.06 1.064.049 1.791.218 2.427.465a4.902 4.902 0 011.772 1.153 4.902 4.902 0 011.153 1.772c.247.636.416 1.363.465 2.427.048 1.067.06 1.407.06 4.123v.08c0 2.643-.012 2.987-.06 4.043-.049 1.064-.218 1.791-.465 2.427a4.902 4.902 0 01-1.153 1.772 4.902 4.902 0 01-1.772 1.153c-.636.247-1.363.416-2.427.465-1.067.048-1.407.06-4.123.06h-.08c-2.643 0-2.987-.012-4.043-.06-1.064-.049-1.791-.218-2.427-.465a4.902 4.902 0 01-1.772-1.153 4.902 4.902 0 01-1.153-1.772c-.247-.636-.416-1.363-.465-2.427-.047-1.024-.06-1.379-.06-3.808v-.63c0-2.43.013-2.784.06-3.808.049-1.064.218-1.791.465-2.427a4.902 4.902 0 011.153-1.772A4.902 4.902 0 015.45 2.525c.636-.247 1.363-.416 2.427-.465C8.901 2.013 9.256 2 11.685 2h.63zm-.081 1.802h-.468c-2.456 0-2.784.011-3.807.058-.975.045-1.504.207-1.857.344-.467.182-.8.398-1.15.748-.35.35-.566.683-.748 1.15-.137.353-.3.882-.344 1.857-.047 1.023-.058 1.351-.058 3.807v.468c0 2.456.011 2.784.058 3.807.045.975.207 1.504.344 1.857.182.466.399.8.748 1.15.35.35.683.566 1.15.748.353.137.882.3 1.857.344 1.054.048 1.37.058 4.041.058h.08c2.597 0 2.917-.01 3.96-.058.976-.045 1.505-.207 1.858-.344.466-.182.8-.398 1.15-.748.35-.35.566-.683.748-1.15.137-.353.3-.882.344-1.857.048-1.055.058-1.37.058-4.041v-.08c0-2.597-.01-2.917-.058-3.96-.045-.976-.207-1.505-.344-1.858a3.097 3.097 0 00-.748-1.15 3.098 3.098 0 00-1.15-.748c-.353-.137-.882-.3-1.857-.344-1.023-.047-1.351-.058-3.807-.058zM12 6.865a5.135 5.135 0 110 10.27 5.135 5.135 0 010-10.27zm0 1.802a3.333 3.333 0 100 6.666 3.333 3.333 0 000-6.666zm5.338-3.205a1.2 1.2 0 110 2.4 1.2 1.2 0 010-2.4z" clipRule="evenodd" />
                          </svg>
                        </div>
                        <h3 className="font-bold text-gray-900">Instagram / Facebook</h3>
                      </div>
                      <button
                        onClick={() => copyToClipboard(generatedPosts.instagram, 'instagram')}
                        className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${copiedState.instagram ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
                        data-testid="copy-instagram"
                      >
                        {copiedState.instagram ? 'Copied!' : 'Copy Post'}
                      </button>
                    </div>
                    <div className="bg-gray-50 p-4 rounded-xl whitespace-pre-wrap text-gray-700 text-sm border border-gray-100">
                      {generatedPosts.instagram}
                    </div>
                  </div>
                </div>

                {/* Twitter Card */}
                <div className="bg-white rounded-[24px] shadow-sm border border-gray-100 overflow-hidden">
                  <div className="bg-blue-400 p-1"></div>
                  <div className="p-6">
                    <div className="flex justify-between items-center mb-4">
                      <div className="flex items-center gap-2">
                        <div className="w-8 h-8 rounded-full bg-blue-50 flex items-center justify-center text-[#0066FF]">
                          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84" />
                          </svg>
                        </div>
                        <h3 className="font-bold text-gray-900">X (Twitter)</h3>
                      </div>
                      <button
                        onClick={() => copyToClipboard(generatedPosts.twitter, 'twitter')}
                        className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${copiedState.twitter ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
                        data-testid="copy-twitter"
                      >
                        {copiedState.twitter ? 'Copied!' : 'Copy Post'}
                      </button>
                    </div>
                    <div className="bg-gray-50 p-4 rounded-xl whitespace-pre-wrap text-gray-700 text-sm border border-gray-100">
                      {generatedPosts.twitter}
                    </div>
                  </div>
                </div>

                {/* Email Card */}
                <div className="bg-white rounded-[24px] shadow-sm border border-gray-100 overflow-hidden">
                  <div className="bg-[#34C759] p-1"></div>
                  <div className="p-6">
                    <div className="flex justify-between items-center mb-4">
                      <div className="flex items-center gap-2">
                        <div className="w-8 h-8 rounded-full bg-green-50 flex items-center justify-center text-green-600">
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                          </svg>
                        </div>
                        <h3 className="font-bold text-gray-900">Email Marketing</h3>
                      </div>
                      <button
                        onClick={() => copyToClipboard(generatedPosts.email, 'email')}
                        className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${copiedState.email ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
                        data-testid="copy-email"
                      >
                        {copiedState.email ? 'Copied!' : 'Copy Email'}
                      </button>
                    </div>
                    <div className="bg-gray-50 p-4 rounded-xl whitespace-pre-wrap text-gray-700 text-sm border border-gray-100">
                      {generatedPosts.email}
                    </div>
                  </div>
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
