"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const router = useRouter();
  const [statusMessage, setStatusMessage] = useState("");

  const [integrations, setIntegrations] = useState([
    { id: "ayrshare", name: "Ayrshare", category: "marketing", status: "disconnected", icon: "📱", description: "Single API for posting and retrieving messages across social networks." },
    { id: "cal_com", name: "Cal.com", category: "operations", status: "disconnected", icon: "📅", description: "Zero-Config Booking & Calendar Sync." },
    { id: "mailerlite", name: "MailerLite", category: "marketing", status: "disconnected", icon: "📨", description: "Embedded, No-Jargon Email Campaigns." },
    { id: "mercadopago", name: "Mercado Pago", category: "finance", status: "disconnected", icon: "🌎", description: "Accept credit cards and local payment methods in Latin America." },
    { id: "shippo", name: "Shippo", category: "operations", status: "disconnected", icon: "📦", description: "Painless Shipping Labels & Tracking." },
    { id: "taxjar", name: "TaxJar", category: "finance", status: "disconnected", icon: "🏛️", description: "Automatically calculate and track sales tax for your orders." },
    { id: "twilio", name: "Twilio Conversations", category: "operations", status: "disconnected", icon: "🔔", description: "Central omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat." },
    { id: "whereby", name: "Whereby", category: "operations", status: "disconnected", icon: "📹", description: "Zero-Setup Online Lessons and video conferencing." },
    { id: "resend", name: "Resend", category: "marketing", status: "disconnected", icon: "📧", description: "Transactional and Marketing Emails." },
    { id: "whatsapp_cloud_api", name: "WhatsApp Cloud API", category: "social", status: "disconnected", icon: "💬", description: "Direct WhatsApp Cloud API connection for messages." },
    { id: "whatsapp", name: "Twilio for WhatsApp", category: "social", status: "disconnected", icon: "💬", description: "Central WhatsApp Inbox for Work Triage and Customer Assistant powered by Twilio." },
    { id: "meta", name: "Meta Graph API", category: "social", status: "disconnected", icon: "💬", description: "Central Instagram and Facebook Inbox." },
    { id: "front", name: "Front", category: "operations", status: "disconnected", icon: "📥", description: "Central omnichannel inbox aggregating messages across all channels." },
    { id: "zoom", name: "Zoom", category: "operations", status: "disconnected", icon: "📹", description: "Automated Online Lesson Links." }
  ]);

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  const [showTwilioModal, setShowTwilioModal] = useState(false);
  const [showWhatsAppModal, setShowWhatsAppModal] = useState(false);
  const [showWhatsAppCloudApiModal, setShowWhatsAppCloudApiModal] = useState(false);
  const [whatsappTwilioCreds, setWhatsappTwilioCreds] = useState({ accountSid: '', authToken: '', phoneNumber: '' });
  const [twilioChannels, setTwilioChannels] = useState({
    whatsapp: true,
    instagram: false,
    facebook: false,
    sms: true,
  });

  const handleConnect = async (id: string) => {
    const integration = integrations.find((item) => item.id === id);
    if (integration?.status === 'connected') {
      setStatusMessage(`${integration.name} settings are ready to manage.`);
      return;
    }
    if (id === 'ayrshare') {
      setIntegrations(prev => prev.map(integration =>
        integration.id === id ? { ...integration, status: "connected" } : integration
      ));
      setStatusMessage("Ayrshare connected. Opening the social inbox.");
      router.push('/inbox');
      return;
    }
    if (id === 'twilio') {
      setShowTwilioModal(true);
      setStatusMessage("Choose Twilio channels to finish connecting.");
      return;
    }
    if (id === 'whatsapp') {
      setShowWhatsAppModal(true);
      setStatusMessage("Enter your Twilio API credentials to connect WhatsApp.");
      return;
    }
    if (id === 'whatsapp_cloud_api') {
      setShowWhatsAppCloudApiModal(true);
      setStatusMessage("Continue with Meta to connect WhatsApp Cloud API.");
      return;
    }
    setStatusMessage(`Connecting ${integration?.name || id}...`);
    try {
      const res = await fetch(`/api/integrations/${id}/connect`, { method: "POST" });
      if (!res.ok) {
        setStatusMessage(`Unable to start ${integration?.name || id} connection.`);
        return;
      }
      const data = await res.json();
      const oauthUrl = data.authorization_url || data.url;
      if (oauthUrl) {
        window.location.assign(oauthUrl);
        return;
      }
      setIntegrations(prev => prev.map(integration =>
        integration.id === id ? { ...integration, status: "connected" } : integration
      ));
      setStatusMessage(`${integration?.name || id} connected.`);
    } catch {
      setStatusMessage(`Unable to start ${integration?.name || id} connection.`);
    }
  };

  const saveTwilioIntegration = () => {
    setIntegrations(prev => prev.map(integration =>
      integration.id === 'twilio' ? { ...integration, status: "connected" } : integration
    ));
    setShowTwilioModal(false);
    setStatusMessage("Twilio Conversations connected.");
    router.push('/inbox');
  };

  const saveWhatsAppIntegration = async () => {
    try {
      const res = await fetch(`/api/integrations/whatsapp/connect`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          bot_token: whatsappTwilioCreds.accountSid,
          api_token: whatsappTwilioCreds.authToken,
          from_phone: whatsappTwilioCreds.phoneNumber,
          integration_id: 'twilio',
          base_url: 'https://api.twilio.com'
        })
      });

      if (!res.ok) {
        setStatusMessage("Failed to connect Twilio for WhatsApp.");
        return;
      }
      setIntegrations(prev => prev.map(integration =>
        integration.id === 'whatsapp' ? { ...integration, status: "connected" } : integration
      ));
      setShowWhatsAppModal(false);
      setStatusMessage("Twilio for WhatsApp connected.");
      router.push('/inbox');
    } catch (e) {
      setStatusMessage("Failed to connect Twilio for WhatsApp.");
    }
  };

  const saveWhatsAppCloudApiIntegration = async () => {
    try {
      const res = await fetch(`/api/integrations/whatsapp_cloud_api/connect`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          integration_id: 'whatsapp_cloud_api',
        })
      });

      if (!res.ok) {
        setStatusMessage("Failed to connect WhatsApp Cloud API.");
        return;
      }
      setIntegrations(prev => prev.map(integration =>
        integration.id === 'whatsapp_cloud_api' ? { ...integration, status: "connected" } : integration
      ));
      setShowWhatsAppCloudApiModal(false);
      setStatusMessage("WhatsApp Cloud API connected.");
      router.push('/inbox');
    } catch (e) {
      setStatusMessage("Failed to connect WhatsApp Cloud API.");
    }
  };

  return (
    <AppShell
      title="Tool Integrations"
      subtitle="Supercharge your workflow by connecting your favorite marketing, finance, and operations tools."
      statusItems={[{ label: "Premium Link", value: "Active", tone: "good" }]}
    >
      <div className="flex flex-col font-inter">
        {/* Twilio for WhatsApp Connect Modal */}
        {showWhatsAppModal && (
          <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card glassmorphism w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-white/40 dark:border-white/10 bg-white/90 dark:bg-zinc-900/90">
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-teal-50/10 rounded-xl flex items-center justify-center text-2xl text-teal-600 border border-teal-100/30">
                  💬
                </div>
                <button
                  onClick={() => setShowWhatsAppModal(false)}
                  className="min-h-[44px] p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Connect Twilio for WhatsApp API</h2>
              <p className="text-gray-600 dark:text-gray-300 mb-6 text-sm leading-relaxed">
                Enter your Twilio API credentials to securely link your WhatsApp Business account. Incoming messages will be automatically routed into Work Triage.
              </p>

              <div className="space-y-4 mb-6">
                <div>
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Account SID</label>
                  <input
                    type="text"
                    value={whatsappTwilioCreds.accountSid}
                    onChange={(e) => setWhatsappTwilioCreds(prev => ({ ...prev, accountSid: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-zinc-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0f766e] focus:border-transparent outline-none"
                    placeholder="AC..."
                  />
                </div>
                <div>
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Auth Token</label>
                  <input
                    type="password"
                    value={whatsappTwilioCreds.authToken}
                    onChange={(e) => setWhatsappTwilioCreds(prev => ({ ...prev, authToken: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-zinc-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0f766e] focus:border-transparent outline-none"
                    placeholder="Hidden for security"
                  />
                </div>
                <div>
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">WhatsApp Phone Number</label>
                  <input
                    type="text"
                    value={whatsappTwilioCreds.phoneNumber}
                    onChange={(e) => setWhatsappTwilioCreds(prev => ({ ...prev, phoneNumber: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-zinc-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#0f766e] focus:border-transparent outline-none"
                    placeholder="+1234567890"
                  />
                </div>
              </div>

              <button
                onClick={saveWhatsAppIntegration}
                className="w-full bg-[#0f766e] hover:bg-[#0d645d] text-white py-3 rounded-xl font-bold text-sm shadow-sm transition-colors flex items-center justify-center gap-2"
              >
                Save & Connect
              </button>
            </div>
          </div>
        )}


        {/* WhatsApp Cloud API Connect Modal */}
        {showWhatsAppCloudApiModal && (
          <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card glassmorphism w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-white/40 dark:border-white/10 bg-white/90 dark:bg-zinc-900/90">
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-teal-50/10 rounded-xl flex items-center justify-center text-2xl text-teal-600 border border-teal-100/30">
                  💬
                </div>
                <button
                  onClick={() => setShowWhatsAppCloudApiModal(false)}
                  className="min-h-[44px] p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Connect WhatsApp Cloud API</h2>
              <p className="text-gray-600 dark:text-gray-300 mb-6 text-sm leading-relaxed">
                Connect your WhatsApp Business Account directly using the WhatsApp Cloud API. You will be redirected to Facebook to complete the onboarding flow securely.
              </p>

              <button
                onClick={saveWhatsAppCloudApiIntegration}
                className="w-full bg-[#1877F2] hover:bg-[#166FE5] text-white py-3 rounded-xl font-bold text-sm shadow-sm transition-colors flex items-center justify-center gap-2"
              >
                Continue with Meta
              </button>
            </div>
          </div>
        )}


        {/* Twilio Conversations Connect Modal */}
        {showTwilioModal && (
          <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card glassmorphism w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-white/40 dark:border-white/10 bg-white/90 dark:bg-zinc-900/90">
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-teal-50/10 rounded-xl flex items-center justify-center text-2xl text-teal-600 border border-teal-100/30">
                  🔔
                </div>
                <button
                  onClick={() => setShowTwilioModal(false)}
                  className="min-h-[44px] p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Connect Twilio Conversations</h2>
              <p className="text-gray-600 dark:text-gray-300 mb-6 text-sm leading-relaxed">
                Select the channels you want to route into your central inbox. You can update this later without losing message history.
              </p>

              <div className="space-y-4 mb-6">
                {Object.entries(twilioChannels).map(([key, value]) => (
                  <div key={key} className="flex items-center justify-between p-3 rounded-xl border border-gray-100 dark:border-gray-800 bg-gray-50 dark:bg-zinc-800">
                    <span className="text-sm font-semibold text-gray-800 dark:text-gray-200 capitalize">{key}</span>
                    <button
                      onClick={() => setTwilioChannels(prev => ({ ...prev, [key]: !prev[key as keyof typeof twilioChannels] }))}
                      className={`min-h-[44px] w-12 h-6 rounded-full transition-colors relative ${value ? 'bg-[#34C759]' : 'bg-gray-300'}`}
                    >
                      <div className={`w-5 h-5 bg-white rounded-full absolute top-0.5 transition-transform ${value ? 'translate-x-6' : 'translate-x-0.5'}`} />
                    </button>
                  </div>
                ))}
              </div>

              <button
                onClick={saveTwilioIntegration}
                className="w-full bg-[#0f766e] hover:bg-[#0d645d] text-white py-3 rounded-xl font-bold text-sm shadow-sm transition-colors"
              >
                Save & Connect
              </button>
            </div>
          </div>
        )}

        <main className="flex-1 max-w-5xl mx-auto w-full">
          {/* Navigation Tabs */}
          <div className="flex gap-4 mb-8 border-b border-gray-200 dark:border-gray-800 pb-4 overflow-x-auto hide-scrollbar">
            {["all", "marketing", "operations", "finance", "social"].map(tab => (
              <button
                key={tab}
                aria-pressed={activeTab === tab}
                onClick={() => setActiveTab(tab)}
                className={`min-h-[44px] px-4 py-2 rounded-full text-sm font-semibold whitespace-nowrap transition-colors ${
                  activeTab === tab
                    ? "bg-[#0f766e] text-white"
                    : "bg-white dark:bg-zinc-800 text-gray-600 dark:text-gray-300 border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-zinc-700"
                }`}
              >
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            ))}
          </div>

          {statusMessage && (
            <div className="mb-6 rounded-lg border border-teal-100 bg-teal-50/30 px-4 py-3 text-sm font-semibold text-teal-800 dark:text-teal-200" role="status">
              {statusMessage}
            </div>
          )}

          {/* Integration Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filteredIntegrations.map(integration => (
              <div key={integration.id}
                   className="p-6 shadow-sm flex flex-col transition-shadow hover:shadow-md glassmorphism border border-white/40 dark:border-white/10"
                   style={{ background: 'rgba(255, 255, 255, 0.65)' }}
              >
                <div className="flex justify-between items-start mb-4">
                  <div className="w-12 h-12 bg-gray-50 dark:bg-zinc-800 rounded-xl flex items-center justify-center text-2xl border border-gray-100 dark:border-gray-700">
                    {integration.icon}
                  </div>
                  <span className={`text-xs font-bold px-2 py-1 rounded-md uppercase tracking-wide ${
                    integration.status === 'connected' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' : 'bg-gray-100 text-gray-500 dark:bg-zinc-850 dark:text-zinc-400'
                  }`}>
                    {integration.status}
                  </span>
                </div>
                <h3 className="font-bold font-outfit text-gray-900 dark:text-white text-lg mb-2">{integration.name}</h3>
                <p className="text-gray-500 dark:text-gray-400 text-sm mb-6 flex-1">{integration.description}</p>

                <button
                  onClick={() => handleConnect(integration.id)}
                  className={`min-h-[44px] w-full py-3 font-semibold text-sm transition-transform active:scale-[0.98] ${
                    integration.status === 'connected'
                      ? "bg-gray-50 dark:bg-zinc-800 text-gray-750 dark:text-gray-200 border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-zinc-700"
                      : "text-white shadow-sm bg-[#0f766e] hover:bg-[#0d645d] border-none"
                  }`}>
                  {integration.status === 'connected' ? 'Manage' : 'Connect'}
                </button>
              </div>
            ))}
          </div>
        </main>
      </div>
    </AppShell>
  );
}
