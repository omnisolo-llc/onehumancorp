"use client";

import React, { useState } from 'react';

export default function ViralGiveawayGenerator() {
  const [giveawayName, setGiveawayName] = useState('');
  const [prize, setPrize] = useState('');
  const [generatedLink, setGeneratedLink] = useState('');

  const handleGenerate = () => {
    if (!giveawayName || !prize) return;
    const link = `https://ohc.app/giveaway/${encodeURIComponent(giveawayName.toLowerCase().replace(/\s+/g, '-'))}`;
    setGeneratedLink(link);
  };

  return (
    <div className="min-h-screen bg-gray-50 p-4 md:p-8 font-sans">
      <div className="max-w-md mx-auto bg-white/80 backdrop-blur-md shadow-lg rounded-2xl p-6 border border-gray-100">
        <h1 className="text-2xl font-semibold text-gray-900 mb-2">Viral Giveaway Builder</h1>
        <p className="text-gray-600 mb-6 text-sm">Create a social-sharing giveaway to capture new leads instantly.</p>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Giveaway Name</label>
            <input
              type="text"
              className="w-full px-4 py-2 border border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
              placeholder="e.g. Summer Cake Box"
              value={giveawayName}
              onChange={(e) => setGiveawayName(e.target.value)}
              data-testid="giveaway-name-input"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Prize</label>
            <input
              type="text"
              className="w-full px-4 py-2 border border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
              placeholder="e.g. Free Custom Cake"
              value={prize}
              onChange={(e) => setPrize(e.target.value)}
              data-testid="prize-input"
            />
          </div>
          <button
            onClick={handleGenerate}
            disabled={!giveawayName || !prize}
            className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-medium py-3 rounded-xl transition-colors mt-2"
            data-testid="generate-button"
          >
            Generate Viral Link
          </button>
        </div>

        {generatedLink && (
          <div className="mt-8 p-4 bg-green-50 border border-green-100 rounded-xl animate-fade-in">
            <h3 className="text-green-800 font-medium mb-1">Your giveaway is ready!</h3>
            <p className="text-green-600 text-sm mb-3">Share this link to start collecting entries.</p>
            <div className="flex items-center gap-2">
              <input
                type="text"
                readOnly
                value={generatedLink}
                className="flex-1 bg-white border border-green-200 text-gray-600 text-sm px-3 py-2 rounded-lg outline-none"
                data-testid="generated-link"
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
