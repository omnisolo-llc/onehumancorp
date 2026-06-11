"use client";

import React, { useState, useEffect, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import { OneTapReferral } from "../components/OneTapReferral";

function BookingForm() {
  const searchParams = useSearchParams();
  const tenant = searchParams?.get("tenant") || "default-store";

  const [services, setServices] = useState<any[]>([]);
  const [selectedService, setSelectedService] = useState("");
  const [selectedDate, setSelectedDate] = useState("");
  const [selectedTime, setSelectedTime] = useState("");
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [checkoutUrl, setCheckoutUrl] = useState("");

  useEffect(() => {
    // Fetch available services on mount
    fetch("/api/v1/booking/services", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ tenant_id: tenant }),
    })
      .then((res) => res.json())
      .then((data) => {
        if (data && data.services) {
          setServices(data.services);
          if (data.services.length > 0) {
            setSelectedService(data.services[0].id);
          }
        }
        setLoading(false);
      })
      .catch((e) => {
        console.error("Failed to fetch services:", e);
        setLoading(false);
      });
  }, [tenant]);

  // Generate some dummy slots based on the date
  const generateTimeSlots = () => {
    if (!selectedDate) return [];
    return ["09:00", "10:00", "11:00", "13:00", "14:00", "15:00", "16:00"];
  };

  const handleBooking = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedService || !selectedDate || !selectedTime) return;

    setSubmitting(true);
    const start_time = `${selectedDate}T${selectedTime}:00Z`;
    // For simplicity, assuming service duration is 1 hour
    const endTimeObj = new Date(new Date(start_time).getTime() + 60 * 60 * 1000);
    const end_time = endTimeObj.toISOString();

    try {
      const res = await fetch("/api/v1/booking/create_unified", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenant,
          service_id: selectedService,
          start_time,
          end_time,
          customer_id: "anonymous-customer-id", // typically from auth context
        }),
      });

      const data = await res.json();
      if (data.success && data.booking?.checkout_url) {
        setCheckoutUrl(data.booking.checkout_url);
      } else {
        alert("Failed to book slot. " + (data.error || ""));
        setSubmitting(false);
      }
    } catch (err) {
      console.error(err);
      alert("Error booking slot");
      setSubmitting(false);
    }
  };

  if (checkoutUrl) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter p-6">
        <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-2xl text-center border border-white/40">
          <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-6">🗓️</div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Slot Reserved!</h1>
          <p className="text-gray-600 text-sm leading-relaxed mb-6">
            Your time slot is reserved. Please complete the deposit payment to confirm your booking.
          </p>
          <a
            href={checkoutUrl}
            className="block w-full py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 transition-all mb-6"
          >
            Pay Deposit
          </a>

          <OneTapReferral tenantId={tenant} source="booking_deposit" />
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
          <p className="text-gray-500 text-sm mt-1">Select a service and time slot.</p>
        </div>

        {/* Form Content */}
        <form onSubmit={handleBooking} className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">
          {loading ? (
             <div className="text-sm text-gray-500 text-center py-10">Loading services...</div>
          ) : (
            <>
              <div>
                <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">Select Service</label>
                <div className="space-y-3">
                  {services.length === 0 ? (
                    <div className="text-sm text-gray-500 p-4 border border-gray-100 rounded-xl text-center">No services available.</div>
                  ) : (
                    services.map((svc) => (
                      <label key={svc.id} className={`flex items-center justify-between p-4 rounded-xl border cursor-pointer transition-all ${selectedService === svc.id ? 'border-blue-500 bg-blue-50' : 'border-gray-200 hover:border-gray-300'}`}>
                        <div>
                          <div className="font-semibold text-sm text-gray-900">{svc.title}</div>
                          <div className="text-xs text-gray-500 mt-1">${(svc.priceCents || svc.price_cents || 0) / 100} deposit</div>
                        </div>
                        <input
                          type="radio"
                          name="service"
                          value={svc.id}
                          checked={selectedService === svc.id}
                          onChange={() => setSelectedService(svc.id)}
                          className="w-4 h-4 text-blue-600 focus:ring-blue-500"
                        />
                      </label>
                    ))
                  )}
                </div>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">Select Date</label>
                <input
                  type="date"
                  required
                  value={selectedDate}
                  onChange={(e) => setSelectedDate(e.target.value)}
                  className="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:bg-white transition-all"
                />
              </div>

              {selectedDate && (
                <div>
                  <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">Select Time</label>
                  <div className="grid grid-cols-3 gap-2">
                    {generateTimeSlots().map((time) => (
                      <button
                        key={time}
                        type="button"
                        onClick={() => setSelectedTime(time)}
                        className={`py-2 text-sm font-medium rounded-lg border transition-all ${selectedTime === time ? 'border-blue-600 bg-blue-600 text-white' : 'border-gray-200 text-gray-700 hover:border-gray-300'}`}
                      >
                        {time}
                      </button>
                    ))}
                  </div>
                </div>
              )}

              <div className="pt-4">
                <button
                  type="submit"
                  disabled={!selectedService || !selectedDate || !selectedTime || submitting}
                  className="w-full py-4 px-4 rounded-xl font-bold text-[15px] bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {submitting ? 'Processing...' : 'Continue to Deposit'}
                </button>
              </div>
            </>
          )}
        </form>
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
