"use client";

import React from 'react';
import { ReferAFriendDashboardWidget } from '../components/ReferAFriendDashboardWidget';

export default function ReferAFriendWidgetStandalonePage() {
  return (
    <div className="min-h-screen bg-[#f5f5f7] dark:bg-[#000000] p-6 flex flex-col items-center justify-center">
      <div className="w-full max-w-[400px]">
        <h1 className="text-2xl font-bold mb-6 text-center text-[#1d1d1f] dark:text-[#f5f5f7]">Growth Toolkit</h1>
        <ReferAFriendDashboardWidget />
      </div>
    </div>
  );
}
