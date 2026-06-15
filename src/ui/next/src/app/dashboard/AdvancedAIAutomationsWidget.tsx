"use client";

import React, { useState } from 'react';

export function AdvancedAIAutomationsWidget() {
  const [isEnabled, setIsEnabled] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [showUnlocked, setShowUnlocked] = useState(false);

  const handleEnableClick = () => {
    setShowModal(true);
  };

  const handleShareClick = () => {
    window.open('https://twitter.com/intent/tweet?text=Check+out+OHC', '_blank');
    setIsVerifying(true);

    // Simulate verification delay
    setTimeout(() => {
      setIsVerifying(false);
      setShowUnlocked(true);

      // Simulate success delay before closing modal
      setTimeout(() => {
        setShowModal(false);
        setIsEnabled(true);
      }, 1500);
    }, 2000);
  };

  return (
    <div className="mb-6 ohc-growth-card glassmorphism p-6 rounded-[16px] border border-gray-200 dark:border-gray-800 bg-white dark:bg-black shadow-sm">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white">Advanced AI Automations</h2>
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
            Let AI handle your routine customer interactions and data entry.
          </p>
        </div>
        <div>
          {isEnabled ? (
            <span className="text-green-600 dark:text-green-400 font-medium flex items-center gap-1">
              ✅ Enabled
            </span>
          ) : (
            <button
              onClick={handleEnableClick}
              className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg transition-colors"
            >
              Enable
            </button>
          )}
        </div>
      </div>

      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
          <div className="bg-white dark:bg-gray-900 rounded-2xl p-6 max-w-md w-full shadow-xl border border-gray-200 dark:border-gray-800">
            <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
              Unlock Advanced Features
            </h2>

            {showUnlocked ? (
              <div className="py-8 text-center">
                <div className="text-4xl mb-4">🎉</div>
                <p className="text-lg font-bold text-green-600 dark:text-green-400">Unlocked!</p>
              </div>
            ) : (
              <>
                <p className="text-gray-600 dark:text-gray-300 mb-6">
                  Advanced AI Automations are available on the Pro plan. Upgrade today, or share with your network to unlock this feature for free!
                </p>

                <div className="flex flex-col gap-3">
                  <button
                    className="w-full py-3 bg-gray-900 dark:bg-white text-white dark:text-gray-900 font-semibold rounded-xl hover:bg-black dark:hover:bg-gray-100 transition-colors"
                  >
                    Upgrade to Pro
                  </button>

                  <div className="text-center text-sm text-gray-500 font-medium">or</div>

                  <button
                    onClick={handleShareClick}
                    disabled={isVerifying}
                    className="w-full py-3 bg-[#1DA1F2] hover:bg-[#1a8cd8] text-white font-semibold rounded-xl transition-colors disabled:opacity-70 flex items-center justify-center gap-2"
                  >
                    {isVerifying ? (
                      'Verifying Share...'
                    ) : (
                      'Share on X to Unlock'
                    )}
                  </button>
                </div>
              </>
            )}

            {!showUnlocked && !isVerifying && (
              <button
                onClick={() => setShowModal(false)}
                className="mt-4 w-full py-2 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 text-sm font-medium"
              >
                Cancel
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
