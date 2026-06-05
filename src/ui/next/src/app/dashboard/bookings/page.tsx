"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
function getTenantId() { return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default"; }

export default function BookingDashboard() {
  const [bookings, setBookings] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchBookings() {
      const tenant = getTenantId();
      try {
        const res = await fetch(`/api/ui/bookings?tenant_id=${tenant}`);
        if (res.ok) {
          const data = await res.json();
          setBookings(data);
        }
      } catch (e) {
        console.error("Failed to fetch bookings", e);
      } finally {
        setLoading(false);
      }
    }
    fetchBookings();
  }, []);

  const formatDate = (dateString: string) => {
    try {
      const date = new Date(dateString);
      return new Intl.DateTimeFormat('en-US', {
        month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit'
      }).format(date);
    } catch {
      return dateString;
    }
  };

  const statusTone = (status: string) => {
    switch (status.toLowerCase()) {
      case "confirmed": return "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400 border-green-200";
      case "pending": return "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400 border-yellow-200";
      case "completed": return "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300 border-gray-200";
      case "cancelled": return "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400 border-red-200";
      default: return "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400 border-blue-200";
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-950 font-inter p-4 md:p-8">
      <div className="max-w-[1024px] mx-auto space-y-6">

        {/* Header */}
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-8">
          <div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Provider Dashboard</h1>
            <p className="text-gray-500 dark:text-gray-400 text-sm mt-1">Manage your upcoming bookings, schedule, and client requests.</p>
          </div>
          <div className="flex items-center gap-3">
            <Link href="/calendar" className="px-4 py-2 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
              View Calendar
            </Link>
            <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl text-sm font-medium transition-colors shadow-sm">
              + New Booking
            </button>
          </div>
        </div>

        {/* Stats Row */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px] p-6 shadow-sm">
            <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-1">Pending Requests</div>
            <div className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">
              {bookings.filter(b => b.status === 'pending').length}
            </div>
          </div>
          <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px] p-6 shadow-sm">
            <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-1">Confirmed Upcoming</div>
            <div className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">
              {bookings.filter(b => b.status === 'confirmed').length}
            </div>
          </div>
          <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px] p-6 shadow-sm">
            <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-1">Today's Bookings</div>
            <div className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">
              {bookings.filter(b => b.start_time && new Date(b.start_time).toDateString() === new Date().toDateString()).length}
            </div>
          </div>
        </div>

        {/* Upcoming Bookings List */}
        <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[16px] shadow-sm overflow-hidden">
          <div className="px-6 py-5 border-b border-gray-100 dark:border-gray-800/50 flex justify-between items-center bg-white/40 dark:bg-transparent">
            <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white">Upcoming Bookings</h2>
          </div>

          <div className="divide-y divide-gray-100 dark:divide-gray-800/50">
            {loading ? (
              <div className="p-8 text-center text-gray-500 text-sm">Loading bookings from database...</div>
            ) : bookings.length === 0 ? (
              <div className="p-12 flex flex-col items-center justify-center text-center">
                <div className="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center text-2xl mb-4">📅</div>
                <h3 className="text-gray-900 dark:text-white font-medium mb-1">No upcoming bookings</h3>
                <p className="text-gray-500 dark:text-gray-400 text-sm max-w-[250px]">When customers book your services, they will appear here.</p>
              </div>
            ) : (
              bookings.map((booking) => (
                <div key={booking.id} className="p-4 md:p-6 hover:bg-white/80 dark:hover:bg-gray-800/40 transition-colors flex flex-col md:flex-row md:items-center justify-between gap-4">
                  <div className="flex items-start gap-4">
                    <div className="hidden md:flex w-12 h-12 bg-blue-50 dark:bg-blue-900/20 rounded-full items-center justify-center text-blue-600 dark:text-blue-400 text-xl font-bold font-outfit shrink-0">
                      {(booking.customer_name || "?").charAt(0).toUpperCase()}
                    </div>
                    <div>
                      <div className="flex items-center gap-2 mb-1">
                        <h3 className="font-semibold text-gray-900 dark:text-white text-[15px]">
                          {booking.customer_name || "Unknown Customer"}
                        </h3>
                        <span className={`text-[10px] uppercase tracking-wider font-bold px-2 py-0.5 rounded-full border ${statusTone(booking.status || 'pending')}`}>
                          {booking.status || 'Pending'}
                        </span>
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400 flex flex-col md:flex-row md:items-center gap-1 md:gap-3">
                        <span className="flex items-center gap-1">
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                          {booking.start_time ? formatDate(booking.start_time) : "TBD"}
                        </span>
                        <span className="hidden md:inline text-gray-300 dark:text-gray-600">•</span>
                        <span className="flex items-center gap-1">
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                          Product ID: {booking.product_id?.slice(0,8) || "Custom"}
                        </span>
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-2 w-full md:w-auto mt-2 md:mt-0">
                    <button className="flex-1 md:flex-none px-4 py-2 bg-white dark:bg-[#16161a] border border-gray-200 dark:border-gray-700 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
                      Manage
                    </button>
                    {booking.status === 'pending' && (
                      <button className="flex-1 md:flex-none px-4 py-2 bg-gray-900 dark:bg-white text-white dark:text-gray-900 rounded-lg text-sm font-medium hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors">
                        Approve
                      </button>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
