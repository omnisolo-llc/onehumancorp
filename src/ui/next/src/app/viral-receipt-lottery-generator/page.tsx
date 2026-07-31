"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralReceiptLotteryGeneratorPage() {
  const router = useRouter();
  const [tenantId, setTenantId] = useState('default');
  const [generating, setGenerating] = useState(false);
  const [result, setResult] = useState<string | null>(null);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name') || 'default';
      setTenantId(storedTenant);
    }
  }, []);

  const handleGenerate = () => {
    setGenerating(true);
    setTimeout(() => {
      setGenerating(false);
      const randId = Math.random().toString(36).substring(2, 10);
      setResult(`https://ohc.app/win/${randId}`);
    }, 1000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white">
        <h1 className="text-2xl font-bold text-gray-900">Viral Receipt Lottery 🎟</h1>
        <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm hover:bg-gray-300">
          Back to Dashboard
        </button>
      </header>
      <main className="flex-1 p-6 max-w-4xl mx-auto w-full">
        <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-200 text-center">
            <h2 className="text-xl font-bold mb-4">Generate Lottery Link</h2>
            <button
                id="generate-btn"
                onClick={handleGenerate}
                disabled={generating}
                className="px-6 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 disabled:opacity-50 font-semibold transition"
            >
                {generating ? 'Generating...' : 'Generate Link'}
            </button>
            {result && (
                <div id="result-area" className="mt-6">
                    <p className="text-sm text-gray-600 mb-2">Share this link:</p>
                    <input
                        id="share-link"
                        type="text"
                        readOnly
                        value={result}
                        className="w-full max-w-md mx-auto p-3 text-center border border-gray-300 rounded-lg bg-gray-50"
                    />
                    <div id="preview-url" className="mt-4 text-xs text-gray-500">
                        {result}
                    </div>
                </div>
            )}
        </div>
      </main>
      <PoweredByOHC tenantId={tenantId} />
    </div>
  );
}
