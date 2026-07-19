"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";

declare global {
  interface Window {
    fbAsyncInit: () => void;
    FB: any;
  }
}

const INTEGRATION_TEMPLATE = [
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
];

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isConfirmedUsableConnection(value: unknown): boolean {
  if (!isRecord(value) || value.success !== true) return false;
  if (value.status === "connected" && value.usable === true) return true;
  return isRecord(value.integration) && value.integration.status === "connected" && value.integration.usable === true;
}

export default function Integrations() {
  const [activeTab, setActiveTab] = useState("all");
  const router = useRouter();
  const [statusMessage, setStatusMessage] = useState("");

  const [integrations, setIntegrations] = useState(INTEGRATION_TEMPLATE);
  useEffect(() => {
    async function loadIntegrations() {
      try {
        const res = await fetch("/api/v1/integrations");
        if (res.ok) {
          const data = await res.json();
          if (data && data.success && Array.isArray(data.integrations)) {
            const connectedIds = data.integrations
              .filter((i: unknown) => isRecord(i) && typeof i.id === "string" && i.status === "connected" && i.usable === true)
              .map((i: Record<string, unknown>) => i.id);

            setIntegrations(prev => prev.map(integration =>
              connectedIds.includes(integration.id)
                ? { ...integration, status: "connected" }
                : integration
            ));
          }
        }
      } catch (e) {
        console.error("Failed to load integrations", e);
      }
    }
    loadIntegrations();
  }, []);

  const filteredIntegrations = activeTab === "all" ? integrations : integrations.filter(i => i.category === activeTab);

  const [showTwilioModal, setShowTwilioModal] = useState(false);
  const [showWhatsAppModal, setShowWhatsAppModal] = useState(false);
  const [showWhatsAppCloudApiModal, setShowWhatsAppCloudApiModal] = useState(false);
  const [twilioCreds, setTwilioCreds] = useState({ accountSid: '', authToken: '' });
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
    setStatusMessage(`${integration?.name || id} connection is unavailable until secure provider verification is configured.`);
  };

  const saveTwilioIntegration = async () => {
    if (!twilioCreds.accountSid.trim() || !twilioCreds.authToken.trim() || !Object.values(twilioChannels).some(Boolean)) {
      setStatusMessage('Twilio credentials and at least one channel are required.');
      return;
    }
    try {
      const response = await fetch('/api/v1/integrations/twilio/connect', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ bot_token: twilioCreds.accountSid.trim(), api_token: twilioCreds.authToken.trim() }),
      });
      if (!response.ok) throw new Error('Twilio Conversations connection is unavailable.');
      if (!isConfirmedUsableConnection(await response.json())) throw new Error('Unconfirmed Twilio connection');
      setTwilioCreds({ accountSid: '', authToken: '' });
      setIntegrations(prev => prev.map(integration =>
        integration.id === 'twilio' ? { ...integration, status: "connected" } : integration
      ));
      setShowTwilioModal(false);
      setStatusMessage("Twilio Conversations connected.");
      router.push('/inbox');
    } catch {
      setStatusMessage('Twilio Conversations connection could not be confirmed.');
    }
  };

  const saveWhatsAppIntegration = async () => {
    if (!whatsappTwilioCreds.accountSid.trim() || !whatsappTwilioCreds.authToken.trim() || !whatsappTwilioCreds.phoneNumber.trim()) {
      setStatusMessage("Twilio credentials and a WhatsApp phone number are required.");
      return;
    }
    try {
      const res = await fetch(`/api/v1/integrations/whatsapp/connect`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          bot_token: whatsappTwilioCreds.accountSid.trim(),
          api_token: whatsappTwilioCreds.authToken.trim(),
          from_phone: whatsappTwilioCreds.phoneNumber.trim(),
        })
      });

      if (!res.ok || !isConfirmedUsableConnection(await res.json())) {
        setStatusMessage("Failed to connect Twilio for WhatsApp.");
        return;
      }
      setWhatsappTwilioCreds({ accountSid: '', authToken: '', phoneNumber: '' });
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

  useEffect(() => {
    const appId = process.env.NEXT_PUBLIC_META_APP_ID;
    if (appId && typeof window !== "undefined" && !document.getElementById("facebook-jssdk")) {
      window.fbAsyncInit = function () {
        window.FB.init({
          appId,
          cookie: true,
          xfbml: true,
          version: "v19.0",
        });
      };

      (function(d, s, id) {
        var js, fjs = d.getElementsByTagName(s)[0];
        if (d.getElementById(id)) return;
        js = d.createElement(s) as HTMLScriptElement;
        js.id = id;
        js.src = "https://connect.facebook.net/en_US/sdk.js";
        if (fjs && fjs.parentNode) {
          fjs.parentNode.insertBefore(js, fjs);
        } else {
          document.head.appendChild(js);
        }
      }(document, 'script', 'facebook-jssdk'));
    }
  }, []);

  const saveWhatsAppCloudApiIntegration = async () => {
    try {
      const doBackendConnect = async (token?: string) => {
        const res = await fetch(`/api/v1/integrations/whatsapp_cloud_api/connect`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            api_token: token,
          })
        });

        if (!res.ok || !isConfirmedUsableConnection(await res.json())) {
          setStatusMessage("Failed to connect WhatsApp Cloud API.");
          return;
        }
        setIntegrations(prev => prev.map(integration =>
          integration.id === 'whatsapp_cloud_api' ? { ...integration, status: "connected" } : integration
        ));
        setShowWhatsAppCloudApiModal(false);
        setStatusMessage("WhatsApp Cloud API connected.");
        router.push('/inbox');
      };

      if (typeof window !== "undefined" && window.FB) {
        window.FB.login((response: any) => {
          if (response.authResponse) {
            doBackendConnect(response.authResponse.accessToken);
          } else {
            setStatusMessage("WhatsApp Cloud API connection cancelled.");
          }
        }, { scope: 'whatsapp_business_management,whatsapp_business_messaging' });
      } else {
        setStatusMessage("WhatsApp Cloud API signup is unavailable because the Meta SDK did not load.");
      }
    } catch (e) {
      setStatusMessage("Failed to connect WhatsApp Cloud API.");
    }
  };

  return (
    <AppShell
      title="Tool Integrations"
      subtitle="Supercharge your workflow by connecting your favorite marketing, finance, and operations tools."
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

              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Connect Twilio for WhatsApp</h2>
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
                    className="glass-control w-full px-3 py-2 rounded-lg outline-none"
                    placeholder="AC..."
                  />
                </div>
                <div>
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Auth Token</label>
                  <input
                    type="password"
                    value={whatsappTwilioCreds.authToken}
                    onChange={(e) => setWhatsappTwilioCreds(prev => ({ ...prev, authToken: e.target.value }))}
                    className="glass-control w-full px-3 py-2 rounded-lg outline-none"
                    placeholder="Hidden for security"
                  />
                </div>
                <div>
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">WhatsApp Phone Number</label>
                  <input
                    type="text"
                    value={whatsappTwilioCreds.phoneNumber}
                    onChange={(e) => setWhatsappTwilioCreds(prev => ({ ...prev, phoneNumber: e.target.value }))}
                    className="glass-control w-full px-3 py-2 rounded-lg outline-none"
                    placeholder="+1234567890"
                  />
                </div>
              </div>

              <button
                onClick={saveWhatsAppIntegration}
                disabled={!whatsappTwilioCreds.accountSid.trim() || !whatsappTwilioCreds.authToken.trim() || !whatsappTwilioCreds.phoneNumber.trim()}
                className="w-full bg-[#0f766e] hover:bg-[#0d645d] disabled:cursor-not-allowed disabled:opacity-50 text-white py-3 rounded-xl font-bold text-sm shadow-sm transition-colors flex items-center justify-center gap-2"
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
                Enter your Twilio credentials and select the channels you want to route into your central inbox.
              </p>

              <div className="space-y-4 mb-6">
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300">
                  Twilio Account SID
                  <input
                    aria-label="Twilio Account SID"
                    type="text"
                    value={twilioCreds.accountSid}
                    onChange={(event) => setTwilioCreds((previous) => ({ ...previous, accountSid: event.target.value }))}
                    className="glass-control mt-1 w-full rounded-lg px-3 py-2 outline-none"
                    placeholder="AC..."
                  />
                </label>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300">
                  Twilio Auth Token
                  <input
                    aria-label="Twilio Auth Token"
                    type="password"
                    value={twilioCreds.authToken}
                    onChange={(event) => setTwilioCreds((previous) => ({ ...previous, authToken: event.target.value }))}
                    className="glass-control mt-1 w-full rounded-lg px-3 py-2 outline-none"
                    placeholder="Hidden for security"
                  />
                </label>
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
                disabled={!twilioCreds.accountSid.trim() || !twilioCreds.authToken.trim() || !Object.values(twilioChannels).some(Boolean)}
                className="w-full bg-[#0f766e] hover:bg-[#0d645d] disabled:cursor-not-allowed disabled:opacity-50 text-white py-3 rounded-xl font-bold text-sm shadow-sm transition-colors"
              >
                Connect Twilio
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
            <h2 className="text-xl font-bold mb-4 col-span-full">Connect Custom Software</h2>
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
          <h2 className="text-xl font-bold mb-4 mt-12 col-span-full">Social Media Accounts</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div className="p-6 shadow-sm flex flex-col transition-shadow hover:shadow-md glassmorphism border border-white/40 dark:border-white/10" style={{ background: 'rgba(255, 255, 255, 0.65)' }}>
              <h3 className="font-bold font-outfit text-gray-900 dark:text-white text-lg mb-2">Social Channels</h3>
              <p className="text-gray-500 dark:text-gray-400 text-sm mb-6 flex-1">Connect Instagram, Facebook, and Twitter</p>
               <button disabled className="text-gray-500 bg-gray-100 min-h-[44px] w-full py-3 font-semibold text-sm rounded-lg">Unavailable</button>
            </div>
          </div>
        </main>
      </div>
    </AppShell>
  );
}
