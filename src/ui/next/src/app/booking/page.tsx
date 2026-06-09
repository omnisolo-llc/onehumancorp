"use client";

import React, { useState, useEffect, Suspense } from "react";
import { useSearchParams, useRouter } from "next/navigation";

function BookingForm() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const tenant = searchParams?.get("tenant") || "default-store";

  // Hardcoded for testing, but in a real app this might come from searchParams or list of services API
  const productId = searchParams?.get("product_id") || "e2e-product-class";

  const [date, setDate] = useState<string>(new Date().toISOString().split('T')[0]);
  const [availableSlots, setAvailableSlots] = useState<{start_time: string, end_time: string}[]>([]);
  const [selectedSlot, setSelectedSlot] = useState<{start_time: string, end_time: string} | null>(null);
  const [loadingSlots, setLoadingSlots] = useState(false);
  const [reserving, setReserving] = useState(false);
  const [submitted, setSubmitted] = useState(false);

  useEffect(() => {
    fetchSlots();
  }, [date, productId, tenant]);

  const fetchSlots = async () => {
    setLoadingSlots(true);
    try {
      const res = await fetch(`/api/v1/booking/availability?tenant_id=${tenant}&product_id=${productId}&date=${date}`);
      if (res.ok) {
        const data = await res.json();
        setAvailableSlots(data.available_slots || []);
      } else {
        setAvailableSlots([]);
      }
    } catch (e) {
      console.error(e);
      setAvailableSlots([]);
    }
    setLoadingSlots(false);
  };

  const handleReserve = async () => {
    if (!selectedSlot) return;
    setReserving(true);
    try {
      const res = await fetch("/api/v1/booking/reserve", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenant,
          customer_id: "e2e-customer-ava", // Hardcoded for this specific demo UI, normally fetched from auth/session
          product_id: productId,
          start_time: selectedSlot.start_time,
          end_time: selectedSlot.end_time,
          requires_deposit: true,
        }),
      });

      if (res.ok) {
        const data = await res.json();
        if (data.deposit_stripe_link) {
           window.location.href = data.deposit_stripe_link;
        } else {
           setSubmitted(true);
        }
      } else {
        alert("Failed to reserve slot. It may have been taken.");
      }
    } catch (e) {
      console.error(e);
    }
    setReserving(false);
  };

  if (submitted) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter p-6">
        <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-2xl text-center border border-white/40">
          <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-6">✅</div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Booking Confirmed!</h1>
          <p className="text-gray-600 text-sm leading-relaxed">
            Your appointment has been successfully scheduled.
          </p>
          <button
            onClick={() => setSubmitted(false)}
            className="mt-8 w-full py-3 px-4 rounded-xl font-bold text-sm bg-gray-900 text-white hover:bg-black transition-all"
          >
            Book Another
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Book an Appointment</h1>
          <p className="text-gray-500 text-sm mt-1">Select a date and time that works for you.</p>
        </div>

        {/* Form Content */}
        <div className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">

          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">Select Date</label>
            <input
              type="date"
              required
              value={date}
              onChange={(e) => { setDate(e.target.value); setSelectedSlot(null); }}
              className="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:bg-white transition-all"
            />
          </div>

          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">Available Slots</label>
            {loadingSlots ? (
                <div className="text-sm text-gray-500 text-center py-4">Loading slots...</div>
            ) : availableSlots.length === 0 ? (
                <div className="text-sm text-gray-500 text-center py-4 border border-dashed rounded-xl border-gray-300">No slots available for this date.</div>
            ) : (
                <div className="grid grid-cols-2 gap-3">
                    {availableSlots.map((slot, i) => {
                        const timeString = new Date(slot.start_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                        const isSelected = selectedSlot?.start_time === slot.start_time;
                        return (
                            <button
                                key={i}
                                onClick={() => setSelectedSlot(slot)}
                                className={`py-3 rounded-xl text-sm font-medium transition-all ${isSelected ? 'bg-blue-600 text-white shadow-md' : 'bg-gray-50 text-gray-700 hover:bg-gray-100 border border-gray-200'}`}
                            >
                                {timeString}
                            </button>
                        );
                    })}
                </div>
            )}
          </div>

          <div className="pt-4">
             <button
              disabled={!selectedSlot || reserving}
              onClick={handleReserve}
              className={`w-full py-4 px-4 rounded-xl font-bold text-[15px] text-white shadow-md transition-all ${(!selectedSlot || reserving) ? 'bg-gray-400 cursor-not-allowed shadow-none' : 'bg-blue-600 hover:bg-blue-700 shadow-blue-500/20 active:scale-[0.98]'}`}
            >
              {reserving ? 'Reserving...' : 'Confirm & Pay Deposit'}
            </button>
          </div>
        </div>

        <div className="py-4 text-center border-t border-gray-100 bg-gray-50" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
          <a
            href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`}
            target="_blank"
            rel="noopener noreferrer"
            style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>
            ⚡ Powered by OHC
          </a>
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

export default function Booking() {
  return (
    <Suspense fallback={<div className="min-h-screen bg-gray-50 flex items-center justify-center">Loading...</div>}>
      <BookingForm />
    </Suspense>
  );
}
