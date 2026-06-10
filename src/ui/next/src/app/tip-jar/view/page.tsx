'use client';

import '../../globals.css';


import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';
import Link from 'next/link';

import { Suspense } from 'react';

function TipJarViewContent() {
  const searchParams = useSearchParams();
  const [data, setData] = useState<{ message: string; showBranding: boolean }>({
    message: '',
    showBranding: true,
  });
  const [amount, setAmount] = useState<number | null>(null);

  useEffect(() => {
    const dataParam = searchParams.get('data');
    if (dataParam) {
      try {
        setData(JSON.parse(atob(dataParam)));
      } catch (e) {
        console.error('Invalid tip jar data');
      }
    }
  }, [searchParams]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-[#0F172A] via-[#1E1B4B] to-[#0F172A] text-white flex flex-col items-center justify-center p-4">
      <div className="glassmorphism max-w-md w-full p-8 rounded-[24px] relative z-10 text-center">
        <div className="w-20 h-20 bg-gradient-to-tr from-pink-500 to-purple-500 rounded-full mx-auto mb-6 flex items-center justify-center text-4xl shadow-lg">
          💖
        </div>
        <h1 className="text-3xl font-bold font-outfit mb-2">Support Me</h1>
        <p className="text-gray-300 mb-8 whitespace-pre-wrap">{data.message || 'Thanks for stopping by!'}</p>

        <div className="flex justify-center gap-4 mb-6">
          {[5, 10, 20].map((preset) => (
            <button
              key={preset}
              onClick={() => setAmount(preset)}
              className={`flex-1 py-3 px-4 rounded-xl font-semibold transition-all duration-200 ${
                amount === preset
                  ? 'bg-purple-600 text-white shadow-lg shadow-purple-500/30 border border-purple-400'
                  : 'bg-white/5 text-gray-300 hover:bg-white/10 border border-white/10'
              }`}
            >
              ${preset}
            </button>
          ))}
        </div>

        <div className="relative mb-8">
          <span className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">$</span>
          <input
            type="number"
            min="1"
            placeholder="Custom amount"
            value={amount || ''}
            onChange={(e) => setAmount(Number(e.target.value))}
            className="w-full bg-white/5 border border-white/10 rounded-xl py-3 pl-8 pr-4 text-white focus:outline-none focus:ring-2 focus:ring-purple-500 transition-all text-center placeholder-gray-500"
          />
        </div>

        <button
          className="w-full bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-bold py-4 px-6 rounded-xl transition-all transform hover:scale-[1.02] active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-purple-500/25"
          disabled={!amount || amount <= 0}
        >
          {amount ? `Pay $${amount.toFixed(2)}` : 'Select an amount'}
        </button>

        {data.showBranding && (
          <div className="mt-8 pt-6 border-t border-white/10">
            <Link
              href="/onboarding?ref=tip_jar_footer&source=tip_jar"
              className="group inline-flex flex-col items-center justify-center opacity-80 hover:opacity-100 transition-opacity"
            >
              <span className="text-xs text-gray-400 mb-1">Powered by OHC</span>
              <span className="text-sm font-medium bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent group-hover:from-purple-300 group-hover:to-pink-300 transition-all">
                Create your own tip jar for free &rarr;
              </span>
            </Link>
          </div>
        )}
      </div>

      {/* Background decoration */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none z-0">
        <div className="absolute top-1/4 left-1/4 w-[500px] h-[500px] bg-purple-600/20 rounded-full blur-[120px]" />
        <div className="absolute bottom-1/4 right-1/4 w-[600px] h-[600px] bg-pink-600/10 rounded-full blur-[150px]" />
      </div>
    </div>
  );
}


export default function TipJarViewPage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <TipJarViewContent />
    </Suspense>
  );
}