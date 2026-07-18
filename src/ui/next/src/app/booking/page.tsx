"use client";
import React, { useState, useEffect, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import { OneTapReferral } from "../components/OneTapReferral";

type BookingSlot = { start_time: string; end_time: string };
const SAFE_ID = /^[A-Za-z0-9._-]{1,128}$/;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseSlots(value: unknown): BookingSlot[] | null {
  if (!isRecord(value) || !Array.isArray(value.available_slots)) return null;
  const slots: BookingSlot[] = [];
  for (const candidate of value.available_slots) {
    if (!isRecord(candidate) || typeof candidate.start_time !== "string" || typeof candidate.end_time !== "string") return null;
    const start = Date.parse(candidate.start_time);
    const end = Date.parse(candidate.end_time);
    if (!Number.isFinite(start) || !Number.isFinite(end) || end <= start) return null;
    slots.push({ start_time: candidate.start_time, end_time: candidate.end_time });
  }
  return slots;
}

function safeCheckoutUrl(value: unknown): string | null {
  if (typeof value !== "string" || !value.trim()) return null;
  try {
    const url = new URL(value);
    let decodedUrl = url.href;
    try {
      decodedUrl = decodeURIComponent(decodedUrl);
    } catch {
      return null;
    }
    if (/cs_test/i.test(decodedUrl)) return null;
    const stripeCheckout = url.protocol === "https:" && url.hostname === "checkout.stripe.com";
    return stripeCheckout && !url.username && !url.password ? url.toString() : null;
  } catch {
    return null;
  }
}

function BookingForm() {
  const searchParams = useSearchParams();
  const tenant = searchParams?.get("tenant")?.trim() ?? "";
  const serviceId = searchParams?.get("service_id")?.trim() ?? "";
  const hasBookingContext = SAFE_ID.test(tenant) && SAFE_ID.test(serviceId);

  const [description, setDescription] = useState("");
  const [selectedDate, setSelectedDate] = useState("");
  const [selectedSlot, setSelectedSlot] = useState("");
  const [customerName, setCustomerName] = useState("");
  const [customerEmail, setCustomerEmail] = useState("");
  const [availableSlots, setAvailableSlots] = useState<BookingSlot[]>([]);
  const [isLoadingSlots, setIsLoadingSlots] = useState(false);
  const [slotError, setSlotError] = useState("");
  const [submitError, setSubmitError] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const [submitted, setSubmitted] = useState(false);
  const [checkoutUrl, setCheckoutUrl] = useState("");
  const [checkoutUnavailable, setCheckoutUnavailable] = useState(false);

  useEffect(() => {
    if (!selectedDate) {
      setAvailableSlots([]);
      return;
    }

    async function fetchSlots() {
      setIsLoadingSlots(true);
      setSlotError("");
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
        if (!res.ok) throw new Error("Availability request failed");
        const slots = parseSlots(await res.json());
        if (!slots) throw new Error("Invalid availability response");
        setAvailableSlots(slots);
      } catch {
        setAvailableSlots([]);
        setSlotError("Available times could not be loaded. Please try another date or try again later.");
      } finally {
        setIsLoadingSlots(false);
      }
    }

    fetchSlots();
  }, [selectedDate, tenant, serviceId]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitError("");
    if (!customerEmail.trim()) {
      setSubmitError("Enter an email address before booking.");
      return;
    }
    if (!selectedSlot) {
      setSubmitError("Please select a time slot.");
      return;
    }

    const slot = availableSlots.find(s => s.start_time === selectedSlot);
    if (!slot) {
      setSubmitError("The selected time is no longer available. Please choose another slot.");
      return;
    }

    setIsSubmitting(true);
    try {
      const res = await fetch("/api/v1/booking/engine/reserve", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          customer_name: customerName.trim(),
          customer_email: customerEmail.trim(),
          product_id: serviceId,
          start_time: slot.start_time,
          end_time: slot.end_time,
        })
      });

      if (!res.ok) throw new Error("Reservation request failed");
      const data: unknown = await res.json();
      if (!isRecord(data) || typeof data.booking_id !== "string" || !data.booking_id.trim()) {
        throw new Error("Invalid reservation response");
      }
      const checkout = safeCheckoutUrl(data.deposit_stripe_link);
      setCheckoutUrl(checkout || "");
      setCheckoutUnavailable(!checkout);
      setSubmitted(true);
    } catch {
      setSubmitError("The booking request could not be confirmed. Please try again.");
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!hasBookingContext) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
        <p className="max-w-md rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700" role="alert">
          A valid booking link is required.
        </p>
      </div>
    );
  }

  if (submitted) {
    if (checkoutUrl) {
      return (
        <div className="min-h-screen bg-gray-50 flex items-start justify-center p-0 sm:p-4 font-inter overflow-x-hidden">
          <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] rounded-2xl shadow-xl max-w-md w-full p-8 text-center" data-testid="booking-checkout-container">
            <div className="w-16 h-16 bg-blue-100 text-[#0071E3] rounded-full flex items-center justify-center mx-auto mb-6">
              <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
              </svg>
            </div>
            <h2 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Almost there!</h2>
            <p className="text-gray-600 mb-8">Please complete your deposit to secure your time slot.</p>
            <a href={checkoutUrl} target="_blank" rel="noopener noreferrer" className="w-full inline-block bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] text-[#1D1D1F]  font-semibold py-3 px-6 rounded-xl hover:bg-blue-700 transition-colors" data-testid="pay-deposit-btn">
              Pay Deposit
            </a>
          </div>
        </div>
      );
    }

    return (
      <div className="min-h-screen bg-gray-50 flex items-start justify-center p-0 sm:p-4 font-inter overflow-x-hidden">
        <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] rounded-2xl shadow-xl max-w-md w-full p-8 text-center" data-testid="booking-success-container">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
            <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Booking request confirmed.</h2>
          <p className="text-gray-600 mb-8">{checkoutUnavailable ? "Deposit checkout is unavailable because no real checkout session was returned." : "Your booking was confirmed."}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 flex items-start justify-center p-0 sm:p-4 font-inter overflow-x-hidden">
      <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] rounded-2xl shadow-xl max-w-[375px] mx-auto w-full overflow-hidden">
        <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] text-[#1D1D1F] px-8 py-10  text-center">
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
                  <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
                    {availableSlots.map((slot) => {
                      const d = new Date(slot.start_time);
                      const timeString = d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                      return (
                        <button
                          key={slot.start_time}
                          type="button"
                          onClick={() => setSelectedSlot(slot.start_time)}
                          className={`min-h-[44px] min-w-[44px] py-2 px-3 text-sm flex items-center justify-center font-medium rounded-lg border transition-all ${
                            selectedSlot === slot.start_time
                              ? 'bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] text-[#1D1D1F] border-[#0071E3]  shadow-md transform scale-[1.02]'
                              : 'bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] border-gray-200 text-gray-700 hover:border-blue-300 hover:bg-blue-50'
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
                {slotError && <p className="mt-2 text-sm text-red-600" role="alert">{slotError}</p>}
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

          {submitError && <p className="text-sm text-red-600" role="alert">{submitError}</p>}

          <button
            type="submit"
            disabled={isSubmitting}
            className="w-full bg-gray-900 hover:bg-black  font-semibold py-3.5 px-6 rounded-xl transition-all transform active:scale-[0.98] shadow-lg flex items-center justify-center gap-2"
          >
            <span>{isSubmitting ? "Confirming…" : "Confirm Booking"}</span>
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
