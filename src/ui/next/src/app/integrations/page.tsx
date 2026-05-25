"use client";

import { useState } from "react";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const [connectedTools, setConnectedTools] = useState<Record<string, { status: string; message?: string }>>({});
  const [loadingTool, setLoadingTool] = useState<string | null>(null);

  const handleConnect = async (toolId: string) => {
    if (toolId === "meta" || toolId === "google_calendar") {
      setLoadingTool(toolId);
      try {
        const response = await fetch(`/api/integrations/${toolId}`, { method: "POST" });
        const data = await response.json();

        setConnectedTools(prev => ({
          ...prev,
          [toolId]: { status: data.status, message: data.message }
        }));
      } catch (error) {
        console.error(`Failed to connect ${toolId}`, error);
      } finally {
        setLoadingTool(null);
      }
    } else {
      // Simulate generic connection
      setLoadingTool(toolId);
      setTimeout(() => {
        setConnectedTools(prev => ({
          ...prev,
          [toolId]: { status: "connected" }
        }));
        setLoadingTool(null);
      }, 1000);
    }
  };

  const integrations = [
    { id: "meta", name: "Meta Graph API", category: "marketing", defaultStatus: "disconnected", icon: "📱", description: "Unified Native Social Media Inbox for Instagram, Facebook, and WhatsApp." },
    { id: "google_calendar", name: "Google Calendar", category: "operations", defaultStatus: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "cal_com", name: "Cal.com", category: "operations", defaultStatus: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "resend", name: "Resend", category: "marketing", defaultStatus: "disconnected", icon: "📨", description: "AI-Powered Email Marketing and simple customer newsletters." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", defaultStatus: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "shippo", name: "Shippo", category: "operations", defaultStatus: "disconnected", icon: "📦", description: "Automated Label Generation and real-time shipping rates." },
    { id: "twilio", name: "Twilio", category: "operations", defaultStatus: "disconnected", icon: "🔔", description: "Reliable SMS alerts for new orders and customer notifications." },
    { id: "zoom", name: "Zoom", category: "operations", defaultStatus: "disconnected", icon: "📹", description: "Auto-Generated Meeting Links for online services." }
  ];

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  // Get all unique messages from connected tools to display in the dashboard
  const connectedMessages = Object.entries(connectedTools)
    .filter(([_, data]) => data.message)
    .map(([id, data]) => ({ id, message: data.message as string }));

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
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-12">
          {filteredIntegrations.map(integration => {
            const currentStatus = connectedTools[integration.id]?.status || integration.defaultStatus;
            const isLoading = loadingTool === integration.id;

            return (
              <div key={integration.id}
                   className="rounded-[16px] p-6 shadow-sm flex flex-col transition-shadow hover:shadow-md"
                   style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}
              >
                <div className="flex justify-between items-start mb-4">
                  <div className="w-12 h-12 bg-gray-50 rounded-xl flex items-center justify-center text-2xl border border-gray-100">
                    {integration.icon}
                  </div>
                  <span className={`text-xs font-bold px-2 py-1 rounded-md uppercase tracking-wide ${
                    currentStatus === 'connected' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-500'
                  }`}>
                    {currentStatus}
                  </span>
                </div>
                <h3 className="font-bold font-outfit text-gray-900 text-lg mb-2">{integration.name}</h3>

                {isLoading && integration.id === 'meta' ? (
                  <div className="text-gray-500 text-sm mb-6 flex-1 flex flex-col gap-2">
                    <div className="flex items-center gap-2">
                       <div className="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                       <span>Our AI is scanning your profile to build your catalog...</span>
                    </div>
                  </div>
                ) : (
                  <p className="text-gray-500 text-sm mb-6 flex-1">{integration.description}</p>
                )}

                <button
                  onClick={() => handleConnect(integration.id)}
                  disabled={isLoading}
                  className={`w-full py-3 rounded-[8px] font-semibold text-sm transition-transform active:scale-[0.98] ${
                  currentStatus === 'connected'
                    ? "bg-gray-50 text-gray-700 border border-gray-200 hover:bg-gray-100"
                    : isLoading
                    ? "bg-blue-400 text-white cursor-not-allowed"
                    : "text-[#F5F5F7] shadow-sm hover:bg-[#005bd3]"
                }`} style={(currentStatus === 'connected' || isLoading) ? {} : { background: '#0066FF' }}>
                  {isLoading ? 'Connecting...' : currentStatus === 'connected' ? 'Manage' : 'Connect'}
                </button>
              </div>
            );
          })}
        </div>

        {/* Unified Dashboard Section */}
        {connectedMessages.length > 0 && (
          <div className="mt-8 rounded-[16px] p-6 shadow-sm flex flex-col"
               style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}
          >
             <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-4">Unified Dashboard</h2>
             <div className="flex flex-col gap-4">
               {connectedMessages.map(msg => (
                 <div key={msg.id} className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm flex items-start gap-3">
                   <div className="text-2xl mt-1">✨</div>
                   <div>
                     <p className="text-gray-800 font-medium">{msg.message}</p>
                     <p className="text-gray-500 text-sm mt-1">Successfully synced and ready to use.</p>
                   </div>
                 </div>
               ))}
             </div>
          </div>
        )}

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
