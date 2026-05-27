'use client';

import React, { useState } from 'react';

export default function EmailMarketingPage() {
  const [subject, setSubject] = useState('');
  const [content, setContent] = useState('');
  const [aiSuggestion, setAiSuggestion] = useState('');
  const [status, setStatus] = useState('');

  const generateAiSuggestion = () => {
    setAiSuggestion('Boost your sales with our new summer collection! 🌞');
    setContent('Check out our new arrivals...');
  };

  const handleSend = () => {
    setStatus('Campaign Sent Successfully!');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header
        className="px-6 py-4 flex items-center justify-between border-b"
        style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
          borderBottom: '1px solid rgba(255, 255, 255, 0.4)',
          position: 'sticky',
          top: 0,
          zIndex: 50,
        }}
      >
        <h1
          className="text-2xl font-bold font-outfit"
          style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}
        >
          Email Campaigns
        </h1>
        <button className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>
      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-8">
        <section className="bg-white rounded-2xl shadow-sm border p-6 border-gray-100">
          <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Create New Campaign</h2>
          <div className="flex flex-col gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Subject</label>
              <input
                type="text"
                className="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 outline-none"
                value={subject}
                onChange={(e) => setSubject(e.target.value)}
                placeholder="Enter subject line..."
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Content</label>
              <textarea
                className="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 outline-none min-h-[150px]"
                value={content}
                onChange={(e) => setContent(e.target.value)}
                placeholder="Write your email content..."
              />
            </div>

            <div className="flex justify-between items-center mt-4">
              <button
                onClick={generateAiSuggestion}
                className="px-4 py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition-colors font-medium flex items-center gap-2"
              >
                <span>✨</span> AI Suggestion
              </button>

              <button
                onClick={handleSend}
                className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-bold"
              >
                Send Campaign
              </button>
            </div>

            {aiSuggestion && (
               <div className="mt-4 p-3 bg-gray-50 border rounded-lg text-sm text-gray-600">
                 <strong>AI Suggestion:</strong> {aiSuggestion}
               </div>
            )}

            {status && (
              <div className="mt-4 p-4 bg-green-50 text-green-800 border border-green-200 rounded-lg font-medium text-center">
                {status}
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}