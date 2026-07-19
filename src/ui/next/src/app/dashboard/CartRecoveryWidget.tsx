import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export function CartRecoveryWidget() {
  const [abandonedCartCount, setAbandonedCartCount] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        const res = await fetch('/api/v1/growth/campaign/abandoned-carts-count');
        if (!res.ok) throw new Error(`Cart recovery request failed (${res.status})`);
        const data = await res.json();
        if (typeof data.count !== 'number' || !Number.isFinite(data.count)) {
          throw new Error('Cart recovery response did not include a count');
        }
        setAbandonedCartCount(data.count);
      } catch {
        setError(true);
      } finally {
        setLoading(false);
      }
    };
    fetchMetrics();
  }, []);

  return (
    <div className="rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm p-6 border border-white/40 dark:border-white/10 hover:shadow-xl transition-all h-full flex flex-col justify-between group">
      <div>
        <div className="flex justify-between items-start mb-4">
          <div className="flex items-center gap-2">
            <span className="text-2xl" role="img" aria-label="shopping cart">🛒</span>
            <span className="font-bold text-gray-800 dark:text-gray-200">Cart Recovery Agent</span>
          </div>
          <span className="text-xs font-semibold px-2 py-1 rounded-full bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300">Metrics</span>
        </div>

        {loading ? (
          <div className="animate-pulse space-y-2">
            <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
            <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
          </div>
        ) : error ? (
          <p className="text-sm text-gray-600 dark:text-gray-400" role="status">
            Cart recovery data is unavailable.
          </p>
        ) : (
          <div className="space-y-3">
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Current abandoned-cart volume reported by your store.
            </p>
            <div className="flex gap-4">
              <div className="bg-white/50 dark:bg-black/20 p-3 rounded-lg flex-1">
                <p className="text-xs text-gray-500 uppercase tracking-wider mb-1 font-semibold">Abandoned carts</p>
                <p className="text-2xl font-bold text-gray-900 dark:text-white">{abandonedCartCount}</p>
              </div>
            </div>
          </div>
        )}
      </div>

      <div className="mt-6">
        <Link href="/cart-recovery" className="inline-flex items-center text-sm font-semibold text-[#0071E3] dark:text-blue-400 hover:underline group-hover:text-blue-700">
          Configure Agent <span className="ml-1 transition-transform group-hover:translate-x-1">→</span>
        </Link>
      </div>
    </div>
  );
}
