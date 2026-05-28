import React from 'react';

interface Props {
  amount: number;
  fee: number;
  onAccept: () => void;
}

export default function CapitalOfferCard({ amount, fee, onAccept }: Props) {
  const total = amount + fee;

  return (
    <div className="relative overflow-hidden rounded-2xl bg-white/70 backdrop-blur-[20px] shadow-xl border border-white/20 p-6">
      {/* Shimmer effect placeholder */}
      <div className="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent -translate-x-full animate-[shimmer_2s_infinite]" />

      <div className="relative z-10">
        <h2 className="text-2xl font-bold text-gray-900 mb-2">
          Growth Boost: ${amount}
        </h2>

        <p className="text-sm text-gray-600 mb-6 leading-relaxed">
          Take ${amount} instantly to your OHC Wallet. We'll automatically keep 10% of your future sales until ${total} is repaid. No hidden fees.
        </p>

        <button
          onClick={onAccept}
          className="w-full h-[44px] bg-black hover:bg-gray-800 text-white font-medium rounded-xl transition-colors flex items-center justify-center shadow-md active:scale-[0.98]"
        >
          Get ${amount} Now
        </button>
      </div>
    </div>
  );
}
