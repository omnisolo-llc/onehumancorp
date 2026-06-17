"use client";
import React, { useState, useEffect } from 'react';

type Booking = {
  id: string;
  customerName: string;
  serviceId: string;
  startTime: string;
  endTime: string;
  status: string;
};

export default function BookingsDashboard() {
  const [bookings, setBookings] = useState<Booking[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  // In a real implementation, this would fetch from an API
  // For the purpose of the initial scaffold, we provide basic layout
  // We're focusing on the 375px mobile responsiveness
  useEffect(() => {
    // Simulate API load
    setTimeout(() => {
      setBookings([
        {
          id: 'booking-1',
          customerName: 'Alice Student',
          serviceId: 'piano_lesson_101',
          startTime: new Date().toISOString(),
          endTime: new Date(Date.now() + 3600000).toISOString(),
          status: 'confirmed',
        }
      ]);
      setIsLoading(false);
    }, 500);
  }, []);

  return (
    <div className="p-4 sm:p-6 lg:p-8 min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-2xl font-bold text-gray-900 mb-6 font-outfit">Appointments & Bookings</h1>

        {isLoading ? (
          <div className="text-center text-gray-500">Loading bookings...</div>
        ) : bookings.length === 0 ? (
          <div className="text-center text-gray-500 bg-white p-8 rounded-xl border border-gray-200">
            No upcoming bookings.
          </div>
        ) : (
          <div className="space-y-4">
            {bookings.map((booking) => (
              <div key={booking.id} className="bg-white rounded-xl shadow-sm border border-gray-200 p-4 sm:p-6 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-lg font-semibold text-gray-900">{booking.customerName}</h3>
                  <p className="text-sm text-gray-600">Service: {booking.serviceId}</p>
                  <p className="text-sm text-gray-600 mt-1">
                    {new Date(booking.startTime).toLocaleDateString()} at {new Date(booking.startTime).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                  </p>
                </div>
                <div className="flex flex-row sm:flex-col items-center sm:items-end justify-between sm:justify-center w-full sm:w-auto gap-4 sm:gap-2">
                  <span className={`px-3 py-1 text-xs font-medium rounded-full ${
                    booking.status === 'confirmed' ? 'bg-green-100 text-green-800' :
                    booking.status === 'pending' ? 'bg-yellow-100 text-yellow-800' :
                    'bg-gray-100 text-gray-800'
                  }`}>
                    {booking.status.toUpperCase()}
                  </span>
                  <button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
                    View Details
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
