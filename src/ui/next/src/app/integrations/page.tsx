"use client";

import { useState } from "react";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const [darkMode, setDarkMode] = useState(false);

  const integrations = [
    { id: "chatwoot", name: "Chatwoot", category: "operations", status: "disconnected", icon: "💬", description: "Unified Social Media Inbox for Instagram, Facebook, and WhatsApp." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Smart Calendar Sync & Booking Pages without double-booking." },
    { id: "resend", name: "Resend", category: "marketing", status: "disconnected", icon: "📧", description: "Integrated Customer Email Campaigns. Simple and fast." },
    { id: "stripe", name: "Stripe", category: "finance", status: "connected", icon: "💳", description: "Global Payment Collection Links for credit cards and Apple Pay." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Alternative payment links for Latin America." },
    { id: "shippo", name: "Shippo", category: "operations", status: "disconnected", icon: "📦", description: "Instant Shipping Label Generation with live carrier rates." },
    { id: "twilio", name: "Twilio", category: "operations", status: "disconnected", icon: "🔔", description: "Reliable Customer SMS Notifications." },
    { id: "zoom", name: "Zoom", category: "operations", status: "disconnected", icon: "📹", description: "Auto-Generated Video Meeting Links for appointments." },
    { id: "google_meet", name: "Google Meet", category: "operations", status: "disconnected", icon: "🎥", description: "Seamless Google Calendar video integration." }
  ];

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  // Dynamic Glassmorphism styles
  const glassCardClass = darkMode
    ? "glass-card-dark"
    : "glass-card-light";

  const buttonClass = darkMode
    ? "bg-[#0066FF] hover:bg-[#0071E3] text-white"
    : "bg-[#0066FF] hover:bg-[#0071E3] text-white";

  const buttonConnectedClass = darkMode
    ? "bg-[rgba(255,255,255,0.1)] text-[#F5F5F7] border border-[rgba(255,255,255,0.1)] hover:bg-[rgba(255,255,255,0.2)]"
    : "bg-[rgba(255,255,255,0.6)] text-[#1D1D1F] border border-[rgba(255,255,255,0.4)] hover:bg-[rgba(255,255,255,1)]";

  return (
    <div className={`flex flex-col min-h-screen font-inter transition-colors duration-300 ${darkMode ? 'bg-[#1D1D1F] text-[#F5F5F7]' : 'bg-[#F5F5F7] text-[#1D1D1F]'}`}>

      {/* Premium Dashboard Header */}
      <div className={`${darkMode ? 'bg-[#16161A]' : 'bg-gray-900'} text-white px-6 py-8 shadow-md`}>
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <span className="bg-[#0066FF] text-white text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wide">Premium Dashboard</span>
            </div>
            <h1 className="text-3xl font-bold font-outfit mb-1">Tool Integrations</h1>
            <p className="text-gray-400 text-sm">Supercharge your workflow by connecting your favorite tools.</p>
          </div>
          <div className="flex gap-4 items-center">
            <button
              onClick={() => setDarkMode(!darkMode)}
              className="text-sm px-3 py-1 rounded-[8px] border border-white/20 bg-white/10 hover:bg-white/20 transition-colors"
              aria-label="Toggle Dark Mode"
            >
              {darkMode ? 'Light Mode' : 'Dark Mode'}
            </button>
            <div className="hidden md:flex w-16 h-16 bg-white/10 rounded-[16px] border border-white/20 items-center justify-center text-3xl">
              🧩
            </div>
          </div>
        </div>
      </div>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full relative">

        {/* Navigation Tabs */}
        <div className="flex gap-4 mb-8 pb-4 overflow-x-auto hide-scrollbar">
          {["all", "marketing", "operations", "finance"].map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 rounded-[8px] text-sm font-semibold whitespace-nowrap transition-colors ${
                activeTab === tab
                  ? (darkMode ? "bg-[#0066FF] text-white" : "bg-[#0066FF] text-white")
                  : (darkMode ? "bg-[rgba(22,22,26,0.7)] text-gray-300 border border-[rgba(255,255,255,0.1)] hover:bg-white/10" : "bg-[rgba(255,255,255,0.6)] text-gray-600 border border-[rgba(255,255,255,0.4)] hover:bg-white")
              }`}
            >
              {tab.charAt(0).toUpperCase() + tab.slice(1)}
            </button>
          ))}
        </div>

        {/* Integration Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredIntegrations.map(integration => (
            <div key={integration.id} data-testid={`integration-card-${integration.id}`} className={`${glassCardClass} p-6 flex flex-col transition-shadow hover:shadow-lg`}>
              <div className="flex justify-between items-start mb-4">
                <div className={`w-12 h-12 rounded-[8px] flex items-center justify-center text-2xl ${darkMode ? 'bg-white/10 border border-white/10' : 'bg-white border border-[rgba(255,255,255,0.4)] shadow-sm'}`}>
                  {integration.icon}
                </div>
                <span className={`text-xs font-bold px-2 py-1 rounded-[8px] uppercase tracking-wide ${
                  integration.status === 'connected'
                    ? (darkMode ? 'bg-[#00C24B]/20 text-[#34C759]' : 'bg-[#34C759]/10 text-[#34C759]')
                    : (darkMode ? 'bg-white/10 text-gray-400' : 'bg-gray-100 text-gray-500')
                }`}>
                  {integration.status}
                </span>
              </div>
              <h3 className={`font-bold font-outfit text-lg mb-2 ${darkMode ? 'text-[#F5F5F7]' : 'text-gray-900'}`}>{integration.name}</h3>
              <p className={`text-sm mb-6 flex-1 ${darkMode ? 'text-gray-400' : 'text-gray-500'}`}>{integration.description}</p>

              <button className={`w-full py-3 rounded-[8px] font-semibold text-sm transition-transform active:scale-[0.98] ${
                integration.status === 'connected' ? buttonConnectedClass : buttonClass
              }`}>
                {integration.status === 'connected' ? 'Manage Settings' : 'Connect'}
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

        .glass-card-light {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          border: 1px solid rgba(255, 255, 255, 0.4);
          border-radius: 16px;
        }

        .glass-card-dark {
          background: rgba(22, 22, 26, 0.7);
          backdrop-filter: blur(30px) saturate(210%);
          border: 1px solid rgba(255, 255, 255, 0.1);
          border-radius: 16px;
        }
      `}} />
    </div>
  );
}
