"use client";

import React, { useState, useEffect } from 'react';
import { syncManager } from '../../../lib/syncManager';

export default function TerminalPage() {
  const [pin, setPin] = useState('');
  const [activeStaff, setActiveStaff] = useState<any | null>(null);
  const [clockedIn, setClockedIn] = useState(false);
  const [error, setError] = useState('');
  const [isOffline, setIsOffline] = useState(false);
  const [syncStatus, setSyncStatus] = useState(syncManager.getStatus());

  // Network listener & Sync Status
  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    setIsOffline(!navigator.onLine);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    const unsubscribe = syncManager.subscribe(setSyncStatus);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
      unsubscribe();
    };
  }, []);

  const handlePinEntry = (digit: string) => {
    if (pin.length < 4) {
      const newPin = pin + digit;
      setPin(newPin);
      setError('');

      if (newPin.length === 4) {
        // Mock Auth
        if (newPin === '1234') {
          setActiveStaff({ id: 'staff-1', name: 'Fatima', role: 'Owner' });
        } else {
          setError('Invalid PIN');
          setPin('');
        }
      }
    }
  };

  const handleClockAction = (type: 'CLOCK_IN' | 'CLOCK_OUT') => {
    setClockedIn(type === 'CLOCK_IN');
  };

  const handleNewOrder = () => {
     syncManager.enqueue({
        mutation_type: 'INVENTORY_DEDUCT',
        product_id: 'falafel-plate',
        quantity_deducted: 1,
        amount: 1200,
        currency: 'USD'
     });
     alert('Order placed (Offline-ready)');
  };

  if (!activeStaff) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter">
        <div className="w-[375px] h-[812px] bg-black text-white p-8 flex flex-col items-center relative overflow-hidden rounded-[40px] border border-gray-800">
           <div className="mt-20 mb-12 text-center">
             <h1 className="text-2xl font-bold font-outfit mb-2">Terminal Locked</h1>
             <p className="text-gray-400 text-sm tracking-wide uppercase">Enter PIN</p>
           </div>

           <div className="flex gap-4 mb-16">
              {[...Array(4)].map((_, i) => (
                <div key={i} className={`w-3 h-3 rounded-full border border-white/20 transition-all ${pin.length > i ? 'bg-white border-white scale-110 shadow-[0_0_10px_rgba(255,255,255,0.5)]' : 'bg-transparent'}`}></div>
              ))}
           </div>

           {error && <p className="text-red-500 mb-6 text-sm font-bold tracking-tight">{error}</p>}

           <div className="grid grid-cols-3 gap-8 w-full max-w-[280px]">
             {[1, 2, 3, 4, 5, 6, 7, 8, 9].map(num => (
               <button
                 key={num}
                 onClick={() => handlePinEntry(num.toString())}
                 className="w-16 h-16 rounded-full bg-white/5 text-2xl font-outfit hover:bg-white/10 active:scale-95 transition-all border border-white/5 flex items-center justify-center"
               >
                 {num}
               </button>
             ))}
             <div className="col-start-2">
               <button
                 onClick={() => handlePinEntry('0')}
                 className="w-16 h-16 rounded-full bg-white/5 text-2xl font-outfit hover:bg-white/10 active:scale-95 transition-all border border-white/5 flex items-center justify-center"
               >
                 0
               </button>
             </div>
             <div className="col-start-3 flex items-center justify-center">
               <button onClick={() => setPin('')} className="text-gray-500 hover:text-white transition text-sm font-bold uppercase tracking-widest">Clear</button>
             </div>
           </div>

           {syncStatus.pendingCount > 0 && (
              <div className="absolute bottom-12 flex items-center gap-2 px-4 py-2 bg-white/5 rounded-full border border-white/10 backdrop-blur-md">
                 <div className={`w-2 h-2 rounded-full ${syncStatus.isSyncing ? 'bg-blue-500 animate-pulse' : 'bg-amber-500'}`} />
                 <span className="text-[10px] font-bold text-gray-400 uppercase tracking-widest">{syncStatus.pendingCount} Sync Pending</span>
              </div>
           )}
        </div>
      </div>
    );
  }

  return (
     <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter py-10">
      <div className="w-[375px] h-[812px] bg-black text-white shadow-2xl overflow-hidden flex flex-col relative border border-gray-800 rounded-[40px]">

        {/* Header */}
        <div className="pt-16 pb-6 px-8 bg-black/40 backdrop-blur-[20px] border-b border-white/10 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold font-outfit tracking-tight">{activeStaff.name}</h1>
            <p className="text-blue-400 font-bold text-[10px] uppercase tracking-widest mt-0.5">{activeStaff.role}</p>
          </div>
          <div className="flex items-center gap-4">
             {isOffline && <div className="w-2 h-2 rounded-full bg-amber-500 animate-pulse" />}
             <button onClick={() => setActiveStaff(null)} className="text-[10px] font-bold text-gray-500 hover:text-white uppercase tracking-widest transition">
               Lock
             </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-6 py-8">

           <div className="bg-white/5 rounded-[32px] p-8 border border-white/10 mb-8 text-center backdrop-blur-md">
             <div className={`w-20 h-20 mx-auto rounded-full flex items-center justify-center mb-6 transition-all ${clockedIn ? 'bg-green-500/10 text-green-400 shadow-[0_0_30px_rgba(34,197,94,0.1)]' : 'bg-white/5 text-gray-600'}`}>
                <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
             </div>
             <h2 className="text-2xl font-bold font-outfit mb-2">
               {clockedIn ? 'Clocked In' : 'Not Clocked In'}
             </h2>
             <p className="text-xs text-gray-500 mb-8 leading-relaxed">
                {clockedIn ? 'Your shift is active. All sales tracked locally.' : 'Clock in to begin your shift and sync data.'}
             </p>

             <button
               onClick={() => handleClockAction(clockedIn ? 'CLOCK_OUT' : 'CLOCK_IN')}
               className={`w-full py-4 rounded-2xl font-bold transition-all active:scale-[0.98] ${clockedIn ? 'bg-white/5 text-red-400 border border-red-500/20 hover:bg-red-500/10' : 'bg-white text-black shadow-xl shadow-white/5'}`}
             >
               {clockedIn ? 'Clock Out' : 'Clock In'}
             </button>
           </div>

           <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-[0.2em] mb-6 px-2">Quick Actions</h3>

           <div className="grid grid-cols-2 gap-4">
             <button
                onClick={handleNewOrder}
                className="bg-white/5 p-5 rounded-[24px] border border-white/10 text-left active:scale-[0.98] transition-all hover:bg-white/[0.07]"
             >
               <div className="text-blue-400 mb-4 bg-blue-500/10 w-10 h-10 rounded-xl flex items-center justify-center">
                 <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
               </div>
               <span className="font-bold text-sm">New Order</span>
             </button>

             <button className="bg-white/5 p-5 rounded-[24px] border border-white/10 text-left active:scale-[0.98] transition-all hover:bg-white/[0.07]">
               <div className="text-purple-400 mb-4 bg-purple-500/10 w-10 h-10 rounded-xl flex items-center justify-center">
                 <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
               </div>
               <span className="font-bold text-sm">Reports</span>
             </button>
           </div>
        </div>

        {syncStatus.pendingCount > 0 && (
           <div className={`mt-auto px-8 py-4 bg-white/5 border-t border-white/10 backdrop-blur-2xl flex items-center justify-between transition-all ${syncStatus.isSyncing ? 'animate-pulse' : ''}`}>
              <span className="text-[10px] font-bold text-gray-400 uppercase tracking-widest">{syncStatus.isSyncing ? 'Syncing...' : 'Sync Pending'}</span>
              <div className="flex items-center gap-2">
                 <span className="text-xs font-bold">{syncStatus.pendingCount}</span>
                 <div className="w-1.5 h-1.5 rounded-full bg-blue-500" />
              </div>
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
