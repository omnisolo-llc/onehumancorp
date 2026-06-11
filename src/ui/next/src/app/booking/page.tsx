
"use client";

import React, { useState, useEffect, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import { OneTapReferral } from "../components/OneTapReferral";

function BookingForm() {
  const searchParams = useSearchParams();
  const tenant = searchParams?.get("tenant") || "default-store";
  const [services, setServices] = useState<any[]>([]);
  const [selectedService, setSelectedService] = useState<any>(null);
  const [date, setDate] = useState<string>("");
  const [time, setTime] = useState<string>("");
  const [customerEmail, setCustomerEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("/api/v1/booking/services", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ tenant_id: tenant })
    })
    .then(res => res.json())
    .then(data => {
      setServices(data.services || []);
      setLoading(false);
    })
    .catch(() => setLoading(false));
  }, [tenant]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedService || !date || !time) return;

    const start_time = new Date(`${date}T${time}:00Z`);
    const end_time = new Date(start_time.getTime() + 60 * 60 * 1000); // 1 hr duration

    try {
      // If the service requires a deposit, create a checkout session
      const amount = selectedService.deposit_required || selectedService.price_cents || 0;
      if (amount > 0) {
        const checkoutRes = await fetch("/api/v1/booking/checkout", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            tenant_id: tenant,
            product_id: selectedService.id,
            amount_cents: amount,
            customer_id: "c1"
          })
        });
        const checkoutData = await checkoutRes.json();
        if (checkoutData.checkout_url) {
           window.location.href = checkoutData.checkout_url;
           return;
        }
      }

      // Fallback: Create unified booking directly if no deposit required or checkout fails
      const bookingRes = await fetch("/api/v1/booking/create_unified", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenant,
          service_id: selectedService.id,
          start_time: start_time.toISOString(),
          end_time: end_time.toISOString(),
          customer_id: "c1" // Temporary, real impl would map email to customer_id or create customer
        })
      });
      const bookingData = await bookingRes.json();

      if (bookingData.success) {
         setSubmitted(true);
      }
    } catch(err) {
      console.error(err);
    }
  };

  if (loading) return <div className="flex justify-center items-center h-screen">Loading...</div>;

  if (submitted) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter p-6">
        <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-2xl text-center border border-white/40">
          <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-6">✅</div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Booking Confirmed!</h1>
          <p className="text-gray-600 text-sm leading-relaxed">
            We've secured your time slot. You will receive an email confirmation shortly.
          </p>
          <button
            onClick={() => setSubmitted(false)}
            className="mt-8 w-full py-3 px-4 rounded-xl font-bold text-sm bg-gray-900 text-white hover:bg-black transition-all mb-6"
          >
            Book Another Slot
          </button>
          <OneTapReferral tenantId={tenant} source="booking_success" />
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Book a Service</h1>
          <p className="text-gray-500 text-sm mt-1">Select a service, choose a time, and secure your booking.</p>
        </div>

        <form onSubmit={handleSubmit} className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">
          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">1. Select Service</label>
            <div className="grid gap-3">
              {services.map(s => (
                <div
                  key={s.id}
                  onClick={() => setSelectedService(s)}
                  className={`p-4 border rounded-xl cursor-pointer transition-colors ${selectedService?.id === s.id ? 'border-blue-500 bg-blue-50' : 'border-gray-200 hover:border-blue-300'}`}
                >
                  <div className="font-semibold text-gray-900 text-sm">{s.title}</div>
                  <div className="text-gray-500 text-xs mt-1">{s.description || 'No description'}</div>
                  <div className="text-gray-700 text-sm mt-2 font-medium">${(s.price_cents / 100).toFixed(2)}</div>
                </div>
              ))}
            </div>
          </div>

          {selectedService && (
            <>
              <div>
                <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">2. Select Date & Time</label>
                <div className="flex gap-4">
                  <input
                    type="date"
                    required
                    value={date}
                    onChange={(e) => setDate(e.target.value)}
                    className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-blue-500 outline-none"
                  />
                  <input
                    type="time"
                    required
                    value={time}
                    onChange={(e) => setTime(e.target.value)}
                    className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-blue-500 outline-none"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">3. Your Information</label>
                <input
                  type="email"
                  required
                  placeholder="Email address"
                  value={customerEmail}
                  onChange={(e) => setCustomerEmail(e.target.value)}
                  className="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-blue-500 outline-none"
                />
              </div>

              <div className="pt-4">
                <button
                  type="submit"
                  className="w-full py-4 px-4 rounded-xl font-bold text-[15px] bg-blue-600 text-white hover:bg-blue-700 shadow-md transition-all"
                >
                  Confirm Booking & Pay Deposit
                </button>
              </div>
            </>
          )}
        </form>
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
