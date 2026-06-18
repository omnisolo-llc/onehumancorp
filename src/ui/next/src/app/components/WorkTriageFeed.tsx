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


function getCardTokens(item: TriageItem) {
  const s = (item.source || "").toLowerCase();
  if (s.includes("message") || s.includes("dm") || s.includes("email") || s.includes("inquiry")) {
    return {
      icon: "✉️",
      title: item.source || "Message",
      bgClass: "bg-blue-50/50 dark:bg-blue-900/10",
      borderClass: "border-blue-400/50 dark:border-blue-500/30",
      barClass: "bg-blue-500",
      textClass: "text-blue-900 dark:text-blue-100",
      btnClass: "bg-blue-500 hover:bg-blue-600 text-white"
    };
  }
  if (s.includes("book") || s.includes("appointment") || s.includes("schedule")) {
    return {
      icon: "📅",
      title: item.source || "Booking",
      bgClass: "bg-green-50/50 dark:bg-green-900/10",
      borderClass: "border-green-400/50 dark:border-green-500/30",
      barClass: "bg-green-500",
      textClass: "text-green-900 dark:text-green-100",
      btnClass: "bg-green-500 hover:bg-green-600 text-white"
    };
  }
  if (s.includes("alert") || s.includes("proactive") || s.includes("inventory")) {
    return {
      icon: "⚠️",
      title: item.source || "Alert",
      bgClass: "bg-orange-50/50 dark:bg-orange-900/10",
      borderClass: "border-orange-400/50 dark:border-orange-500/30",
      barClass: "bg-orange-500",
      textClass: "text-orange-900 dark:text-orange-100",
      btnClass: "bg-orange-500 hover:bg-orange-600 text-white"
    };
  }
  return {
    icon: "✨",
    title: item.source || "Triage Action",
    bgClass: "bg-white/50 dark:bg-black/20",
    borderClass: "border-white/40 dark:border-white/10",
    barClass: "bg-[#0066FF]",
    textClass: "text-[#1D1D1F] dark:text-[#F5F5F7]",
    btnClass: "bg-[#0066FF] hover:bg-[#0052CC] text-white"
  };
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
          All caught up! You're a hero.
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
      {items.map((item) => {
        const tokens = getCardTokens(item);
        return (
          <div
            key={item.id}
            className={`mb-6 p-6 rounded-[16px] glassmorphism border ${tokens.borderClass} ${tokens.bgClass} shadow-lg relative overflow-hidden flex flex-col gap-4`}
            data-testid={`triage-card-${item.id}`}
          >
            <div className={`absolute top-0 left-0 w-1 h-full ${tokens.barClass}`}></div>
            <div className="flex justify-between items-start">
              <div>
                <h2 className={`text-xl font-bold font-outfit ${tokens.textClass} flex items-center gap-2`}>
                  <span className="text-2xl">{tokens.icon}</span> {tokens.title}
                </h2>
              </div>
              <span className={`app-badge ${badgeTone(item.priority)}`}>
                {item.priority || "Normal"}
              </span>
            </div>

            <div className={`text-sm font-medium ${tokens.textClass} whitespace-pre-wrap break-words opacity-80`}>
              {item.context}
            </div>

            {item.action_type && (
              <div className="p-4 rounded-[12px] bg-white/60 dark:bg-black/40 flex flex-col gap-2 border border-black/5 dark:border-white/5">
                <div className={`text-xs uppercase tracking-wider font-semibold ${tokens.textClass}`}>
                  Proposed Action: {item.action_type}
                </div>
                <div className="text-sm font-medium text-gray-900 dark:text-gray-100 whitespace-pre-wrap break-words">
                  {item.action_payload}
                </div>
              </div>
            )}

            <div className="flex flex-col sm:flex-row gap-3 w-full pt-2">
              <button
                onClick={() => onDecision(item.id, true)}
                className={`w-full flex-1 px-6 py-2.5 min-h-[44px] min-w-[44px] rounded-[16px] ${tokens.btnClass} font-medium shadow-md transition-transform active:scale-[0.98] flex items-center justify-center cursor-pointer`}
                data-testid={`triage-approve-${item.id}`}
              >
                {tokens.icon} Approve & Execute
              </button>
              <button
                onClick={() => onDecision(item.id, false)}
                className={`w-full flex-1 px-6 py-2.5 min-h-[44px] min-w-[44px] rounded-[16px] bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-white/10 hover:bg-white/80 dark:hover:bg-black/40 ${tokens.textClass} font-medium transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer`}
                data-testid={`triage-dismiss-${item.id}`}
              >
                Dismiss
              </button>
            </div>
          </div>
        );
      })}
    </div>
  );
}
