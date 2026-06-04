"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

export default function TeamInvitesPage() {
  const router = useRouter();

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 p-6 md:p-12">
      <h1 className="text-3xl font-bold mb-6">Referral Program</h1>
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h2 className="text-sm font-semibold text-gray-500 mb-2">Team Invites Sent</h2>
          <div className="text-4xl font-bold text-indigo-900">0</div>
        </div>
        <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h2 className="text-sm font-semibold text-gray-500 mb-2">Active Referrals</h2>
          <div className="text-4xl font-bold text-indigo-900">0</div>
        </div>
        <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h2 className="text-sm font-semibold text-gray-500 mb-2">Revenue from Referrals</h2>
          <div className="text-4xl font-bold text-indigo-900">$0.00</div>
        </div>
        <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h2 className="text-sm font-semibold text-gray-500 mb-2">Pending Rewards</h2>
          <div className="text-4xl font-bold text-indigo-900">$0.00</div>
        </div>
      </div>
      <button className="mt-8 bg-indigo-600 text-white px-6 py-3 rounded-xl font-semibold shadow-md">Invite a Business</button>
    </div>
  );
}
