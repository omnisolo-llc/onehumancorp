"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function ReviewCampaignsPage() {
  const router = useRouter();
  const [productName, setProductName] = useState('');
  const [customerSegment, setCustomerSegment] = useState('recent');
  const [generatedDraft, setGeneratedDraft] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [isSent, setIsSent] = useState(false);

  const handleGenerate = () => {
    setIsGenerating(true);
    setGeneratedDraft(
      `Subject: How are you loving your ${productName || 'recent purchase'}?\n\n` +
      `Hi [Customer Name],\n\n` +
      `Thank you so much for shopping with us! We noticed you recently received your ${productName || 'order'} and we hope you are absolutely loving it.\n\n` +
      `As a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review.\n\n` +
      `Click here to leave a review: [Review Link]\n\n` +
      `To say thanks, we'll send you a 10% discount code for your next purchase as soon as your review is published!\n\n` +
      `Warmly,\n` +
      `The ${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'Store' : 'Store'} Team`
    );
    setIsGenerating(false);
    setIsSent(false);
  };

  const handleSend = () => {
    // Simulate sending
    setIsSent(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Automated Review Campaigns ⭐️</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        <div className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Turn Customers into Advocates</h2>
           <p className="text-gray-600 text-sm">
             Generate highly-converting, personalized review request emails using AI. Businesses that ask for reviews see a <strong>3x increase</strong> in customer acquisition through word-of-mouth.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          {/* Campaign Settings */}
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Campaign Configuration</h3>
            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="product-name" className="block text-sm font-medium text-gray-700 mb-1">Product to Feature (Optional)</label>
                <input
                  id="product-name"
                  type="text"
                  value={productName}
                  onChange={(e) => setProductName(e.target.value)}
                  placeholder="e.g. Signature Coffee Blend"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label htmlFor="customer-segment" className="block text-sm font-medium text-gray-700 mb-1">Target Audience</label>
                <select
                  id="customer-segment"
                  value={customerSegment}
                  onChange={(e) => setCustomerSegment(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white"
                >
                  <option value="recent">Recent Buyers (Last 14 Days)</option>
                  <option value="loyal">Repeat Customers</option>
                  <option value="all">All Past Customers</option>
                </select>
              </div>
              <button
                onClick={handleGenerate}
                disabled={isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${isGenerating ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? (
                  <>
                    <svg className="animate-spin -ml-1 mr-2 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Drafting with AI...
                  </>
                ) : (
                  <>
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    Generate Email Draft
                  </>
                )}
              </button>
            </div>
          </section>

          {/* AI Draft Preview */}
          <section className="w-full md:w-1/2 p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4 flex items-center gap-2" style={{ color: '#1D1D1F' }}>
              <span className="text-yellow-500">✨</span> AI Generated Draft
            </h3>

            {generatedDraft ? (
              <div className="flex-1 flex flex-col">
                <div className="flex-1 bg-gray-50 border border-gray-100 rounded-xl p-4 mb-4">
                  <pre className="whitespace-pre-wrap text-sm text-gray-700 font-inter font-medium" style={{ fontFamily: 'inherit' }}>
                    {generatedDraft}
                  </pre>
                </div>

                {isSent ? (
                  <div className="w-full py-3 bg-green-50 text-green-700 font-bold rounded-xl text-center border border-green-200">
                    ✅ Campaign sent to {customerSegment === 'recent' ? '48' : customerSegment === 'loyal' ? '12' : '156'} customers!
                  </div>
                ) : (
                  <button
                    onClick={handleSend}
                    className="w-full py-3 bg-gray-900 hover:bg-black text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2"
                  >
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                    Send to Audience ({customerSegment === 'recent' ? '48' : customerSegment === 'loyal' ? '12' : '156'} Customers)
                  </button>
                )}
              </div>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                <svg className="w-12 h-12 mb-3 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                <p className="text-sm font-medium">Configure your campaign to generate a high-converting email draft.</p>
              </div>
            )}
          </section>
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
