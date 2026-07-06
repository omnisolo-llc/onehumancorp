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
      <div className="w-full mb-4 p-4 glassmorphism border border-[#FF3B30]/50 bg-[#FF3B30]/10 text-[#FF3B30] text-center min-h-[44px] flex items-center justify-center">
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
            className="mb-6 p-6 glassmorphism border border-orange-400/50 dark:border-[#FF9500]/30 bg-orange-50/50 dark:bg-orange-900/10 shadow-lg relative overflow-hidden"
            data-testid={`triage-card-${item.id}`}
          >
            <div className="absolute top-0 left-0 w-1 h-full bg-[#FF9500]"></div>
            <div className="flex justify-between items-start mb-3">
              <div>
                <h2 className="text-xl font-bold font-outfit text-orange-900 dark:text-orange-100 flex items-center gap-2">
                  <span className="text-2xl">✨</span> Needs Attention Today
                </h2>
                <p className="text-orange-800/80 dark:text-orange-200/80 mt-1 text-sm font-medium">
                  {item.context}
                </p>
              </div>
              <span className={`app-badge ${badgeTone(item.priority)}`}>
                {item.priority || "High"}
              </span>
            </div>

            {item.action_type && (
              <div className="mt-4 mb-5 p-4 rounded-xl bg-white/60 dark:bg-black/40 border border-orange-200 dark:border-orange-900/50">
                <div className="text-xs uppercase tracking-wider font-semibold text-orange-800 dark:text-orange-300 mb-1">
                  Suggested Action: {item.action_type}
                </div>
                <div className="text-sm font-medium text-gray-900 dark:text-gray-100 whitespace-pre-wrap break-words">
                  {item.action_payload}
                </div>
              </div>
            )}

            <div className="flex flex-col gap-3 mt-2 w-full">
              <button
                onClick={() => onDecision(item.id, true)}
                className="w-full px-6 py-2.5 min-h-[44px] min-w-[44px] bg-[#FF9500] hover:bg-orange-600 text-white font-medium shadow-sm transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer"
                data-testid={`triage-approve-${item.id}`}
              >
                Approve & Send
              </button>
              <button
                onClick={() => onDecision(item.id, false)}
                className="w-full px-6 py-2.5 min-h-[44px] min-w-[44px] bg-white/50 dark:bg-black/30 border border-orange-200 dark:border-orange-900/30 hover:bg-white/80 dark:hover:bg-black/50 text-orange-900 dark:text-orange-100 font-medium transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer glassmorphism"
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
            className="mb-6 p-6 glassmorphism border border-white/40 dark:border-white/10 shadow-sm overflow-hidden flex flex-col gap-4"
            data-testid={`triage-card-${item.id}`}
          >
            <div className="flex justify-between items-start">
              <div className="flex items-center gap-2">
                <span className="text-xl">✨</span>
                <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                  {item.source || "Triage Action"}
                </h2>
              </div>
              <span className={`app-badge ${badgeTone(item.priority)}`}>
                {item.priority || "Normal"}
              </span>
            </div>

            <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] whitespace-pre-wrap break-words">
              {item.context}
            </div>

            {item.action_type && (
              <div className="p-4 rounded-xl bg-[#0066FF]/5 dark:bg-[#0066FF]/10 flex flex-col gap-2 border border-[#0066FF]/20 dark:border-[#0066FF]/30">
                <div className="text-xs uppercase tracking-wider font-semibold text-[#0066FF] dark:text-[#3388FF]">
                  Proposed Action: {item.action_type}
                </div>
                <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] whitespace-pre-wrap break-words">
                  {item.action_payload}
                </div>
              </div>
            )}

            <div className="flex flex-col sm:flex-row gap-3 w-full pt-2">
              <button
                onClick={() => onDecision(item.id, true)}
                className="w-full flex-1 px-6 py-2.5 min-h-[44px] min-w-[44px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-medium shadow-md transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer"
                data-testid={`triage-approve-${item.id}`}
              >
                ✨ Approve & Send
              </button>
              <button
                onClick={() => onDecision(item.id, false)}
                className="w-full flex-1 px-6 py-2.5 min-h-[44px] min-w-[44px] bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-white/10 hover:bg-white/80 dark:hover:bg-black/40 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer glassmorphism"
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
