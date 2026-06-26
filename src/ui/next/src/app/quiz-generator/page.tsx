'use client';

import React, { useState } from 'react';
import Head from 'next/head';

export default function QuizGeneratorPage() {
  const [topic, setTopic] = useState('');
  const [prize, setPrize] = useState('');
  const [generatedUrl, setGeneratedUrl] = useState('');

  const handleGenerate = () => {
    // Generate a simple link
    const url = `${window.location.origin}/quiz?topic=${encodeURIComponent(topic)}&prize=${encodeURIComponent(prize)}`;
    setGeneratedUrl(url);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-4">
      <Head>
        <title>Viral Quiz Generator</title>
      </Head>

      <div className="bg-white p-8 rounded-xl shadow-lg w-full max-w-lg border border-gray-100">
        <h1 className="text-3xl font-bold mb-6 text-gray-900 text-center">Viral Quiz Generator</h1>
        <h2 className="text-xl font-semibold mb-4 text-gray-700 border-b pb-2">Quiz Details</h2>

        <div className="space-y-4">
          <div>
            <label htmlFor="topic" className="block text-sm font-medium text-gray-700 mb-1">
              Quiz Topic
            </label>
            <input
              id="topic"
              type="text"
              value={topic}
              onChange={(e) => setTopic(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-shadow"
              placeholder="e.g. What kind of startup founder are you?"
            />
          </div>

          <div>
            <label htmlFor="prize" className="block text-sm font-medium text-gray-700 mb-1">
              Prize / Incentive (Optional)
            </label>
            <input
              id="prize"
              type="text"
              value={prize}
              onChange={(e) => setPrize(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-shadow"
              placeholder="e.g. Get a free business plan template!"
            />
          </div>

          <button
            onClick={handleGenerate}
            disabled={!topic}
            className={`w-full py-3 rounded-lg font-bold text-white transition-all shadow-md ${
              topic ? 'bg-[#0071E3] hover:bg-blue-700 active:scale-95' : 'bg-gray-400 cursor-not-allowed'
            }`}
          >
            Generate Quiz Link
          </button>
        </div>

        {generatedUrl && (
          <div className="mt-8 p-4 bg-green-50 rounded-lg border border-green-200">
            <h3 className="text-lg font-semibold text-green-800 mb-2">Link Ready!</h3>
            <p className="text-sm text-green-600 mb-3">Share this link to start collecting leads:</p>
            <div className="flex">
              <input
                type="text"
                readOnly
                value={generatedUrl}
                className="flex-1 px-3 py-2 border border-green-300 rounded-l-lg bg-white text-gray-700 text-sm focus:outline-none focus:ring-2 focus:ring-[#34C759]"
              />
              <button
                onClick={() => navigator.clipboard.writeText(generatedUrl)}
                className="px-4 py-2 bg-green-600 text-white font-semibold rounded-r-lg hover:bg-green-700 transition-colors"
              >
                Copy
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
