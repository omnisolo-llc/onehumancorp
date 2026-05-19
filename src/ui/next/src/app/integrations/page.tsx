"use client";

import React, { useState } from 'react';

// True responsive design, mobile-first
export default function IntegrationsPage() {
  const [activeTab, setActiveTab] = useState('social_media');

  const tabs = [
    { id: 'social_media', name: 'Marketing & Advertising' },
    { id: 'operations', name: 'Operations' },
    { id: 'email_marketing', name: 'Email Marketing' },
    { id: 'payment', name: 'Finance & Payments' },
    { id: 'shipping', name: 'Shipping' },
    { id: 'video', name: 'Video Services' },
  ];

  const tools = [
    { id: 'ayrshare', name: 'Ayrshare', desc: 'Unified Social Media Inbox & Cross-Posting', category: 'social_media', icon: 'M13 10V3L4 14h7v7l9-11h-7z' },
    { id: 'cal_com', name: 'Cal.com', desc: 'Zero-Config Booking & Calendar Sync', category: 'operations', icon: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' },
    { id: 'listmonk', name: 'Listmonk', desc: 'Embedded, No-Jargon Email Campaigns', category: 'email_marketing', icon: 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' },
    { id: 'mercadopago', name: 'Mercado Pago', desc: 'LATAM Local Payments Gateway', category: 'payment', icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z' },
    { id: 'easypost', name: 'EasyPost', desc: 'Painless Shipping Labels & Tracking', category: 'shipping', icon: 'M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4' },
    { id: 'twilio', name: 'Twilio', desc: 'Global SMS Alerts & Customer Notifications', category: 'operations', icon: 'M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z' },
    { id: 'jitsi', name: 'Jitsi Meet', desc: 'Zero-Setup Online Lessons', category: 'video', icon: 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z' },
  ];

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      <div className="flex flex-col flex-1 w-full max-w-4xl mx-auto bg-white shadow-xl md:my-8 md:rounded-2xl border border-gray-200 overflow-hidden relative">

        {/* Header */}
        <div className="bg-gray-900 text-white p-6 pb-4">
            <h1 className="text-2xl font-bold font-outfit mb-1">Integrations</h1>
            <p className="text-gray-400 text-sm">Manage your custom software connections here.</p>
        </div>

        {/* Tabs - Scrollable */}
        <div className="flex overflow-x-auto bg-gray-900 pb-2 hide-scrollbar px-4 space-x-2">
            {tabs.map(tab => (
                <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`whitespace-nowrap px-4 py-2 rounded-full text-sm font-medium transition-colors ${activeTab === tab.id ? 'bg-white text-gray-900' : 'bg-gray-800 text-gray-300 hover:bg-gray-700'}`}
                >
                    {tab.name}
                </button>
            ))}
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto p-4 md:p-8 bg-gray-50">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {tools.filter(t => t.category === activeTab).map(tool => (
                  <div key={tool.id} className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between glassmorphism h-full">
                      <div className="flex items-start space-x-4 mb-4">
                          <div className="w-12 h-12 bg-blue-50 text-blue-600 rounded-xl flex items-center justify-center shadow-sm flex-shrink-0">
                              <svg className="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d={tool.icon} />
                              </svg>
                          </div>
                          <div>
                              <h2 className="text-lg font-bold font-outfit text-gray-900">{tool.name}</h2>
                              <p className="text-sm text-gray-500 font-medium leading-relaxed">{tool.desc}</p>
                          </div>
                      </div>
                      <button className="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 rounded-xl shadow-sm active:scale-[0.98] transition-transform mt-auto text-sm">
                          Configure
                      </button>
                  </div>
              ))}
            </div>
            {tools.filter(t => t.category === activeTab).length === 0 && (
                <div className="text-center py-16 text-gray-500 text-sm">
                    No integrations available in this category yet.
                </div>
            )}
        </div>

        {/* Navigation Bar Mock - Mobile Only */}
        <div className="md:hidden bg-white border-t border-gray-200 flex justify-around p-4 pb-safe">
             <button className="flex flex-col items-center text-gray-400 hover:text-blue-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
                <span className="text-[10px] mt-1 font-medium">Dashboard</span>
             </button>
             <button className="flex flex-col items-center text-blue-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z" /></svg>
                <span className="text-[10px] mt-1 font-medium">Integrations</span>
             </button>
        </div>

      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); }
        .pb-safe { padding-bottom: env(safe-area-inset-bottom); }
      `}} />
    </div>
  );
}
