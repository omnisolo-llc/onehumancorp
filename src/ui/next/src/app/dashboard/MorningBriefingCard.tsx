"use client";

import { useEffect, useState } from "react";
import { WithTooltip } from "../../components/TooltipRegistry";


type TriageItem = {
  id: string;
  tenant_id: string;
  source?: string;
  priority?: string;
  context?: string;
  action_type?: string;
  action_payload?: string;
  status?: string;
  created_at: string;
};

export function MorningBriefingCard({ tenant }: { tenant: string }) {
  const [briefing, setBriefing] = useState<string>("Loading your Morning Briefing...");
  const [loading, setLoading] = useState(true);
  const [triageItems, setTriageItems] = useState<TriageItem[]>([]);
  const [triageLoading, setTriageLoading] = useState(true);
  const [chatMessage, setChatMessage] = useState("");
  const [chatHistory, setChatHistory] = useState<{ role: "user" | "agent"; text: string }[]>([]);
  const [isChatting, setIsChatting] = useState(false);

  useEffect(() => {
    async function loadBriefing() {
try {
        const [resBriefing, resTriage] = await Promise.all([
          fetch(`/api/ui/dashboard/analytics/briefing?tenant_id=${encodeURIComponent(tenant)}`),
          fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenant)}`)
        ]);

        if (resBriefing.ok) {
          const data = await resBriefing.json();
          if (data.briefing) {
            setBriefing(data.briefing);
          } else {
            setBriefing("Good morning. No new insights at this time.");
          }
        } else {
          setBriefing("Unable to load Morning Briefing.");
        }

        if (resTriage.ok) {
          const triageData = await resTriage.json();
          const rows = Array.isArray(triageData) ? triageData : (Array.isArray(triageData?.items) ? triageData.items : []);
          setTriageItems(rows);
        }
      } catch {
        setBriefing("Unable to load Morning Briefing.");
      } finally {
        setLoading(false);
        setTriageLoading(false);
      }
    }
    loadBriefing();
  }, [tenant]);

  const handleTriageDecision = async (id: string, approved: boolean) => {
    try {
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenant)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
      });
      if (!res.ok) throw new Error("Failed to update action");

      setTriageItems(prev => prev.filter(i => i.id !== id));
    } catch (e) {
      console.error(e);
    }
  };

  const handleChat = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!chatMessage.trim() || isChatting) return;

    const userMsg = chatMessage.trim();
    setChatHistory((prev) => [...prev, { role: "user", text: userMsg }]);
    setChatMessage("");
    setIsChatting(true);

    try {
      const res = await fetch(`/api/ui/dashboard/analytics/chat?tenant_id=${encodeURIComponent(tenant)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: userMsg }),
      });
      if (res.ok) {
        const data = await res.json();
        setChatHistory((prev) => [...prev, { role: "agent", text: data.reply || "I encountered an error retrieving that information." }]);
      } else {
        setChatHistory((prev) => [...prev, { role: "agent", text: "I encountered an error retrieving that information." }]);
      }
    } catch {
      setChatHistory((prev) => [...prev, { role: "agent", text: "I encountered an error retrieving that information." }]);
    } finally {
      setIsChatting(false);
    }
  };

  return (
    <div className="p-6 mb-6 shadow-sm w-full relative overflow-hidden group bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:saturate-[210%] dark:border-white/10">
      <div className="absolute top-0 right-0 w-24 h-24 bg-white/40 rounded-bl-full -z-10 group-hover:scale-110 transition-transform"></div>

      <div className="flex flex-col gap-4">
        <div>
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 flex items-center gap-2">
            <span className="text-2xl">🌅</span>
            <WithTooltip id="morning-briefing" defaultText="Your AI Decision Assistant's daily summary.">
              <span>Morning Briefing</span>
            </WithTooltip>
          </h2>
          {loading ? (
            <div className="animate-pulse h-6 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
          ) : (
            <p className="text-[#1D1D1F] dark:text-[#F5F5F7] font-medium leading-relaxed" data-testid="morning-briefing-text">
              {briefing}
            </p>
          )}
        </div>


        {/* Action Cards Section */}
        {!triageLoading && triageItems.filter(item => item.source === 'Decision Assistant').length > 0 && (
          <div className="mt-4 pb-2 w-full overflow-x-auto flex gap-4 snap-x hide-scrollbar">
            {triageItems.filter(item => item.source === 'Decision Assistant').map((item) => (
              <div key={item.id} className="snap-start shrink-0 w-[280px] p-4 rounded-[12px] glassmorphism border border-orange-200 dark:border-orange-900/50 bg-orange-50/40 dark:bg-orange-900/10 flex flex-col justify-between">
                <div>
                  <div className="flex justify-between items-start mb-2">
                    <span className="text-xs uppercase tracking-wider font-semibold text-orange-800 dark:text-orange-300">Suggested Action</span>
                    <span className="text-[10px] font-bold uppercase text-white bg-[#FF9500] px-2 py-0.5 rounded-full">New</span>
                  </div>
                  <p className="text-sm font-medium text-gray-800 dark:text-gray-200 mb-3">{item.context}</p>
                </div>
                <div className="flex gap-2 w-full">
                  <button
                    onClick={() => handleTriageDecision(item.id, true)}
                    className="flex-1 min-h-[44px] min-w-[44px] rounded-[8px] bg-[#0066FF] hover:bg-[#0052CC] text-white text-sm font-semibold shadow-sm transition-colors flex items-center justify-center"
                    data-testid={`action-card-approve-${item.id}`}
                  >
                    {item.action_type || "Execute Action"}
                  </button>
                  <button
                    onClick={() => handleTriageDecision(item.id, false)}
                    className="min-h-[44px] min-w-[44px] px-3 rounded-[8px] border border-gray-300 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-300 transition-colors flex items-center justify-center"
                    data-testid={`action-card-dismiss-${item.id}`}
                    aria-label="Dismiss Action"
                  >
                    ✕
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}

        <div className="mt-4 pt-4 border-t border-indigo-200/50 dark:border-indigo-900/50">
          <h3 className="text-sm font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-3 flex items-center gap-2">
            <span className="text-indigo-500">✨</span> Insight Chat
          </h3>

          {chatHistory.length > 0 && (
            <div className="mb-4 max-h-[200px] overflow-y-auto pr-2 flex flex-col gap-3">
              {chatHistory.map((msg, i) => (
                <div key={i} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
                  <div className={`px-4 py-2 rounded-[12px] max-w-[80%] text-sm ${msg.role === 'user' ? 'bg-[#0066FF] text-white rounded-br-none' : 'bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-bl-none shadow-sm border border-gray-100 dark:border-gray-700'}`}>
                    {msg.text}
                  </div>
                </div>
              ))}
              {isChatting && (
                <div className="flex justify-start">
                  <div className="px-4 py-2 rounded-[12px] bg-white dark:bg-gray-800 text-gray-500 rounded-bl-none shadow-sm border border-gray-100 dark:border-gray-700 text-sm italic">
                    Thinking...
                  </div>
                </div>
              )}
            </div>
          )}

          <form onSubmit={handleChat} className="flex gap-2">
            <input
              type="text"
              value={chatMessage}
              onChange={(e) => setChatMessage(e.target.value)}
              placeholder="Ask about your business metrics..."
              className="flex-1 rounded-[8px] border border-indigo-200 bg-white/80 dark:bg-black/30 px-4 py-2.5 text-sm text-[#1D1D1F] dark:text-[#F5F5F7] shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 transition-colors"
              disabled={isChatting}
              data-testid="insight-chat-input"
            />
            <button
              type="submit"
              disabled={isChatting || !chatMessage.trim()}
              className="rounded-[8px] min-h-[44px] bg-[#0066FF] px-4 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-[#0052CC] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:opacity-50 transition-colors"
              data-testid="insight-chat-submit"
            >
              Ask
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
