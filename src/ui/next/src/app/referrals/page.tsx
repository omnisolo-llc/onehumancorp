"use client";

import React from 'react';

export default function ReferralDashboard() {
  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden text-center p-8 justify-center">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Referral Dashboard</h1>
        <div id="referral-link" className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
          <span className="text-sm text-gray-700 truncate mr-2 font-medium">ohc://join?ref=DEFAULT</span>
        </div>
        <button className="w-full bg-blue-600 text-white font-bold p-4 active:scale-[0.98] transition-all hover:bg-blue-700 rounded-[8px]">
          Share to Instagram
        </button>
      </div>
    </div>
  );
}
