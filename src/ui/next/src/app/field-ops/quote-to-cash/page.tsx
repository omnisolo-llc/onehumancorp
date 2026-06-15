"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../../../lib/sync/SyncManager';
import StripeTerminalClient from '../../pos/terminal/StripeTerminalClient';

const t = (text: string) => text;

type DraftQuote = {
  description: string;
  amountCents: number;
};

export default function QuoteToCashPage() {
  const [isOffline, setIsOffline] = useState(false);
  const [jobDescription, setJobDescription] = useState('');
  const [generatedQuote, setGeneratedQuote] = useState<DraftQuote | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    if (typeof window !== 'undefined') {
        setIsOffline(!navigator.onLine);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);

        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
        };
    }
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter md:py-10 w-full overflow-hidden">
      <div className="w-full max-w-[375px] min-h-[100dvh] md:h-[812px] md:min-h-0 bg-white md:shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{t('Quick Quote')}</h1>
            {isOffline ? (
              <span className="inline-block mt-1 text-yellow-800 font-bold text-xs bg-yellow-100 px-2 py-1 rounded border border-yellow-200 shadow-sm">{t('Saved Offline')}</span>
            ) : (
              <span className="inline-block mt-1 text-green-800 font-bold text-xs bg-green-100 px-2 py-1 rounded border border-green-200 shadow-sm">{t('Online')}</span>
            )}
          </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-[#F5F5F7]">
           <div className="app-card rounded-2xl p-6 shadow-sm border border-gray-100 mb-6 bg-white text-center">
             <label className="block text-sm font-medium text-gray-700 mb-2 text-left">
                Tell OHC about the job
             </label>
             <textarea
               className="w-full border border-gray-300 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-shadow min-h-[80px] mb-4"
               placeholder="E.g., Found a leak under the sink, requires immediate pipe replacement quote for $150."
               value={jobDescription}
               onChange={(e) => setJobDescription(e.target.value)}
             />
             <button
               onClick={() => {
                 setIsGenerating(true);
                 setTimeout(() => {
                   const amountMatch = jobDescription.match(/\$([0-9]+)/);
                   const amountCents = amountMatch ? parseInt(amountMatch[1]) * 100 : 0;
                   setGeneratedQuote({ description: jobDescription, amountCents });
                   setIsGenerating(false);
                 }, 500);
               }}
               className="charge-btn w-full py-4 rounded-[8px] bg-blue-600 text-white font-bold shadow-md shadow-blue-500/20 hover:bg-blue-700 transition-colors flex items-center justify-center gap-2"
               disabled={isGenerating || !jobDescription}
             >
               <span>🎤</span> Generate Quote
             </button>
           </div>

           {generatedQuote && (
             <div className="bg-white/65 backdrop-blur-[30px] border border-white/40 rounded-2xl p-6 mb-6 shadow-lg saturate-[210%]">
               <h3 className="text-xl font-bold font-outfit text-gray-900 mb-4 border-b border-gray-100 pb-2">Draft Quote</h3>
               <div className="flex justify-between items-center mb-3">
                 <span className="text-gray-600 font-medium">Services & Materials</span>
                 <span className="text-gray-900 font-bold">${(generatedQuote.amountCents / 100).toFixed(2)}</span>
               </div>
               <p className="text-xs text-gray-500 mb-6 italic border-l-2 border-blue-500 pl-3">
                 "{generatedQuote.description}"
               </p>
               <div className="flex justify-between items-center border-t border-gray-200 pt-4">
                 <span className="text-lg font-bold font-outfit text-gray-900">Total Deposit</span>
                 <span className="text-2xl font-bold font-outfit text-[#0066FF]">${(generatedQuote.amountCents / 100).toFixed(2)}</span>
               </div>
             </div>
           )}

           {generatedQuote && generatedQuote.amountCents > 0 && (
              <div className="mt-4">
                <StripeTerminalClient
                   amount={generatedQuote.amountCents}
                   productId="draft_quote"
                   tenantId={typeof window !== 'undefined' ? localStorage.getItem('tenant_id') || 'default_tenant' : 'default_tenant'}
                   onOptimisticReserve={() => {
                     // Optionally store draft quote content so sync picks it up
                     SyncManager.getInstance().enqueue({
                        id: `quote-${Date.now()}`,
                        type: 'draft_quote',
                        notes: generatedQuote.description,
                        amount: generatedQuote.amountCents
                     });
                   }}
                />
              </div>
           )}
        </div>

      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
