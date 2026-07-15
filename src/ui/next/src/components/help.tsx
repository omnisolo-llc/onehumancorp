"use client";

import React, { createContext, useContext, useState, useEffect, useRef, ReactNode } from "react";
import DOMPurify from 'dompurify';
import { useRouter } from 'next/navigation';
import { WithTooltip } from './TooltipRegistry';
import { InteractiveWalkthrough, Step } from './Walkthrough';

// --- Walkthrough System ---

type HelpArticle = { title: string; desc: string; link?: string; category?: string };
type HelpVideo = { id: number; title: string; duration: string; video_url?: string; };
type HelpTab = "center" | "chat" | "videos" | "whatsnew";
type ChatMessage = { id: string; role: "bot" | "user"; text: string; linkUrl?: string; linkTitle?: string };

const helpTabs = [
  { id: "center", label: "Help" },
  { id: "chat", label: "Ask anything" },
  { id: "videos", label: "Videos" },
  { id: "whatsnew", label: "New" }
] as const;

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isSafeLink(url: unknown): url is string {
  return typeof url === "string" && (url.startsWith("/") || url.startsWith("https://") || url.startsWith("http://"));
}

function normalizeArticles(data: unknown): HelpArticle[] {
  if (!Array.isArray(data)) return [];
  return data.flatMap((item) => {
    if (!isRecord(item) || typeof item.title !== "string" || typeof item.desc !== "string") return [];
    return [{ title: item.title, desc: item.desc, link: isSafeLink(item.link) ? item.link : undefined }];
  });
}

function normalizeVideos(data: unknown): HelpVideo[] {
  if (!Array.isArray(data)) return [];
  return data.flatMap((item) => {
    if (!isRecord(item) || typeof item.id !== "number" || typeof item.title !== "string" || typeof item.duration !== "string") return [];
    return [{ id: item.id, title: item.title, duration: item.duration }];
  });
}

function normalizeChatReply(data: unknown): Omit<ChatMessage, "id" | "role"> {
  if (!isRecord(data) || typeof data.reply !== "string" || !data.reply.trim()) {
    throw new Error("Invalid chat reply");
  }

  const link = isRecord(data.link) ? data.link : undefined;
  return {
    text: data.reply,
    linkUrl: isSafeLink(link?.url) ? link.url : undefined,
    linkTitle: typeof link?.title === "string" && link.title.trim() ? link.title : undefined
  };
}

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
          steps={steps.map(s => ({ targetId: s.targetId, title: s.title, content: s.content, position: "top" }))}
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
  const router = useRouter();
  const { startWalkthrough } = useWalkthrough();
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<HelpTab>("center");
  const [chatMessages, setChatMessages] = useState<ChatMessage[]>([
    { id: "welcome", role: "bot", text: "Hi! I'm your AI Support Agent. How can I help you grow your business today?" }
  ]);
  const [chatInput, setChatInput] = useState("");
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    const handleOpenHelpChat = () => {
      setOpen(true);
      setTab("chat");
    };
    window.addEventListener('open-help-chat', handleOpenHelpChat);
    return () => window.removeEventListener('open-help-chat', handleOpenHelpChat);
  }, []);
  const nextMessageId = useRef(1);

  const [helpArticles, setHelpArticles] = useState<HelpArticle[]>([]);

  useEffect(() => {
    fetch("/api/help")
      .then(res => {
        if (!res.ok) throw new Error("Failed to load help articles");
        return res.json();
      })
      .then(data => {
        setHelpArticles(normalizeArticles(data));
      })
      .catch(() => {});
  }, []);

  const filteredArticles = helpArticles.filter(a =>
    a.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    a.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );
  const [videos, setVideos] = useState<HelpVideo[]>([]);
  const [activeVideo, setActiveVideo] = useState<HelpVideo | null>(null);

  useEffect(() => {
    fetch("/api/videos")
      .then(res => {
        if (!res.ok) throw new Error("Failed to load videos");
        return res.json();
      })
      .then(data => {
        setVideos(normalizeVideos(data));
      })
      .catch(() => {});
  }, []);

  const handleChatSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const val = chatInput.trim();
    if (!val) return;

    setChatInput("");
    setChatMessages(prev => [...prev, { id: `user-${nextMessageId.current++}`, role: "user", text: val }]);

    try {
      const response = await fetch("/api/chat", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ message: val }) });
      if (!response.ok) throw new Error("Failed to fetch chat reply");
      const data = await response.json();
      const reply = normalizeChatReply(data);
      setChatMessages(prev => [...prev, { id: `bot-${nextMessageId.current++}`, role: "bot", ...reply }]);
    } catch (err) {
      setChatMessages(prev => [...prev, { id: `bot-${nextMessageId.current++}`, role: "bot", text: "Sorry, I'm having trouble connecting right now." }]);
    }
  };

  return (
    <>
      <div className="fixed bottom-6 right-6 z-[90]" data-ui-overlay="true">
        <WithTooltip id="help-btn-tooltip" defaultText="Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.">
          <button
            id="ohc-floating-help-btn"
            onClick={() => setOpen(!open)}
            className="w-14 h-14 bg-blue-600/90 backdrop-blur-[30px] saturate-200 text-white rounded-full shadow-[0_12px_40px_rgba(37,99,235,0.4)] flex items-center justify-center hover:bg-blue-700/90 active:scale-95 transition-all min-h-[44px] min-w-[44px]"
            aria-label="Open help chat"
          >
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>
        </WithTooltip>
      </div>

      {open && (
        <div id="ohc-floating-help-widget" data-ui-overlay="true" className="fixed bottom-24 right-4 sm:right-6 w-[calc(100vw-32px)] sm:w-[380px] h-[75vh] sm:h-[550px] max-h-[700px] backdrop-blur-[40px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.12)] flex flex-col overflow-hidden z-[90] border border-white/60 transition-all font-inter">
          <div className="flex border-b border-white/30 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] overflow-x-auto scrollbar-hide relative pr-12">
            {helpTabs.map((t) => (
              <button
                key={t.id}
                onClick={() => setTab(t.id)}
                className={`flex-1 min-w-[80px] min-h-[44px] px-3 py-3 text-sm font-bold transition-all whitespace-nowrap ${
                  tab === t.id ? "border-b-2 border-blue-600 text-blue-600" : "text-gray-600 hover:text-gray-900 hover:bg-white/20 dark:hover:bg-[#16161a]/20"
                }`}
                aria-pressed={tab === t.id}
              >
                {t.label}
              </button>
            ))}
            <button
              id="ohc-floating-help-close"
              onClick={() => setOpen(false)}
              className="absolute right-2 top-2 p-1.5 text-gray-500 hover:bg-gray-100 hover:text-gray-800 rounded-full transition-colors z-10 min-h-[32px] min-w-[32px] flex items-center justify-center"
              aria-label="Close Help Widget"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>

          <div className="flex-1 overflow-y-auto p-4 bg-gray-50">
            {tab === "center" && (
              <div>
                <h3 className="font-bold font-outfit text-gray-900 mb-4 text-xl">In-App Help Center</h3>
                <input type="text" placeholder="Search for help..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} className="w-full p-4 border border-white/50 rounded-2xl mb-6 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] min-h-[44px]" />
                <div className="space-y-6 mb-8">
                  {Array.from(
                    filteredArticles.reduce((acc, article) => {
                      const cat = article.category || "Other";
                      if (!acc.has(cat)) acc.set(cat, []);
                      acc.get(cat)!.push(article);
                      return acc;
                    }, new Map<string, HelpArticle[]>())
                  ).map(([category, articles], cIdx) => (
                    <div key={cIdx}>
                      <h4 className="font-bold font-outfit text-gray-800 mb-3 text-lg">{category}</h4>
                      <div className="space-y-3">
                        {articles.map((a, aIdx) => (
                          <div key={aIdx} className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-5 rounded-2xl shadow-sm border border-white/60 cursor-pointer hover:border-blue-300 hover:shadow-md transition-all">
                            {a.link ? (
                              <a href={a.link} className="block min-h-[44px]"><h4 className="font-bold font-outfit text-blue-700 text-base hover:underline">{a.title}</h4></a>
                            ) : (
                              <h4 className="font-bold font-outfit text-gray-800 text-base">{a.title}</h4>
                            )}
                            <p className="text-sm text-gray-600 mt-2 leading-relaxed">{a.desc}</p>
                          </div>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>

                                <h3 className="font-bold font-outfit text-gray-900 mb-4 text-lg">Interactive Tours</h3>
                <div className="space-y-3">
                  <WithTooltip id="walkthrough-btn-tooltip" defaultText="Start an interactive guide to learn how to use OHC.">
                  <button onClick={() => { setOpen(false); fetch("/api/walkthrough/store-setup").then(res => res.json()).then(data => data && data.length > 0 ? startWalkthrough(data) : startWalkthrough([{ targetId: "bio-input-tooltip", title: "Business Description", content: "Enter your business description." }, { targetId: "generate-btn-tooltip", title: "Generate", content: "Click to generate!" }])); }} className="w-full text-left bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-blue-100 hover:bg-blue-100/90 hover:shadow-md transition-all min-h-[44px]">
                    <span className="font-bold font-outfit text-blue-800 text-base block">Tour: Set up your store</span>
                  </button>
                  </WithTooltip>
                  <button onClick={() => { setOpen(false); fetch("/api/walkthrough/pos").then(res => res.json()).then(data => data && data.length > 0 ? startWalkthrough(data) : startWalkthrough([{ targetId: "pos-keypad", title: "Enter Amount", content: "Type in the total sale amount using the keypad." }, { targetId: "charge-btn", title: "Charge Customer", content: "Tap here to process the payment. It's that easy!" }])); }} className="w-full text-left bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-blue-100 hover:bg-blue-100/90 hover:shadow-md transition-all min-h-[44px]">
                    <span className="font-bold font-outfit text-blue-800 text-base block">Tour: Accept your first payment</span>
                  </button>
                  <button onClick={() => { setOpen(false); fetch("/api/walkthrough/assistant").then(res => res.json()).then(data => data && data.length > 0 ? startWalkthrough(data) : startWalkthrough([{ targetId: "ai-chat-trigger", title: "Open Assistant", content: "Click here to open your AI Support Agent." }, { targetId: "ohc-help-input-area", title: "Ask Anything", content: "Type your request here and the agent will handle it while you sleep." }])); }} className="w-full text-left bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-blue-100 hover:bg-blue-100/90 hover:shadow-md transition-all min-h-[44px]">
                    <span className="font-bold font-outfit text-blue-800 text-base block">Tour: Activate your AI Support Agent</span>
                  </button>
                  <button onClick={() => { setOpen(false); fetch("/api/walkthrough/meeting-room").then(res => res.json()).then(data => data && data.length > 0 ? startWalkthrough(data) : startWalkthrough([{ targetId: "help-widget-container", title: "Virtual Meeting Room", content: "Agents join the Virtual Meeting Room to debate and plan before executing tasks." }, { targetId: "help-widget-container", title: "UltraPlan Protocol", content: "Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol)." }])); }} className="w-full text-left bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-blue-100 hover:bg-blue-100/90 hover:shadow-md transition-all min-h-[44px]">
                    <span className="font-bold font-outfit text-blue-800 text-base block">Tour: Virtual Meeting Room & UltraPlan</span>
                  </button>
                  <button onClick={() => { setOpen(false); startWalkthrough([{ targetId: "help-widget-container", title: "Set up your store", content: "Customize your offers, design, and settings to start accepting customers." }, { targetId: "help-widget-container", title: "Accept your first payment", content: "Connect your bank to receive funds securely." }, { targetId: "help-widget-container", title: "Activate your AI Support Agent", content: "Let our agents handle common questions and triage messages for you." }]); }} className="w-full text-left bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-blue-100 hover:bg-blue-100/90 hover:shadow-md transition-all min-h-[44px]">
                    <span className="font-bold font-outfit text-blue-800 text-base block">Tour: Store Setup</span>
                  </button>
                  <button
                    id="kairos-walkthrough-btn"
                    onClick={() => {
                      setOpen(false);
                      router.push("/kairos?walkthrough=true");
                    }}
                    className="w-full text-left bg-indigo-50/80 backdrop-blur-[30px] saturate-200 p-4 rounded-2xl shadow-sm border border-indigo-100 hover:bg-indigo-100/90 hover:shadow-md transition-all min-h-[44px]"
                  >
                    <span className="font-bold font-outfit text-indigo-800 text-base block">Tour: KAIROS AI OS Orchestration</span>
                  </button>
                </div>
              </div>
            )}

            {tab === "chat" && (
              <div className="flex flex-col h-full bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] rounded-xl p-2">
                <div className="flex-1 space-y-4 overflow-y-auto pr-2 pb-2">
                  {chatMessages.map((msg) => {
                    const className = `p-3 rounded-2xl text-sm w-4/5 ${
                      msg.role === "bot"
                        ? "backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/60 dark:border-white/10 shadow-sm text-blue-900 rounded-tl-none"
                        : "backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/60 dark:border-white/10 shadow-sm text-gray-800 rounded-tr-none ml-auto"
                    }`;
                    return msg.role === "bot" ? (
                      <div key={msg.id} className={className}>
                        <div dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(msg.text) }} />
                        {msg.linkUrl && (
                          <div className="mt-2 pt-2 border-t border-blue-100">
                            <a href={msg.linkUrl} className="text-blue-600 font-medium hover:underline text-xs">Read the full article →</a>
                          </div>
                        )}
                      </div>
                    ) : (
                      <div key={msg.id} className={className}>{msg.text}</div>
                    );
                  })}
                </div>
                <form onSubmit={handleChatSubmit} className="mt-4 flex gap-2 pt-3 border-t border-white/50">
                  <input
                    type="text"
                    placeholder="Ask anything..."
                    value={chatInput}
                    onChange={(e) => setChatInput(e.target.value)}
                    className="flex-1 p-3 border border-white/60 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] shadow-[0_8px_32px_rgba(0,0,0,0.08)] min-h-[44px]"
                  />
                  <button type="submit" disabled={!chatInput.trim()} className="bg-blue-600/90 backdrop-blur-[30px] saturate-[210%] text-white p-3 rounded-xl hover:bg-blue-700/90 shadow-sm active:scale-95 transition-all disabled:opacity-50 disabled:cursor-not-allowed min-w-[44px] min-h-[44px] flex items-center justify-center" aria-label="Send message">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                  </button>
                </form>
              </div>
            )}

            {tab === "videos" && (
              <div>
                <h3 className="font-bold font-outfit text-gray-900 mb-4 text-xl">Tutorials</h3>
                <div className="grid grid-cols-2 gap-4">
                  {videos.map((v) => (
                    <div key={v.id} onClick={() => setActiveVideo(v)} className="aspect-[9/16] bg-gray-200 rounded-2xl flex items-center justify-center relative overflow-hidden group cursor-pointer shadow-sm border border-white/30">
                      <div className="absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-all"></div>
                        <div className="w-10 h-10 bg-white/90 backdrop-blur-3xl saturate-[210%] rounded-full flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
                        <svg className="w-5 h-5 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                      </div>
                      <div className="absolute bottom-2 left-2 right-2 z-10">
                        <p className="text-white text-xs font-bold drop-shadow-md line-clamp-2 leading-tight">{v.title}</p>
                        <p className="text-white/80 text-[10px] font-medium mt-0.5">{v.duration}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {tab === "whatsnew" && (
              <div>
                <h3 className="font-bold font-outfit text-gray-900 mb-4 text-xl">What's New</h3>
                <div className="w-full aspect-video bg-gray-200 rounded-2xl mb-6 relative overflow-hidden border border-white/50 shadow-md flex items-center justify-center">
                   <div className="w-full h-full bg-gradient-to-br from-blue-100 to-indigo-100 flex items-center justify-center text-blue-400">
                     <svg className="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 002-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>
                   </div>
                </div>
                <div className="app-card border border-white/50 p-5 rounded-2xl shadow-sm mb-6">
                  <span className="inline-block px-2 py-1 bg-blue-100 text-blue-700 text-xs font-bold rounded-md mb-2">LATEST</span>
                  <h4 className="font-bold font-outfit text-gray-900 text-base mb-2">New AI Store Builder</h4>
                  <p className="text-sm text-gray-600 leading-relaxed mb-4">You can now generate a complete storefront from just a short description of your business. Try it out in the Storefront Builder.</p>

                  <WithTooltip id="changelog-nav-tooltip" defaultText="See what's new in the latest OneHumanCorp updates.">
                    <a href="/changelog" className="inline-flex items-center text-blue-600 text-sm font-bold hover:text-blue-800 transition-colors bg-blue-50/80 px-4 py-2 rounded-xl min-h-[44px]">
                      Read full release notes
                      <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
                    </a>
                  </WithTooltip>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Video Player Modal */}
      {activeVideo && (
        <div
          className="fixed inset-0 z-[100] flex items-center justify-center bg-gray-900/80 backdrop-blur-md saturate-200 p-4 animate-fade-in"
          onClick={() => setActiveVideo(null)}
          role="dialog"
          aria-modal="true"
        >
          <div
            className="bg-black backdrop-blur-3xl rounded-3xl shadow-[0_12px_40px_rgba(0,0,0,0.5)] flex flex-col overflow-hidden border border-white/20 w-full max-w-[375px] mx-auto aspect-[9/16] relative animate-pop-in"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Header */}
            <div className="absolute top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/90 to-transparent z-10 flex justify-between items-start pt-6">
              <h3 className="text-white font-bold font-outfit text-base pr-4 line-clamp-2 drop-shadow-md leading-tight">{activeVideo.title}</h3>
              <WithTooltip id="video-close-btn-tooltip" defaultText="Close video player">
                <button
                  onClick={() => setActiveVideo(null)}
                  className="text-white/80 hover:text-white bg-white/20 hover:bg-white/30 backdrop-blur-3xl saturate-[210%] border border-white/20 rounded-full p-2 transition-all min-h-[44px] min-w-[44px] flex items-center justify-center flex-shrink-0"
                  aria-label="Close video"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </WithTooltip>
            </div>

            {/* Real Video Player area */}
            <div className="flex-1 flex items-center justify-center relative bg-black">
               <video
                 controls
                 className="w-full h-full object-contain"
                 src={activeVideo.video_url || undefined}
                 autoPlay
               >
                 Your browser does not support the video tag.
               </video>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
