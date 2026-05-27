import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

interface Props {
  message: string;
  onClose: () => void;
}

export const GrowthMilestonePrompt: React.FC<Props & { onUpgrade?: () => void }> = ({ message, onClose, onUpgrade }) => {
  const router = useRouter();

  return (
    <div className="fixed inset-0 z-[100] flex items-end justify-center sm:items-center sm:p-0">
      <div
        className="fixed inset-0 transition-opacity bg-black/40"
        onClick={onClose}
      />

      <div
        className="relative w-full max-w-md transform overflow-hidden rounded-t-3xl sm:rounded-2xl p-6 shadow-2xl transition-all font-inter"
        style={{
          background: 'rgba(255, 255, 255, 0.85)',
          backdropFilter: 'blur(30px) saturate(210%)',
          border: '1px solid rgba(255, 255, 255, 0.5)'
        }}
      >
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10 opacity-60"></div>

        <div className="flex flex-col items-center text-center">
          <div className="w-12 h-12 bg-indigo-100 rounded-2xl flex items-center justify-center text-2xl shadow-sm text-indigo-600 mb-4 border border-white">
            🚀
          </div>

          <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2" style={{ letterSpacing: '-0.02em' }}>
            Business Growth Milestone
          </h2>

          <p className="text-gray-700 mb-8 text-[15px] leading-relaxed px-2 font-medium">
            {message || "Your menu is growing! Upgrade to Starter for $29/mo to add up to 100 items and unlock custom domains."}
          </p>

          <div className="w-full space-y-3">
            <button
              onClick={() => {
                if (onUpgrade) {
                    onUpgrade();
                } else {
                    onClose();
                    router.push('/checkout?tier=Starter');
                }
              }}
              style={{ minHeight: '44px' }}
              className="w-full py-3 bg-indigo-600 text-white rounded-xl font-semibold shadow-sm hover:bg-indigo-700 transition-all flex items-center justify-center gap-2"
            >
              Upgrade to Starter via Stripe
            </button>

            <button
              onClick={onClose}
              style={{ minHeight: '44px' }}
              className="w-full py-3 bg-white/50 text-gray-600 rounded-xl font-medium hover:bg-gray-100 transition-all border border-gray-200/50"
            >
              Maybe Later
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
