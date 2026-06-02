'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function BookingPage() {
  const [product, setProduct] = useState({ id: 'prod_123', name: 'Guitar Lesson', duration: '1 Hour', price: 50 });
  const [selectedDate, setSelectedDate] = useState<string>('');
  const [dates, setDates] = useState<{date: string, day: string}[]>([]);
  const [availableSlots, setAvailableSlots] = useState<{startTime: string, endTime: string}[]>([]);
  const [selectedSlot, setSelectedSlot] = useState<string>('');
  const [bookingStatus, setBookingStatus] = useState<'idle' | 'loading' | 'success' | 'error'>('idle');
  const [depositLink, setDepositLink] = useState<string>('');

  useEffect(() => {
    // Generate next 7 days
    const next7Days = [];
    for (let i = 0; i < 7; i++) {
      const d = new Date();
      d.setDate(d.getDate() + i);
      const dateStr = d.toISOString().split('T')[0];
      const dayStr = d.toLocaleDateString('en-US', { weekday: 'short' });
      next7Days.push({ date: dateStr, day: dayStr });
    }
    setDates(next7Days);
    if (next7Days.length > 0) {
      setSelectedDate(next7Days[0].date);
    }
  }, []);

  useEffect(() => {
    if (!selectedDate) return;

    // Simulate API call for now to match UI mockup
    const mockSlots = [
      { startTime: `${selectedDate}T09:00:00Z`, endTime: `${selectedDate}T10:00:00Z` },
      { startTime: `${selectedDate}T10:30:00Z`, endTime: `${selectedDate}T11:30:00Z` },
      { startTime: `${selectedDate}T13:00:00Z`, endTime: `${selectedDate}T14:00:00Z` },
      { startTime: `${selectedDate}T15:00:00Z`, endTime: `${selectedDate}T16:00:00Z` }
    ];
    setAvailableSlots(mockSlots);
  }, [selectedDate]);

  const handleBook = async () => {
    if (!selectedSlot) return;

    setBookingStatus('loading');

    try {
      const slot = availableSlots.find(s => s.startTime === selectedSlot);
      if (!slot) throw new Error('Slot not found');

      const res = await fetch('/api/booking', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          product_id: product.id,
          start_time: slot.startTime,
          end_time: slot.endTime,
        })
      });

      if (!res.ok) {
        throw new Error('Failed to book');
      }

      const data = await res.json();
      setDepositLink(data.deposit_stripe_link || '#');
      setBookingStatus('success');
    } catch (e) {
      console.error(e);
      setBookingStatus('error');
    }
  };

  if (bookingStatus === 'success') {
    return (
      <div className="min-h-screen font-inter flex flex-col items-center justify-center p-6" style={{ backgroundColor: '#F5F5F7' }}>
        <div className="bg-white rounded-[16px] shadow-sm p-8 max-w-md w-full text-center" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Booking Reserved!</h2>
          <p className="text-gray-500 mb-6">Your time slot has been secured. Please complete the deposit payment to finalize the booking.</p>
          <a href={depositLink} className="block w-full py-3 px-4 rounded-lg font-medium text-white transition-colors" style={{ backgroundColor: '#0071E3' }}>
            Pay Deposit
          </a>
          <button onClick={() => setBookingStatus('idle')} className="block w-full mt-3 py-3 px-4 rounded-lg font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 transition-colors">
            Back to Booking
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen font-inter pb-24" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 border-b flex items-center gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
        </Link>
        <h1 className="text-xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Book an Appointment</h1>
      </header>

      <main className="p-4 sm:p-6 max-w-lg mx-auto space-y-6">

        {/* Service Details */}
        <div className="bg-white rounded-[16px] shadow-sm p-5" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
          <h2 className="text-lg font-semibold font-outfit text-gray-900">{product.name}</h2>
          <div className="flex items-center gap-4 mt-2 text-sm text-gray-500">
            <div className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
              <span>{product.duration}</span>
            </div>
            <div className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08-.402-2.599-1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
              <span>${product.price} Deposit</span>
            </div>
          </div>
        </div>

        {/* Date Selection */}
        <div>
          <h3 className="text-md font-semibold font-outfit text-gray-900 mb-3 ml-1">Select Date</h3>
          <div className="flex overflow-x-auto gap-3 pb-2 snap-x scrollbar-hide">
            {dates.map((d) => {
              const isSelected = selectedDate === d.date;
              const dateObj = new Date(d.date);
              const dayNum = dateObj.getDate();

              return (
                <button
                  key={d.date}
                  onClick={() => { setSelectedDate(d.date); setSelectedSlot(''); }}
                  className={`snap-start shrink-0 w-16 h-20 rounded-[12px] flex flex-col items-center justify-center transition-all ${
                    isSelected
                      ? 'bg-blue-600 text-white shadow-md'
                      : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
                  }`}
                >
                  <span className={`text-xs font-medium mb-1 ${isSelected ? 'text-blue-100' : 'text-gray-500'}`}>{d.day}</span>
                  <span className="text-lg font-bold">{dayNum}</span>
                </button>
              );
            })}
          </div>
        </div>

        {/* Time Selection */}
        <div>
          <h3 className="text-md font-semibold font-outfit text-gray-900 mb-3 ml-1">Available Times</h3>

          {availableSlots.length > 0 ? (
            <div className="grid grid-cols-2 gap-3">
              {availableSlots.map((slot, idx) => {
                const timeStr = new Date(slot.startTime).toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
                const isSelected = selectedSlot === slot.startTime;

                return (
                  <button
                    key={idx}
                    onClick={() => setSelectedSlot(slot.startTime)}
                    className={`py-3 px-4 rounded-[12px] font-medium text-sm transition-all border ${
                      isSelected
                        ? 'border-blue-600 bg-blue-50 text-blue-700 ring-1 ring-blue-600'
                        : 'border-gray-200 bg-white text-gray-700 hover:border-gray-300 hover:bg-gray-50'
                    }`}
                  >
                    {timeStr}
                  </button>
                );
              })}
            </div>
          ) : (
            <div className="bg-white rounded-[12px] p-6 text-center border border-gray-100">
              <p className="text-gray-500 text-sm">No available slots for this date.</p>
            </div>
          )}
        </div>

      </main>

      {/* Fixed Bottom Bar for Mobile */}
      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white border-t border-gray-200 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.05)]" style={{ zIndex: 100 }}>
        <button
          onClick={handleBook}
          disabled={!selectedSlot || bookingStatus === 'loading'}
          className={`w-full max-w-lg mx-auto flex items-center justify-center py-3.5 px-4 rounded-[12px] font-medium text-white transition-colors ${
            !selectedSlot || bookingStatus === 'loading'
              ? 'bg-gray-300 cursor-not-allowed'
              : 'bg-[#0071E3] hover:bg-blue-700 active:scale-[0.98]'
          }`}
        >
          {bookingStatus === 'loading' ? 'Processing...' : 'Continue to Payment'}
        </button>
      </div>

    </div>
  );
}
