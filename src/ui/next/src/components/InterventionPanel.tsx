"use client";

import React, { useState } from "react";
import { Info, AlertCircle, X, Send } from "lucide-react";

interface InterventionPanelProps {
  taskId: string;
  toolCallId: string;
  reason: string;
  onResolve: (input: string, resolutionType: "input" | "approve" | "abort") => Promise<void>;
  onClose: () => void;
}

export function InterventionPanel({
  taskId,
  toolCallId,
  reason,
  onResolve,
  onClose,
}: InterventionPanelProps) {
  const [input, setInput] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (type: "input" | "approve" | "abort") => {
    setIsSubmitting(true);
    setError(null);
    try {
      await onResolve(input, type);
    } catch (err: any) {
      setError(err.message || "Failed to resolve intervention");
      setIsSubmitting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="w-full max-w-lg glassmorphism rounded-[24px] border border-white/40 dark:border-white/10 shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
        {/* Header */}
        <div className="p-6 flex items-center justify-between border-b border-white/20">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center text-blue-600 dark:text-blue-400">
              <Info size={24} />
            </div>
            <div>
              <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                Human Intervention Needed
              </h2>
              <p className="text-xs text-gray-500 dark:text-gray-400 font-medium">
                Task: {taskId.slice(0, 8)}... • {toolCallId.slice(0, 8)}...
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors text-gray-500"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 overflow-y-auto flex-1 space-y-6">
          <div className="p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-100 dark:border-blue-800/50 rounded-[16px]">
            <h4 className="text-sm font-bold text-blue-800 dark:text-blue-300 mb-1 flex items-center gap-2">
              <AlertCircle size={16} />
              Reason for Pause
            </h4>
            <p className="text-sm text-blue-700 dark:text-blue-400 leading-relaxed">
              {reason}
            </p>
          </div>

          <div className="space-y-3">
            <label htmlFor="user-input" className="block text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] ml-1">
              Your Response
            </label>
            <textarea
              id="user-input"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="Provide information or instructions for the agent..."
              className="w-full min-h-[120px] p-4 rounded-[16px] border border-gray-200 dark:border-gray-700 bg-white/50 dark:bg-gray-900/50 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all resize-none text-sm"
              disabled={isSubmitting}
            />
          </div>

          {error && (
            <div className="p-3 bg-red-50 dark:bg-red-900/20 border border-red-100 dark:border-red-800/50 rounded-[12px] text-xs text-red-600 dark:text-red-400 flex items-center gap-2">
              <AlertCircle size={14} />
              {error}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-6 bg-gray-50/50 dark:bg-gray-900/50 border-t border-white/20 flex flex-col gap-3">
          <button
            onClick={() => handleSubmit("input")}
            disabled={isSubmitting || !input.trim()}
            className="w-full min-h-[48px] bg-[#0066FF] hover:bg-[#0052CC] disabled:opacity-50 disabled:cursor-not-allowed text-white font-bold rounded-[12px] flex items-center justify-center gap-2 transition-all shadow-lg shadow-blue-500/20"
          >
            {isSubmitting ? (
              <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
            ) : (
              <>
                <Send size={18} />
                Send to Agent
              </>
            )}
          </button>

          <div className="flex gap-3">
            <button
              onClick={() => handleSubmit("approve")}
              disabled={isSubmitting}
              className="flex-1 min-h-[44px] bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold rounded-[12px] hover:bg-gray-50 dark:hover:bg-gray-700 transition-all text-sm"
            >
              Quick Approve
            </button>
            <button
              onClick={() => handleSubmit("abort")}
              disabled={isSubmitting}
              className="flex-1 min-h-[44px] bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-red-600 dark:text-red-400 font-semibold rounded-[12px] hover:bg-red-50 dark:hover:bg-red-900/20 transition-all text-sm"
            >
              Abort Task
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
