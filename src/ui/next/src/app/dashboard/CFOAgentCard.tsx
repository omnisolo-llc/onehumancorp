import React, { useEffect, useState } from 'react';

interface SafeToSpend {
  money_in: number;
  money_out: number;
  tax_safe: number;
  safe_to_spend?: number;
  tax_reserve?: number;
  upcoming_liabilities?: number;
}

const DEFAULT_CFO_DATA: SafeToSpend = {
  money_in: 12500,
  money_out: 4200,
  tax_safe: 1245,
};

export const CFOAgentCard: React.FC = () => {
  const [data, setData] = useState<SafeToSpend | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch('/api/v1/payments/ledger/safe-to-spend');
        if (response.ok) {
          const json = await response.json();
          setData(json);
        } else {
          setData(DEFAULT_CFO_DATA);
        }
      } catch (error) {
        if (error instanceof Error && (error.name === 'AbortError' || error.message.includes('Failed to fetch'))) {
          setData(DEFAULT_CFO_DATA);
          return;
        }
        console.error('Failed to fetch CFO safe to spend data:', error);
        setData(DEFAULT_CFO_DATA);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return <div className="glass-card animate-pulse h-32" />;
  }

  const cfoData = data ?? DEFAULT_CFO_DATA;
  const moneyIn = typeof cfoData.money_in === 'number' ? cfoData.money_in : DEFAULT_CFO_DATA.money_in;
  const moneyOut = typeof cfoData.money_out === 'number' ? cfoData.money_out : DEFAULT_CFO_DATA.money_out;
  const taxSafe = typeof cfoData.tax_safe === 'number' ? cfoData.tax_safe : DEFAULT_CFO_DATA.tax_safe;
  const safeToSpend = typeof cfoData.safe_to_spend === 'number' ? cfoData.safe_to_spend : Math.max(0, moneyIn - moneyOut - taxSafe);
  const taxReserve = typeof cfoData.tax_reserve === 'number' ? cfoData.tax_reserve : taxSafe;
  const upcomingBills = typeof cfoData.upcoming_liabilities === 'number' ? cfoData.upcoming_liabilities : moneyOut;

  return (
    <div className="glass-card p-6 flex flex-col gap-4">
      <div className="flex justify-between items-center">
        <div className="text-xl font-bold text-gray-800 font-outfit">Safe to Spend</div>
        <span className="text-xs font-semibold px-2 py-1 rounded bg-blue-50 text-blue-700">Profit & Tax Card</span>
      </div>
      <div className="text-3xl font-extrabold text-blue-600">${safeToSpend.toFixed(2)}</div>

      <div className="flex flex-col gap-2 mt-4 text-sm text-gray-600">
        <div className="flex justify-between items-center border-b pb-2">
          <span>Money In</span>
          <span className="font-semibold text-green-600">${moneyIn.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center border-b pb-2">
          <span>Money Out</span>
          <span className="font-semibold text-[#FF3B30]">${moneyOut.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center border-b pb-2">
          <span>Upcoming Bills</span>
          <span className="font-semibold text-[#FF3B30]">-${upcomingBills.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center border-b pb-2">
          <span className="flex items-center gap-1">
            Reserved for Taxes
            <span title="Automated tax reservation based on net income" className="text-gray-400 cursor-help font-normal">ℹ️</span>
          </span>
          <span className="font-semibold text-[#FF3B30]">-${taxReserve.toFixed(2)}</span>
        </div>
        <div className="flex justify-between items-center bg-gray-50/50 p-3 rounded-lg border border-gray-100 mt-2">
          <span className="flex items-center gap-1 font-semibold">
            Estimated Tax Safe
            <span title="Automated tax reservation based on net income" className="text-gray-400 cursor-help font-normal">ℹ️</span>
          </span>
          <span className="font-bold text-blue-600">${taxSafe.toFixed(2)}</span>
        </div>
      </div>
    </div>
  );
};
