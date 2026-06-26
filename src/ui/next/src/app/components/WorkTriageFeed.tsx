"use client";

type TriageItem = {
  id: string;
  tenant_id: string;
  customer_id?: string;
  source?: string;
  priority?: string;
  context?: string;
  action_type?: string;
  action_payload?: string;
  status?: string;
  created_at: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return (
    localStorage.getItem("tenant_id") ||
    localStorage.getItem("tenant") ||
    "default"
  );
}

function badgeTone(priority?: string) {
  const normalized = (priority || "").toLowerCase();
  if (["urgent", "high"].includes(normalized)) return "bad";
  if (["action needed", "medium"].includes(normalized)) return "warn";
  if (["fyi", "low"].includes(normalized)) return "good";
  return "neutral";
}

export function WorkTriageFeed({
  items,
  loading,
  error,
  onDecision,
}: {
  items: TriageItem[];
  loading: boolean;
  error: string;
  onDecision: (id: string, approved: boolean) => void;
}) {
  if (error) {
    return (
      <div className="w-full mb-4 p-4 glassmorphism rounded-[16px] border border-[#FF3B30]/50 bg-[#FF3B30]/10 text-[#FF3B30] text-center min-h-[44px] flex items-center justify-center">
        {error}
      </div>
    );
  }

  if (loading && items.length === 0) {
    return (
      <div className="flex justify-center items-center py-12 min-h-[44px]">
        <div className="w-6 h-6 border-2 border-[#0066FF] border-t-transparent rounded-full animate-spin"></div>
      </div>
    );
  }

  if (!loading && items.length === 0) {
    return (
      <div
        className="glassmorphism flex flex-col items-center justify-center p-12 text-center border border-white/40 dark:border-white/10 shadow-sm"
        data-testid="triage-feed-empty"
      >
        <div className="w-16 h-16 bg-[#e8f7ef] dark:bg-[rgba(23,166,106,0.2)] rounded-full flex items-center justify-center mb-4">
          <svg
            className="w-8 h-8 text-[#17a66a]"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M5 13l4 4L19 7"
            ></path>
          </svg>
        </div>
        <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-2">
          You're all caught up!
        </h3>
        <p className="text-sm text-gray-500 dark:text-gray-400">
          There are no pending triage actions right now.
        </p>
      </div>
    );
  }

  return (
    <div
      className="w-full max-w-[375px] sm:max-w-full mx-auto"
      data-testid="work-triage-feed"
    >
      {items
        .filter((item) => item.source === "Proactive Context Agent")
        .map((item) => (
          <div
            key={item.id}
            className="ohc-card w-full glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[24px] shadow-sm flex flex-col mb-4 overflow-hidden transition-all duration-300 relative"
            data-testid={`triage-card-${item.id}`}
          >
            <div className="absolute top-0 left-0 w-1 h-full bg-[#0066FF]"></div>
            <div className="p-5 border-b border-[rgba(255,255,255,0.2)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] backdrop-blur-[30px] backdrop-saturate-[210%]">
              <div className="flex justify-between items-start mb-3">
                <div className="flex items-center gap-2">
                  <span className="text-xl">✨</span>
                  <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">
                    Needs Attention Today
                  </span>
                </div>
                <span className={`app-badge ${badgeTone(item.priority)}`}>
                  {item.priority || "High"}
                </span>
              </div>
              <div className="text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words">
                {item.context}
              </div>
            </div>

            {item.action_type && (
              <div className="p-5 bg-[#0066FF]/10 dark:bg-[#0066FF]/20 backdrop-blur-[30px] saturate-[210%] flex flex-col gap-2">
                <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF]">
                  Suggested Action: {item.action_type}
                </div>
                <div className="proposed-action rounded-[16px] border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/50 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] p-4 text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
                  {item.action_payload || "No specific payload"}
                </div>
              </div>
            )}

            <div className="p-5 pt-2 flex flex-col sm:flex-row gap-3 w-full border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
              <button
                onClick={() => onDecision(item.id, true)}
                className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                data-testid={`triage-approve-${item.id}`}
              >
                Approve & Execute
              </button>
              <button
                onClick={() => onDecision(item.id, false)}
                className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center shadow-sm"
                data-testid={`triage-dismiss-${item.id}`}
              >
                Dismiss
              </button>
            </div>
          </div>
        ))}

      {items
        .filter((item) => item.source !== "Proactive Context Agent")
        .map((item) => (
          <div
            key={item.id}
            className="ohc-card w-full glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[24px] shadow-sm flex flex-col mb-4 overflow-hidden transition-all duration-300"
            data-testid={`triage-card-${item.id}`}
          >
            <div className="p-5 border-b border-[rgba(255,255,255,0.2)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] backdrop-blur-[30px] backdrop-saturate-[210%]">
              <div className="flex justify-between items-start mb-3">
                <div className="flex items-center gap-2">
                  <span className="text-xl">✨</span>
                  <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">
                    {item.source || "Triage Action"}
                  </span>
                </div>
                <span className={`app-badge ${badgeTone(item.priority)}`}>
                  {item.priority || "Normal"}
                </span>
              </div>
              <div className="text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words">
                {item.context}
              </div>
            </div>

            {item.action_type && (
              <div className="p-5 bg-[#0066FF]/10 dark:bg-[#0066FF]/20 backdrop-blur-[30px] saturate-[210%] flex flex-col gap-2">
                <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF]">
                  Proposed Action: {item.action_type}
                </div>
                <div className="proposed-action rounded-[16px] border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/50 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] p-4 text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
                  {item.action_payload || "No specific payload"}
                </div>
              </div>
            )}

            <div className="p-5 pt-2 flex flex-col sm:flex-row gap-3 w-full border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
              <button
                onClick={() => onDecision(item.id, true)}
                className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                data-testid={`triage-approve-${item.id}`}
              >
                ✨ Approve & Execute
              </button>
              <button
                onClick={() => onDecision(item.id, false)}
                className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center shadow-sm"
                data-testid={`triage-dismiss-${item.id}`}
              >
                Dismiss
              </button>
            </div>
          </div>
        ))}
    </div>
  );
}
