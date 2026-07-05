"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LoyaltyProgramPage() {
  const router = useRouter();
  const [giveAmount, setGiveAmount] = useState('10');
  const [getAmount, setGetAmount] = useState('10');
  const [rewardType, setRewardType] = useState('percentage');
  const [generatedDraft, setGeneratedDraft] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [isSent, setIsSent] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showUpgradeModal, setShowUpgradeModal] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleGenerate = async () => {
    setIsGenerating(true);
    setGeneratedDraft('');
    setIsSent(false);

    try {
      const response = await fetch('/api/v1/growth/loyalty/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          give_amount: giveAmount,
          get_amount: getAmount,
          reward_type: rewardType,
          store_name: typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'Store' : 'Store'
        })
      });

      const data = await response.json();
      setGeneratedDraft(`${data.message}\n\n⚡ Powered by OHC`);
    } catch (error) {
      console.error("Failed to generate draft", error);
      setGeneratedDraft("Failed to generate email draft. Please try again.");
    } finally {
      setIsGenerating(false);
    }
  };

  const handleSend = () => {
    if (!hasPro) {
      setShowUpgradeModal(true);
      return;
    }
    // Simulate sending
    setIsSent(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white backdrop-blur-[30px] saturate-[210%] border-white/40">
         <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Customer Loyalty Program 🤝</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        <div className="glassmorphism rounded-2xl p-6 md:p-8 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Turn Customers into Promoters</h2>
           <p className="text-gray-600 text-sm">
             Set up a &quot;Give X, Get Y&quot; referral program. We&apos;ll use AI to generate the perfect email campaign to send to your top customers to get them sharing.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          {/* Campaign Settings */}
          <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="glassmorphism p-6 rounded-2xl shadow-sm border border-white/40">
              <h3 className="text-lg font-bold font-outfit text-gray-900 mb-4">Program Rules</h3>

              <div className="flex flex-col gap-4">
                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-2">Reward Type</label>
                  <select
                    value={rewardType}
                    onChange={(e) => setRewardType(e.target.value)}
                    className="w-full bg-white border border-gray-200 rounded-lg p-3 text-sm focus:ring-2 focus:ring-indigo-500 outline-none"
                  >
                    <option value="percentage">% Discount</option>
                    <option value="fixed">$ Store Credit</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-2">Friend Gets ({rewardType === 'percentage' ? '%' : '$'})</label>
                  <input
                    type="number"
                    value={giveAmount}
                    onChange={(e) => setGiveAmount(e.target.value)}
                    placeholder="e.g. 10"
                    className="w-full bg-white border border-gray-200 rounded-lg p-3 text-sm focus:ring-2 focus:ring-indigo-500 outline-none"
                  />
                  <p className="text-xs text-gray-500 mt-1">Discount for the referred friend.</p>
                </div>

                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-2">You Give ({rewardType === 'percentage' ? '%' : '$'})</label>
                  <input
                    type="number"
                    value={getAmount}
                    onChange={(e) => setGetAmount(e.target.value)}
                    placeholder="e.g. 10"
                    className="w-full bg-white border border-gray-200 rounded-lg p-3 text-sm focus:ring-2 focus:ring-indigo-500 outline-none"
                  />
                  <p className="text-xs text-gray-500 mt-1">Reward for the referring customer.</p>
                </div>
              </div>

              <button
                onClick={handleGenerate}
                disabled={isGenerating || !giveAmount || !getAmount}
                className={`mt-6 w-full py-3 rounded-xl font-bold text-sm shadow-md transition-all flex items-center justify-center gap-2
                  ${isGenerating || !giveAmount || !getAmount ? 'bg-gray-300 text-gray-500 cursor-not-allowed' : 'bg-black text-white hover:bg-gray-800 hover:-translate-y-0.5'}`}
              >
                {isGenerating ? (
                  <>
                    <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                    Generating...
                  </>
                ) : (
                  <>
                    <span className="text-lg">✨</span> Generate Email
                  </>
                )}
              </button>
            </div>
          </div>

          {/* Preview Area */}
          <div className="w-full md:w-2/3 flex flex-col gap-4">
             <h3 className="text-lg font-bold font-outfit text-gray-900">Email Draft Preview</h3>

             {generatedDraft ? (
               <div className="glassmorphism rounded-2xl shadow-sm border border-white/40 overflow-hidden flex flex-col h-full min-h-[400px]">
                  <div className="bg-white/40 border-b border-gray-200 p-4 flex gap-2 items-center">
                    <div className="flex gap-1.5">
                      <div className="w-3 h-3 rounded-full bg-red-400"></div>
                      <div className="w-3 h-3 rounded-full bg-yellow-400"></div>
                      <div className="w-3 h-3 rounded-full bg-green-400"></div>
                    </div>
                    <div className="text-xs text-gray-500 font-medium ml-2">New Message</div>
                  </div>
                  <div className="p-6 flex-1 flex flex-col">
                    <textarea
                      value={generatedDraft}
                      onChange={(e) => setGeneratedDraft(e.target.value)}
                      className="w-full flex-1 bg-transparent border-none outline-none resize-none text-gray-800 text-sm leading-relaxed"
                      spellCheck={false}
                    />
                  </div>
                  <div className="bg-white/40 border-t border-gray-200 p-4 flex justify-between items-center">
                    <span className="text-xs text-gray-500">Drafted by The Promoter AI Agent</span>
                    <button
                      onClick={handleSend}
                      disabled={isSent}
                      className={`px-6 py-2 rounded-lg font-bold text-sm transition-colors ${isSent ? 'bg-[#34C759] text-white' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                    >
                      {isSent ? 'Sent to Top Customers! ✅' : 'Send to Top Customers'}
                    </button>
                  </div>
               </div>
             ) : (
               <div className="glassmorphism rounded-2xl border-2 border-dashed border-gray-300 flex flex-col items-center justify-center text-gray-400 min-h-[400px]">
                  <span className="text-4xl mb-4">📧</span>
                  <p className="font-medium text-sm">Configure your rules and click Generate</p>
               </div>
             )}
          </div>
        </div>
      </main>

      {/* Upgrade Modal */}
      {showUpgradeModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-6 shadow-2xl">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Upgrade to Pro</h2>
            <p className="text-gray-600 text-sm mb-6">Automated email campaigns are a Pro feature. Upgrade your plan to send this email to your customers and start driving more referrals.</p>
            <div className="flex gap-3 justify-end">
              <button
                onClick={() => setShowUpgradeModal(false)}
                className="px-4 py-2 font-medium text-gray-600 hover:text-gray-900"
              >
                Cancel
              </button>
              <button
                onClick={() => router.push('/upgrade-roi')}
                className="px-4 py-2 bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-bold rounded-lg shadow-md hover:shadow-lg transition-all"
              >
                View Plans
              </button>
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
          border: 1px solid rgba(255, 255, 255, 0.4);
        }
        @media (prefers-color-scheme: dark) {
          .glassmorphism {
            background: rgba(22, 22, 26, 0.7);
            border: 1px solid rgba(255, 255, 255, 0.1);
          }
        }
      `}} />
    </div>
  );
}