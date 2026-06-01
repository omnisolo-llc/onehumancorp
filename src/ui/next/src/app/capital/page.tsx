"use client";

import React, { useState, useEffect } from "react";
import Head from "next/head";

interface CapitalAdvanceOffer {
  id: string;
  amount_cents: number;
  fee_cents: number;
  total_repayment_cents: number;
  repayment_percentage: number;
  status: string;
}

export default function CapitalPage() {
  const [offer, setOffer] = useState<CapitalAdvanceOffer | null>(null);
  const [amount, setAmount] = useState<number>(1000);
  const [accepted, setAccepted] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    fetch("/api/capital/offers")
      .then((res) => {
        if (!res.ok) throw new Error("Failed to load offers");
        return res.json();
      })
      .then((data: CapitalAdvanceOffer[]) => {
        if (data.length > 0) {
          setOffer(data[0]);
          setAmount(data[0].amount_cents / 100);
        }
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  const handleAccept = async () => {
    if (!offer) return;
    try {
      setLoading(true);
      const res = await fetch("/api/capital/accept", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ advance_id: offer.id, amount_cents: amount * 100 }),
      });
      if (!res.ok) throw new Error("Failed to accept offer");
      const success = await res.json();
      if (success) {
        setAccepted(true);
      } else {
        throw new Error("Offer acceptance was not successful");
      }
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  if (loading && !offer) {
    return <div className="p-4">Loading...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">Error: {error}</div>;
  }

  if (accepted) {
    return (
      <div className="p-8 max-w-[375px] mx-auto text-center mt-12 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-[16px] shadow-xl">
        <h2 className="text-3xl font-bold text-[#1D1D1F] mb-4">🎉 Success!</h2>
        <p className="text-lg text-gray-700 mb-6">
          Funds have been instantly added to your account.
        </p>
      </div>
    );
  }

  if (!offer) {
    return (
      <div className="p-8 max-w-[375px] mx-auto text-center mt-12 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-[16px] shadow-xl">
        <h2 className="text-2xl font-bold text-[#1D1D1F] mb-4">No Offers Available</h2>
        <p className="text-gray-600">Keep growing your business. We'll let you know when you qualify for an advance.</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <Head>
        <title>Capital Advance | OHC</title>
      </Head>
      <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-[16px] shadow-lg p-6 relative overflow-hidden">
        <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-[#0066FF] to-indigo-500"></div>
        <h2 className="text-2xl font-bold text-[#1D1D1F] mb-2 font-outfit">Capital Advance</h2>
        <p className="text-sm text-gray-600 mb-6 font-inter">
          You're approved for a cash advance to grow your business.
        </p>

        <div className="mb-6">
          <label className="block text-sm font-medium text-gray-700 mb-2 font-inter">
            Select Amount
          </label>
          <input
            type="range"
            min="500"
            max="1500"
            step="100"
            value={amount}
            onChange={(e) => setAmount(Number(e.target.value))}
            className="w-full h-2 bg-gray-200 rounded-[8px] appearance-none cursor-pointer accent-[#0066FF]"
          />
          <div className="flex justify-between text-xs text-gray-500 mt-2">
            <span>$500</span>
            <span className="font-bold text-[#0066FF] text-lg">${amount}</span>
            <span>$1,500</span>
          </div>
        </div>

        <div className="bg-white/40 backdrop-blur-[10px] p-4 rounded-[8px] mb-6 border border-white/20">
          <div className="flex justify-between text-sm mb-2">
            <span className="text-gray-600 font-inter">Advance Amount</span>
            <span className="font-semibold text-[#1D1D1F]">${amount}</span>
          </div>
          <div className="flex justify-between text-sm mb-2">
            <span className="text-gray-600 font-inter">One-time Fee</span>
            <span className="font-semibold text-[#1D1D1F]">${(amount * 0.1).toFixed(0)}</span>
          </div>
          <div className="flex justify-between text-sm font-semibold border-t border-gray-200/50 pt-2 mt-2">
            <span className="text-[#1D1D1F] font-inter">Total Repayment</span>
            <span className="text-[#1D1D1F]">${(amount * 1.1).toFixed(0)}</span>
          </div>
        </div>

        <p className="text-xs text-gray-500 mb-6 font-inter leading-relaxed">
          We’ll automatically deduct <span className="font-bold text-gray-800">8%</span> of your daily sales until the total repayment is complete. No hidden fees or monthly schedules.
        </p>

        <button
          onClick={handleAccept}
          disabled={loading}
          className="w-full bg-[#0066FF] hover:bg-[#0052cc] text-white font-bold py-[14px] px-4 rounded-[8px] transition duration-200 shadow-md disabled:opacity-50 disabled:cursor-not-allowed font-outfit"
        >
          {loading ? "Processing..." : "Get Funds Instantly"}
        </button>
      </div>
    </div>
  );
}
