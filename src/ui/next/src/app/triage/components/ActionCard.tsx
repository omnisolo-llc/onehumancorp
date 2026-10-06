export type TriageItem = {
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

export interface ActionCardProps {
  item: TriageItem;
  isSelected: boolean;
  isProcessing: boolean;
  editingId: string | null;
  editValue: string;
  getSourceIcon: (source: string) => string;
  badgeTone: (priority?: string) => string;
  onSelect: (id: string | null) => void;
  onDecision: (id: string, approved: boolean, edited_payload?: string) => void;
  onEditChange: (value: string) => void;
  onReview: () => void;
  onCancelEdit: () => void;
}

export function ActionCard({
  item,
  isSelected,
  isProcessing,
  editingId,
  editValue,
  getSourceIcon,
  badgeTone,
  onSelect,
  onDecision,
  onEditChange,
  onReview,
  onCancelEdit,
}: ActionCardProps) {
  return (
    <div
      data-testid={`triage-card-${item.id}`}
      className="omnisolo-card w-full glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] shadow-sm flex flex-col mb-4 overflow-hidden transition-all duration-300"
    >
      {/* Header Context */}
      <button
        type="button"
        aria-expanded={isSelected}
        aria-controls={`triage-details-${item.id}`}
        disabled={isProcessing}
        className="w-full text-left p-5 border-b border-[rgba(255,255,255,0.2)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] backdrop-blur-[30px] backdrop-saturate-[210%] cursor-pointer min-h-[44px]"
        onClick={() => onSelect(isSelected ? null : item.id)}
        data-testid={`triage-card-header-${item.id}`}
      >
        <div className="flex justify-between items-start mb-3">
          <div className="flex items-center gap-2">
            <span className="text-xl">{getSourceIcon(item.source || "")}</span>
            <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">
              {item.customer_id || item.source || "Unknown Source"}
            </span>
          </div>
          <span className={`app-badge ${badgeTone(item.priority)}`}>
            {item.priority || "Normal"}
          </span>
        </div>
        <div className="text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words line-clamp-2">
          {item.context || "No context provided"}
        </div>
        {!isSelected && item.action_type && (
          <div className="mt-2 text-[12px] text-[#0066FF] dark:text-[#3388FF] font-medium flex items-center gap-1">
            <span>✨</span> AI Drafted: {item.action_type} (Tap to review)
          </div>
        )}
      </button>

      {/* Slide-in / Expanded Detail View */}
      {isSelected && (
        <div
          id={`triage-details-${item.id}`}
          className="animate-in slide-in-from-top-2 duration-200 fade-in"
        >
          {item.action_type && (
            <div className="p-5 bg-[#0066FF]/10 dark:bg-[#0066FF]/20 backdrop-blur-[30px] saturate-[210%] flex flex-col gap-2">
              <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF]">
                Proposed Action: {item.action_type}
              </div>
              <div className="proposed-action border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/50 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] p-4 text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
                {item.action_payload || "No specific payload"}
              </div>
            </div>
          )}

          {/* Meta Details */}
          <div className="px-5 py-3 flex justify-between bg-white/30 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] text-[11px] text-gray-500 dark:text-gray-400">
            <span>
              {new Date(item.created_at || Date.now()).toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
              })}
            </span>
            <span>
              {new Date(item.created_at || Date.now()).toLocaleDateString()}
            </span>
          </div>

          {/* Action Buttons */}
          {editingId === item.id ? (
            <div className="p-5 flex flex-col gap-3 border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
              <textarea
                aria-label="Edit draft"
                disabled={isProcessing}
                value={editValue}
                onChange={(e) => onEditChange(e.target.value)}
                className="w-full min-h-[88px] text-[13px] text-gray-900 dark:text-white bg-white/80 dark:bg-gray-800/80 border border-gray-300 dark:border-gray-600 rounded-xl p-3 focus:outline-none focus:ring-2 focus:ring-[#0066FF] shadow-inner resize-y"
                data-testid={`triage-edit-textarea-${item.id}`}
                placeholder="Edit the draft payload..."
              />
              <div className="flex flex-col sm:flex-row gap-3 w-full pt-2">
                <button
                  onClick={() => onDecision(item.id, true, editValue)}
                  disabled={isProcessing}
                  className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                  data-testid={`triage-save-btn-${item.id}`}
                >
                  {isProcessing ? "Processing..." : "Save & Approve"}
                </button>
                <button
                  onClick={onCancelEdit}
                  disabled={isProcessing}
                  className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                  data-testid={`triage-cancel-btn-${item.id}`}
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <div className="p-5 pt-2 flex flex-col sm:flex-row gap-3 w-full border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
              {item.action_type ? (
                <>
                  <button
                    disabled={isProcessing}
                    className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                    data-testid={`triage-review-btn-${item.id}`}
                    onClick={onReview}
                  >
                    Review Draft
                  </button>
                  <button
                    disabled={isProcessing}
                    className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                    data-testid={`triage-approve-${item.id}`}
                    onClick={() => onDecision(item.id, true)}
                  >
                    {isProcessing ? "Processing..." : "Approve as-is"}
                  </button>
                </>
              ) : (
                <button
                  disabled={isProcessing}
                  className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                  data-testid={`triage-approve-${item.id}`}
                  onClick={() => onDecision(item.id, true)}
                >
                  {isProcessing ? "Processing..." : "Approve"}
                </button>
              )}
              <button
                disabled={isProcessing}
                className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                data-testid={`triage-dismiss-${item.id}`}
                onClick={() => onDecision(item.id, false)}
              >
                Dismiss
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
