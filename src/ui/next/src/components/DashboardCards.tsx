import Link from 'next/link';

export const GrowthCards = () => (
  <>
    <Link href="/zero-click-builder" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
      <div className="flex items-start justify-between mb-4">
        <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⚡</div>
        <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Growth</div>
      </div>
      <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Zero-Click Builder</h3>
      <p className="text-sm text-gray-600 dark:text-gray-400">Generate a business in 30 seconds to show friends how fast OHC is.</p>
    </Link>

    <Link href="/referrals" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
      <div className="flex items-start justify-between mb-4">
        <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🤝</div>
        <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Earn $50</div>
      </div>
      <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Referrals</h3>
      <p className="text-sm text-gray-600 dark:text-gray-400">Invite other business owners to OHC and earn premium credits.</p>
    </Link>

    <Link href="/referrals" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
      <div className="flex items-start justify-between mb-4">
        <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎁</div>
        <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Referrals</div>
      </div>
      <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Referral Program</h3>
      <p className="text-sm text-gray-600 dark:text-gray-400">Invite your network and earn credits for every business that signs up.</p>
    </Link>

    <Link href="/affiliate-badge-builder" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
      <div className="flex items-start justify-between mb-4">
        <div className="w-12 h-12 rounded-full bg-orange-50 dark:bg-orange-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🏆</div>
        <div className="text-orange-600 dark:text-orange-400 font-semibold text-sm bg-orange-50 dark:bg-orange-900/30 px-3 py-1 rounded-full">Viral</div>
      </div>
      <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Affiliate Badge Builder</h3>
      <p className="text-sm text-gray-600 dark:text-gray-400">Create an embeddable badge to grow your affiliate network.</p>
    </Link>
  </>
);
