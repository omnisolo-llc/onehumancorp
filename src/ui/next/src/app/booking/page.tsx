"use client";
import React, { useState, useEffect, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import { OneTapReferral } from "../components/OneTapReferral";

function BookingForm() {
  const searchParams = useSearchParams();
  const tenant = searchParams?.get("tenant") || "default-store";
  const serviceId = searchParams?.get("service_id") || "service-1";

  const [description, setDescription] = useState("");
  const [selectedDate, setSelectedDate] = useState("");
  const [selectedSlot, setSelectedSlot] = useState("");
  const [customerName, setCustomerName] = useState("");
  const [customerEmail, setCustomerEmail] = useState("");
  const [availableSlots, setAvailableSlots] = useState<{start_time: string, end_time: string}[]>([]);
  const [isLoadingSlots, setIsLoadingSlots] = useState(false);

  const [submitted, setSubmitted] = useState(false);
  const [checkoutUrl, setCheckoutUrl] = useState("");

  useEffect(() => {
    if (!selectedDate) {
      setAvailableSlots([]);
      return;
    }

    async function fetchSlots() {
      setIsLoadingSlots(true);
      try {
        const res = await fetch("/api/v1/booking/engine/availability", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            tenant_id: tenant,
            product_id: serviceId,
            date: selectedDate
          })
        });
        const data = await res.json();
        if (data.available_slots) {
          setAvailableSlots(data.available_slots);
        } else {
          setAvailableSlots([]);
        }
      } catch (err) {
        console.error("Failed to fetch slots", err);
        setAvailableSlots([]);
      } finally {
        setIsLoadingSlots(false);
      }
    }

    fetchSlots();
  }, [selectedDate, tenant, serviceId]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedSlot) {
      alert("Please select a time slot.");
      return;
    }

    const slot = availableSlots.find(s => s.start_time === selectedSlot);
    if (!slot) return;

    try {
      const res = await fetch("/api/v1/booking/engine/reserve", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenant,
          customer_id: customerEmail || "guest", // Use guest fallback for e2e
          product_id: serviceId,
          start_time: slot.start_time,
          end_time: slot.end_time,
          requires_deposit: true, // Demo always requires deposit
          timezone: "UTC"
        })
      });

      const data = await res.json();
      if (data.deposit_stripe_link) {
        setCheckoutUrl(data.deposit_stripe_link);
      }
      setSubmitted(true);
    } catch (err) {
      console.error(err);
      alert("Failed to create booking request.");
    }
  };

  if (submitted) {
    if (checkoutUrl) {
      return (
        <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4 font-inter">
          <div className="bg-white rounded-2xl shadow-xl max-w-md w-full p-8 text-center" data-testid="booking-checkout-container">
            <div className="w-16 h-16 bg-blue-100 text-[#0071E3] rounded-full flex items-center justify-center mx-auto mb-6">
              <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
              </svg>
            </div>
            <h2 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Almost there!</h2>
            <p className="text-gray-600 mb-8">Please complete your deposit to secure your time slot.</p>
            <a href={checkoutUrl} className="w-full inline-block bg-[#0071E3] text-white font-semibold py-3 px-6 rounded-xl hover:bg-blue-700 transition-colors" data-testid="pay-deposit-btn">
              Pay Deposit
            </a>
          </div>
        </div>
      );
    }

    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4 font-inter">
        <div className="bg-white rounded-2xl shadow-xl max-w-md w-full p-8 text-center" data-testid="booking-success-container">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
            <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Request Sent!</h2>
          <p className="text-gray-600 mb-8">We've received your request. We're currently generating a quote for you and will send it over shortly for your review!</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4 font-inter">
      <div className="bg-white rounded-2xl shadow-xl max-w-lg w-full overflow-hidden">
        <div className="bg-[#0071E3] px-8 py-10 text-white text-center">
          <h1 className="text-3xl font-bold font-outfit tracking-tight mb-2">Book an Appointment</h1>
          <p className="text-blue-100 font-medium">Select a time that works for you.</p>
        </div>

        <form onSubmit={handleSubmit} className="p-8 space-y-6">
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Name</label>
                <input
                  type="text"
                  required
                  value={customerName}
                  onChange={(e) => setCustomerName(e.target.value)}
                  className="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors"
                  placeholder="Jane Doe"
                />
              </div>
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Email</label>
                <input
                  type="email"
                  required
                  value={customerEmail}
                  onChange={(e) => setCustomerEmail(e.target.value)}
                  className="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors"
                  placeholder="jane@example.com"
                />
              </div>
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-1">Select Date</label>
              <input
                type="date"
                required
                value={selectedDate}
                onChange={(e) => setSelectedDate(e.target.value)}
                className="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors"
              />
            </div>

            {selectedDate && (
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Available Times</label>
                {isLoadingSlots ? (
                  <div className="text-sm text-gray-500 text-center py-4">Loading slots...</div>
                ) : availableSlots.length > 0 ? (
                  <div className="grid grid-cols-3 gap-2">
                    {availableSlots.map((slot) => {
                      const d = new Date(slot.start_time);
                      const timeString = d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                      return (
                        <button
                          key={slot.start_time}
                          type="button"
                          onClick={() => setSelectedSlot(slot.start_time)}
                          className={`py-2 text-sm font-medium rounded-lg border transition-all ${
                            selectedSlot === slot.start_time
                              ? 'bg-[#0071E3] border-[#0071E3] text-white shadow-md transform scale-[1.02]'
                              : 'bg-white border-gray-200 text-gray-700 hover:border-blue-300 hover:bg-blue-50'
                          }`}
                        >
                          {timeString}
                        </button>
                      );
                    })}
                  </div>
                ) : (
                  <div className="text-sm text-gray-500 bg-gray-50 p-4 rounded-xl text-center border border-gray-100">
                    No slots available for this date.
                  </div>
                )}
              </div>
            )}

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-1">Job Details</label>
              <textarea
                required
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                rows={3}
                className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors resize-none"
                placeholder="What do you need help with?"
              />
            </div>
          </div>

          <button
            type="submit"
            className="w-full bg-gray-900 hover:bg-black text-white font-semibold py-3.5 px-6 rounded-xl transition-all transform active:scale-[0.98] shadow-lg flex items-center justify-center gap-2"
          >
            <span>Confirm Booking</span>
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
            </svg>
          </button>
        </form>

        <div className="bg-gray-50 p-6 border-t border-gray-100 text-center">
          <OneTapReferral
            tenantId={tenant}
            source="booking_footer"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '13px',
              fontWeight: 600,
              color: '#6b7280',
              textDecoration: 'none',
              transition: 'color 0.2s',
              padding: '6px 12px',
              backgroundColor: 'white',
              borderRadius: '20px',
              border: '1px solid #e5e7eb',
              boxShadow: '0 1px 2px rgba(0,0,0,0.05)'
            }}
          />
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


export default function Booking() {
  return (
    <Suspense fallback={<div className="min-h-screen bg-gray-50 flex items-center justify-center">Loading...</div>}>
      <BookingForm />
    </Suspense>
  );
}
