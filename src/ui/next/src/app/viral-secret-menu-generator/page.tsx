'use client';

import React, { useState } from 'react';

import { useEffect } from 'react';

export default function ViralSecretMenuGenerator() {
  const [itemName, setItemName] = useState('');
  const [itemDesc, setItemDesc] = useState('');
  const [accessCode, setAccessCode] = useState('');
  const [sharesReq, setSharesReq] = useState('');
  const [copied, setCopied] = useState(false);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Generate the URL based on the current inputs
  const previewUrl = `/api/v1/growth/secret-menu/embed?item_name=${encodeURIComponent(
    itemName
  )}&item_desc=${encodeURIComponent(itemDesc)}&access_code=${encodeURIComponent(
    accessCode
  )}&shares_req=${encodeURIComponent(sharesReq)}`;

  const absoluteUrl = mounted ? `${window.location.origin}${previewUrl}` : previewUrl;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(absoluteUrl);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] text-[#1D1D1F] dark:text-[#F5F5F7] font-sans p-8">
      <div className="max-w-4xl mx-auto bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:border-white/10 rounded-[20px] p-8 shadow-sm">
        <h1 className="text-3xl font-bold mb-6 font-outfit text-pink-600 dark:text-pink-400">
          Viral Secret Menu Generator 🤫
        </h1>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-semibold mb-1" htmlFor="itemName">
                Item Name
              </label>
              <input
                id="itemName"
                type="text"
                className="w-full rounded-[12px] bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-gray-700 p-3"
                value={itemName}
                onChange={(e) => setItemName(e.target.value)}
                placeholder="e.g. Double Smash Burger"
              />
            </div>

            <div>
              <label className="block text-sm font-semibold mb-1" htmlFor="itemDesc">
                Item Description
              </label>
              <input
                id="itemDesc"
                type="text"
                className="w-full rounded-[12px] bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-gray-700 p-3"
                value={itemDesc}
                onChange={(e) => setItemDesc(e.target.value)}
                placeholder="e.g. Extra cheese, extra smash."
              />
            </div>

            <div>
              <label className="block text-sm font-semibold mb-1" htmlFor="accessCode">
                Access Code
              </label>
              <input
                id="accessCode"
                type="text"
                className="w-full rounded-[12px] bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-gray-700 p-3"
                value={accessCode}
                onChange={(e) => setAccessCode(e.target.value)}
                placeholder="e.g. SMASHX2"
              />
            </div>

            <div>
              <label className="block text-sm font-semibold mb-1" htmlFor="sharesReq">
                Shares Required
              </label>
              <input
                id="sharesReq"
                type="number"
                className="w-full rounded-[12px] bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-gray-700 p-3"
                value={sharesReq}
                onChange={(e) => setSharesReq(e.target.value)}
                placeholder="e.g. 4"
              />
            </div>

            <div className="mt-6">
              <h3 className="text-lg font-semibold mb-2">Share Link</h3>
              <div className="flex items-center gap-2">
                <div
                  id="shareLink"
                  className="flex-1 bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-gray-700 p-3 rounded-[12px] overflow-hidden text-ellipsis whitespace-nowrap"
                  title={absoluteUrl}
                >
                  {absoluteUrl}
                </div>
                <button
                  id="copyBtn"
                  onClick={handleCopy}
                  className="bg-pink-600 hover:bg-pink-700 text-white font-semibold py-3 px-6 rounded-[12px] transition-colors"
                >
                  {copied ? 'Copied!' : 'Copy Link'}
                </button>
              </div>
            </div>
          </div>

          <div>
            <h3 className="text-lg font-semibold mb-2">Live Preview</h3>
            <div className="w-full h-[500px] border border-gray-200 dark:border-gray-700 rounded-[12px] overflow-hidden bg-white dark:bg-[#1D1D1F]">
              <iframe
                id="previewFrame"
                src={previewUrl}
                className="w-full h-full border-none"
                title="Viral Secret Menu Preview"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}