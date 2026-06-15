"use client";

import { useState } from 'react';
import Link from 'next/link';

export default function LoyaltyDashboard() {
  const [success, setSuccess] = useState(false);
  const [shareUrl, setShareUrl] = useState('');

  const generateProgram = async () => {
    try {
      const response = await fetch('/api/v1/growth/loyalty/generate', {
        method: 'POST',
      });
      if (response.ok) {
        const data = await response.json();
        setSuccess(true);
        setShareUrl(data.share_url || 'https://ohc.app/loyalty/share');
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center p-8">
      <div className="max-w-md w-full bg-white rounded-2xl shadow-xl p-8 text-center border border-gray-100">
        <h1 className="text-2xl font-bold text-gray-900 mb-6">Viral Loyalty Program Generator</h1>

        <button
          onClick={generateProgram}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-xl transition-all"
        >
          Generate Program
        </button>

        {success && (
          <div className="mt-6 p-4 bg-green-50 rounded-xl border border-green-100">
            <p className="text-green-800 font-medium mb-3">Program Generated Successfully</p>
            <a
              href={shareUrl}
              target="_blank"
              rel="noreferrer"
              className="loyalty-share-link block p-3 bg-white rounded-lg text-blue-600 break-all text-sm shadow-sm"
            >
              {shareUrl}
            </a>
          </div>
        )}
      </div>
    </div>
  );
}
