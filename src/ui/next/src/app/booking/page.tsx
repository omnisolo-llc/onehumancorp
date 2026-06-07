"use client";

import React, { useState, useEffect } from "react";

export default function Booking() {
  const [selectedDate, setSelectedDate] = useState<string>("");
  const [slots, setSlots] = useState<{ start_time: string; end_time: string }[]>([]);
  const [selectedSlot, setSelectedSlot] = useState<{ start_time: string; end_time: string } | null>(null);
  const [loading, setLoading] = useState(false);
  const [bookingResult, setBookingResult] = useState<{ booking_id: string; deposit_stripe_link: string } | null>(null);
  const [error, setError] = useState("");

  const tenantId = "e2e-tenant";
  const productId = "e2e-product-class";
  const resourceId = "e2e-resource-leo";
  const customerId = "e2e-customer-new";

  useEffect(() => {
    if (!selectedDate) {
      setSlots([]);
      setSelectedSlot(null);
      return;
    }

    setLoading(true);
    setError("");
    fetch(`/api/ui/bookings/availability?tenant_id=${tenantId}&product_id=${productId}&date=${selectedDate}`)
      .then(res => res.json())
      .then(data => {
        if (data.available_slots) {
          setSlots(data.available_slots);
        } else {
          setSlots([]);
        }
      })
      .catch(err => setError(err.message))
      .finally(() => setLoading(false));
  }, [selectedDate]);

  const handleBook = async () => {
    if (!selectedSlot) return;

    setLoading(true);
    setError("");

    try {
      const res = await fetch("/api/ui/bookings/reserve", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenantId,
          customer_id: customerId,
          product_id: productId,
          start_time: selectedSlot.start_time,
          end_time: selectedSlot.end_time,
          resource_id: resourceId,
        }),
      });

      if (!res.ok) {
        throw new Error(await res.text());
      }

      const data = await res.json();
      setBookingResult(data);
    } catch (err: any) {
      setError(err.message || "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  if (bookingResult) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter p-6">
        <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-2xl text-center border border-white/40">
          <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-6">✅</div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Slot Reserved!</h1>
          <p className="text-gray-600 text-sm leading-relaxed mb-6">
            Your appointment has been reserved. Please pay the deposit to confirm.
          </p>
          <a
            href={bookingResult.deposit_stripe_link}
            target="_blank"
            rel="noreferrer"
            className="w-full block py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 transition-all"
          >
            Pay Deposit
          </a>
          <button
            onClick={() => {
              setBookingResult(null);
              setSelectedSlot(null);
              setSelectedDate("");
            }}
            className="mt-4 w-full py-3 px-4 rounded-xl font-bold text-sm bg-gray-100 text-gray-900 hover:bg-gray-200 transition-all"
          >
            Book Another Slot
          </button>
        </div>
      </div>
    );
  }

  // Basic calendar dates for next 7 days
  const today = new Date();
  const dates = Array.from({ length: 7 }).map((_, i) => {
    const d = new Date(today);
    d.setDate(today.getDate() + i);
    return d.toISOString().split("T")[0];
  });

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Book an Appointment</h1>
          <p className="text-gray-500 text-sm mt-1">Select a date and time for your session.</p>
        </div>

        <div className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">
          {error && <div className="text-red-500 text-sm p-3 bg-red-50 rounded-lg">{error}</div>}

          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-4 uppercase tracking-wider text-[10px]">Select a Date</label>
            <div className="grid grid-cols-4 gap-2">
              {dates.map(date => {
                const isSelected = date === selectedDate;
                const dateObj = new Date(date);
                const dayName = dateObj.toLocaleDateString('en-US', { weekday: 'short' });
                const dayNum = dateObj.getDate();
                return (
                  <button
                    key={date}
                    onClick={() => setSelectedDate(date)}
                    className={`flex flex-col items-center justify-center p-2 rounded-xl border ${isSelected ? 'border-blue-600 bg-blue-50 text-blue-700' : 'border-gray-200 hover:bg-gray-50'}`}
                  >
                    <span className="text-[10px] font-bold uppercase">{dayName}</span>
                    <span className="text-lg font-outfit font-semibold">{dayNum}</span>
                  </button>
                );
              })}
            </div>
          </div>

          {selectedDate && (
            <div>
              <label className="block text-sm font-semibold text-gray-900 mb-4 uppercase tracking-wider text-[10px]">Available Slots</label>
              {loading && <div className="text-sm text-gray-500 text-center py-4">Checking availability...</div>}
              {!loading && slots.length === 0 && <div className="text-sm text-gray-500 text-center py-4">No slots available on this date.</div>}
              {!loading && slots.length > 0 && (
                <div className="grid grid-cols-2 gap-3">
                  {slots.map(slot => {
                    const isSelected = selectedSlot === slot;
                    const timeString = new Date(slot.start_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                    return (
                      <button
                        key={slot.start_time}
                        onClick={() => setSelectedSlot(slot)}
                        className={`py-3 px-4 rounded-xl border text-sm font-medium transition-all ${isSelected ? 'bg-blue-600 text-white border-blue-600 shadow-md' : 'bg-white border-gray-200 text-gray-700 hover:border-gray-300 hover:bg-gray-50'}`}
                      >
                        {timeString}
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          )}
        </div>

        {selectedSlot && (
          <div className="p-6 bg-white border-t border-gray-100">
            <button
              onClick={handleBook}
              disabled={loading}
              className={`w-full py-4 px-4 rounded-xl font-bold text-[15px] text-white shadow-md transition-all ${loading ? 'bg-blue-400 cursor-not-allowed' : 'bg-blue-600 hover:bg-blue-700 active:scale-[0.98]'}`}
            >
              {loading ? 'Booking...' : 'Confirm & Pay Deposit'}
            </button>
          </div>
        )}
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
