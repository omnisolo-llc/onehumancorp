import React, { useState } from 'react';

interface UnlockOfferWidgetProps {
  tenantId: string;
  onUnlock: (code: string) => void;
}

export function UnlockOfferWidget({ tenantId, onUnlock }: UnlockOfferWidgetProps) {
  const [isClaiming, setIsClaiming] = useState(false);
  const [hasClaimed, setHasClaimed] = useState(false);
  const [promoCode, setPromoCode] = useState<string | null>(null);

  const handleShareAndClaim = async () => {
    setIsClaiming(true);

    const message = "Check out this amazing store I found! 🚀 #SmallBiz";
    const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(message)}`;

    // Open the share window
    window.open(shareUrl, '_blank');

    // Simulate verification
    setTimeout(() => {
      const code = "SHARED10";
      setPromoCode(code);
      setHasClaimed(true);
      setIsClaiming(false);
      onUnlock(code);
    }, 2000);
  };

  if (hasClaimed && promoCode) {
    return (
      <div className="bg-green-50 border border-green-200 rounded-xl p-4 mb-6 shadow-sm">
        <div className="flex items-center gap-3 mb-2">
          <span className="text-xl">🎉</span>
          <h3 className="font-bold text-green-900 font-outfit text-sm">
            Offer Unlocked!
          </h3>
        </div>
        <p className="text-green-800 text-sm mb-3">
          Your 10% discount has been applied. Or use code:
        </p>
        <div className="bg-white border border-green-200 rounded px-3 py-2 text-center font-bold text-green-700 tracking-widest uppercase">
          {promoCode}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-indigo-50 border border-indigo-100 rounded-xl p-4 mb-6 shadow-sm overflow-hidden relative">
      <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500"></div>

      <div className="flex items-center gap-3 mb-2 mt-1">
        <span className="text-xl">🎁</span>
        <h3 className="font-bold text-indigo-900 font-outfit text-sm">
          Want 10% off?
        </h3>
      </div>

      <p className="text-indigo-800 text-xs font-medium mb-4">
        Share this store with your friends on X (Twitter) to instantly unlock a 10% discount code!
      </p>

      <button
        onClick={handleShareAndClaim}
        disabled={isClaiming}
        className={`w-full py-2.5 bg-black hover:bg-gray-800 text-white font-semibold rounded-lg shadow transition-all flex items-center justify-center gap-2 text-sm ${isClaiming ? 'opacity-70 cursor-wait' : 'hover:-translate-y-0.5'}`}
      >
        {isClaiming ? (
          <>
            <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Verifying Share...
          </>
        ) : (
          <>
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
            Share to Unlock
          </>
        )}
      </button>
    </div>
  );
}