"use client";

import { useState } from "react";
import Link from 'next/link';

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const [showTwilioModal, setShowTwilioModal] = useState(false);
  const [twilioConfig, setTwilioConfig] = useState({ bot_token: '', api_token: '' });
  const [isConnecting, setIsConnecting] = useState(false);
  const [twilioStatus, setTwilioStatus] = useState("disconnected");

  const integrations = [
    { id: "ayrshare", name: "Ayrshare", category: "marketing", status: "disconnected", icon: "📱", description: "Unified API for posting and retrieving messages across social networks." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "listmonk", name: "Listmonk", category: "marketing", status: "disconnected", icon: "📨", description: "Embedded, No-Jargon Email Campaigns." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "easypost", name: "EasyPost", category: "operations", status: "disconnected", icon: "📦", description: "Painless Shipping Labels & Tracking." },
    { id: "twilio", name: "Twilio Conversations", category: "operations", status: twilioStatus, icon: "🔔", description: "Omnichannel Inbox for WhatsApp, Instagram, Facebook and SMS." },
    { id: "jitsi", name: "Jitsi Meet", category: "operations", status: "disconnected", icon: "📹", description: "Zero-Setup Online Lessons and video conferencing." }
  ];

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  const handleConnectTwilio = async () => {
    setIsConnecting(true);
    try {
      const response = await fetch('/api/integrations/twilio/connect', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(twilioConfig)
      });
      if (response.ok) {
        setTwilioStatus("connected");
        setShowTwilioModal(false);
        alert("Twilio Omnichannel connected successfully!");
      } else {
        alert("Failed to connect Twilio. Check your credentials.");
      }
    } catch (e) {
      alert("Network error. Please try again.");
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>

      {/* Premium Dashboard Header */}
      <div className="bg-gradient-to-r from-gray-900 to-black text-white px-6 py-8 shadow-md">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <span className="bg-gradient-to-r from-yellow-300 to-yellow-500 text-black text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wide">Premium</span>
              <Link href="/dashboard" className="text-gray-400 hover:text-white ml-4 text-sm">&larr; Back</Link>
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
            <div key={integration.id}
                 className="rounded-[16px] p-6 shadow-sm flex flex-col transition-shadow hover:shadow-md relative overflow-hidden"
                 style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}
            >
              {integration.id === 'twilio' && (
                <div className="absolute top-0 right-0 bg-blue-500 text-white text-[10px] font-bold px-3 py-1 rounded-bl-lg">
                  RECOMMENDED
                </div>
              )}
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

              <button
                onClick={() => {
                  if (integration.id === 'twilio') setShowTwilioModal(true);
                  else alert(`Connecting to ${integration.name}...`);
                }}
                className={`w-full py-3 rounded-[8px] font-semibold text-sm transition-transform active:scale-[0.98] ${
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

      {/* Twilio Modal */}
      {showTwilioModal && (
        <div className="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-2xl w-full max-w-md shadow-2xl overflow-hidden flex flex-col font-inter">
            <div className="p-6 border-b border-gray-100">
              <h2 className="text-2xl font-bold font-outfit text-gray-900">Connect Omnichannel Inbox</h2>
              <p className="text-sm text-gray-500 mt-1">Configure your Twilio Conversations API to route WhatsApp, Instagram, and SMS into one unified inbox.</p>
            </div>

            <div className="p-6 flex-1 overflow-y-auto">
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-1">Twilio Account SID (Bot Token)</label>
                  <input
                    type="text"
                    className="w-full border border-gray-300 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 outline-none text-black bg-white"
                    placeholder="AC..."
                    value={twilioConfig.bot_token}
                    onChange={e => setTwilioConfig({...twilioConfig, bot_token: e.target.value})}
                  />
                </div>
                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-1">Twilio Auth Token (API Token)</label>
                  <input
                    type="password"
                    className="w-full border border-gray-300 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 outline-none text-black bg-white"
                    placeholder="••••••••••••"
                    value={twilioConfig.api_token}
                    onChange={e => setTwilioConfig({...twilioConfig, api_token: e.target.value})}
                  />
                </div>

                <div className="pt-4 border-t border-gray-100">
                  <h3 className="text-sm font-semibold text-gray-700 mb-3">Enabled Channels</h3>
                  <div className="space-y-3">
                    <label className="flex items-center gap-3 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer">
                      <input type="checkbox" defaultChecked className="w-4 h-4 text-blue-600 rounded border-gray-300" />
                      <span className="flex-1 text-sm font-medium text-black">WhatsApp Business</span>
                      <span className="text-xl">💬</span>
                    </label>
                    <label className="flex items-center gap-3 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer">
                      <input type="checkbox" defaultChecked className="w-4 h-4 text-blue-600 rounded border-gray-300" />
                      <span className="flex-1 text-sm font-medium text-black">Instagram Direct</span>
                      <span className="text-xl">📸</span>
                    </label>
                    <label className="flex items-center gap-3 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer">
                      <input type="checkbox" defaultChecked className="w-4 h-4 text-blue-600 rounded border-gray-300" />
                      <span className="flex-1 text-sm font-medium text-black">Facebook Messenger</span>
                      <span className="text-xl">📘</span>
                    </label>
                  </div>
                </div>
              </div>
            </div>

            <div className="p-6 bg-gray-50 border-t border-gray-100 flex justify-end gap-3">
              <button
                onClick={() => setShowTwilioModal(false)}
                className="px-5 py-2.5 rounded-lg text-sm font-semibold text-gray-700 hover:bg-gray-200 transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleConnectTwilio}
                disabled={isConnecting}
                className="px-5 py-2.5 rounded-lg text-sm font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-sm disabled:opacity-50"
              >
                {isConnecting ? 'Connecting...' : 'Connect to Inbox'}
              </button>
            </div>
          </div>
        </div>
      )}

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
