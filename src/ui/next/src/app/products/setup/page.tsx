'use client';

import React, { useState } from 'react';
import Link from 'next/link';

export default function ProductSetupPage() {
  const [isRecurring, setIsRecurring] = useState(false);
  const [interval, setInterval] = useState('monthly');

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter pb-20">
      <div className="flex items-center justify-between mb-6 border-b border-gray-200 pb-4">
        <div className="flex items-center">
          <Link href="/dashboard" className="text-blue-500 font-semibold mr-4">&lt; Back</Link>
          <h1 className="text-xl font-bold font-outfit text-gray-900">Edit Product</h1>
        </div>
        <button id="btn-save-product" className="text-sm font-bold text-white bg-blue-600 px-3 py-1.5 rounded-lg shadow-sm">Save</button>
      </div>

      <div className="mb-6 space-y-4">
        <div>
          <label className="block text-sm font-bold text-gray-700 mb-1">Product Name</label>
          <input type="text" defaultValue="VIP Membership" className="w-full p-3 rounded-xl border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition-all" />
        </div>

        <div>
          <label className="block text-sm font-bold text-gray-700 mb-1">Price ($)</label>
          <input type="number" defaultValue="49.00" className="w-full p-3 rounded-xl border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none transition-all" />
        </div>
      </div>

      <div className="bg-white p-5 rounded-2xl border border-gray-200 shadow-sm mb-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)' }}>
        <div className="flex justify-between items-center mb-2">
          <div>
            <h2 className="font-bold text-gray-900 text-lg">Recurring Billing</h2>
            <p className="text-sm text-gray-500">Make this a subscription</p>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" id="toggle-recurring" className="sr-only peer" checked={isRecurring} onChange={(e) => setIsRecurring(e.target.checked)} />
            <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
          </label>
        </div>

        {isRecurring && (
          <div className="mt-4 pt-4 border-t border-gray-100">
            <label className="block text-sm font-bold text-gray-700 mb-2">Billing Interval</label>
            <select
              id="select-interval"
              className="w-full p-3 rounded-xl border border-gray-200 bg-gray-50 outline-none"
              value={interval}
              onChange={(e) => setInterval(e.target.value)}
            >
              <option value="weekly">Weekly</option>
              <option value="monthly">Monthly</option>
              <option value="yearly">Yearly</option>
            </select>
            <p className="text-xs text-gray-500 mt-3">
              AI Finance will handle failed payments and customer magic links automatically. No setup required.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
