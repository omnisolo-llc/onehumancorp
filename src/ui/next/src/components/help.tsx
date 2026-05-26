"use client";

import React, { createContext, useContext, useState, useEffect, useRef, ReactNode } from "react";
import DOMPurify from 'dompurify';
import { useRouter } from 'next/navigation';
import { WithTooltip } from './TooltipRegistry';
import { InteractiveWalkthrough } from './Walkthrough';

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
  const router = useRouter();
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

  const filteredArticles = helpArticles.filter(a =>
    a.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    a.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );
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
              <div>
                <h3 className="font-bold text-gray-900 mb-4 text-lg">Help Center</h3>
                <input type="text" placeholder="Search for help..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} className="w-full p-3 border border-gray-200 rounded-xl mb-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                <div className="space-y-2 mb-4">
                  {filteredArticles.map((a, idx) => (
                    <div key={idx} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-300">
                      {a.link ? (
                        <a href={a.link}><h4 className="font-bold text-gray-800 text-sm hover:underline">{a.title}</h4></a>
                      ) : (
                        <h4 className="font-bold text-gray-800 text-sm">{a.title}</h4>
                      )}
                      <p className="text-xs text-gray-500 mt-1">{a.desc}</p>
                    </div>
                  ))}
                </div>

                <h3 className="font-bold text-gray-900 mb-2 text-md">Interactive Tours</h3>
                <div className="space-y-2">
                  <button onClick={() => startWalkthrough([{ targetId: "bio-input", message: "Enter your business description." }, { targetId: "generate-btn", message: "Click to generate!" }])} className="w-full text-left bg-blue-50 p-3 rounded-xl shadow-sm border border-blue-100 hover:bg-blue-100 transition-colors">
                    <span className="font-bold text-blue-800 text-sm block">Tour: Set up your store</span>
                  </button>
                  <button onClick={() => startWalkthrough([{ targetId: "stripe-setup-btn", message: "Click here to connect Stripe and start accepting payments." }])} className="w-full text-left bg-blue-50 p-3 rounded-xl shadow-sm border border-blue-100 hover:bg-blue-100 transition-colors">
                    <span className="font-bold text-blue-800 text-sm block">Tour: Accept your first payment</span>
                  </button>
                  <button onClick={() => startWalkthrough([{ targetId: "generate-btn", message: "Activate your AI agent." }])} className="w-full text-left bg-blue-50 p-3 rounded-xl shadow-sm border border-blue-100 hover:bg-blue-100 transition-colors">
                    <span className="font-bold text-blue-800 text-sm block">Tour: Activate your AI Support Agent</span>
                  </button>
                  <button onClick={() => startWalkthrough([{ targetId: "help-widget-container", message: "Agents join the Virtual Meeting Room to debate and plan before executing tasks." }, { targetId: "help-widget-container", message: "Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol)." }])} className="w-full text-left bg-blue-50 p-3 rounded-xl shadow-sm border border-blue-100 hover:bg-blue-100 transition-colors">
                    <span className="font-bold text-blue-800 text-sm block">Tour: Virtual Meeting Room & UltraPlan</span>
                  </button>
                  <button
                    id="kairos-walkthrough-btn"
                    onClick={() => {
                      setOpen(false);
                      router.push("/kairos?walkthrough=true");
                    }}
                    className="w-full text-left bg-indigo-50 p-3 rounded-xl shadow-sm border border-indigo-100 hover:bg-indigo-100 transition-colors"
                  >
                    <span className="font-bold text-indigo-800 text-sm block">Tour: KAIROS AI OS Orchestration</span>
                  </button>
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
              <div>
                <h3 className="font-bold text-gray-900 mb-4 text-lg">Tutorials</h3>
                <div className="grid grid-cols-2 gap-4">
                  {videos.map((v) => (
                    <div key={v.id} className="aspect-[9/16] bg-gray-200 rounded-xl flex items-center justify-center relative overflow-hidden group cursor-pointer">
                      <div className="absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-all"></div>
                      <div className="w-10 h-10 bg-white/90 backdrop-blur-sm rounded-full flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
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
                <h3 className="font-bold text-gray-900 mb-4 text-lg">What's New</h3>
                <div className="w-full aspect-video bg-gray-200 rounded-xl mb-4 relative overflow-hidden border border-gray-100 shadow-sm flex items-center justify-center">
                   <div className="w-full h-full bg-gradient-to-br from-blue-50 to-indigo-50 flex items-center justify-center text-blue-200">
                     <svg className="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
                   </div>
                </div>
                <div className="border-l-2 border-blue-600 pl-4 mb-6">
                  <span className="text-xs font-bold text-blue-600 mb-1 block">TODAY</span>
                  <h4 className="font-bold text-gray-800 text-sm mb-1">New AI Store Builder</h4>
                  <p className="text-xs text-gray-600">You can now generate a complete storefront from just a short description of your business.</p>
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
