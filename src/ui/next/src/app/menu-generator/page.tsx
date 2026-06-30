"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function MenuGeneratorPage() {
  const router = useRouter();
  const [restaurantName, setRestaurantName] = useState('');
  const [description, setDescription] = useState('');
  const [menuItemsText, setMenuItemsText] = useState('');
  const [tenant, setTenant] = useState('DEFAULT');
  const [menuLink, setMenuLink] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
    }
  }, []);

  const handleGenerate = async () => {
    setIsGenerating(true);

    try {
      const response = await fetch('/api/v1/growth/campaign/generate-menu', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          restaurantName,
          description,
          menuItemsText,
          tenantId: tenant,
        }),
      });

      if (response.ok) {
        const data = await response.json();
        const publicUrl = `${window.location.origin}${data.url}`;
        setMenuLink(publicUrl);
      } else {
        console.error("Failed to generate menu");
      }
    } catch (error) {
      console.error("Error generating menu:", error);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleCopy = () => {
    if (navigator.clipboard && menuLink) {
      navigator.clipboard.writeText(`${menuLink}\n\n⚡ Powered by OHC`);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <div className="flex flex-col min-h-screen bg-[#F5F5F7] dark:bg-[#000000] font-inter">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border-white/40 dark:border-white/10">
        <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">Viral AI Menu Generator 🪄</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 dark:bg-gray-800 rounded-md text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-700 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-3xl mx-auto">
        <div className="glassmorphism p-6 md:p-8 border border-white/40 dark:border-white/10 shadow-lg mb-8 bg-white/50 dark:bg-black/30">
          <div className="mb-6">
            <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Menu Details</h2>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Enter your restaurant details. We'll generate a beautiful digital menu with a built-in viral loop to help you get more customers.
            </p>
          </div>

          <div className="flex flex-col gap-5">
            <div>
              <label htmlFor="restaurant-name" className="block text-sm font-semibold text-gray-800 dark:text-gray-200 mb-1">
                Restaurant / Store Name
              </label>
              <input
                id="restaurant-name"
                type="text"
                placeholder="e.g. Maya's Artisan Bakery"
                value={restaurantName}
                onChange={(e) => setRestaurantName(e.target.value)}
                className="w-full px-4 py-2 rounded-xl bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label htmlFor="restaurant-desc" className="block text-sm font-semibold text-gray-800 dark:text-gray-200 mb-1">
                Short Description
              </label>
              <input
                id="restaurant-desc"
                type="text"
                placeholder="e.g. Freshly baked goods made with love."
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="w-full px-4 py-2 rounded-xl bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label htmlFor="menu-items" className="block text-sm font-semibold text-gray-800 dark:text-gray-200 mb-1">
                Menu Items (Format: Item Name - Price)
              </label>
              <textarea
                id="menu-items"
                placeholder="Sourdough Bread - $8&#10;Chocolate Chip Cookie - $3&#10;Cold Brew Coffee - $4"
                rows={4}
                value={menuItemsText}
                onChange={(e) => setMenuItemsText(e.target.value)}
                className="w-full px-4 py-2 rounded-xl bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
              />
            </div>

            <button
              onClick={handleGenerate}
              disabled={isGenerating || !restaurantName || !menuItemsText}
              className={`w-full py-3 rounded-xl font-bold font-outfit shadow-md transition-all flex items-center justify-center gap-2
                ${(isGenerating || !restaurantName || !menuItemsText)
                  ? 'bg-gray-300 dark:bg-gray-700 text-gray-500 cursor-not-allowed'
                  : 'bg-indigo-600 hover:bg-indigo-700 text-white hover:shadow-lg hover:-translate-y-0.5'}`}
            >
              {isGenerating ? 'Generating Menu...' : 'Generate AI Menu Link'}
            </button>
          </div>
        </div>

        {menuLink && (
          <div className="glassmorphism p-6 border border-green-200 dark:border-green-900/30 shadow-lg bg-green-50/50 dark:bg-green-900/10 animate-fade-in">
            <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-2">Link Ready! 🎉</h3>
            <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">
              Your digital menu is ready to be shared. Customers who view it will see a built-in referral link!
            </p>

            <div className="flex items-center gap-2 bg-white dark:bg-black/40 p-2 rounded-xl border border-green-200 dark:border-green-800">
              <input
                type="text"
                readOnly
                value={menuLink}
                className="bg-transparent border-none outline-none text-sm w-full text-gray-700 dark:text-gray-200 px-2"
              />
              <button
                onClick={handleCopy}
                className={`px-4 py-2 min-w-[100px] text-sm font-bold rounded-lg transition-all ${copied ? 'bg-[#34C759] text-white' : 'bg-green-100 text-green-700 hover:bg-green-200 dark:bg-green-800 dark:text-green-100'}`}
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
              <a
                href={menuLink}
                target="_blank"
                rel="noopener noreferrer"
                className="px-4 py-2 text-sm font-bold rounded-lg bg-indigo-600 text-white hover:bg-indigo-700 transition-all flex items-center gap-1"
              >
                Preview <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" /></svg>
              </a>
            </div>
          </div>
        )}

        <div className="mt-6 text-center">
          <a href="/onboarding?ref=menu" target="_blank" className="text-xs font-semibold text-gray-500 hover:text-gray-700">⚡ Powered by OHC</a>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fade-in { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fade-in 0.3s ease-out forwards; }
      `}} />
    </div>
  );
}
