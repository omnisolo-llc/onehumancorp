"use client";

import { useState } from "react";

export default function DynamicPricingConfig() {
  const [enabled, setEnabled] = useState(false);
  const [minPrice, setMinPrice] = useState(800);
  const [maxPrice, setMaxPrice] = useState(1200);
  const [itemId, setItemId] = useState("");
  const [itemType, setItemType] = useState("product");
  const [loading, setLoading] = useState(false);
  const [statusMsg, setStatusMsg] = useState("");

  const handleSave = async () => {
    if (!itemId) {
      setStatusMsg("Please enter an item ID.");
      return;
    }
    setLoading(true);
    setStatusMsg("");
    try {
      const res = await fetch("/api/v1/pricing/dynamic/configure", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          item_id: itemId,
          item_type: itemType,
          enabled,
          min_price_cents: minPrice,
          max_price_cents: maxPrice,
        }),
      });
      const data = await res.json();
      if (data.success) {
        setStatusMsg("Configuration saved successfully.");
      } else {
        setStatusMsg("Failed to save configuration.");
      }
    } catch (e) {
      console.error(e);
      setStatusMsg("An error occurred.");
    }
    setLoading(false);
  };

  return (
    <div className="p-6 max-w-lg mx-auto bg-white/30 backdrop-blur-md rounded-2xl shadow-xl mt-10 border border-white/20">
      <h1 className="text-2xl font-bold mb-4 text-gray-800">Smart Pricing Configuration</h1>
      <p className="text-gray-600 mb-6 text-sm">
        Allow our AI to autonomously adjust your prices within safe boundaries to maximize yield and clear inventory.
      </p>

      <div className="mb-4">
        <label className="block text-gray-700 font-semibold mb-2">Item ID</label>
        <input
          type="text"
          value={itemId}
          onChange={(e) => setItemId(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter product or booking ID"
        />
      </div>

      <div className="mb-6">
        <label className="block text-gray-700 font-semibold mb-2">Item Type</label>
        <select
          value={itemType}
          onChange={(e) => setItemType(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="product">Product</option>
          <option value="booking">Booking</option>
        </select>
      </div>

      <div className="mb-6 flex items-center justify-between">
        <label className="font-semibold text-gray-700">Enable Smart Pricing</label>
        <input
          type="checkbox"
          checked={enabled}
          onChange={(e) => setEnabled(e.target.checked)}
          className="w-5 h-5 text-blue-600 rounded"
        />
      </div>

      <div className="mb-4">
        <label className="block text-gray-700 font-semibold mb-2">Minimum Price (Cents)</label>
        <input
          type="range"
          min="100"
          max="2000"
          step="100"
          value={minPrice}
          onChange={(e) => setMinPrice(Number(e.target.value))}
          className="w-full"
        />
        <div className="text-right text-sm text-gray-500 mt-1">{minPrice} cents</div>
      </div>

      <div className="mb-6">
        <label className="block text-gray-700 font-semibold mb-2">Maximum Price (Cents)</label>
        <input
          type="range"
          min="100"
          max="3000"
          step="100"
          value={maxPrice}
          onChange={(e) => setMaxPrice(Number(e.target.value))}
          className="w-full"
        />
        <div className="text-right text-sm text-gray-500 mt-1">{maxPrice} cents</div>
      </div>

      {enabled && (
        <div className="mb-6 p-4 bg-blue-50 border border-blue-100 rounded-lg text-sm text-blue-800">
          <strong>Preview:</strong> Your item's price may automatically adjust down to ${minPrice / 100} during low demand, or up to ${maxPrice / 100} during high demand.
        </div>
      )}

      <button
        onClick={handleSave}
        disabled={loading}
        className="w-full bg-black text-white font-semibold py-3 rounded-xl hover:bg-gray-800 transition disabled:opacity-50"
      >
        {loading ? "Saving..." : "Save Configuration"}
      </button>

      {statusMsg && <p className="mt-4 text-center text-sm font-medium text-gray-700">{statusMsg}</p>}
    </div>
  );
}
