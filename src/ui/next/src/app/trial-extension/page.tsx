"use client";

import React, { useState, useEffect } from 'react';

export default function TrialExtensionPage() {
  const [unlocked, setUnlocked] = useState(false);
  const [copied, setCopied] = useState(false);
  const [shareLink, setShareLink] = useState('');

  useEffect(() => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
    setShareLink(`https://ohc.store/join?ref=${tenant}`);
  }, []);

  const handleShareTwitter = () => {
    const text = encodeURIComponent(`I just built my online store in minutes using OHC! Setup was super easy. Start your own business today: ${shareLink} 🚀 Powered by OHC`);
    window.open(`https://twitter.com/intent/tweet?text=${text}`, '_blank');
    setUnlocked(true);
  };

  const handleCopyLink = () => {
    navigator.clipboard.writeText(shareLink);
    setCopied(true);
    setUnlocked(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-md w-full space-y-8">
        <div className="text-center">
          <div className="w-16 h-16 bg-white rounded-2xl shadow-sm flex items-center justify-center text-3xl mx-auto mb-4">
            ⏳
          </div>
          <h2 className="mt-6 text-3xl font-extrabold text-gray-900 font-outfit">Your Trial is Expiring</h2>
          <p className="mt-2 text-sm text-gray-600">
            Keep growing your business! Get an extra 14 days of Pro for free by sharing your store.
          </p>
        </div>

        <div className="mt-8 bg-white py-8 px-4 shadow-xl sm:rounded-2xl sm:px-10 border border-gray-100 relative overflow-hidden">
          {/* Background pattern */}
          <div className="absolute top-0 right-0 -mt-4 -mr-4 w-24 h-24 bg-blue-50 rounded-full opacity-50"></div>
          <div className="absolute bottom-0 left-0 -mb-4 -ml-4 w-20 h-20 bg-green-50 rounded-full opacity-50"></div>

          {unlocked ? (
            <div className="text-center relative z-10 animate-fade-in" style={{ animation: 'fadeIn 0.5s ease-out' }}>
              <div className="w-20 h-20 bg-green-100 rounded-full flex items-center justify-center text-4xl mx-auto mb-6 shadow-inner text-green-600">
                🎉
              </div>
              <h3 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Trial Extended!</h3>
              <p className="text-gray-600 mb-6">
                You've successfully unlocked 14 more days of OHC Pro.
              </p>
              <button
                onClick={() => window.location.href = '/dashboard'}
                className="w-full flex justify-center py-3 px-4 border border-transparent rounded-xl shadow-sm text-sm font-medium text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition-colors"
              >
                Go to Dashboard
              </button>
            </div>
          ) : (
            <div className="space-y-6 relative z-10">
              <div className="bg-blue-50 border border-blue-100 rounded-xl p-4">
                <h3 className="text-sm font-medium text-blue-800 mb-1">Unlock 14 Days Free</h3>
                <p className="text-xs text-blue-600">Share your store to get more time with Pro features.</p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Share on Twitter</label>
                <button
                  onClick={handleShareTwitter}
                  className="w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-xl shadow-sm text-sm font-medium text-white bg-[#1DA1F2] hover:bg-[#1a8cd8] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#1DA1F2] transition-colors"
                >
                  <svg className="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24"><path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"/></svg>
                  Share to Unlock
                </button>
              </div>

              <div className="relative">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-gray-300"></div>
                </div>
                <div className="relative flex justify-center text-sm">
                  <span className="px-2 bg-white text-gray-500">Or</span>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Copy Invite Link</label>
                <div className="flex rounded-md shadow-sm">
                  <div className="relative flex-grow focus-within:z-10">
                    <input
                      type="text"
                      readOnly
                      value={shareLink}
                      className="focus:ring-blue-500 focus:border-blue-500 block w-full rounded-none rounded-l-xl pl-3 sm:text-sm border-gray-300 bg-gray-50 text-gray-500"
                    />
                  </div>
                  <button
                    type="button"
                    onClick={handleCopyLink}
                    className={`-ml-px relative inline-flex items-center space-x-2 px-4 py-2 border border-gray-300 text-sm font-medium rounded-r-xl focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors ${copied ? 'bg-green-50 text-green-700 border-green-200' : 'bg-white text-gray-700 hover:bg-gray-50'}`}
                  >
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                </div>
                {copied && (
                   <p className="mt-2 text-xs text-green-600">Link copied! You unlocked 14 more days.</p>
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
