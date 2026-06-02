"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const router = useRouter();

  const [integrations, setIntegrations] = useState([
    { id: "ayrshare", name: "Ayrshare", category: "marketing", status: "disconnected", icon: "📱", description: "Unified API for posting and retrieving messages across social networks." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "mailerlite", name: "MailerLite", category: "marketing", status: "disconnected", icon: "📨", description: "Embedded, No-Jargon Email Campaigns." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "shippo", name: "Shippo", category: "operations", status: "disconnected", icon: "📦", description: "Painless Shipping Labels & Tracking." },
    { id: "twilio", name: "Twilio Conversations", category: "operations", status: "disconnected", icon: "🔔", description: "Unified omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat." },
    { id: "whereby", name: "Whereby", category: "operations", status: "disconnected", icon: "📹", description: "Zero-Setup Online Lessons and video conferencing." },
    { id: "resend", name: "Resend", category: "marketing", status: "disconnected", icon: "📧", description: "Transactional and Marketing Emails." },
    { id: "meta", name: "Meta Graph API", category: "social", status: "disconnected", icon: "💬", description: "Unified Instagram and Facebook Inbox." },
    { id: "front", name: "Front", category: "operations", status: "disconnected", icon: "📥", description: "Unified omnichannel inbox aggregating messages across all channels." },
    { id: "shopify", name: "Shopify Sync", category: "operations", status: "disconnected", icon: "🛍️", description: "Sync products and inventory from your existing Shopify store." },
    { id: "stripe", name: "Stripe Connect", category: "finance", status: "connected", icon: "💳", description: "Process payments securely and manage payouts." },
    { id: "calendly", name: "Calendly", category: "operations", status: "disconnected", icon: "🗓️", description: "Automated meeting scheduling and calendar sync." }
  ]);

  const [isTwilioModalOpen, setIsTwilioModalOpen] = useState(false);

  const filteredIntegrations = activeTab === "all"
    ? integrations
    : integrations.filter(i => i.category === activeTab);

  const handleConnect = (id: string) => {
    if (id === 'calendly') {
      alert("Connecting Calendly via OAuth...");
    } else if (id === 'twilio') {
      setIsTwilioModalOpen(true);
    } else if (id === 'stripe') {
      router.push('/smart-pricing');
    } else if (id === 'ayrshare') {
      alert("Connecting Ayrshare via OAuth...");
    } else if (id === 'meta') {
       router.push('/inbox');
    } else if (id === 'front') {
       router.push('/inbox');
    } else if (id === 'shopify') {
       router.push('/inventory');
    } else {
      alert(`Connecting ${id} via OAuth...`);
    }

    setIntegrations(integrations.map(i =>
      i.id === id ? { ...i, status: 'connected' } : i
    ));
  };

  const handleDisconnect = (id: string) => {
    setIntegrations(integrations.map(i =>
      i.id === id ? { ...i, status: 'disconnected' } : i
    ));
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      {/* Twilio Conversations Connect Modal */}
      {isTwilioModalOpen && (
        <div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm">
          <div className="bg-white rounded-2xl w-full max-w-md overflow-hidden shadow-2xl relative">
            <button
              onClick={() => setIsTwilioModalOpen(false)}
              className="absolute top-4 right-4 text-gray-400 hover:text-gray-600 p-1"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
            <div className="p-6">
               <div className="w-12 h-12 rounded-xl bg-blue-50 text-blue-600 flex items-center justify-center text-2xl mb-4 border border-blue-100">
                  🔔
               </div>
               <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Connect Twilio Conversations</h2>
               <p className="text-gray-500 text-sm mb-6 leading-relaxed">
                  Enter your Twilio API credentials to enable the unified omnichannel inbox for SMS, WhatsApp, and Web Chat.
               </p>

               <div className="space-y-4">
                  <div>
                    <label className="block text-xs font-bold text-gray-700 uppercase tracking-wide mb-1.5">Account SID</label>
                    <input type="text" className="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" placeholder="ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" />
                  </div>
                  <div>
                    <label className="block text-xs font-bold text-gray-700 uppercase tracking-wide mb-1.5">Auth Token</label>
                    <input type="password" className="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" placeholder="••••••••••••••••••••••••••••••••" />
                  </div>
                  <div>
                    <label className="block text-xs font-bold text-gray-700 uppercase tracking-wide mb-1.5">Conversations Service SID</label>
                    <input type="text" className="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" placeholder="ISxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" />
                  </div>
               </div>

               <button
                  onClick={() => setIsTwilioModalOpen(false)}
                  className="w-full mt-6 bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 rounded-xl transition-colors shadow-sm"
               >
                  Save & Connect
               </button>
            </div>
          </div>
        </div>
      )}
      <div className="bg-gray-900 text-white pt-12 pb-6 px-6 md:px-8 border-b border-gray-800 relative overflow-hidden">
        <div className="absolute inset-0 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')] opacity-10"></div>
        <div className="max-w-5xl mx-auto flex items-center justify-between relative z-10">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <span className="bg-gradient-to-r from-yellow-300 to-yellow-500 text-black text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wide">Premium</span>
            </div>
            <h1 className="text-3xl font-bold font-outfit mb-1">Tool Integrations</h1>
            <h2 className="sr-only">Connect Custom Software</h2>
            <h2 className="sr-only">Social Media Accounts</h2>
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
          {["all", "marketing", "operations", "finance", "social"].map(tab => (
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

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredIntegrations.map((integration) => (
            <div
              key={integration.id}
              className="bg-white rounded-2xl p-6 border border-gray-100 shadow-sm hover:shadow-md transition-shadow relative overflow-hidden group"
            >
              {integration.status === 'connected' && (
                <div className="absolute top-0 right-0 w-16 h-16 overflow-hidden">
                  <div className="absolute top-4 -right-4 bg-green-500 text-white text-[10px] font-bold py-1 px-8 transform rotate-45">
                    SYNCED
                  </div>
                </div>
              )}

              <div className="flex items-start gap-4 mb-4">
                <div className="w-12 h-12 rounded-xl bg-gray-50 border border-gray-100 flex items-center justify-center text-2xl shrink-0 group-hover:scale-105 transition-transform">
                  {integration.icon}
                </div>
                <div>
                  <h3 className="font-bold font-outfit text-gray-900 text-lg">{integration.name}</h3>
                  <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">{integration.category}</span>
                </div>
              </div>

              <p className="text-gray-600 text-sm mb-6 leading-relaxed line-clamp-3">
                {integration.description}
              </p>

              <div className="mt-auto pt-4 border-t border-gray-50 flex gap-2">
                <button
                  onClick={() => handleConnect(integration.id)}
                  className={`flex-1 py-2.5 rounded-xl text-sm font-bold transition-colors ${
                    integration.status === 'connected'
                      ? 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                      : 'bg-gray-900 text-white hover:bg-gray-800 shadow-sm'
                  }`}
                >
                  {integration.status === 'connected' ? 'Manage' : 'Connect'}
                </button>
                {integration.status === 'connected' && (
                   <button
                     onClick={() => handleDisconnect(integration.id)}
                     className="px-4 py-2.5 rounded-xl text-sm font-bold bg-white border border-gray-200 text-red-600 hover:bg-red-50 transition-colors"
                   >
                     Disconnect
                   </button>
                )}
              </div>
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
