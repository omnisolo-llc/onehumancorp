"use client";

import React, { useState } from 'react';
import Link from 'next/link';
import { AppShell } from '../components/AppShell';
import { useAuth } from '../components/AuthProvider';

export default function EdgeStorefrontSetup() {
  const [step, setStep] = useState(1);
  const [selectedFocus, setSelectedFocus] = useState<string | null>(null);
  const [isPublishing, setIsPublishing] = useState(false);
  const [publishedUrl, setPublishedUrl] = useState<string | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const { session } = useAuth();

  const handlePublish = async () => {
    setIsPublishing(true);
    setErrorMsg(null);
    try {
      if (!session?.tenant_id) {
        throw new Error('Not authenticated');
      }

      // 1. Create a real Site to get an ID
      const createRes = await fetch('/api/v1/builder/sites', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${session.access_token}`
        },
        body: JSON.stringify({
          domain: 'edge-storefront.ohc.app'
        })
      });

      if (!createRes.ok) {
        throw new Error(`Failed to initialize site: ${createRes.status}`);
      }

      const siteData = await createRes.json();
      const siteId = siteData.id;

      // 2. Use the actual AI storefront generation endpoint
      const generateRes = await fetch('/api/v1/builder/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${session.access_token}`
        },
        body: JSON.stringify({
          description: `Focus: ${selectedFocus}. Auto-generated edge storefront via OHC Promoter Agent.`
        })
      });

      if (!generateRes.ok) {
        throw new Error(`Failed to generate storefront: ${generateRes.status}`);
      }

      // 3. Finally publish it
      const publishRes = await fetch(`/api/v1/builder/sites/${siteId}/publish`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${session.access_token}`
        }
      });

      if (!publishRes.ok) {
        throw new Error(`Failed to publish site: ${publishRes.status}`);
      }

      setPublishedUrl(`/api/v1/builder/edge/${session.tenant_id}/${siteId}`);
      setStep(3);
    } catch (e: any) {
      setErrorMsg(e.message);
    } finally {
      setIsPublishing(false);
    }
  };

  const copyToClipboard = () => {
    if (publishedUrl) {
      navigator.clipboard.writeText(window.location.origin + publishedUrl);
      alert('Link copied to clipboard!');
    }
  };

  return (
    <AppShell title="Publish Storefront">
      <div className="max-w-[375px] mx-auto min-h-[calc(100vh-64px)] bg-[#f5f5f7] dark:bg-[#000] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-inter relative pb-24">

        <div className="glass-container w-full bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] shadow-sm overflow-hidden p-6">
          {step === 1 && (
            <div className="animate-[fadeIn_0.4s_ease-out_forwards]">
              <div className="text-4xl mb-4 text-center">✨</div>
              <h2 className="text-2xl font-bold font-outfit text-center mb-2">Publish Storefront</h2>
              <p className="text-sm text-gray-600 dark:text-gray-400 text-center mb-6">Deploy a lightning-fast edge storefront for instant consumer discovery.</p>

              <button
                id="start-setup-btn"
                onClick={() => setStep(2)}
                className="w-full bg-[#0071E3] hover:bg-[#0066FF] text-white py-4 rounded-[8px] font-semibold text-[16px] transition-all active:scale-[0.98] shadow-md"
              >
                Start Setup
              </button>
            </div>
          )}

          {step === 2 && (
            <div className="animate-[fadeIn_0.4s_ease-out_forwards]">
              <div className="flex items-center gap-3 mb-6">
                <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-xl">🤖</div>
                <div>
                  <h3 className="font-bold text-sm">Promoter Agent</h3>
                  <p className="text-xs text-gray-500">Storefront Assistant</p>
                </div>
              </div>

              <div className="bg-gray-100 dark:bg-gray-800 p-4 rounded-xl rounded-tl-none mb-6 text-sm">
                Hi! Should I feature your custom cakes or ready-to-buy items first on the storefront?
              </div>

              <div className="flex flex-col gap-3 mb-8">
                <button
                  id="select-custom-cakes-btn"
                  onClick={() => setSelectedFocus('Custom Cakes')}
                  className={`p-4 border ${selectedFocus === 'Custom Cakes' ? 'border-[#0071E3] bg-blue-50 dark:bg-blue-900/20' : 'border-gray-200 dark:border-gray-700'} rounded-[12px] text-left transition-colors`}
                >
                  <div className="font-semibold text-[15px]">Custom Cakes</div>
                  <div className="text-xs text-gray-500 mt-1">Highlight inquiries and bookings</div>
                </button>
                <button
                  onClick={() => setSelectedFocus('Ready-to-buy')}
                  className={`p-4 border ${selectedFocus === 'Ready-to-buy' ? 'border-[#0071E3] bg-blue-50 dark:bg-blue-900/20' : 'border-gray-200 dark:border-gray-700'} rounded-[12px] text-left transition-colors`}
                >
                  <div className="font-semibold text-[15px]">Ready-to-buy</div>
                  <div className="text-xs text-gray-500 mt-1">Highlight daily inventory</div>
                </button>
              </div>

              {errorMsg && <div className="text-red-500 text-sm mb-4 p-3 bg-red-50 dark:bg-red-900/20 rounded-lg">{errorMsg}</div>}

              <button
                id="generate-storefront-btn"
                disabled={!selectedFocus || isPublishing}
                onClick={handlePublish}
                className="w-full bg-[#0071E3] disabled:opacity-50 hover:bg-[#0066FF] text-white py-4 rounded-[8px] font-semibold text-[16px] transition-all active:scale-[0.98] shadow-md flex justify-center items-center gap-2"
              >
                {isPublishing ? (
                  <>
                    <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Generating & Deploying...
                  </>
                ) : 'Generate & Publish'}
              </button>
            </div>
          )}

          {step === 3 && (
            <div className="animate-[fadeIn_0.4s_ease-out_forwards] text-center">
              <div className="w-16 h-16 bg-green-100 dark:bg-green-900/30 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 text-2xl">
                ✓
              </div>
              <h2 className="text-xl font-bold font-outfit mb-2">Storefront Live!</h2>
              <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">Your edge-cached storefront is now deployed globally.</p>

              <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border border-gray-200 dark:border-gray-700 flex items-center justify-between mb-8 overflow-hidden">
                <span className="text-sm font-mono truncate mr-2" title={publishedUrl || ''}>{publishedUrl}</span>
                <button
                  id="copy-link-btn"
                  onClick={copyToClipboard}
                  className="p-2 text-[#0071E3] hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-md transition-colors flex-shrink-0"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                </button>
              </div>

              <Link
                href="/dashboard"
                className="block w-full bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-[#1D1D1F] dark:text-white py-4 rounded-[8px] font-semibold text-[16px] transition-all"
              >
                Back to Dashboard
              </Link>
            </div>
          )}
        </div>
      </div>
    </AppShell>
  );
}
