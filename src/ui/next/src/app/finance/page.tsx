"use client";

import { useState, useEffect } from "react";

export default function FinanceDashboard() {
  const [offers, setOffers] = useState<any[]>([]);
  const [advances, setAdvances] = useState<any[]>([]);

  useEffect(() => {
    // In a real app, we'd fetch from our API
    // For now we mock it to demonstrate the UI
    setOffers([
      {
        id: "offer_1",
        offer_amount: 1500,
        fee_percentage: 10,
        repayment_rate: 8,
      }
    ]);
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200" style={{background: 'rgba(255, 255, 255, 0.45)', backdropFilter: 'blur(40px) saturate(250%)', borderRadius: '24px', border: '1px solid rgba(255, 255, 255, 0.5)', boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.07)'}}>

        {/* Header */}
        <div className="pt-12 pb-4 px-6 border-b border-gray-100/50 bg-white/30 backdrop-blur-md sticky top-0 z-20 flex justify-between items-center">
            <h1 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">Finance</h1>
        </div>

        <div className="flex-1 overflow-y-auto px-4 py-6 space-y-6">
          {offers.map(offer => (
            <div key={offer.id} className="relative overflow-hidden rounded-2xl p-6" style={{background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', boxShadow: '0 4px 24px rgba(0, 0, 0, 0.04)'}}>
              <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10 opacity-50"></div>
              <h2 className="text-lg font-bold font-outfit text-gray-900 mb-2">Need to stock up for the holidays?</h2>
              <p className="text-gray-600 text-sm mb-6 leading-relaxed">
                You have <strong className="text-[#0071E3]">${offer.offer_amount.toLocaleString()}</strong> in instant capital available. Repay automatically from sales.
              </p>

              <div className="bg-white/50 rounded-xl p-4 mb-6 border border-gray-100">
                <div className="flex justify-between mb-2">
                  <span className="text-xs text-gray-500 font-semibold uppercase tracking-wider">Fee</span>
                  <span className="text-sm font-bold text-gray-900">${(offer.offer_amount * (offer.fee_percentage / 100)).toLocaleString()}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-xs text-gray-500 font-semibold uppercase tracking-wider">Repayment</span>
                  <span className="text-sm font-bold text-gray-900">{offer.repayment_rate}% of daily sales</span>
                </div>
              </div>

              <button className="w-full py-4 rounded-xl font-bold transition-all shadow-md active:scale-[0.98] flex items-center justify-center gap-2 text-white bg-[#0071E3] hover:bg-blue-600 border border-blue-500">
                 Get Funds Instantly
              </button>
            </div>
          ))}

          {offers.length === 0 && advances.length === 0 && (
             <div className="text-center text-gray-500 text-sm mt-10">No offers currently available. Keep growing your sales!</div>
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
