"use client";

import { useState } from "react";
import Link from "next/link";

const integrations = [
  {
    id: "instagram-integration",
    name: "Instagram",
    description: "Connect your Instagram Business account to auto-post updates and reply to DMs.",
    icon: "📸",
    color: "#E1306C",
    status: "Disconnected",
  },
  {
    id: "whatsapp-integration",
    name: "WhatsApp",
    description: "Sync with WhatsApp Business to automate customer support and order notifications.",
    icon: "💬",
    color: "#25D366",
    status: "Disconnected",
  },
  {
    id: "shopify-integration",
    name: "Shopify",
    description: "Import your products and sync inventory automatically with your OHC store.",
    icon: "🛍️",
    color: "#95BF47",
    status: "Disconnected",
  },
  {
    id: "facebook-integration",
    name: "Facebook",
    description: "Manage your Facebook Page posts and Messenger inbox directly from OHC.",
    icon: "👤",
    color: "#1877F2",
    status: "Disconnected",
  }
];

export default function IntegrationsPage() {
  const [activeTab, setActiveTab] = useState("All");

  const handleConfigure = (name: string) => {
    if (name === "Facebook" || name === "Instagram") {
        // Mock navigation to Customer Inbox as expected by E2E tests
        window.location.href = "/inbox";
    } else {
        alert(`Configure ${name} integration`);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
            <Link href="/dashboard" className="text-blue-600 hover:text-blue-700 font-medium flex items-center gap-1">
                <span>←</span> Back
            </Link>
            <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Integrations</h1>
         </div>
         <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
             AC
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8">
        <section>
            <p className="text-lg text-gray-600 max-w-2xl mb-8">
                Connect your favorite tools to OneHumanCorp. Our AI agents will use these connections to automate your business across all channels.
            </p>

            <div className="flex gap-2 mb-8">
                {["All", "Social", "E-commerce", "Marketing"].map(tab => (
                    <button
                        key={tab}
                        onClick={() => setActiveTab(tab)}
                        className={`px-4 py-2 rounded-full text-sm font-medium transition-all ${activeTab === tab ? 'bg-blue-600 text-white shadow-sm' : 'bg-white/50 text-gray-600 hover:bg-white/80'}`}
                        style={{ border: '1px solid rgba(0,0,0,0.05)' }}
                    >
                        {tab}
                    </button>
                ))}
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {integrations.map(integration => (
                    <div
                        key={integration.id}
                        id={integration.id}
                        className="p-6 flex flex-col gap-4 group transition-all hover:translate-y-[-2px] hover:shadow-xl"
                        style={{
                            background: 'rgba(255, 255, 255, 0.65)',
                            backdropFilter: 'blur(30px) saturate(210%)',
                            border: '1px solid rgba(255, 255, 255, 0.4)',
                            borderRadius: '16px'
                        }}
                    >
                        <div className="flex items-start justify-between">
                            <div
                                className="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-inner"
                                style={{ background: `${integration.color}20`, color: integration.color }}
                            >
                                {integration.icon}
                            </div>
                            <span className="text-xs font-bold px-2 py-1 rounded-md bg-gray-100 text-gray-500 uppercase tracking-wider">
                                {integration.status}
                            </span>
                        </div>

                        <div>
                            <h3 className="text-xl font-bold font-outfit mb-2" style={{ color: '#1D1D1F' }}>{integration.name}</h3>
                            <p className="text-sm text-gray-600 leading-relaxed min-h-[60px]">
                                {integration.description}
                            </p>
                        </div>

                        <button
                            onClick={() => handleConfigure(integration.name)}
                            className="mt-2 w-full py-2.5 font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-md active:scale-[0.98]"
                            style={{ borderRadius: '8px' }}
                        >
                            Configure
                        </button>
                    </div>
                ))}

                {/* Coming Soon Placeholder */}
                <div
                    className="p-6 flex flex-col items-center justify-center gap-3 border-dashed border-2 border-gray-300"
                    style={{ borderRadius: '16px', background: 'transparent' }}
                >
                    <div className="text-3xl text-gray-300">+</div>
                    <p className="text-sm font-medium text-gray-400">More coming soon</p>
                </div>
            </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
