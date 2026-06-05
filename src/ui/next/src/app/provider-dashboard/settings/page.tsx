"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";

export default function ProviderDashboardSettings() {
  const [schedule, setSchedule] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchSchedule() {
      try {
        const response = await fetch('/api/v1/provider/schedule');
        if (response.ok) {
           const data = await response.json();
           setSchedule(data.schedule);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    }

    fetchSchedule();
  }, []);

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100 flex items-center">
          <Link href="/provider-dashboard/bookings" className="mr-4 w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center text-gray-700 hover:bg-gray-200 transition-colors">
            ←
          </Link>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Settings</h1>
            <p className="text-gray-500 text-sm mt-1">Configure your availability.</p>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">
          <div className="bg-white border border-gray-100 rounded-2xl p-5 shadow-sm">
             <h3 className="font-bold text-gray-900 mb-4">Availability Schedule</h3>

             {loading ? (
                 <div className="flex justify-center py-4"><div className="w-6 h-6 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div></div>
             ) : (
                 <div className="mb-4">
                   {schedule ? (
                       <p className="text-sm text-green-600">Schedule is active.</p>
                   ) : (
                       <p className="text-sm text-gray-500">No schedule configured.</p>
                   )}
                 </div>
             )}

             <p className="text-sm text-gray-500 mb-4">Set the days and times you are available for bookings.</p>
             <button className="w-full py-3 px-4 rounded-xl font-bold text-sm bg-gray-900 text-white hover:bg-black transition-all">
               Edit Schedule
             </button>
          </div>
        </div>
      </div>
    </div>
  );
}
