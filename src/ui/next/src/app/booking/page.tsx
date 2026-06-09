"use client";

import React, { useState, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import { OneTapReferral } from "../components/OneTapReferral";

function BookingForm() {
  const searchParams = useSearchParams();
  const tenant = searchParams?.get("tenant") || "default-store";
  const [description, setDescription] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Simulating form submission
    await fetch("/api/v1/booking/request", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        description,
        fileName: file?.name,
        timestamp: new Date().toISOString()
      }),
    });

    setSubmitted(true);
  };

  if (submitted) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter p-6">
        <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] rounded-[24px] p-8 shadow-2xl text-center border border-white/40">
          <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-6">✅</div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Request Sent!</h1>
          <p className="text-gray-600 text-sm leading-relaxed">
            We've received your inquiry. We'll review it and send over a custom quote and available timeslots shortly.
          </p>
          <button
            onClick={() => setSubmitted(false)}
            className="mt-8 w-full py-3 px-4 rounded-xl font-bold text-sm bg-gray-900 text-white hover:bg-black transition-all mb-6"
          >
            Submit Another Request
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
    <div className="flex flex-col items-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white sticky top-0 z-10 border-b border-gray-100">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Request a Service</h1>
          <p className="text-gray-500 text-sm mt-1">Tell us what you need, and we'll send a quote and available times.</p>
        </div>

        {/* Form Content */}
        <form onSubmit={handleSubmit} className="flex-1 px-6 py-6 overflow-y-auto hide-scrollbar space-y-6">

          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">What do you need help with?</label>
            <textarea
              required
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="e.g. I have a leaky faucet in the kitchen that needs fixing."
              className="w-full min-h-[120px] bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-800 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:bg-white transition-all"
            />
          </div>

          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2 uppercase tracking-wider text-[10px]">Attach a Photo (Optional)</label>
            <div className="relative border-2 border-dashed border-gray-300 rounded-xl p-6 flex flex-col items-center justify-center text-center hover:bg-gray-50 transition-colors">
              <input
                aria-label="Attach a photo"
                type="file"
                accept="image/*"
                onChange={(e) => setFile(e.target.files?.[0] || null)}
                className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              />
              <div className="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center text-gray-500 mb-2">
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
              className="w-full py-4 px-4 rounded-xl font-bold text-[15px] bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all"
            >
              Get a Quote
            </button>
          </div>
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
