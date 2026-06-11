"use client";

import React, { useState, Suspense, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import { OneTapReferral } from "../components/OneTapReferral";

interface TimeSlot {
  start_time: string;
  end_time: string;
}

function BookingForm() {
  const searchParams = useSearchParams();
  const tenant = searchParams?.get("tenant") || "default-store";

  // A hypothetical product ID - in real life this might come from the URL too
  const productId = searchParams?.get("product_id") || "e2e-product";

  const [date, setDate] = useState<string>("");
  const [availableSlots, setAvailableSlots] = useState<TimeSlot[]>([]);
  const [selectedSlot, setSelectedSlot] = useState<TimeSlot | null>(null);

  const [description, setDescription] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [submitted, setSubmitted] = useState(false);
  const [loadingSlots, setLoadingSlots] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Default to today's date
  useEffect(() => {
    const today = new Date().toISOString().split("T")[0];
    setDate(today);
  }, []);

  useEffect(() => {
    if (!date) return;

    let isMounted = true;

    async function fetchSlots() {
      setLoadingSlots(true);
      setError(null);
      try {
        const res = await fetch(`/api/v1/booking/availability?product_id=${productId}&date=${date}`, {
            headers: {
                "x-tenant-id": tenant
            }
        });
        if (!res.ok) {
            throw new Error('Failed to fetch availability');
        }
        const data = await res.json();
        if (isMounted) {
            setAvailableSlots(data.available_slots || []);
        }
      } catch (e: any) {
        if (isMounted) {
            setError(e.message || 'Error fetching slots');
        }
      } finally {
        if (isMounted) {
            setLoadingSlots(false);
        }
      }
    }

    fetchSlots();

    return () => { isMounted = false; };
  }, [date, productId, tenant]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedSlot) {
        setError("Please select a time slot.");
        return;
    }

    try {
        const res = await fetch("/api/v1/booking/reserve", {
          method: "POST",
          headers: {
              "Content-Type": "application/json",
              "x-tenant-id": tenant
          },
          body: JSON.stringify({
            customer_id: "e2e-customer", // In reality this would come from auth context
            product_id: productId,
            start_time: selectedSlot.start_time,
            end_time: selectedSlot.end_time,
            requires_deposit: false
          }),
        });

        if (!res.ok) {
            throw new Error("Failed to reserve the slot.");
        }

        // Also send the request description for omni-channel
        await fetch("/api/v1/booking/request", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "x-tenant-id": tenant
            },
            body: JSON.stringify({
                description: `Booking for ${selectedSlot.start_time}: ${description}`,
                fileName: file?.name,
                timestamp: new Date().toISOString()
            }),
        });

        setSubmitted(true);
    } catch (e: any) {
        setError(e.message || "Failed to submit booking");
    }
  };

  if (submitted) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter p-6">
        <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-2xl text-center border border-white/40">
          <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-6">✅</div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Booking Confirmed!</h1>
          <p className="text-gray-600 text-sm leading-relaxed">
            We've received your booking. You will receive an email confirmation shortly.
          </p>
          <button
            onClick={() => {
                setSubmitted(false);
                setSelectedSlot(null);
            }}
            className="mt-8 w-full py-3 px-4 rounded-xl font-bold text-sm bg-gray-900 text-white hover:bg-black transition-all mb-6"
          >
            Make Another Booking
          </button>

          <OneTapReferral tenantId={tenant} source="booking_success" />

          <div className="mt-6 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
            <a
              href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`}
              target="_blank"
              rel="noopener noreferrer"
              style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>
              ⚡ Powered by OHC
            </a>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center min-h-screen font-inter py-10" style={{ backgroundColor: '#F5F5F7' }}>
      <div className="w-[375px] max-w-[375px] min-h-[812px] shadow-2xl overflow-hidden flex flex-col relative"
           style={{
               background: 'rgba(255, 255, 255, 0.65)',
               backdropFilter: 'blur(30px) saturate(210%)',
               border: '1px solid rgba(255, 255, 255, 0.4)',
               borderRadius: '16px'
           }}
      >

        {/* Header */}
        <div className="pt-12 pb-6 px-6 sticky top-0 z-10" style={{ borderBottom: '1px solid rgba(255, 255, 255, 0.4)' }}>
          <h1 className="text-2xl font-bold font-outfit tracking-tight" style={{ color: '#1D1D1F' }}>Book an Appointment</h1>
          <p className="text-sm mt-1" style={{ color: '#6b7280' }}>Select a time slot and tell us what you need.</p>
        </div>

        {/* Form Content */}
        <form onSubmit={handleSubmit} className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">

          {/* Date Picker */}
          <div>
              <label className="block text-sm font-semibold mb-2 uppercase tracking-wider text-[10px]" style={{ color: '#1D1D1F' }}>Select Date</label>
              <input
                 type="date"
                 value={date}
                 onChange={(e) => {
                     setDate(e.target.value);
                     setSelectedSlot(null);
                 }}
                 className="w-full bg-white/50 border border-white/40 rounded-xl px-4 py-3 text-sm text-gray-800 focus:outline-none focus:ring-2 transition-all"
                 style={{ backdropFilter: 'blur(10px)' }}
                 required
              />
          </div>

          {/* Time Slots */}
          <div>
             <label className="block text-sm font-semibold mb-2 uppercase tracking-wider text-[10px]" style={{ color: '#1D1D1F' }}>Available Times</label>
             {loadingSlots ? (
                 <div className="text-sm text-gray-500 py-4 text-center">Loading slots...</div>
             ) : error ? (
                 <div className="text-sm text-red-500 py-4 text-center">{error}</div>
             ) : availableSlots.length === 0 ? (
                 <div className="text-sm text-gray-500 py-4 text-center">No slots available for this date.</div>
             ) : (
                 <div className="grid grid-cols-2 gap-3">
                     {availableSlots.map((slot, idx) => {
                         const start = new Date(slot.start_time);
                         const isSelected = selectedSlot?.start_time === slot.start_time;
                         return (
                             <button
                                 key={idx}
                                 type="button"
                                 onClick={() => setSelectedSlot(slot)}
                                 className={`py-3 rounded-xl text-sm font-medium transition-all ${isSelected ? 'text-white' : 'text-gray-800 bg-white/50 hover:bg-white/80'}`}
                                 style={{
                                     backgroundColor: isSelected ? '#0066FF' : undefined,
                                     border: '1px solid rgba(255, 255, 255, 0.4)'
                                 }}
                             >
                                 {start.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                             </button>
                         )
                     })}
                 </div>
             )}
          </div>

          <div>
            <label className="block text-sm font-semibold mb-2 uppercase tracking-wider text-[10px]" style={{ color: '#1D1D1F' }}>What do you need help with?</label>
            <textarea
              required
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="e.g. I have a leaky faucet in the kitchen that needs fixing."
              className="w-full min-h-[120px] bg-white/50 border border-white/40 rounded-xl px-4 py-3 text-sm text-gray-800 placeholder-gray-400 focus:outline-none focus:ring-2 focus:bg-white transition-all"
              style={{ backdropFilter: 'blur(10px)' }}
            />
          </div>

          <div>
            <label className="block text-sm font-semibold mb-2 uppercase tracking-wider text-[10px]" style={{ color: '#1D1D1F' }}>Attach a Photo (Optional)</label>
            <div className="relative border-2 border-dashed border-gray-300/50 rounded-xl p-6 flex flex-col items-center justify-center text-center hover:bg-white/50 transition-colors bg-white/30 backdrop-blur-md">
              <input
                aria-label="Attach a photo"
                type="file"
                accept="image/*"
                onChange={(e) => setFile(e.target.files?.[0] || null)}
                className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              />
              <div className="w-10 h-10 bg-white/50 rounded-full flex items-center justify-center text-gray-500 mb-2 shadow-sm border border-white/40">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
              </div>
              <span className="text-sm font-medium text-gray-700">
                {file ? file.name : "Tap to upload a photo"}
              </span>
            </div>
          </div>

          <div className="pt-4">
             <button
              type="submit"
              disabled={!selectedSlot}
              className="w-full py-4 px-4 rounded-xl font-bold text-[15px] text-white transition-all shadow-md"
              style={{
                  backgroundColor: selectedSlot ? '#0066FF' : '#9ca3af',
                  opacity: selectedSlot ? 1 : 0.7,
                  cursor: selectedSlot ? 'pointer' : 'not-allowed'
              }}
            >
              Confirm Booking
            </button>
          </div>
        </form>
        <div className="py-4 text-center border-t border-gray-100" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
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
    <Suspense fallback={<div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center">Loading...</div>}>
      <BookingForm />
    </Suspense>
  );
}
