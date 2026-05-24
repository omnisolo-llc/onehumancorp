"use client";

import React, { createContext, useContext, useState, useEffect, useRef, ReactNode } from "react";
import DOMPurify from 'dompurify';
import { WithTooltip } from './TooltipRegistry';
import { InteractiveWalkthrough } from './Walkthrough';

// --- Tooltip Registry & Component ---
type TooltipContextType = {
  registerTooltip: (id: string, text: string) => void;
  getTooltip: (id: string) => string | undefined;
};

const TooltipContext = createContext<TooltipContextType | undefined>(undefined);

export function TooltipRegistryProvider({ children }: { children: ReactNode }) {
  const [tooltips, setTooltips] = useState<Record<string, string>>({});

  useEffect(() => {
    fetch("/api/tooltips")
      .then(res => res.json())
      .then(data => setTooltips(prev => ({ ...data, ...prev })))
      .catch(() => {});
  }, []);

  const registerTooltip = (id: string, text: string) => {
    setTooltips((prev) => ({ ...prev, [id]: text }));
  };

  const getTooltip = (id: string) => tooltips[id];

  return (
    <TooltipContext.Provider value={{ registerTooltip, getTooltip }}>
      {children}
    </TooltipContext.Provider>
  );
}

export function useTooltipRegistry() {
  const context = useContext(TooltipContext);
  if (!context) throw new Error("useTooltipRegistry must be used within TooltipRegistryProvider");
  return context;
}

export function Tooltip({ id, defaultText, children }: { id?: string; defaultText: string; children: ReactNode }) {
  const [visible, setVisible] = useState(false);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Use context safely, fallback if used outside provider
  const context = useContext(TooltipContext);

  // Register tooltip on mount if ID is provided and registry exists
  useEffect(() => {
    if (id && context && !context.getTooltip(id)) {
      context.registerTooltip(id, defaultText);
    }
  }, [id, defaultText, context]);

  // Determine text to display: Try registry first, fallback to defaultText
  const displayText = (id && context && context.getTooltip(id)) || defaultText;

  const handleMouseEnter = () => setVisible(true);
  const handleMouseLeave = () => setVisible(false);

  const handleTouchStart = () => {
    timeoutRef.current = setTimeout(() => setVisible(true), 500); // long press
  };
  const handleTouchEnd = () => {
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    setVisible(false);
  };

  return (
    <div
      className="relative inline-block w-full"
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onTouchStart={handleTouchStart}
      onTouchEnd={handleTouchEnd}
      onTouchCancel={handleTouchEnd}
    >
      {children}
      {visible && (
        <div className="absolute z-50 bottom-full left-1/2 transform -translate-x-1/2 mb-2 w-max max-w-xs px-3 py-2 text-sm text-white bg-gray-900 rounded-lg shadow-lg pointer-events-none text-center leading-tight">
          {displayText}
          <div className="absolute top-full left-1/2 transform -translate-x-1/2 border-4 border-transparent border-t-gray-900"></div>
        </div>
      )}
    </div>
  );
}

// --- Walkthrough System ---
type Step = {
  targetId: string;
  message: string;
};

type WalkthroughContextType = {
  startWalkthrough: (steps: Step[]) => void;
  nextStep: () => void;
  endWalkthrough: () => void;
};

const WalkthroughContext = createContext<WalkthroughContextType | undefined>(undefined);

export function WalkthroughProvider({ children }: { children: ReactNode }) {
  const [steps, setSteps] = useState<Step[]>([]);
  const [currentStepIndex, setCurrentStepIndex] = useState(-1);

  const startWalkthrough = (newSteps: Step[]) => {
    setSteps(newSteps);
    setCurrentStepIndex(0);
  };

  const nextStep = () => {
    if (currentStepIndex < steps.length - 1) {
      setCurrentStepIndex(prev => prev + 1);
    } else {
      endWalkthrough();
    }
  };

  const endWalkthrough = () => {
    setSteps([]);
    setCurrentStepIndex(-1);
  };

  useEffect(() => {
    if (currentStepIndex >= 0 && currentStepIndex < steps.length) {
      const step = steps[currentStepIndex];
      const el = document.getElementById(step.targetId);
      if (el) {
        el.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    }
  }, [currentStepIndex, steps]);

  const activeStep = currentStepIndex >= 0 ? steps[currentStepIndex] : null;

  const [highlightStyle, setHighlightStyle] = useState({});

  useEffect(() => {
    if (activeStep) {
      const el = document.getElementById(activeStep.targetId);
      if (el) {
        const rect = el.getBoundingClientRect();
        setHighlightStyle({
          top: rect.top - 8,
          left: rect.left - 8,
          width: rect.width + 16,
          height: rect.height + 16,
        });
      }
    }
  }, [activeStep]);

  return (
    <WalkthroughContext.Provider value={{ startWalkthrough, nextStep, endWalkthrough }}>
      {children}
      {steps.length > 0 && (
        <InteractiveWalkthrough
          steps={steps.map(s => ({ targetId: s.targetId, title: "Quick Guide", content: s.message, position: "top" }))}
          isOpen={steps.length > 0}
          onClose={endWalkthrough}
          onComplete={endWalkthrough}
        />
      )}
    </WalkthroughContext.Provider>
  );
}

export function useWalkthrough() {
  const context = useContext(WalkthroughContext);
  if (!context) throw new Error("useWalkthrough must be used within WalkthroughProvider");
  return context;
}

// --- Help Widget System ---
export function HelpWidget() {
  const { startWalkthrough } = useWalkthrough();
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<"center" | "chat" | "videos" | "whatsnew">("center");
  const [chatMessages, setChatMessages] = useState<{role: "bot" | "user", text: string, linkUrl?: string, linkTitle?: string}[]>([
    { role: "bot", text: "Hi! I'm your AI Support Agent. How can I help you grow your business today?" }
  ]);
  const [chatInput, setChatInput] = useState("");
  const [searchQuery, setSearchQuery] = useState("");

  const [helpArticles, setHelpArticles] = useState<{title: string, desc: string, link?: string}[]>([]);

  useEffect(() => {
    fetch("/api/help")
      .then(res => res.json())
      .then(data => {
        if (data && data.length > 0) setHelpArticles(data);
      })
      .catch(() => {});
  }, []);

  const filteredArticles = helpArticles.filter(a => {
    const query = searchQuery.toLowerCase().trim();
    return (
      a.title.toLowerCase().includes(query) ||
      a.desc.toLowerCase().includes(query)
    );
  });
  const [videos, setVideos] = useState<{id: number, title: string, duration: string}[]>([]);

  useEffect(() => {
    fetch("/api/videos")
      .then(res => res.json())
      .then(data => {
        if (data && data.length > 0) setVideos(data);
      })
      .catch(() => {});
  }, []);

  const handleChatSubmit = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && chatInput.trim()) {
      const val = chatInput.trim();
      setChatInput("");
      setChatMessages(prev => [...prev, { role: "user", text: val }]);

      try {
        const response = await fetch("/api/chat", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ message: val }) });
        const data = await response.json();
        setChatMessages(prev => [...prev, { role: "bot", text: data.reply, linkUrl: data.link?.url, linkTitle: data.link?.title }]);
      } catch (err) {
        setChatMessages(prev => [...prev, { role: "bot", text: "Sorry, I'm having trouble connecting right now." }]);
      }
    }
  };

  return (
    <>
      <div className="fixed bottom-6 right-6 z-[90]">
        <WithTooltip id="help-btn-tooltip" defaultText="Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.">
          <button
            onClick={() => setOpen(!open)}
            className="w-14 h-14 bg-blue-600 text-white rounded-full shadow-2xl flex items-center justify-center hover:bg-blue-700 active:scale-95 transition-all"
            aria-label="Help"
          >
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>
        </WithTooltip>
      </div>

      {open && (
        <div id="help-widget-container" className="fixed bottom-24 right-4 sm:right-6 w-[calc(100vw-32px)] sm:w-[350px] h-[75vh] sm:h-[500px] max-h-[600px] bg-white rounded-2xl shadow-2xl flex flex-col overflow-hidden z-[90] border border-gray-100 transition-all">
          <div className="flex border-b border-gray-200">
            {[
              { id: "center", label: "Help" },
              { id: "chat", label: "Ask AI" },
              { id: "videos", label: "Videos" },
              { id: "whatsnew", label: "New" }
            ].map((t) => (
              <button
                key={t.id}
                onClick={() => setTab(t.id as any)}
                className={`flex-1 py-3 text-sm font-bold transition-all ${
                  tab === t.id ? "border-b-2 border-blue-600 text-blue-600" : "text-gray-500 hover:text-gray-700"
                }`}
              >
                {t.label}
              </button>
            ))}
          </div>

          <div className="flex-1 overflow-y-auto p-4 bg-gray-50">
            {tab === "center" && (
              <div className="animate-in fade-in slide-in-from-bottom-2 duration-300">
                <div className="mb-6">
                  <h3 className="font-bold text-gray-900 mb-3 text-xl font-outfit">Help Center</h3>
                  <div className="relative">
                    <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                    <input
                      type="text"
                      placeholder="Search for help..."
                      value={searchQuery}
                      onChange={(e) => setSearchQuery(e.target.value)}
                      className="w-full pl-10 pr-4 py-3 bg-white border border-gray-200 rounded-2xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all shadow-sm"
                    />
                  </div>
                </div>

                <div className="space-y-3 mb-8">
                  {filteredArticles.length > 0 ? (
                    filteredArticles.map((a, idx) => (
                      <div key={idx} className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-300 hover:shadow-md transition-all group">
                        {a.link ? (
                          <a href={a.link} className="block">
                            <div className="flex justify-between items-start">
                              <h4 className="font-bold text-gray-800 text-sm group-hover:text-blue-600 transition-colors">{a.title}</h4>
                              <svg className="w-4 h-4 text-gray-300 group-hover:text-blue-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
                            </div>
                            <p className="text-xs text-gray-500 mt-1 leading-relaxed">{a.desc}</p>
                          </a>
                        ) : (
                          <>
                            <h4 className="font-bold text-gray-800 text-sm">{a.title}</h4>
                            <p className="text-xs text-gray-500 mt-1 leading-relaxed">{a.desc}</p>
                          </>
                        )}
                      </div>
                    ))
                  ) : (
                    <div className="text-center py-8">
                      <p className="text-sm text-gray-500">No articles found for "{searchQuery}"</p>
                    </div>
                  )}
                </div>

                <div className="mb-8">
                  <h3 className="font-bold text-gray-900 mb-3 text-md font-outfit">Interactive Tours</h3>
                  <div className="grid gap-2">
                  <button onClick={() => startWalkthrough([{ targetId: "builder-actions", message: "These buttons allow you to make your store live or get the code to put it on another website." }, { targetId: "store-preview-container", message: "This is what your customers will see when they visit your store." }])} className="w-full text-left bg-blue-50 p-4 rounded-2xl border border-blue-100 hover:bg-blue-100 transition-all flex items-center gap-3 group">
                      <span className="text-xl">🏪</span>
                      <div>
                        <span className="font-bold text-blue-800 text-sm block">Set up your store</span>
                        <span className="text-[10px] text-blue-600 font-medium">2 steps • 1 min</span>
                      </div>
                    </button>
                    <button onClick={() => startWalkthrough([{ targetId: "stripe-setup-btn", message: "Connect your bank account to start receiving money from customers." }])} className="w-full text-left bg-blue-50 p-4 rounded-2xl border border-blue-100 hover:bg-blue-100 transition-all flex items-center gap-3">
                      <span className="text-xl">💰</span>
                      <div>
                        <span className="font-bold text-blue-800 text-sm block">Accept your first payment</span>
                        <span className="text-[10px] text-blue-600 font-medium">1 step • 30 sec</span>
                      </div>
                    </button>
                    <button onClick={() => startWalkthrough([{ targetId: "nav-agents-link", message: "Hire and manage your AI workforce here." }])} className="w-full text-left bg-blue-50 p-4 rounded-2xl border border-blue-100 hover:bg-blue-100 transition-all flex items-center gap-3">
                      <span className="text-xl">🤖</span>
                      <div>
                        <span className="font-bold text-blue-800 text-sm block">Activate your AI Support Agent</span>
                        <span className="text-[10px] text-blue-600 font-medium">1 step • 30 sec</span>
                      </div>
                    </button>
                  </div>
                </div>

                <div className="mt-8 pt-6 border-t border-gray-200">
                  <h3 className="font-bold text-gray-400 mb-3 text-xs uppercase tracking-widest">Advanced</h3>
                  <a href="/api-docs" className="flex items-center justify-between p-4 bg-gray-100 rounded-2xl hover:bg-gray-200 transition-all group">
                    <div className="flex items-center gap-3">
                      <span className="text-xl">🛠️</span>
                      <div>
                        <span className="font-bold text-gray-700 text-sm block">API Documentation</span>
                        <span className="text-[10px] text-gray-500">For developers and advanced users</span>
                      </div>
                    </div>
                    <svg className="w-4 h-4 text-gray-400 group-hover:text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" /></svg>
                  </a>
                </div>
              </div>
            )}

            {tab === "chat" && (
              <div className="flex flex-col h-full">
                <div className="flex-1 space-y-4 overflow-y-auto pr-2 pb-2">
                  {chatMessages.map((msg, idx) => {
                    const className = `p-3 rounded-2xl text-sm w-4/5 ${
                      msg.role === "bot"
                        ? "bg-blue-50 text-blue-900 rounded-tl-none"
                        : "bg-gray-100 text-gray-800 rounded-tr-none ml-auto"
                    }`;
                    return msg.role === "bot" ? (
                      <div key={idx} className={className}>
                        <div dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(msg.text) }} />
                        {msg.linkUrl && msg.linkTitle && (
                          <div className="mt-2 pt-2 border-t border-blue-100">
                            <a href={msg.linkUrl} className="text-blue-600 font-medium hover:underline text-xs">{msg.linkTitle}</a>
                          </div>
                        )}
                      </div>
                    ) : (
                      <div key={idx} className={className}>{msg.text}</div>
                    );
                  })}
                </div>
                <div className="mt-4 flex gap-2 pt-2 border-t border-gray-100">
                  <input
                    type="text"
                    placeholder="Ask anything..."
                    value={chatInput}
                    onChange={(e) => setChatInput(e.target.value)}
                    onKeyDown={handleChatSubmit}
                    className="flex-1 p-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                  <button className="bg-blue-600 text-white p-3 rounded-xl hover:bg-blue-700 active:scale-95 transition-all">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                  </button>
                </div>
              </div>
            )}

            {tab === "videos" && (
              <div className="animate-in fade-in slide-in-from-bottom-2 duration-300">
                <h3 className="font-bold text-gray-900 mb-4 text-xl font-outfit">Tutorials</h3>
                <div className="grid grid-cols-2 gap-4">
                  {videos.map((v) => (
                    <div
                      key={v.id}
                      onClick={() => alert(`Playing: ${v.title}\n(In a real app, this would open a portrait-optimized video player)`)}
                      className="aspect-[9/16] bg-gray-900 rounded-2xl flex items-center justify-center relative overflow-hidden group cursor-pointer shadow-sm hover:shadow-xl transition-all hover:-translate-y-1"
                    >
                      <div className="absolute inset-0 opacity-40 group-hover:opacity-20 transition-opacity">
                         {/* Mock Video Placeholder using Gradient */}
                         <div className="w-full h-full bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500"></div>
                      </div>
                      <div className="w-12 h-12 bg-white/20 backdrop-blur-md rounded-full flex items-center justify-center shadow-2xl z-10 group-hover:scale-110 transition-transform border border-white/30">
                        <svg className="w-6 h-6 text-white ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                      </div>
                      <div className="absolute bottom-0 left-0 right-0 p-3 z-10 bg-gradient-to-t from-black/80 to-transparent">
                        <p className="text-white text-[11px] font-bold leading-tight line-clamp-2">{v.title}</p>
                        <p className="text-white/70 text-[9px] font-medium mt-1 flex items-center gap-1">
                          <svg className="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                          {v.duration}
                        </p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {tab === "whatsnew" && (
              <div className="animate-in fade-in slide-in-from-bottom-2 duration-300">
                <h3 className="font-bold text-gray-900 mb-4 text-xl font-outfit">What's New</h3>
                <div className="w-full aspect-video bg-gray-900 rounded-2xl mb-6 relative overflow-hidden shadow-lg group">
                   <div className="absolute inset-0 bg-gradient-to-tr from-blue-600 to-purple-600 opacity-90"></div>
                   <div className="absolute inset-0 flex items-center justify-center">
                      <div className="text-center p-6">
                         <div className="bg-white/20 backdrop-blur-md rounded-2xl p-4 mb-3 inline-block">
                            <svg className="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" /></svg>
                         </div>
                         <h4 className="text-white font-bold text-lg leading-tight">AI Store Builder v2.0</h4>
                         <p className="text-white/80 text-xs mt-1">Faster, smarter, and more beautiful.</p>
                      </div>
                   </div>
                   <div className="absolute top-3 right-3 bg-white/20 backdrop-blur-md px-2 py-1 rounded-md border border-white/30">
                      <span className="text-[10px] font-bold text-white uppercase tracking-wider">Update</span>
                   </div>
                </div>

                <div className="space-y-6">
                  <div className="relative pl-6 border-l-2 border-blue-500">
                    <div className="absolute -left-1.5 top-0 w-3 h-3 bg-blue-500 rounded-full border-2 border-white shadow-sm"></div>
                    <span className="text-[10px] font-extrabold text-blue-600 mb-1 block uppercase tracking-tighter">New Feature</span>
                    <h4 className="font-bold text-gray-800 text-sm mb-1">Interactive Storefront Preview</h4>
                    <p className="text-xs text-gray-600 leading-relaxed">See your changes in real-time before you hit publish. Our new preview engine is 3x faster.</p>
                  </div>

                  <div className="relative pl-6 border-l-2 border-gray-200">
                    <div className="absolute -left-1.5 top-0 w-3 h-3 bg-gray-200 rounded-full border-2 border-white shadow-sm"></div>
                    <span className="text-[10px] font-extrabold text-gray-400 mb-1 block uppercase tracking-tighter">Improvement</span>
                    <h4 className="font-bold text-gray-800 text-sm mb-1">Smarter AI Copywriting</h4>
                    <p className="text-xs text-gray-600 leading-relaxed">We've updated our AI to write even more compelling product descriptions for your store.</p>
                  </div>
                </div>
                <WithTooltip id="changelog-nav-tooltip" defaultText="See what's new in the latest OneHumanCorp updates.">
                  <a href="/changelog" className="text-blue-600 text-sm font-bold hover:underline">Read full changelog →</a>
                </WithTooltip>
              </div>
            )}
          </div>
        </div>
      )}
    </>
  );
}
