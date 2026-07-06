import React, { useEffect, useState } from 'react';

interface SafeToSpend {
  money_in: number;
  money_out: number;
  tax_safe: number;
}

export const CFOAgentCard: React.FC = () => {
  const [data, setData] = useState<SafeToSpend | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch('/api/finance/safe-to-spend');
        if (response.ok) {
          const json = await response.json();
          setData(json);
        }
      } catch (error) {
        console.error('Failed to fetch CFO safe to spend data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return <div className="glass-card animate-pulse h-32" />;
  }

  if (!data || typeof data.money_in !== 'number' || typeof data.money_out !== 'number' || typeof data.tax_safe !== 'number') {
    return null;
  }

  return (
    <div className="glass-card p-6 flex flex-col gap-4">
      <div className="text-xl font-bold text-gray-800 font-outfit">Profit & Tax Card</div>

      <div className="flex flex-col gap-2 mt-4 text-sm text-gray-600">
        <div className="flex justify-between items-center border-b pb-2">
          <span>Money In</span>
          <span className="font-semibold text-green-600">${data.money_in.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center border-b pb-2">
          <span>Money Out</span>
          <span className="font-semibold text-[#FF3B30]">${data.money_out.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center bg-gray-50/50 p-3 rounded-lg border border-gray-100 mt-2">
          <span className="flex items-center gap-1 font-semibold">
            Estimated Tax Safe
            <span title="Automated tax reservation based on net income" className="text-gray-400 cursor-help font-normal">ℹ️</span>
          </span>
          <span className="font-bold text-blue-600">${data.tax_safe.toFixed(2)}</span>
        </div>
      </div>
    </div>
  );
};
