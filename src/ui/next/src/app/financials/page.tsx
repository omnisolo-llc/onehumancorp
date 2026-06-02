"use client";

import { useEffect, useState } from "react";

export default function FinancialsDashboard() {
  const [balance, setBalance] = useState<number | null>(null);

  useEffect(() => {
    // Simulating a fetch for the ledger balance to make the component testable via playwright.
    // In a real app this would call the gRPC or REST api that bridges to the Ledger Service.
    setTimeout(() => {
      setBalance(15000); // $150.00
    }, 500);
  }, []);

  return (
    <div className="p-8 max-w-md mx-auto">
      <h1 className="text-2xl font-bold mb-4">Financials</h1>
      <div className="bg-white/70 backdrop-blur-xl p-6 rounded-2xl shadow-sm border border-white/20">
        <h2 className="text-sm text-gray-500 uppercase font-semibold tracking-wider">Total Balance</h2>
        <div className="text-4xl font-light mt-2" data-testid="total-balance">
          {balance !== null ? `$${(balance / 100).toFixed(2)}` : "Loading..."}
        </div>
      </div>

      <div className="mt-8 space-y-4">
        <h3 className="font-semibold text-lg">Recent Activity</h3>
        <div className="p-4 bg-gray-50 rounded-xl flex justify-between items-center">
          <div>
            <div className="font-medium">Deposit</div>
            <div className="text-xs text-gray-500">Today, 2:00 PM</div>
          </div>
          <div className="text-green-600 font-medium">+$150.00</div>
        </div>
      </div>
    </div>
  );
}
