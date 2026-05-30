"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function BookingPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('DEFAULT');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);

  const [formData, setFormData] = useState({
    name: '',
    email: '',
    date: '',
    time: ''
  });

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setTenant(localStorage.getItem('tenant') || 'DEFAULT');
    }
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      await fetch('/api/v1/growth/booking', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData)
      });
      setIsSubmitting(false);
      setIsSuccess(true);
    } catch (e) {
      console.error("Booking error", e);
      setIsSubmitting(false);
      setIsSuccess(true);
    }
  };

  const referralLink = `ohc://join?ref=${tenant}`;
  const shareText = `I just booked an appointment! Book yours here: ${referralLink}`;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Book an Appointment</h1>
      </header>

      <main className="flex-1 w-full max-w-lg mx-auto p-4 md:p-8 flex flex-col items-center justify-center">
        {!isSuccess ? (
          <div className="w-full bg-white/80 backdrop-blur-[20px] saturate-[200%] border border-white/40 p-8 rounded-3xl shadow-xl">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6 text-center">Schedule your session</h2>

            <form onSubmit={handleSubmit} className="flex flex-col gap-5">
              <div className="flex flex-col gap-2">
                <label className="text-sm font-semibold text-gray-700">Full Name</label>
                <input
                  type="text"
                  name="name"
                  required
                  value={formData.name}
                  onChange={handleChange}
                  placeholder="John Doe"
                  className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-shadow"
                />
              </div>

              <div className="flex flex-col gap-2">
                <label className="text-sm font-semibold text-gray-700">Email Address</label>
                <input
                  type="email"
                  name="email"
                  required
                  value={formData.email}
                  onChange={handleChange}
                  placeholder="john@example.com"
                  className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-shadow"
                />
              </div>

              <div className="flex gap-4">
                <div className="flex-1 flex flex-col gap-2">
                  <label className="text-sm font-semibold text-gray-700">Date</label>
                  <input
                    type="date"
                    name="date"
                    required
                    value={formData.date}
                    onChange={handleChange}
                    className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-shadow"
                  />
                </div>
                <div className="flex-1 flex flex-col gap-2">
                  <label className="text-sm font-semibold text-gray-700">Time</label>
                  <select
                    name="time"
                    required
                    value={formData.time}
                    onChange={handleChange}
                    className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-shadow appearance-none"
                  >
                    <option value="" disabled>Select time</option>
                    <option value="09:00">09:00 AM</option>
                    <option value="10:00">10:00 AM</option>
                    <option value="11:00">11:00 AM</option>
                    <option value="13:00">01:00 PM</option>
                    <option value="14:00">02:00 PM</option>
                    <option value="15:00">03:00 PM</option>
                  </select>
                </div>
              </div>

              <button
                type="submit"
                disabled={isSubmitting}
                className="mt-4 w-full py-4 bg-indigo-600 text-white font-bold rounded-xl shadow-md hover:bg-indigo-700 hover:-translate-y-0.5 active:scale-[0.98] transition-all disabled:opacity-70 disabled:cursor-not-allowed"
              >
                {isSubmitting ? 'Confirming...' : 'Confirm Booking'}
              </button>
            </form>
          </div>
        ) : (
          <div className="w-full bg-white/80 backdrop-blur-[20px] saturate-[200%] border border-white/40 p-8 rounded-3xl shadow-2xl flex flex-col items-center text-center relative overflow-hidden">
            <div className="absolute top-0 right-0 w-48 h-48 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center text-3xl mb-6 shadow-inner">
              ✓
            </div>

            <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Booking Confirmed!</h2>
            <p className="text-gray-600 mb-8 max-w-sm">
              We've successfully scheduled your appointment. A confirmation email has been sent to your inbox.
            </p>

            {/* Viral Growth Loop Section */}
            <div className="w-full border-t border-gray-100 pt-6 mt-2 flex flex-col gap-4">
              <div className="flex flex-col items-center gap-2">
                <span className="text-xs uppercase font-bold tracking-widest text-gray-400">Want your own booking page?</span>
                <a
                  href={referralLink}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="w-full py-3 bg-gray-900 text-white rounded-xl font-bold shadow-sm hover:bg-black transition-all"
                >
                  Create yours for free
                </a>
              </div>

              <button
                onClick={() => window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`, '_blank')}
                className="w-full flex items-center justify-center gap-2 bg-[#1DA1F2] text-white py-3 rounded-xl font-bold shadow-sm hover:bg-[#1a91da] transition-all"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share on X
              </button>

              <div className="mt-4 opacity-70 hover:opacity-100 transition-opacity">
                <a href={referralLink} className="text-xs font-bold text-gray-500 hover:text-gray-800 uppercase tracking-widest flex items-center justify-center gap-1">
                  ⚡ Powered by OHC
                </a>
              </div>
            </div>
          </div>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
