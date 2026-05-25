"use client";

import { useState } from "react";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");

  const integrations = [
    { id: "meta", name: "Connect my Instagram", category: "marketing", status: "disconnected", icon: "📱", description: "Read your Instagram DMs and let the AI reply to customers automatically." },
    { id: "cal_com", name: "My Booking Calendar", category: "operations", status: "disconnected", icon: "📅", description: "Let customers book appointments and sync them to your phone." },
    { id: "resend", name: "Email Customers", category: "marketing", status: "disconnected", icon: "📨", description: "Send automated updates and newsletters to your customers." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "shippo", name: "Shipping Labels", category: "operations", status: "disconnected", icon: "📦", description: "Automatically generate and print shipping labels." },
    { id: "twilio", name: "Get order notifications", category: "operations", status: "disconnected", icon: "🔔", description: "Get text messages on your phone when you receive a new order." },
    { id: "zoom", name: "Online Meetings", category: "operations", status: "disconnected", icon: "📹", description: "Automatically create meeting links for online classes or services." }
  ];

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>

      {/* Premium Dashboard Header */}
      <div className="bg-gradient-to-r from-gray-900 to-black text-white px-6 py-8 shadow-md">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <span className="bg-gradient-to-r from-yellow-300 to-yellow-500 text-black text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wide">Premium</span>
            </div>
            <h1 className="text-3xl font-bold font-outfit mb-1">Connect Custom Software</h1>
            <p className="text-gray-400 text-sm">Supercharge your workflow by connecting your favorite software tools.</p>
          </div>
          <div className="hidden md:block w-16 h-16 bg-white/10 rounded-2xl border border-white/20 flex items-center justify-center text-3xl">
            🧩
          </div>
        </div>
      </div>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full">

        {/* Navigation Tabs */}
        <div className="flex gap-4 mb-8 border-b border-gray-200 pb-4 overflow-x-auto hide-scrollbar">
          {["all", "marketing", "operations", "finance"].map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 rounded-full text-sm font-semibold whitespace-nowrap transition-colors ${
                activeTab === tab
                  ? "bg-gray-900 text-white"
                  : "bg-white text-gray-600 border border-gray-200 hover:bg-gray-50"
              }`}
            >
              {tab.charAt(0).toUpperCase() + tab.slice(1)}
            </button>
          ))}
        </div>

        {/* Integration Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredIntegrations.map(integration => (
            <div key={integration.id}
                 className="rounded-[16px] p-6 shadow-sm flex flex-col transition-shadow hover:shadow-md"
                 style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}
            >
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-gray-50 rounded-xl flex items-center justify-center text-2xl border border-gray-100">
                  {integration.icon}
                </div>
                <span className={`text-xs font-bold px-2 py-1 rounded-md uppercase tracking-wide ${
                  integration.status === 'connected' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-500'
                }`}>
                  {integration.status}
                </span>
              </div>
              <h3 className="font-bold font-outfit text-gray-900 text-lg mb-2">{integration.name}</h3>
              <p className="text-gray-500 text-sm mb-6 flex-1">{integration.description}</p>

              <button className={`w-full py-3 rounded-[8px] font-semibold text-sm transition-transform active:scale-[0.98] ${
                integration.status === 'connected'
                  ? "bg-gray-50 text-gray-700 border border-gray-200 hover:bg-gray-100"
                  : "text-[#F5F5F7] shadow-sm hover:bg-[#005bd3]"
              }`} style={integration.status === 'connected' ? {} : { background: '#0066FF' }}>
                {integration.status === 'connected' ? 'Manage' : 'Connect'}
              </button>
            </div>
          ))}
        </div>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
      `}} />
    </div>
  );
}
