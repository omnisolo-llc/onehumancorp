import React, { useEffect, useState } from 'react';

interface FinancialOverview {
  total_revenue: number;
  total_expenses: number;
  tax_reserve: number;
}

export const FinancialOverviewCard: React.FC = () => {
  const [data, setData] = useState<FinancialOverview | null>(null);
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
        console.error('Failed to fetch financial overview data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return <div className="glassmorphism animate-pulse h-32 rounded-xl" />;
  }

  if (!data || typeof data.total_revenue !== 'number' || typeof data.total_expenses !== 'number' || typeof data.tax_reserve !== 'number') {
    return null;
  }

  return (
    <div className="glassmorphism p-6 flex flex-col gap-4 rounded-xl shadow-lg border border-white/40 dark:border-gray-800 mb-6">
      <div className="text-xl font-bold font-outfit text-gray-800 dark:text-gray-100">Financial Overview</div>
      <div className="flex flex-col gap-3 mt-2 text-base text-gray-700 dark:text-gray-300">
        <div className="flex justify-between items-center border-b border-gray-200 dark:border-gray-700 pb-2">
          <span className="font-semibold">Money In</span>
          <span className="font-bold text-green-600">${data.total_revenue.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center border-b border-gray-200 dark:border-gray-700 pb-2">
          <span className="font-semibold">Money Out</span>
          <span className="font-bold text-red-600">${data.total_expenses.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="font-semibold flex items-center gap-1">
            Tax to Save
            <span title="Automated 15% withholding rule" className="text-gray-400 cursor-help">ℹ️</span>
          </span>
          <span className="font-bold text-blue-600">${data.tax_reserve.toFixed(2)}</span>
        </div>
      </div>
    </div>
  );
};
