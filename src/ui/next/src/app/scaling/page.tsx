"use client";

import React, { useState, useEffect } from 'react';

export default function ScalingPage() {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setTimeout(() => setLoading(false), 500);
  }, []);

  if (loading) {
    return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <main id="scaling-screen" className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">
        <h1 className="text-2xl font-bold font-outfit">Scaling Configuration</h1>
        <div className="space-y-4">
            <div className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
              <span className="font-medium">Current Scale: 3 instances</span>
            </div>
            <div className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
              <span className="font-medium">Min 1 Max 10 instance range bounds</span>
            </div>
            <div className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm text-green-700">
              <span className="font-medium">No optimization needed.</span>
            </div>
        </div>
        <div className="flex gap-4">
           <button className="bg-blue-600 text-white px-4 py-2 rounded font-bold hover:bg-blue-700 transition">+</button>
           <button className="bg-gray-100 border border-gray-300 px-4 py-2 rounded font-bold hover:bg-gray-200 transition">-</button>
        </div>
      </main>
    </div>
  );
}
