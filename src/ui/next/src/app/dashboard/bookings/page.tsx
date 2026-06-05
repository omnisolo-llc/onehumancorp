"use client";

import React, { useState, useEffect } from "react";

interface Booking {
  id: string;
  customerName: string;
  serviceName: string;
  startTime: string;
  status: string;
}

export default function ProviderBookingsDashboard() {
  const [bookings, setBookings] = useState<Booking[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchBookings() {
      try {
        const response = await fetch('/api/v1/bookings');
        if (response.ok) {
          const data = await response.json();
          setBookings(data.bookings || []);
        } else {
          setBookings([]);
        }
      } catch (e) {
        setBookings([]);
      } finally {
        setLoading(false);
      }
    }
    fetchBookings();
  }, []);

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter p-4 items-center">
      <div className="w-full max-w-[375px] pt-6">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6 px-2">Upcoming Bookings</h1>

        {loading ? (
          <div className="flex justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
          </div>
        ) : bookings.length === 0 ? (
          <div className="bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-sm border border-white/40 text-center">
            <div className="text-4xl mb-4">📅</div>
            <h2 className="text-xl font-bold text-gray-900 mb-2">No upcoming bookings</h2>
            <p className="text-gray-500 text-sm">Your schedule is clear for now.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {bookings.map((booking) => (
              <div
                key={booking.id}
                className="bg-white/65 backdrop-blur-[30px] rounded-[24px] p-6 shadow-md border border-white/40 flex flex-col"
              >
                <div className="flex justify-between items-start mb-4">
                  <div>
                    <h3 className="font-bold text-lg text-gray-900">{booking.serviceName}</h3>
                    <p className="text-gray-600 text-sm">{booking.customerName}</p>
                  </div>
                  <span className={`px-3 py-1 rounded-full text-xs font-semibold ${
                    booking.status === 'scheduled' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
                  }`}>
                    {booking.status.charAt(0).toUpperCase() + booking.status.slice(1)}
                  </span>
                </div>

                <div className="flex items-center text-gray-600 text-sm bg-gray-100/50 rounded-xl p-3">
                  <span className="mr-2">⏰</span>
                  {new Date(booking.startTime).toLocaleDateString([], { weekday: 'short', month: 'short', day: 'numeric' })}
                  {' • '}
                  {new Date(booking.startTime).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                </div>

                <div className="mt-4 flex space-x-3">
                  <button className="flex-1 bg-white border border-gray-200 text-gray-700 py-2 rounded-xl text-sm font-semibold hover:bg-gray-50 transition-colors">
                    Reschedule
                  </button>
                  <button className="flex-1 bg-gray-900 text-white py-2 rounded-xl text-sm font-semibold hover:bg-black transition-colors">
                    Details
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
