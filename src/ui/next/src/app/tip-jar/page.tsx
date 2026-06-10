'use client';

import '../globals.css';


import React, { useState } from 'react';
import Link from 'next/link'; // this was imported differently before, let's just use Next Link
import { AppShell } from '../components/AppShell';

export default function TipJarGeneratorPage() {
  const [message, setMessage] = useState('');
  const [showBranding, setShowBranding] = useState(true);
  const [generatedLink, setGeneratedLink] = useState('');

  const handleGenerate = () => {
    const data = btoa(JSON.stringify({ message, showBranding }));
    setGeneratedLink(`/tip-jar/view?data=${data}`);
  };

  return (
    <AppShell title="Tip Jar">
      <main className="max-w-4xl mx-auto px-4 py-8 relative z-10">
        <h1 className="text-3xl font-bold mb-4 font-outfit">Create Your Tip Jar</h1>
        <div className="glassmorphism p-6 rounded-[16px] mb-8">
          <label className="block text-sm font-medium mb-2">Message</label>
          <textarea
            className="w-full bg-white/5 border border-white/10 rounded-lg p-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 mb-4"
            rows={4}
            placeholder="e.g. Buy me a coffee!"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
          />
          <label className="flex items-center mb-6 cursor-pointer">
            <input
              type="checkbox"
              className="form-checkbox h-5 w-5 text-purple-600 rounded border-white/20 bg-white/5 focus:ring-purple-500 focus:ring-offset-gray-900"
              checked={!showBranding}
              onChange={(e) => setShowBranding(!e.target.checked)}
            />
            <span className="ml-3 text-sm text-gray-300">Remove "Powered by OHC" Badge</span>
          </label>
          <button
            onClick={handleGenerate}
            className="w-full bg-gradient-to-r from-purple-600 to-indigo-600 text-white font-medium py-3 px-4 rounded-lg hover:from-purple-500 hover:to-indigo-500 transition-all focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-gray-900"
          >
            Generate Tip Jar Link
          </button>
        </div>

        {generatedLink && (
          <div className="glassmorphism p-6 rounded-[16px] animate-fade-in-up">
            <h2 className="text-2xl font-bold mb-4 font-outfit text-white">Your Tip Jar is Ready!</h2>
            <div className="flex flex-col sm:flex-row gap-4 items-center">
              <input
                type="text"
                readOnly
                value={`${window.location.origin}${generatedLink}`}
                className="flex-1 bg-white/5 border border-white/10 rounded-lg p-3 text-white focus:outline-none"
              />
              {/* @ts-ignore */}
              <Link
                href={generatedLink}
                className="bg-white/10 hover:bg-white/20 text-white font-medium py-3 px-6 rounded-lg transition-all whitespace-nowrap"
              >
                Preview Tip Jar
              </Link>
            </div>
          </div>
        )}
      </main>
    </AppShell>
  );
}
