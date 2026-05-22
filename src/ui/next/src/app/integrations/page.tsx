"use client";

import { useState } from "react";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");

  const integrations = [
    { id: "social", name: "Social Media Auto-Poster", category: "marketing", status: "connected", icon: "📱", description: "Automatically post product updates to Facebook, Instagram, and Twitter." },
    { id: "shipping", name: "ShipStation Sync", category: "operations", status: "disconnected", icon: "📦", description: "Sync orders with ShipStation for automated label printing." },
    { id: "mailchimp", name: "Mailchimp Automations", category: "marketing", status: "disconnected", icon: "📧", description: "Trigger email campaigns based on customer purchase behavior." },
    { id: "quickbooks", name: "QuickBooks Online", category: "finance", status: "connected", icon: "📊", description: "Automatically sync sales data and expenses for easier accounting." },
    { id: "stripe", name: "Stripe Advanced", category: "finance", status: "connected", icon: "💳", description: "Accept global payments and manage subscriptions seamlessly." },
    { id: "zendesk", name: "Zendesk Support", category: "operations", status: "disconnected", icon: "🎧", description: "Turn customer inquiries into support tickets automatically." },
    { id: "manychat", name: "Manychat", category: "marketing", status: "disconnected", icon: "💬", description: "Unified inbox for Instagram, Messenger, and WhatsApp." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "twilio", name: "Twilio", category: "operations", status: "disconnected", icon: "🔔", description: "Reliable SMS alerts for new orders and customer notifications." },
    { id: "ayrshare", name: "Ayrshare", category: "marketing", status: "disconnected", icon: "📱", description: "Unified tool for posting and retrieving messages across social networks." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "listmonk", name: "Listmonk", category: "marketing", status: "disconnected", icon: "📨", description: "Embedded, No-Jargon Email Campaigns." },
    { id: "easypost", name: "EasyPost", category: "operations", status: "disconnected", icon: "📦", description: "Painless Shipping Labels & Tracking." },
    { id: "jitsi", name: "Jitsi Meet", category: "operations", status: "disconnected", icon: "📹", description: "Zero-Setup Online Lessons and video conferencing." }
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
            <div key={integration.id} className="bg-white rounded-2xl p-6 border border-gray-200 shadow-sm flex flex-col transition-shadow hover:shadow-md">
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

              <button className={`w-full py-3 rounded-xl font-semibold text-sm transition-transform active:scale-[0.98] ${
                integration.status === 'connected'
                  ? "bg-gray-50 text-gray-700 border border-gray-200 hover:bg-gray-100"
                  : "bg-blue-600 text-white shadow-sm hover:bg-blue-700"
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
