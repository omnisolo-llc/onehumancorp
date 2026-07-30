import React, { useState, useEffect } from "react";
import { fetchAuthData } from "../utils/api";

interface MemberUsageAggregate {
  username: string;
  feature: string;
  tokens_used: number;
  computed_cost: number;
}

export function MemberAnalytics() {
  const [data, setData] = useState<MemberUsageAggregate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetchAuthData("/api/v1/ui/admin/usage");
        if (response.ok) {
          const result = await response.json();
          setData(result);
        } else if (response.status === 403) {
          setError("Admin access required to view member analytics.");
        } else {
          setError("Failed to load member analytics.");
        }
      } catch (err) {
        setError("Network error loading member analytics.");
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  if (loading) {
    return (
      <div className="glassmorphism p-6 flex items-center justify-center min-h-[200px]">
        <span className="text-[#1D1D1F] dark:text-[#F5F5F7]">Loading analytics...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="glassmorphism p-6 border border-red-500 min-h-[200px]">
        <p className="text-[#FF3B30]">{error}</p>
      </div>
    );
  }

  return (
    <div className="glassmorphism p-6 rounded-[16px]">
      <h2 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-6">
        Workspace Member Analytics
      </h2>
      <div className="overflow-x-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-gray-200 dark:border-gray-700">
              <th className="py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-300">Username</th>
              <th className="py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-300">Feature</th>
              <th className="py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-300">Tokens Used</th>
              <th className="py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-300">Computed Cost</th>
            </tr>
          </thead>
          <tbody>
            {data.length === 0 ? (
              <tr>
                <td colSpan={4} className="py-8 text-center text-gray-500">
                  No usage data recorded yet.
                </td>
              </tr>
            ) : (
              data.map((row, idx) => (
                <tr
                  key={`${row.username}-${row.feature}-${idx}`}
                  className="border-b border-gray-100 dark:border-gray-800 last:border-0 hover:bg-black/5 dark:hover:bg-white/5 transition-colors"
                >
                  <td className="py-3 px-4 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium">{row.username}</td>
                  <td className="py-3 px-4 text-gray-600 dark:text-gray-400">{row.feature}</td>
                  <td className="py-3 px-4 text-gray-600 dark:text-gray-400">{row.tokens_used.toLocaleString()}</td>
                  <td className="py-3 px-4 font-mono text-[#34C759]">
                    ${row.computed_cost.toFixed(4)}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
