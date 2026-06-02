"use client";

import React, { useState } from "react";
import Link from "next/link";

const OHCPremiumGlassContainer = ({ children, className = "" }: { children: React.ReactNode, className?: string }) => (
  <div
    className={`rounded-2xl shadow-sm ${className}`}
    style={{
      background: "rgba(255, 255, 255, 0.65)",
      backdropFilter: "blur(30px) saturate(210%)",
      WebkitBackdropFilter: "blur(30px) saturate(210%)",
      border: "1px solid rgba(255, 255, 255, 0.4)",
    }}
  >
    {children}
  </div>
);

const SectionHeader = ({ title, icon }: { title: string; icon: string }) => (
  <div className="flex items-center gap-3 mb-6">
    <div className="w-10 h-10 rounded-xl bg-blue-50 text-[#0066FF] flex items-center justify-center shadow-sm border border-blue-100">
      <span className="text-xl">{icon}</span>
    </div>
    <h2 className="text-xl font-bold text-gray-900 font-outfit tracking-tight">{title}</h2>
  </div>
);

const InsightCard = ({ title, value, description, color = "blue" }: { title: string; value: string; description: string; color?: "blue" | "rose" | "purple" }) => {
  const colorMap = {
    blue: "bg-blue-50 text-[#0066FF] border-blue-100",
    rose: "bg-rose-50 text-rose-600 border-rose-100",
    purple: "bg-purple-50 text-purple-600 border-purple-100",
  };

  return (
    <div className="p-5 rounded-xl bg-white border border-gray-100 shadow-sm flex flex-col h-full hover:shadow-md transition-shadow">
      <div className="text-xs font-semibold uppercase tracking-wider text-gray-500 mb-2">{title}</div>
      <div className={`text-2xl font-bold mb-2 font-outfit ${colorMap[color].split(' ')[1]}`}>
        {value}
      </div>
      <div className="text-sm text-gray-600 leading-relaxed flex-grow">
        {description}
      </div>
    </div>
  );
};

export default function MarketDynamicsReport() {
  const [activeTab, setActiveTab] = useState<"overview" | "competitors" | "gaps" | "solutions">("overview");

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900 font-inter p-4 md:p-8">
      <div className="max-w-6xl mx-auto space-y-8">

        {/* Header Section */}
        <OHCPremiumGlassContainer className="p-6 md:p-8 overflow-hidden relative">
          <div className="absolute top-0 right-0 w-64 h-64 bg-blue-400 rounded-full mix-blend-multiply filter blur-3xl opacity-10 -translate-y-1/2 translate-x-1/2" />
          <div className="absolute bottom-0 left-0 w-64 h-64 bg-purple-400 rounded-full mix-blend-multiply filter blur-3xl opacity-10 translate-y-1/2 -translate-x-1/2" />

          <div className="relative z-10 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
            <div>
              <div className="flex items-center gap-2 mb-2">
                <Link href="/dashboard" className="text-sm font-medium text-gray-500 hover:text-[#0066FF] transition-colors flex items-center gap-1">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
                  Dashboard
                </Link>
                <span className="text-gray-300">•</span>
                <span className="text-sm font-semibold text-[#0066FF] bg-blue-50 px-2 py-0.5 rounded border border-blue-100">Research Intel</span>
              </div>
              <h1 className="text-3xl md:text-4xl font-extrabold tracking-tight font-outfit text-gray-900 mb-2">
                OHC Market Dynamics & Competitor Deep-Dive
              </h1>
              <p className="text-gray-600 max-w-2xl text-base md:text-lg">
                Comprehensive research detailing the SMB platform market, AI-native competitors, and actionable recommendations for the engineering swarm.
              </p>
            </div>

            <div className="flex gap-2 w-full md:w-auto overflow-x-auto pb-2 md:pb-0 hide-scrollbar shrink-0">
              <button
                onClick={() => window.print()}
                className="px-4 py-2 bg-white text-gray-700 rounded-lg text-sm font-semibold border border-gray-200 hover:bg-gray-50 transition-colors shadow-sm min-h-[44px] flex items-center gap-2 whitespace-nowrap"
              >
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" /></svg>
                Export PDF
              </button>
            </div>
          </div>
        </OHCPremiumGlassContainer>

        {/* Navigation Tabs */}
        <div className="flex gap-2 overflow-x-auto pb-2 hide-scrollbar">
          {[
            { id: "overview", label: "Executive Summary", icon: "📊" },
            { id: "competitors", label: "Market Mapping", icon: "🗺️" },
            { id: "gaps", label: "Gap Analysis", icon: "🔍" },
            { id: "solutions", label: "Agentic Solutions", icon: "🤖" },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`px-5 py-2.5 rounded-full text-sm font-semibold whitespace-nowrap transition-all shadow-sm min-h-[44px] border ${
                activeTab === tab.id
                  ? "bg-gray-900 text-white border-transparent"
                  : "bg-white text-gray-600 hover:bg-gray-50 border-gray-200"
              }`}
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </button>
          ))}
        </div>

        {/* Tab Content Area */}
        <div className="min-h-[600px]">

          {/* OVERVIEW TAB */}
          {activeTab === "overview" && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <OHCPremiumGlassContainer className="p-6 md:p-8">
                <SectionHeader title="Executive Summary" icon="📋" />
                <div className="prose prose-gray max-w-none">
                  <p className="text-lg text-gray-700 leading-relaxed mb-6">
                    This report analyzes the global Small and Medium Business (SMB) platform market, focusing on how OneHumanCorp (OHC) can leverage its unique "Hybrid Agentic OS" architecture to solve pervasive pain points that traditional competitors (like Shopify, Wix, Squarespace) and emerging AI-native tools fail to address. Based on deep-dive research into user sentiment across platforms, this report identifies critical gaps in OHC's current feature set and proposes actionable, AI-agentic solutions.
                  </p>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-8">
                  <InsightCard
                    title="Key Differentiator"
                    value="Invisible AI"
                    description="AI acts as infrastructure, not just a bolted-on chatbot. Zero technical knowledge required."
                    color="blue"
                  />
                  <InsightCard
                    title="Setup Velocity"
                    value="< 10 Mins"
                    description="Target time-to-value for new users, significantly outpacing traditional platforms."
                    color="purple"
                  />
                  <InsightCard
                    title="Core Focus"
                    value="Mobile-First"
                    description="100% functionality on 375px screens, allowing management on the go."
                    color="rose"
                  />
                </div>
              </OHCPremiumGlassContainer>

              <OHCPremiumGlassContainer className="p-6 md:p-8">
                 <SectionHeader title="Actionable Recommendations for Engineering Swarm" icon="⚡" />
                 <div className="grid grid-cols-1 gap-4">
                    <div className="p-5 rounded-xl border border-gray-100 bg-white hover:border-[#0066FF] transition-colors shadow-sm">
                      <div className="flex items-center gap-3 mb-2">
                        <span className="bg-red-100 text-red-700 text-xs font-bold px-2 py-1 rounded uppercase">P0 Priority</span>
                        <h3 className="font-bold text-gray-900">Implement AI Agent Department Base Interface</h3>
                      </div>
                      <p className="text-sm text-gray-600">Create the core coordination layer (`src/agents/builtin/core.rs` and related) that allows the 7 defined departments to share memory (`pgvector`) and coordinate via Redis locks.</p>
                    </div>

                    <div className="p-5 rounded-xl border border-gray-100 bg-white hover:border-[#0066FF] transition-colors shadow-sm">
                      <div className="flex items-center gap-3 mb-2">
                        <span className="bg-red-100 text-red-700 text-xs font-bold px-2 py-1 rounded uppercase">P0 Priority</span>
                        <h3 className="font-bold text-gray-900">Build the "Draft-for-Review" UI/UX Flow</h3>
                      </div>
                      <p className="text-sm text-gray-600">Ensure the frontend dashboard natively supports surfacing agent-drafted actions (emails, quotes, orders) for one-tap approval on mobile (375px).</p>
                    </div>

                    <div className="p-5 rounded-xl border border-gray-100 bg-white hover:border-yellow-500 transition-colors shadow-sm">
                      <div className="flex items-center gap-3 mb-2">
                        <span className="bg-yellow-100 text-yellow-800 text-xs font-bold px-2 py-1 rounded uppercase">P1 Priority</span>
                        <h3 className="font-bold text-gray-900">Develop Omnichannel Inbox Architecture</h3>
                      </div>
                      <p className="text-sm text-gray-600">Design the schema and webhook ingestors to unify external communication channels into the central Agent job queue.</p>
                    </div>
                 </div>
              </OHCPremiumGlassContainer>
            </div>
          )}

          {/* COMPETITORS TAB */}
          {activeTab === "competitors" && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <OHCPremiumGlassContainer className="p-6">
                  <SectionHeader title="Top 10 Traditional Platforms" icon="🏢" />
                  <div className="space-y-3">
                    {[
                      { name: "Shopify", desc: "Tech-savvy SMBs, physical products", tag: "E-commerce Giant" },
                      { name: "Wix", desc: "Non-technical SMBs, basic presence", tag: "General Builder" },
                      { name: "Squarespace", desc: "Creative professionals, portfolios", tag: "Design-Focused" },
                      { name: "GoDaddy", desc: "Very basic users", tag: "Basic Builder" },
                      { name: "Weebly (Square)", desc: "Local retail", tag: "Simple E-com" },
                      { name: "WooCommerce", desc: "Highly technical users", tag: "Open-Source" },
                      { name: "BigCommerce", desc: "Scaling mid-market", tag: "Enterprise-Lite" },
                      { name: "Etsy", desc: "Crafters and artisans", tag: "Marketplace" },
                      { name: "Mindbody", desc: "Fitness/wellness", tag: "Specialized Bookings" },
                      { name: "Calendly", desc: "Simple scheduling", tag: "Specialized Scheduling" },
                    ].map((comp, i) => (
                      <div key={i} className="flex justify-between items-center p-3 hover:bg-gray-50 rounded-lg border border-transparent hover:border-gray-100 transition-all">
                        <div>
                          <div className="font-semibold text-gray-900 flex items-center gap-2">
                            <span className="text-gray-400 text-xs w-4">{i + 1}.</span> {comp.name}
                          </div>
                          <div className="text-xs text-gray-500">{comp.desc}</div>
                        </div>
                        <span className="text-[10px] font-bold uppercase tracking-wider text-gray-500 bg-gray-100 px-2 py-1 rounded">{comp.tag}</span>
                      </div>
                    ))}
                  </div>
                </OHCPremiumGlassContainer>

                <OHCPremiumGlassContainer className="p-6">
                  <SectionHeader title="Top 10 AI-Native Tools" icon="🚀" />
                  <div className="space-y-3">
                    {[
                      { name: "Dora AI", desc: "High-end design", tag: "3D Generation" },
                      { name: "10Web", desc: "Cloning and fast setup", tag: "WP Builder" },
                      { name: "Mixo", desc: "Idea validation", tag: "Landing Pages" },
                      { name: "Relume", desc: "Designers", tag: "Wireframing" },
                      { name: "Hostinger AI", desc: "Cost-sensitive users", tag: "Budget Builder" },
                      { name: "Zyro", desc: "Simple businesses", tag: "Basic Tools" },
                      { name: "Shopify Magic", desc: "Copywriting & basic mgmt", tag: "AI Assistant" },
                      { name: "Wix ADI", desc: "Rapid onboarding", tag: "Conversational" },
                      { name: "Kajabi AI", desc: "Digital product creators", tag: "Course Creator" },
                      { name: "Harvey AI", desc: "Vertical AI expansion indicator", tag: "Legal Tech" },
                    ].map((comp, i) => (
                      <div key={i} className="flex justify-between items-center p-3 hover:bg-gray-50 rounded-lg border border-transparent hover:border-gray-100 transition-all">
                        <div>
                          <div className="font-semibold text-gray-900 flex items-center gap-2">
                            <span className="text-purple-400 text-xs w-4">{i + 1}.</span> {comp.name}
                          </div>
                          <div className="text-xs text-gray-500">{comp.desc}</div>
                        </div>
                        <span className="text-[10px] font-bold uppercase tracking-wider text-purple-600 bg-purple-50 border border-purple-100 px-2 py-1 rounded">{comp.tag}</span>
                      </div>
                    ))}
                  </div>
                </OHCPremiumGlassContainer>
              </div>

              <OHCPremiumGlassContainer className="p-6 md:p-8">
                <SectionHeader title="Deep-Dive Audit: Shopify" icon="🛍️" />
                <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                  <div>
                    <h3 className="font-bold text-gray-900 mb-4 border-b pb-2">Success Factors</h3>
                    <ul className="space-y-3">
                      <li className="flex items-start gap-2">
                        <svg className="w-5 h-5 text-green-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                        <span className="text-sm text-gray-700"><strong>Ecosystem:</strong> If Shopify doesn't do it natively, an app does.</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <svg className="w-5 h-5 text-green-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                        <span className="text-sm text-gray-700"><strong>Reliability:</strong> High uptime and secure checkouts.</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <svg className="w-5 h-5 text-green-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                        <span className="text-sm text-gray-700"><strong>Scalability:</strong> Businesses can grow from $0 to $100M+ on the same core platform.</span>
                      </li>
                    </ul>
                  </div>

                  <div>
                    <h3 className="font-bold text-gray-900 mb-4 border-b pb-2">User Sentiment (Reddit/Trustpilot)</h3>
                    <div className="space-y-4">
                      <div className="bg-red-50 p-3 rounded-lg border border-red-100">
                        <div className="font-semibold text-red-800 text-sm mb-1">Pain Point 1: App Fatigue</div>
                        <p className="text-xs text-red-600 italic">"I just wanted to add a calendar for local pickup and it cost me $15/month for an app that broke my theme."</p>
                      </div>
                      <div className="bg-red-50 p-3 rounded-lg border border-red-100">
                        <div className="font-semibold text-red-800 text-sm mb-1">Pain Point 2: Mobile Limits</div>
                        <p className="text-xs text-red-600 italic">"I can't run my store from my phone while at a craft fair."</p>
                      </div>
                      <div className="bg-red-50 p-3 rounded-lg border border-red-100">
                        <div className="font-semibold text-red-800 text-sm mb-1">Pain Point 3: Reactive AI</div>
                        <p className="text-xs text-red-600">AI helps write descriptions, but won't proactively manage the business or restock inventory.</p>
                      </div>
                    </div>
                  </div>
                </div>
              </OHCPremiumGlassContainer>
            </div>
          )}

          {/* GAPS TAB */}
          {activeTab === "gaps" && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <OHCPremiumGlassContainer className="p-6 md:p-8">
                <SectionHeader title="Gap Matrix: Shopify vs. OHC" icon="⚖️" />
                <div className="overflow-x-auto rounded-xl border border-gray-200">
                  <table className="w-full text-sm text-left">
                    <thead className="bg-gray-50 text-gray-700 uppercase font-semibold text-xs border-b border-gray-200">
                      <tr>
                        <th className="px-6 py-4">Feature Area</th>
                        <th className="px-6 py-4">Shopify</th>
                        <th className="px-6 py-4 text-[#0066FF]">OHC Target</th>
                        <th className="px-6 py-4">Gap / Status</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-gray-100">
                      <tr className="bg-white">
                        <td className="px-6 py-4 font-medium text-gray-900">Setup Speed</td>
                        <td className="px-6 py-4 text-gray-600">Hours / Days</td>
                        <td className="px-6 py-4 text-[#0066FF] font-semibold">&lt; 10 mins</td>
                        <td className="px-6 py-4 text-gray-600">OHC wins on speed, but needs robust autonomous onboarding.</td>
                      </tr>
                      <tr className="bg-gray-50">
                        <td className="px-6 py-4 font-medium text-gray-900">Inventory Mgmt</td>
                        <td className="px-6 py-4 text-gray-600">Complex, manual</td>
                        <td className="px-6 py-4 text-[#0066FF] font-semibold">Basic</td>
                        <td className="px-6 py-4 text-gray-600">OHC lacks proactive, AI-driven predictive restock.</td>
                      </tr>
                      <tr className="bg-white">
                        <td className="px-6 py-4 font-medium text-gray-900">Mobile Mgmt</td>
                        <td className="px-6 py-4 text-gray-600">Desktop-reliant</td>
                        <td className="px-6 py-4 text-[#0066FF] font-semibold">Mobile-first</td>
                        <td className="px-6 py-4 text-gray-600">OHC must ensure 100% functionality on 375px screens.</td>
                      </tr>
                      <tr className="bg-gray-50">
                        <td className="px-6 py-4 font-medium text-gray-900">Bookings</td>
                        <td className="px-6 py-4 text-gray-600">Requires 3rd-party App</td>
                        <td className="px-6 py-4 text-[#0066FF] font-semibold">Built-in</td>
                        <td className="px-6 py-4 text-gray-600">OHC needs native, seamless integration with calendar/ops.</td>
                      </tr>
                      <tr className="bg-white">
                        <td className="px-6 py-4 font-medium text-gray-900">AI Integration</td>
                        <td className="px-6 py-4 text-gray-600">Chatbot / Copywriter</td>
                        <td className="px-6 py-4 text-[#0066FF] font-semibold">Invisible Departments</td>
                        <td className="px-6 py-4 text-gray-600">OHC's unique value prop; requires robust orchestration (pgvector, queues).</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </OHCPremiumGlassContainer>

              <OHCPremiumGlassContainer className="p-6 md:p-8">
                <SectionHeader title="Unresolved Market Pain Points" icon="🚨" />
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <div className="p-5 rounded-xl bg-orange-50 border border-orange-100">
                    <div className="w-8 h-8 rounded-full bg-orange-100 text-orange-600 flex items-center justify-center font-bold mb-3">1</div>
                    <h4 className="font-bold text-gray-900 mb-2">Omnichannel Sync Nightmare</h4>
                    <p className="text-sm text-gray-600">Businesses struggle to keep physical POS inventory synced perfectly with online storefronts without lag or manual reconciliation.</p>
                  </div>
                  <div className="p-5 rounded-xl bg-orange-50 border border-orange-100">
                    <div className="w-8 h-8 rounded-full bg-orange-100 text-orange-600 flex items-center justify-center font-bold mb-3">2</div>
                    <h4 className="font-bold text-gray-900 mb-2">Fragmented Communication</h4>
                    <p className="text-sm text-gray-600">Managing Instagram DMs, WhatsApp, email, and site chat is overwhelming for solo founders.</p>
                  </div>
                  <div className="p-5 rounded-xl bg-orange-50 border border-orange-100">
                    <div className="w-8 h-8 rounded-full bg-orange-100 text-orange-600 flex items-center justify-center font-bold mb-3">3</div>
                    <h4 className="font-bold text-gray-900 mb-2">Lack of Proactive Advisory</h4>
                    <p className="text-sm text-gray-600">No platform tells a user <em>what to do</em> with their financial data in plain English.</p>
                  </div>
                </div>
              </OHCPremiumGlassContainer>
            </div>
          )}

          {/* SOLUTIONS TAB */}
          {activeTab === "solutions" && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <OHCPremiumGlassContainer className="p-6 md:p-8">
                <SectionHeader title="Agentic Solutions for Market Gaps" icon="🧠" />

                <div className="space-y-8">
                  <div className="flex flex-col md:flex-row gap-6 p-6 rounded-xl bg-white border border-gray-100 shadow-sm relative overflow-hidden">
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-blue-500"></div>
                    <div className="md:w-1/3">
                      <div className="text-sm font-bold text-blue-600 uppercase tracking-wider mb-2">Solution 1</div>
                      <h3 className="text-xl font-bold text-gray-900 font-outfit">Invisible Local Delivery & Inventory Mesh</h3>
                      <p className="text-sm text-gray-500 mt-2">Solves: Omnichannel Inventory & Order Chaos</p>
                    </div>
                    <div className="md:w-2/3 text-sm text-gray-700 leading-relaxed bg-gray-50 p-4 rounded-lg">
                      <strong>How it works:</strong> The <em>Operations (The Manager)</em> department uses vision models to scan incoming inventory via the mobile app, instantly updating the pgvector store. It continuously monitors sales velocity. When stock drops, it drafts a reorder email to the supplier, requiring only a one-tap approval from the user. For local delivery, it optimizes delivery routes dynamically.
                    </div>
                  </div>

                  <div className="flex flex-col md:flex-row gap-6 p-6 rounded-xl bg-white border border-gray-100 shadow-sm relative overflow-hidden">
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-purple-500"></div>
                    <div className="md:w-1/3">
                      <div className="text-sm font-bold text-purple-600 uppercase tracking-wider mb-2">Solution 2</div>
                      <h3 className="text-xl font-bold text-gray-900 font-outfit">Omnichannel AI Inbox (The Ambassador)</h3>
                      <p className="text-sm text-gray-500 mt-2">Solves: Fragmented Customer Communication</p>
                    </div>
                    <div className="md:w-2/3 text-sm text-gray-700 leading-relaxed bg-gray-50 p-4 rounded-lg">
                      <strong>How it works:</strong> The <em>Customer Success</em> department ingests messages from all channels (IG, WhatsApp, Email). Using context from the CRM memory, it drafts personalized responses. High-confidence answers are auto-sent; complex ones are surfaced as "Drafts for Review" on the mobile dashboard.
                    </div>
                  </div>

                  <div className="flex flex-col md:flex-row gap-6 p-6 rounded-xl bg-white border border-gray-100 shadow-sm relative overflow-hidden">
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-green-500"></div>
                    <div className="md:w-1/3">
                      <div className="text-sm font-bold text-green-600 uppercase tracking-wider mb-2">Solution 3</div>
                      <h3 className="text-xl font-bold text-gray-900 font-outfit">Plain-Language Daily Briefing</h3>
                      <p className="text-sm text-gray-500 mt-2">Solves: Lack of Actionable Insights</p>
                    </div>
                    <div className="md:w-2/3 text-sm text-gray-700 leading-relaxed bg-gray-50 p-4 rounded-lg">
                      <strong>How it works:</strong> The <em>Advisory</em> department synthesizes daily data (sales, web traffic, social engagement) and delivers a 3-bullet push notification every morning. E.g., "Yesterday's revenue was $400. The blue dress is trending on Instagram. I drafted a promotional email for it—tap to approve."
                    </div>
                  </div>
                </div>
              </OHCPremiumGlassContainer>
            </div>
          )}

        </div>
      </div>
    </div>
  );
}
