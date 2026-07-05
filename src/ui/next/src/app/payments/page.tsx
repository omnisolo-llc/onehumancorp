"use client";

import { useState, useEffect } from "react";

export default function PaymentLedger() {
  const [revenue, setRevenue] = useState(0);
  const [amount, setAmount] = useState(50);
  const [status, setStatus] = useState("idle");

  useEffect(() => {
    fetchBalance();
  }, []);

  const fetchBalance = async () => {
    try {
      const res = await fetch("/api/ledger/balance");
      if (res.ok) {
        const data = await res.json();
        setRevenue(data.total_revenue);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleRequestPayment = async () => {
    setStatus("Processing...");
    try {
      const intentRes = await fetch("/api/payments/intent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          amount: amount,
          currency: "USD",
          source: "tap_to_pay"
        })
      });

      if (!intentRes.ok) {
        setStatus("Failed to initialize");
        return;
      }

      const intentData = await intentRes.json();

      if (intentData.status === "succeeded" || intentData.client_secret) {
        setStatus("Approved");
        fetchBalance();
      } else {
        setStatus("Failed to initialize");
      }
    } catch (e) {
      console.error(e);
      setStatus("Error");
    }
  };

  return (
    <div className="max-w-[375px] mx-auto p-4 md:p-6 space-y-6">
      <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-gray-100">Dashboard</h2>

      <div className="glassmorphism p-6 flex flex-col space-y-2">
        <p className="m-0 text-gray-500 dark:text-gray-400 font-inter text-sm uppercase tracking-wide">Today's Revenue</p>
        <h1 className="m-0 text-4xl font-outfit font-bold text-gray-900 dark:text-white tracking-tight" data-testid="total-revenue">
          ${revenue.toFixed(2)}
        </h1>
      </div>

      <div className="glassmorphism p-6 space-y-4">
        <h3 className="text-lg font-semibold font-outfit text-gray-900 dark:text-gray-100 mb-2">Request Payment</h3>

        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(Number(e.target.value))}
          className="w-full px-4 py-3 min-h-[44px] rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-black/50 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-[#0066FF] font-inter"
          data-testid="payment-amount-input"
          aria-label="Payment Amount"
        />

        <button
          onClick={handleRequestPayment}
          disabled={status === "Processing..."}
          data-testid="request-payment-button"
          className={`w-full min-h-[44px] py-3 rounded-xl font-bold font-inter transition-all shadow-md active:scale-[0.98] ${
            status === "Processing..."
              ? "bg-gray-400 dark:bg-gray-600 text-white cursor-not-allowed"
              : "bg-[#0066FF] text-white hover:bg-[#0052cc]"
          }`}
        >
          {status === "Processing..." ? "Waiting for card..." : "Tap to Pay"}
        </button>

        {status !== "idle" && status !== "Processing..." && (
          <p
            data-testid="payment-status"
            className={`text-center mt-4 font-inter text-sm font-medium ${
              status === "Approved" ? "text-[#34C759] dark:text-[#00C24B]" : "text-[#FF3B30] dark:text-[#DE1B1B]"
            }`}
          >
            {status}
          </p>
        )}
      </div>
    </div>
  );
}
