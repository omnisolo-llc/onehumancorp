"use client";

import { useState } from "react";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");

  const integrations = [
    { id: "ayrshare", name: "Ayrshare", category: "marketing", status: "disconnected", icon: "📱", description: "Unified Social Media Inbox and Cross-Posting across all major social networks." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync to prevent double-booking." },
    { id: "listmonk", name: "Listmonk", category: "marketing", status: "disconnected", icon: "📨", description: "Embedded, No-Jargon Email Campaigns for customer segmentation." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Expand payments with local LATAM payment methods." },
    { id: "easypost", name: "EasyPost", category: "operations", status: "disconnected", icon: "📦", description: "Painless Shipping Labels & Tracking with unified API for carriers." },
    { id: "twilio", name: "Twilio", category: "operations", status: "disconnected", icon: "🔔", description: "Global SMS Alerts & Customer Notifications for immediate order updates." },
    { id: "jitsi", name: "Jitsi Meet", category: "operations", status: "disconnected", icon: "📹", description: "Embed Jitsi Meet for Zero-Setup Online Lessons with auto-generated links." }
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
            <h1 className="text-3xl font-bold font-outfit mb-1">Tool Integrations</h1>
            <p className="text-gray-400 text-sm">Supercharge your workflow by connecting your favorite tools.</p>
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
            <div key={integration.id} className="rounded-[16px] p-6 flex flex-col transition-shadow hover:shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-white/50 rounded-xl flex items-center justify-center text-2xl border border-white/60 shadow-sm">
                  {integration.icon}
                </div>
                <span className={`text-xs font-bold px-2 py-1 rounded-md uppercase tracking-wide shadow-sm ${
                  integration.status === 'connected' ? 'bg-[#34C759]/10 text-[#34C759] border border-[#34C759]/20' : 'bg-white/50 text-[#1D1D1F] border border-white/60'
                }`}>
                  {integration.status}
                </span>
              </div>
              <h3 className="font-bold font-outfit text-[#1D1D1F] text-lg mb-2">{integration.name}</h3>
              <p className="text-[#1D1D1F]/70 text-sm mb-6 flex-1">{integration.description}</p>

              <button className={`w-full py-3 rounded-[8px] font-semibold text-sm transition-transform active:scale-[0.98] ${
                integration.status === 'connected'
                  ? "bg-white/50 text-[#1D1D1F] border border-white/60 hover:bg-white/70"
                  : "bg-[#0071E3] text-white shadow-sm hover:bg-[#0071E3]/90"
              }`}>
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
