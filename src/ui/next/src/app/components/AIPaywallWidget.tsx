import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

interface Props {
  remainingActions: number;
}

export function AIPaywallWidget({ remainingActions }: Props) {
  const router = useRouter();
  const [isOpen, setIsOpen] = useState(false);

  if (remainingActions > 10) {
    return null; // Don't show if they have plenty of actions left
  }

  const handleUpgrade = () => {
    router.push('/pricing');
  };

  const handleDismiss = () => {
    setIsOpen(false);
  };

  if (!isOpen && remainingActions <= 10) {
     return (
        <button
           onClick={() => setIsOpen(true)}
           className="fixed bottom-4 right-4 bg-amber-500 text-white p-3 rounded-full shadow-lg hover:bg-amber-600 transition-colors z-50 flex items-center gap-2"
        >
            <span className="font-bold text-sm">⚠️ {remainingActions} AI Actions Left</span>
        </button>
     );
  }

  return (
    <div className="fixed inset-0 bg-black/50 z-[100] flex items-center justify-center p-4 backdrop-blur-[30px] saturate-[210%]">
      <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl max-w-md w-full p-6 relative">
        <button
          onClick={handleDismiss}
          className="absolute top-4 right-4 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
        >
          ✕
        </button>

        <div className="text-center mb-6">
            <div className="w-16 h-16 bg-amber-100 rounded-full flex items-center justify-center mx-auto mb-4 text-amber-600 text-2xl">
                ⚡
            </div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">You're running low on AI power!</h2>
            <p className="text-gray-600 dark:text-gray-300">
                You only have <span className="font-bold text-amber-600">{remainingActions}</span> automated actions left this month.
                Upgrade to Pro for unlimited AI tasks, proactive suggestions, and priority support.
            </p>
        </div>

        <div className="flex flex-col gap-3">
            <button
                onClick={handleUpgrade}
                className="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-3 px-6 rounded-xl transition-colors shadow-md"
            >
                Upgrade to Pro
            </button>
            <button
                onClick={() => router.push('/trial-extension')}
                className="w-full bg-gray-100 hover:bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600 font-medium py-3 px-6 rounded-xl transition-colors"
            >
                Activate Pro access through OHC
            </button>
        </div>
      </div>
    </div>
  );
}
