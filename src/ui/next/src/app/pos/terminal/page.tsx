"use client";

import React, { useState, useEffect } from 'react';
import { useTranslation, useCurrency } from '../../../lib/localizationStore';
import { LocalizationToggle } from '../../../components/LocalizationToggle';

// Offline storage helper for staff data
const OfflineStore = {
  getStaff: () => JSON.parse(localStorage.getItem('ohc_offline_staff') || '[]'),
  setStaff: (staff: any[]) => localStorage.setItem('ohc_offline_staff', JSON.stringify(staff)),

  getEvents: () => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'),
  addEvent: (event: any) => {
    const events = OfflineStore.getEvents();
    events.push(event);
    localStorage.setItem('ohc_offline_events', JSON.stringify(events));
  },
  clearEvents: () => localStorage.setItem('ohc_offline_events', '[]')
};

export default function TerminalPage() {
  const { t } = useTranslation();
  const { currency, convert } = useCurrency();
  const [pin, setPin] = useState('');
  const [activeStaff, setActiveStaff] = useState<any | null>(null);
  const [clockedIn, setClockedIn] = useState(false);
  const [error, setError] = useState('');
  const [syncing, setSyncing] = useState(false);
  const [offlineConversion, setOfflineConversion] = useState(false);

  // Initial Fetch & Background sync
  useEffect(() => {
    // Initial fetch to populate offline store with real data if online
    const fetchStaff = async () => {
      if (navigator.onLine) {
        try {
          const res = await fetch('/api/staff');
          if (res.ok) {
            const data = await res.json();
            if (data.length > 0) {
              OfflineStore.setStaff(data);
            }
          }
        } catch (e) {
          console.error("Failed to fetch initial staff config", e);
        }
      }
    };
    fetchStaff();

    const syncInterval = setInterval(async () => {
      const events = OfflineStore.getEvents();
      if (events.length > 0 && navigator.onLine) {
        setSyncing(true);
        try {
          const res = await fetch('/api/staff/timecard', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(events)
          });
          if (res.ok) {
            OfflineStore.clearEvents();
          }
        } catch (e) {
          console.error("Sync failed", e);
        } finally {
          setSyncing(false);
        }
      }
    }, 10000); // Try syncing every 10 seconds

    return () => clearInterval(syncInterval);
  }, []);

  const handlePinEntry = (digit: string) => {
    if (pin.length < 4) {
      const newPin = pin + digit;
      setPin(newPin);
      setError('');

      if (newPin.length === 4) {
        // Attempt to authenticate offline
        const staff = OfflineStore.getStaff();
        const found = staff.find((s: any) => s.pin_hash === newPin); // Simple mock check

        if (found) {
          setActiveStaff(found);
          // Check last event locally to determine clockedIn status
          const events = OfflineStore.getEvents().filter((e: any) => e.staff_id === found.id);
          if (events.length > 0) {
             const lastEvent = events[events.length - 1];
             setClockedIn(lastEvent.event_type === 'CLOCK_IN');
          }
        } else {
          setError(t('Invalid PIN'));
          setPin('');
        }
      }
    }
  };

  const handleClear = () => setPin('');
  const handleLock = () => {
      setActiveStaff(null);
      setPin('');
  };

  const handleClockAction = (type: 'CLOCK_IN' | 'CLOCK_OUT') => {
    if (!activeStaff) return;

    const event = {
      staff_id: activeStaff.id,
      tenant_id: 'default_tenant',
      event_type: type,
      timestamp: new Date().toISOString(),
      sync_status: 'PENDING'
    };

    OfflineStore.addEvent(event);
    setClockedIn(type === 'CLOCK_IN');
  };

  const handleNewOrder = () => {
    const basePrice = 5000; // $50.00
    const converted = convert(basePrice, 'USD', currency);
    if (converted.isOffline) {
      setOfflineConversion(true);
      setTimeout(() => setOfflineConversion(false), 3000);
    }
    alert(`${t('New Order Total')}: ${converted.amount / 100} ${currency}`);
  };

  if (!activeStaff) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter">
        <div className="w-[375px] h-[812px] p-8 flex flex-col items-center relative overflow-hidden rounded-2xl" style={{ background: 'rgba(22, 22, 26, 0.7)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.1)' }}>
           <div className="absolute top-8 right-8">
              <LocalizationToggle />
           </div>

           <div className="mt-20 mb-12 text-center text-[#F5F5F7]">
             <h1 className="text-2xl font-bold font-outfit mb-2">{t('Terminal Locked')}</h1>
             <p className="text-gray-400 font-inter">{t('Enter your PIN to unlock')}</p>
           </div>

           <div className="flex gap-4 mb-12">
              {[...Array(4)].map((_, i) => (
                <div key={i} className={`w-4 h-4 rounded-full border-2 ${pin.length > i ? 'bg-white border-white' : 'border-gray-600'}`}></div>
              ))}
           </div>

           {error && <p className="text-red-500 mb-4 animate-bounce">{error}</p>}

           <div className="grid grid-cols-3 gap-6 w-full max-w-[280px]">
             {[1, 2, 3, 4, 5, 6, 7, 8, 9].map(num => (
               <button
                 key={num}
                 onClick={() => handlePinEntry(num.toString())}
                 className="w-20 h-20 rounded-full bg-gray-800 text-3xl font-light hover:bg-gray-700 active:bg-gray-600 transition-colors flex items-center justify-center"
               >
                 {num}
               </button>
             ))}
             <div className="col-start-2">
               <button
                 onClick={() => handlePinEntry('0')}
                 className="w-20 h-20 rounded-full bg-gray-800 text-3xl font-light hover:bg-gray-700 active:bg-gray-600 transition-colors flex items-center justify-center"
               >
                 0
               </button>
             </div>
             <div className="col-start-3 flex items-center justify-center">
               <button onClick={handleClear} className="text-gray-400 hover:text-white">{t('Clear')}</button>
             </div>
           </div>

           {syncing && <div className="absolute bottom-4 left-4 text-xs text-blue-400">{t('Syncing...')}</div>}
        </div>
      </div>
    );
  }

  return (
     <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter py-10">
      <div className="w-[375px] h-[812px] rounded-2xl shadow-2xl overflow-hidden flex flex-col relative" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>

        {/* Header */}
        <div className="pt-12 pb-6 px-6 border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)' }}>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">{activeStaff.name}</h1>
            <p className="text-[#0066FF] font-medium font-inter text-sm mt-1">{t(activeStaff.role)}</p>
          </div>
          <div className="flex items-center gap-3">
            <LocalizationToggle />
            <button onClick={handleLock} className="text-sm font-semibold text-gray-500 hover:text-gray-900">
              {t('Lock')}
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-transparent">

           <div className="rounded-2xl p-6 shadow-sm mb-6 text-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
             <div className={`w-16 h-16 mx-auto rounded-full flex items-center justify-center mb-4 ${clockedIn ? 'bg-[#34C759]/20 text-[#34C759]' : 'bg-gray-100 text-gray-400'}`}>
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
             </div>
             <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] mb-1">
               {clockedIn ? t('Clocked In') : t('Not Clocked In')}
             </h2>
             <p className="text-sm font-inter text-gray-500 mb-6">
                {clockedIn ? t('Your time is being tracked locally.') : t('Clock in to start your shift.')}
             </p>

             {clockedIn ? (
               <button
                 onClick={() => handleClockAction('CLOCK_OUT')}
                 className="w-full py-4 rounded-[8px] bg-[#FF3B30]/10 text-[#FF3B30] font-bold font-inter hover:bg-[#FF3B30]/20 transition-colors"
               >
                 {t('Clock Out')}
               </button>
             ) : (
               <button
                 onClick={() => handleClockAction('CLOCK_IN')}
                 className="w-full py-4 rounded-[8px] bg-[#0066FF] text-[#F5F5F7] font-bold font-inter shadow-md shadow-blue-500/20 hover:bg-blue-700 transition-colors"
               >
                 {t('Clock In')}
               </button>
             )}
           </div>

           {/* Role-based UI rendering */}
           <h3 className="text-sm font-bold font-outfit text-gray-400 uppercase tracking-wider mb-4 px-2 mt-8">{t('Quick Actions')}</h3>

           <div className="grid grid-cols-2 gap-4">
             <button
                onClick={handleNewOrder}
                className="p-4 rounded-[16px] shadow-sm text-left active:scale-[0.98]" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}
             >
               <div className="text-[#0066FF] mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
               </div>
               <span className="font-medium font-inter text-[#1D1D1F]">{t('New Order')}</span>
             </button>

             {activeStaff.role === 'Manager' && (
               <button className="p-4 rounded-[16px] shadow-sm text-left active:scale-[0.98]" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                 <div className="text-purple-500 mb-2">
                   <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                 </div>
                 <span className="font-medium font-inter text-[#1D1D1F]">{t('Reports')}</span>
               </button>
             )}

             <button className="p-4 rounded-[16px] shadow-sm text-left active:scale-[0.98]" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
               <div className="text-[#FF9500] mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
               </div>
               <span className="font-medium font-inter text-[#1D1D1F]">{t('Refunds')}</span>
             </button>
           </div>
        </div>

        {syncing && <div className="bg-[#0066FF]/10 text-[#0066FF] font-inter text-xs text-center py-2" style={{ borderTop: '1px solid rgba(0, 102, 255, 0.2)' }}>{t('Syncing offline events...')}</div>}
        {offlineConversion && (
          <div className="absolute bottom-16 left-1/2 -translate-x-1/2 bg-[#FF9500]/20 text-[#FF9500] px-4 py-2 rounded-[8px] text-xs font-bold shadow-lg animate-bounce" style={{ border: '1px solid rgba(255, 149, 0, 0.4)', backdropFilter: 'blur(10px)' }}>
            {t('Using cached rates - Syncing soon')}
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
