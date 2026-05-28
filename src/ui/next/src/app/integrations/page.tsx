"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const router = useRouter();

  const [integrations, setIntegrations] = useState([
    { id: "calendly", name: "Calendly", category: "operations", status: "disconnected", icon: "📅", description: "Automated Booking widget for your store." },
    { id: "manychat", name: "Manychat", category: "operations", status: "disconnected", icon: "💬", description: "Unified social media inbox for Instagram, Facebook, and WhatsApp." },
    { id: "ayrshare", name: "Ayrshare", category: "marketing", status: "disconnected", icon: "📱", description: "Unified API for posting and retrieving messages across social networks." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "listmonk", name: "Listmonk", category: "marketing", status: "disconnected", icon: "📨", description: "Embedded, No-Jargon Email Campaigns." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "easypost", name: "EasyPost", category: "operations", status: "disconnected", icon: "📦", description: "Painless Shipping Labels & Tracking." },
    { id: "twilio", name: "Twilio Conversations", category: "operations", status: "disconnected", icon: "🔔", description: "Unified omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat." },
    { id: "jitsi", name: "Jitsi Meet", category: "operations", status: "disconnected", icon: "📹", description: "Zero-Setup Online Lessons and video conferencing." }
  ]);

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  const [showTwilioModal, setShowTwilioModal] = useState(false);
  const [twilioChannels, setTwilioChannels] = useState({
    whatsapp: true,
    instagram: false,
    facebook: false,
    sms: true,
  });

  const handleConnect = (id: string) => {
    if (id === 'calendly') {
      alert("Connecting Calendly via OAuth...");
      setIntegrations(prev => prev.map(integration =>
        integration.id === id ? { ...integration, status: "connected" } : integration
      ));
      router.push("/dashboard");
    }
    if (id === 'manychat') {
      alert("Connecting Manychat via OAuth...");
      setIntegrations(prev => prev.map(integration =>
        integration.id === id ? { ...integration, status: "connected" } : integration
      ));
      router.push('/inbox');
    }
    if (id === 'twilio') {
      setShowTwilioModal(true);
    }
  };

  const saveTwilioIntegration = () => {
    setIntegrations(prev => prev.map(integration =>
      integration.id === 'twilio' ? { ...integration, status: "connected" } : integration
    ));
    setShowTwilioModal(false);
    router.push('/inbox');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>

      {/* Twilio Conversations Connect Modal */}
      {showTwilioModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-sm">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter">
            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-blue-50 rounded-xl flex items-center justify-center text-2xl text-blue-600 border border-blue-100">
                🔔
              </div>
              <button
                onClick={() => setShowTwilioModal(false)}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Connect Twilio Conversations</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Select the channels you want to route into your unified inbox. You can update this later without losing message history.
            </p>

            <div className="space-y-4 mb-6">
              {Object.entries(twilioChannels).map(([key, value]) => (
                <div key={key} className="flex items-center justify-between p-3 rounded-xl border border-gray-100 bg-gray-50">
                  <span className="text-sm font-semibold text-gray-800 capitalize">{key}</span>
                  <button
                    onClick={() => setTwilioChannels(prev => ({ ...prev, [key]: !prev[key as keyof typeof twilioChannels] }))}
                    className={`w-12 h-6 rounded-full transition-colors relative ${value ? 'bg-[#34C759]' : 'bg-gray-300'}`}
                  >
                    <div className={`w-5 h-5 bg-white rounded-full absolute top-0.5 transition-transform ${value ? 'translate-x-6' : 'translate-x-0.5'}`} />
                  </button>
                </div>
              ))}
            </div>

            <button
              onClick={saveTwilioIntegration}
              className="w-full bg-[#0066FF] text-white py-3 rounded-xl font-bold text-sm shadow-sm hover:bg-[#005bb5] transition-colors"
            >
              Save & Connect
            </button>
          </div>
        </div>
      )}

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

              <button
                onClick={() => handleConnect(integration.id)}
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
