"use client";

import React, { useState } from "react";
import Head from "next/head";

export default function SubscriptionsPortalPage() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleManageSubscriptions = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch("/api/billing/create-billing-portal-session", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(typeof localStorage !== "undefined" && localStorage.getItem("token")
            ? { Authorization: `Bearer ${localStorage.getItem("token")}` }
            : {}),
        },
      });

      if (!response.ok) {
        throw new Error("Failed to create billing portal session");
      }

      const data = await response.json();
      if (data.url) {
        window.location.assign(data.url);
      } else {
        throw new Error("Invalid response from server");
      }
    } catch (err: any) {
      console.error(err);
      setError(err.message || "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#F7F9FC] flex flex-col items-center py-20 px-4 font-sans text-gray-900">
      <Head>
        <title>Manage Subscriptions | OHC</title>
      </Head>

      <div className="max-w-2xl w-full">
        <h1 className="text-3xl font-extrabold mb-2 text-center text-gray-900">
          Your Subscriptions
        </h1>
        <p className="text-center text-gray-500 mb-10">
          Manage your recurring orders, pause, skip, or cancel anytime.
        </p>

        <div className="bg-white p-8 rounded-2xl shadow-sm border border-gray-100 flex flex-col items-center text-center">
          <div className="w-20 h-20 bg-indigo-50 text-indigo-600 rounded-full flex items-center justify-center mb-6">
            <svg
              className="w-10 h-10"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
          </div>

          <h2 className="text-xl font-bold mb-3">Subscription Portal</h2>
          <p className="text-gray-600 mb-8 max-w-md mx-auto">
            Access our secure portal to view your active subscriptions, update payment methods, or modify your delivery schedule.
          </p>

          {error && (
            <div className="mb-4 p-3 bg-red-50 text-red-600 text-sm rounded-lg border border-red-100">
              {error}
            </div>
          )}

          <button
            onClick={handleManageSubscriptions}
            disabled={loading}
            className={`w-full max-w-sm py-3 px-6 rounded-xl text-white font-bold text-lg shadow-md transition-all ${
              loading
                ? "bg-indigo-400 cursor-not-allowed"
                : "bg-indigo-600 hover:bg-indigo-700 hover:shadow-lg transform hover:-translate-y-0.5"
            }`}
          >
            {loading ? (
              <span className="flex items-center justify-center gap-2">
                <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Opening Portal...
              </span>
            ) : (
              "Manage Subscriptions"
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
