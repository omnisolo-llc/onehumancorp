"use client";

import React, { useState } from "react";
import { useRouter } from "next/navigation";

export function QuickCreationFab() {
  const [isOpen, setIsOpen] = useState(false);
  const [intent, setIntent] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const router = useRouter();

  const handleOpen = () => setIsOpen(true);
  const handleClose = () => {
    setIsOpen(false);
    setIntent("");
    setIsSubmitting(false);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!intent.trim()) return;

    setIsSubmitting(true);

    // Slight delay for premium feel, then route
    setTimeout(() => {
      router.push(`/products/new?intent=${encodeURIComponent(intent.trim())}`);
      handleClose(); // Need to close it when routing away just in case
    }, 400);
  };

  return (
    <>
      <button
        type="button"
        onClick={handleOpen}
        className="fixed bottom-6 right-6 z-40 w-14 h-14 bg-[#0066FF] text-white rounded-full shadow-lg hover:bg-blue-600 transition-transform hover:scale-105 flex items-center justify-center dark:bg-[#0071E3]"
        style={{ touchAction: "manipulation", minWidth: "44px", minHeight: "44px" }}
        aria-label="Create New Offering"
        data-testid="fab-create-offering"
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>

      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm animate-fade-in-up">
          <div
            className="w-full max-w-[375px] rounded-[16px] shadow-xl p-6 relative overflow-hidden"
            style={{
              background: 'rgba(255, 255, 255, 0.85)',
              backdropFilter: 'blur(30px) saturate(210%)',
              border: '1px solid rgba(255, 255, 255, 0.4)'
            }}
          >
            <button
              onClick={handleClose}
              className="absolute top-4 right-4 text-gray-500 hover:text-gray-800 transition-colors"
              aria-label="Close"
              data-testid="close-fab-modal"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>

            <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] mb-4 pr-6">What do you want to offer?</h2>

            <form onSubmit={handleSubmit} className="flex flex-col gap-4">
              <input
                type="text"
                autoFocus
                placeholder="e.g. Guitar lessons for beginners, 1 hour"
                value={intent}
                onChange={(e) => setIntent(e.target.value)}
                className="w-full bg-white/50 border border-gray-300 dark:border-gray-400 rounded-[8px] px-4 py-3 text-[#1D1D1F] font-medium focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                disabled={isSubmitting}
                data-testid="offering-intent-input"
              />

              <button
                type="submit"
                disabled={!intent.trim() || isSubmitting}
                className="w-full py-3.5 bg-[#0066FF] text-white font-bold rounded-[8px] shadow-sm hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex justify-center items-center h-[44px]"
                data-testid="submit-intent-btn"
              >
                {isSubmitting ? (
                  <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                ) : (
                  "Create with AI"
                )}
              </button>
            </form>
          </div>
        </div>
      )}
    </>
  );
}
