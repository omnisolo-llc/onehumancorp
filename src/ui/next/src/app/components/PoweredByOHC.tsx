import React from 'react';

interface PoweredByOHCProps {
  tenantId: string;
}

export function PoweredByOHC({ tenantId }: PoweredByOHCProps) {
  const referralUrl = `/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}&source=footer_widget`;

  return (
    <div className="flex justify-center items-center mt-8 pb-4">
      <a
        href={referralUrl}
        className="group flex items-center gap-2 px-4 py-2 rounded-full border border-gray-200 bg-white/50 backdrop-blur-md hover:bg-white/80 hover:shadow-sm transition-all text-xs font-semibold text-gray-500 hover:text-indigo-600 uppercase tracking-widest font-outfit"
      >
        <span className="text-yellow-400 group-hover:scale-110 transition-transform">⚡</span>
        Powered by OHC
      </a>
    </div>
  );
}
