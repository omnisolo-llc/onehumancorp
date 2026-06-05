"use client";

import React, { useState, useEffect } from "react";

interface Booking {
  booking_id: string;
  customer_name: string;
  service_name: string;
  start_time: string;
  end_time: string;
  status: string;
}

export default function CalendarDashboard() {
  const [bookings, setBookings] = useState<Booking[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchBookings = async () => {
      try {
        const res = await fetch("/api/v1/booking/upcoming");
        const data = await res.json();
        if (data.bookings) {
          setBookings(data.bookings);
        }
      } catch (err) {
        console.error("Failed to fetch bookings", err);
      } finally {
        setLoading(false);
      }
    };
    fetchBookings();
  }, []);

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">
              Schedule
            </h1>
            <p className="text-gray-500 text-sm mt-1">Upcoming bookings</p>
          </div>
          <button className="bg-gray-100 p-2 rounded-full hover:bg-gray-200 transition-colors">
            <svg
              className="w-5 h-5 text-gray-700"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 4v16m8-8H4"
              />
            </svg>
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-4 bg-gray-50/50">
          {loading ? (
            <div className="flex justify-center py-10">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
            </div>
          ) : bookings.length === 0 ? (
            <div className="text-center text-gray-500 py-10">
              <p>No upcoming bookings.</p>
            </div>
          ) : (
            bookings.map((b) => {
              const startDate = new Date(b.start_time);
              const endDate = new Date(b.end_time);
              const timeString = `${startDate.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })} - ${endDate.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}`;
              const dateString = startDate.toLocaleDateString(undefined, {
                weekday: "short",
                month: "short",
                day: "numeric",
              });

              return (
                <div
                  key={b.booking_id}
                  className="bg-white/65 backdrop-blur-[20px] saturate-200 border border-white/40 p-5 rounded-2xl shadow-sm hover:shadow-md transition-shadow"
                >
                  <div className="flex justify-between items-start mb-3">
                    <div>
                      <h3 className="font-semibold text-gray-900 font-outfit">
                        {b.customer_name}
                      </h3>
                      <p className="text-sm text-gray-500">{b.service_name}</p>
                    </div>
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium capitalize bg-blue-100 text-blue-800">
                      {b.status}
                    </span>
                  </div>
                  <div className="flex items-center text-sm text-gray-600 bg-gray-50/50 p-2.5 rounded-xl">
                    <svg
                      className="w-4 h-4 mr-2 text-gray-400"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                      />
                    </svg>
                    <span className="font-medium mr-2">{dateString}</span>
                    <span className="text-gray-400">|</span>
                    <span className="ml-2">{timeString}</span>
                  </div>
                </div>
              );
            })
          )}
        </div>
      </div>
      <style
        dangerouslySetInnerHTML={{
          __html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `,
        }}
      />
    </div>
  );
}
