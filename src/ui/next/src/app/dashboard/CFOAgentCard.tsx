import React, { useEffect, useState } from 'react';

interface SafeToSpend {
  current_balance: number;
  tax_reserve: number;
  upcoming_liabilities: number;
  safe_to_spend: number;
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

  if (!data || typeof data.safe_to_spend !== 'number' || typeof data.current_balance !== 'number' || typeof data.tax_reserve !== 'number' || typeof data.upcoming_liabilities !== 'number') {
    return null;
  }

  return (
    <div className="glass-card p-6 flex flex-col gap-4">
      <div className="text-xl font-bold text-gray-800 font-outfit">Safe to Spend</div>
      <div className="text-4xl font-extrabold text-blue-600">${data.safe_to_spend.toFixed(2)}</div>

      <div className="flex flex-col gap-2 mt-4 text-sm text-gray-600">
        <div className="flex justify-between items-center border-b pb-2">
          <span>Current Balance</span>
          <span className="font-semibold">${data.current_balance.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center border-b pb-2">
          <span className="flex items-center gap-1">
            Reserved for Taxes
            <span title="Automated 15% withholding rule" className="text-gray-400 cursor-help">ℹ️</span>
          </span>
          <span className="font-semibold text-red-500">-${data.tax_reserve.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center">
          <span>Upcoming Bills (Next 7 Days)</span>
          <span className="font-semibold text-red-500">-${data.upcoming_liabilities.toFixed(2)}</span>
        </div>
      </div>
    </div>
  );
};
