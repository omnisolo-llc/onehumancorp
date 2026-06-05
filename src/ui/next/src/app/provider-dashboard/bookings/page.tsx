"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";

interface Booking {
    id: string;
    customerName: string;
    serviceName: string;
    startTime: string;
    status: 'pending' | 'confirmed' | 'completed' | 'cancelled';
    depositStatus: 'paid' | 'pending';
}

export default function ProviderDashboardBookings() {
  const [bookings, setBookings] = useState<Booking[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchBookings() {
      try {
        const response = await fetch('/api/v1/provider/bookings');
        if (!response.ok) {
          throw new Error('Failed to fetch bookings');
        }
        const data = await response.json();
        setBookings(data.bookings || []);
      } catch (err) {
        console.error(err);
        setError("Could not load bookings. Please try again later.");
      } finally {
        setLoading(false);
      }
    }

    fetchBookings();
  }, []);

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Upcoming Bookings</h1>
            <p className="text-gray-500 text-sm mt-1">Manage your appointments.</p>
          </div>
          <Link href="/provider-dashboard/settings" className="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center text-gray-700 hover:bg-gray-200 transition-colors">
            ⚙️
          </Link>
        </div>

        {/* Content */}
        <div className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-4 bg-gray-50/50 backdrop-blur-[20px] saturate-200">
          {loading ? (
            <div className="flex justify-center py-10"><div className="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div></div>
          ) : error ? (
             <div className="flex flex-col items-center justify-center py-20 text-center">
               <h3 className="font-semibold text-red-600">Error Loading Bookings</h3>
               <p className="text-sm text-gray-500 mt-2">{error}</p>
             </div>
          ) : bookings.length === 0 ? (
             <div className="flex flex-col items-center justify-center py-20 text-center">
               <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center text-2xl mb-4">📅</div>
               <h3 className="font-semibold text-gray-900">No upcoming bookings</h3>
               <p className="text-sm text-gray-500 mt-2">When customers book your services, they will appear here.</p>
             </div>
          ) : (
            bookings.map((booking) => (
              <div key={booking.id} className="bg-white border border-gray-100/50 rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow relative overflow-hidden group">
                <div className="absolute top-0 left-0 w-1 h-full bg-blue-500 rounded-l-2xl"></div>
                <div className="flex justify-between items-start mb-3">
                  <div>
                    <h3 className="font-bold text-gray-900">{booking.customerName}</h3>
                    <p className="text-sm text-gray-500">{booking.serviceName}</p>
                  </div>
                  <span className={`px-2.5 py-1 text-xs font-semibold rounded-full ${
                    booking.status === 'confirmed' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'
                  }`}>
                    {booking.status.charAt(0).toUpperCase() + booking.status.slice(1)}
                  </span>
                </div>

                <div className="flex items-center text-sm text-gray-600 mb-4 bg-gray-50 rounded-xl p-3">
                  <span className="mr-2">⏰</span>
                  {new Date(booking.startTime).toLocaleString([], { weekday: 'short', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })}
                </div>

                <div className="flex space-x-2">
                  <button className="flex-1 py-2.5 px-4 rounded-xl text-sm font-semibold bg-gray-900 text-white hover:bg-black transition-colors">
                    Manage
                  </button>
                  <button className="flex-1 py-2.5 px-4 rounded-xl text-sm font-semibold bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors">
                    Message
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
