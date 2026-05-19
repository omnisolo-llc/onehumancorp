"use client";

import React, { createContext, useContext, useState, useEffect, useRef, ReactNode } from "react";

// --- Tooltip Registry & Component ---
type TooltipContextType = {
  registerTooltip: (id: string, text: string) => void;
  getTooltip: (id: string) => string | undefined;
};

const TooltipContext = createContext<TooltipContextType | undefined>(undefined);

export function TooltipRegistryProvider({ children }: { children: ReactNode }) {
  const [tooltips, setTooltips] = useState<Record<string, string>>({});

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
      {activeStep && (
        <div className="fixed inset-0 z-[100] pointer-events-none flex flex-col">
          <div className="absolute inset-0 bg-transparent pointer-events-auto" onClick={endWalkthrough} />

          <div className="absolute inset-0 pointer-events-none overflow-hidden">
            {/* Highlight box */}
            <div
              className="absolute border-4 border-blue-500 rounded-xl transition-all duration-300 pointer-events-none shadow-[0_0_0_9999px_rgba(0,0,0,0.5)] z-[100]"
              style={{ ...highlightStyle, position: 'absolute' }}
            />

            <div className="pointer-events-auto fixed bottom-24 left-1/2 -translate-x-1/2 bg-white rounded-2xl shadow-2xl p-6 max-w-sm w-full z-[101]">
               <div className="flex justify-between items-start mb-2">
                 <h3 className="font-bold text-gray-900 text-lg">Quick Guide</h3>
                 <span className="text-xs font-semibold text-gray-500 bg-gray-100 px-2 py-1 rounded-full">
                   {currentStepIndex + 1} of {steps.length}
                 </span>
               </div>
               <p className="text-gray-700 mb-6 leading-relaxed text-sm">{activeStep.message}</p>
               <div className="flex justify-between items-center">
                 <button
                   onClick={endWalkthrough}
                   className="text-gray-500 text-sm font-medium hover:text-gray-700 px-2 py-1"
                 >
                   Skip
                 </button>
                 <button
                   onClick={nextStep}
                   className="bg-blue-600 text-white px-6 py-2 rounded-xl text-sm font-bold shadow-md hover:bg-blue-700 active:scale-95 transition-all"
                 >
                   {currentStepIndex < steps.length - 1 ? "Next" : "Got it"}
                 </button>
               </div>
            </div>
          </div>
        </div>
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
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<"center" | "chat" | "videos" | "whatsnew">("center");
  const [chatMessages, setChatMessages] = useState<{role: "bot" | "user", text: string}[]>([
    { role: "bot", text: "Hi! I'm your AI Support Agent. How can I help you grow your business today?" }
  ]);
  const [chatInput, setChatInput] = useState("");

  const handleChatSubmit = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && chatInput.trim()) {
      const val = chatInput.trim();
      setChatInput("");
      setChatMessages(prev => [...prev, { role: "user", text: val }]);

      try {
        const response = await fetch("/api/chat", { method: "POST", body: JSON.stringify({ message: val }) });
        const data = await response.json();
        setChatMessages(prev => [...prev, { role: "bot", text: data.reply }]);
      } catch (err) {
        setChatMessages(prev => [...prev, { role: "bot", text: "Sorry, I'm having trouble connecting right now." }]);
      }
    }
  };

  return (
    <>
      <button
        onClick={() => setOpen(!open)}
        className="fixed bottom-6 right-6 w-14 h-14 bg-blue-600 text-white rounded-full shadow-2xl flex items-center justify-center hover:bg-blue-700 active:scale-95 transition-all z-[90]"
        aria-label="Help"
      >
        <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      </button>

      {open && (
        <div className="fixed bottom-24 right-6 w-[350px] h-[500px] bg-white rounded-2xl shadow-2xl flex flex-col overflow-hidden z-[90] border border-gray-100">
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
                <input type="text" placeholder="Search for help..." className="w-full p-3 border border-gray-200 rounded-xl mb-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                <div className="space-y-2">
                  <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-300">
                    <h4 className="font-bold text-gray-800 text-sm">Getting Started</h4>
                    <p className="text-xs text-gray-500 mt-1">Learn the basics of setting up your store.</p>
                  </div>
                  <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-300">
                    <h4 className="font-bold text-gray-800 text-sm">Accepting Payments</h4>
                    <p className="text-xs text-gray-500 mt-1">Connect your bank and get paid.</p>
                  </div>
                </div>
              </div>
            )}

            {tab === "chat" && (
              <div className="flex flex-col h-full">
                <div className="flex-1 space-y-4 overflow-y-auto pr-2 pb-2">
                  {chatMessages.map((msg, idx) => (
                    <div
                      key={idx}
                      className={`p-3 rounded-2xl text-sm w-4/5 ${
                        msg.role === "bot"
                          ? "bg-blue-50 text-blue-900 rounded-tl-none"
                          : "bg-gray-100 text-gray-800 rounded-tr-none ml-auto"
                      }`}
                    >
                      {msg.text}
                    </div>
                  ))}
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
                <div className="aspect-video bg-gray-200 rounded-xl mb-4 flex items-center justify-center relative overflow-hidden group cursor-pointer">
                  <div className="absolute inset-0 bg-black/20 group-hover:bg-black/10 transition-all"></div>
                  <div className="w-12 h-12 bg-white rounded-full flex items-center justify-center shadow-lg z-10">
                    <svg className="w-6 h-6 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                  </div>
                  <p className="absolute bottom-2 left-3 text-white text-sm font-bold z-10 drop-shadow-md">Set up your store</p>
                </div>
              </div>
            )}

            {tab === "whatsnew" && (
              <div>
                <h3 className="font-bold text-gray-900 mb-4 text-lg">What's New</h3>
                <div className="border-l-2 border-blue-600 pl-4 mb-6">
                  <span className="text-xs font-bold text-blue-600 mb-1 block">TODAY</span>
                  <h4 className="font-bold text-gray-800 text-sm mb-1">New AI Store Builder</h4>
                  <p className="text-xs text-gray-600">You can now generate a complete storefront from just a short description of your business.</p>
                </div>
                <a href="#" className="text-blue-600 text-sm font-bold hover:underline">Read full changelog →</a>
              </div>
            )}
          </div>
        </div>
      )}
    </>
  );
}
