"use client";

import React, { useState, useEffect } from "react";

export default function MigrationPage() {
  const [url, setUrl] = useState("");
  const [platform, setPlatform] = useState("shopify");
  const [status, setStatus] = useState<"idle" | "loading" | "completed" | "error">("idle");
  const [metrics, setMetrics] = useState<any>(null);
  const [migrationId, setMigrationId] = useState<string | null>(null);

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (status === "loading" && migrationId) {
      interval = setInterval(async () => {
        try {
          const res = await fetch(`/api/v1/migration/${migrationId}/status`);
          if (res.ok) {
            const data = await res.json();
            if (data.status === "completed") {
              setStatus("completed");
              setMetrics(data.metrics);
              clearInterval(interval);
            } else if (data.status === "error") {
              setStatus("error");
              clearInterval(interval);
            }
          }
        } catch (e) {
          console.error("Polling error", e);
        }
      }, 2000);
    }
    return () => clearInterval(interval);
  }, [status, migrationId]);

  const handleMigrate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!url) return;

    setStatus("loading");
    try {
      const res = await fetch("/api/v1/migration", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ url, platform }),
      });
      if (res.ok) {
        const data = await res.json();
        setMigrationId(data.id);
      } else {
        setStatus("error");
      }
    } catch (err) {
      setStatus("error");
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 p-4">
      <div className="w-full max-w-[375px] bg-white/80 backdrop-blur-md rounded-2xl shadow-xl p-6 border border-gray-100">

        {status === "idle" && (
          <form onSubmit={handleMigrate} className="flex flex-col gap-4">
            <h2 className="text-xl font-semibold text-gray-800 text-center mb-2">
              Moving from another platform?
            </h2>
            <p className="text-sm text-gray-500 text-center mb-4">
              Just paste your link and our AI will do the rest.
            </p>

            <div className="flex flex-col gap-1">
              <label className="text-xs font-medium text-gray-600">Platform</label>
              <select
                value={platform}
                onChange={(e) => setPlatform(e.target.value)}
                className="w-full px-3 py-2 border border-gray-200 rounded-lg outline-none focus:ring-2 focus:ring-blue-500 bg-transparent"
              >
                <option value="shopify">Shopify</option>
                <option value="wix">Wix</option>
              </select>
            </div>

            <div className="flex flex-col gap-1">
              <label className="text-xs font-medium text-gray-600">Store URL</label>
              <input
                type="url"
                required
                placeholder="https://yourstore.myshopify.com"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                className="w-full px-3 py-2 border border-gray-200 rounded-lg outline-none focus:ring-2 focus:ring-blue-500 bg-transparent"
              />
            </div>

            <button
              type="submit"
              className="mt-4 w-full bg-black text-white font-medium py-3 rounded-xl hover:bg-gray-800 transition-colors"
            >
              Migrate
            </button>
          </form>
        )}

        {status === "loading" && (
          <div className="flex flex-col items-center justify-center py-8 gap-4">
            <div className="w-12 h-12 border-4 border-gray-200 border-t-blue-500 rounded-full animate-spin"></div>
            <h3 className="font-medium text-gray-700">Scouting your catalog...</h3>
            <p className="text-sm text-gray-500 text-center">
              Our AI agents are crawling your store and structuring your products.
            </p>
          </div>
        )}

        {status === "completed" && (
          <div className="flex flex-col items-center gap-4 py-4">
            <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center text-3xl mb-2">
              ✓
            </div>
            <h3 className="font-semibold text-lg text-gray-800">Migration Complete!</h3>
            <p className="text-sm text-gray-600 text-center bg-gray-50 p-4 rounded-xl w-full">
              We imported <strong>{metrics?.products_imported || 0}</strong> products and{" "}
              <strong>{metrics?.images_imported || 0}</strong> photos. Looks good?
            </p>
            <button className="w-full bg-blue-600 text-white font-medium py-3 rounded-xl hover:bg-blue-700 transition-colors mt-2">
              1-Tap Approve
            </button>
          </div>
        )}

        {status === "error" && (
          <div className="flex flex-col items-center gap-4 py-4 text-center">
             <div className="w-12 h-12 bg-red-100 text-red-600 rounded-full flex items-center justify-center text-2xl mb-2">
              !
            </div>
            <h3 className="font-semibold text-gray-800">Migration Failed</h3>
            <p className="text-sm text-gray-500">Something went wrong while migrating your store. Please try again.</p>
            <button
              onClick={() => setStatus("idle")}
              className="w-full bg-gray-200 text-gray-800 font-medium py-3 rounded-xl mt-4"
            >
              Try Again
            </button>
          </div>
        )}

      </div>
    </div>
  );
}
