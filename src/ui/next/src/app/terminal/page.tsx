"use client";

import React, { useState, useEffect } from 'react';

type CachedStaff = {
  id: string;
  name: string;
  role: string;
  pin: string; // Hashed/stored locally for offline
};

export default function TerminalPage() {
  const [pinInput, setPinInput] = useState('');
  const [error, setError] = useState(false);
  const [activeStaff, setActiveStaff] = useState<CachedStaff | null>(null);
  const [cachedStaffList, setCachedStaffList] = useState<CachedStaff[]>([]);

  // Local state to simulate offline mesh sync
  const [offlineEvents, setOfflineEvents] = useState<any[]>([]);

  useEffect(() => {
    // In a real app, this would sync from the backend to local storage
    const staff: CachedStaff[] = [
      { id: '1', name: 'Sarah', role: 'Cashier', pin: '1234' },
      { id: '2', name: 'Fatima', role: 'Manager', pin: '0000' }
    ];
    setCachedStaffList(staff);

    const savedEvents = localStorage.getItem('ohc_offline_timecards');
    if (savedEvents) {
      setOfflineEvents(JSON.parse(savedEvents));
    }
  }, []);

  const handleKeyPress = (num: string) => {
    if (pinInput.length < 4) {
      setPinInput(prev => prev + num);
      setError(false);
    }
  };

  const handleBackspace = () => {
    setPinInput(prev => prev.slice(0, -1));
    setError(false);
  };

  useEffect(() => {
    if (pinInput.length === 4) {
      const match = cachedStaffList.find(s => s.pin === pinInput);
      if (match) {
        setActiveStaff(match);
        setPinInput('');
      } else {
        setError(true);
        setTimeout(() => setPinInput(''), 500);
      }
    }
  }, [pinInput, cachedStaffList]);

  const handleClockEvent = async (type: 'CLOCK_IN' | 'CLOCK_OUT') => {
    if (!activeStaff) return;

    const event = {
      staff_member_id: activeStaff.id,
      pin: activeStaff.pin,
      event_type: type,
      client_timestamp: new Date().toISOString(),
      sync_id: Math.random().toString(36).substring(7)
    };

    // Optimistic UI & offline queue
    const updatedEvents = [...offlineEvents, event];
    setOfflineEvents(updatedEvents);
    localStorage.setItem('ohc_offline_timecards', JSON.stringify(updatedEvents));

    // Try sync to cloud
    try {
      const endpoint = type === 'CLOCK_IN' ? '/api/v1/terminal/clock-in' : '/api/v1/terminal/clock-out';
      await fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(event)
      });
      // In a real app, remove from offline queue after successful sync
    } catch (e) {
      console.warn("Offline mode active. Event queued locally.", e);
    }

    // Return to lock screen
    setActiveStaff(null);
  };

  if (activeStaff) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
        <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
          <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
             <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-green-100 text-green-700 flex items-center justify-center font-bold text-lg border border-green-200">
                   {activeStaff.name.charAt(0)}
                </div>
                <div>
                   <h1 className="text-lg font-bold font-outfit text-gray-900">{activeStaff.name}</h1>
                   <p className="text-xs text-gray-500 font-medium">{activeStaff.role} Mode</p>
                </div>
             </div>
             <button onClick={() => setActiveStaff(null)} className="text-gray-400 hover:text-gray-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" /></svg>
             </button>
          </header>

          <div className="flex-1 overflow-y-auto px-4 py-6 flex flex-col gap-4">
             <div className="bg-white/65 backdrop-blur-[30px] border border-white/40 p-6 rounded-2xl shadow-sm text-center">
                <h2 className="text-2xl font-bold font-outfit text-gray-800 mb-2">Ready to work?</h2>
                <p className="text-sm text-gray-500 mb-6">Your device is connected and ready to process local orders.</p>

                <div className="flex gap-3">
                   <button onClick={() => handleClockEvent('CLOCK_IN')} className="flex-1 py-4 bg-green-600 text-white rounded-xl font-bold shadow-md hover:bg-green-700 active:scale-95 transition-all">Clock In</button>
                   <button onClick={() => handleClockEvent('CLOCK_OUT')} className="flex-1 py-4 bg-red-50 text-red-600 border border-red-200 rounded-xl font-bold hover:bg-red-100 active:scale-95 transition-all">Clock Out</button>
                </div>
             </div>

             <div className="grid grid-cols-2 gap-3 mt-4">
                <div className="bg-white/65 backdrop-blur-[30px] border border-white/40 p-4 rounded-xl shadow-sm flex flex-col items-center justify-center aspect-square active:scale-95 transition-all cursor-pointer">
                   <span className="text-3xl mb-2">🧾</span>
                   <span className="font-semibold text-gray-800">New Order</span>
                </div>
                <div className="bg-white/65 backdrop-blur-[30px] border border-white/40 p-4 rounded-xl shadow-sm flex flex-col items-center justify-center aspect-square active:scale-95 transition-all cursor-pointer">
                   <span className="text-3xl mb-2">📦</span>
                   <span className="font-semibold text-gray-800">Pickups</span>
                </div>
                {activeStaff.role === 'Manager' && (
                  <>
                     <div className="bg-white/65 backdrop-blur-[30px] border border-white/40 p-4 rounded-xl shadow-sm flex flex-col items-center justify-center aspect-square active:scale-95 transition-all cursor-pointer">
                        <span className="text-3xl mb-2">📊</span>
                        <span className="font-semibold text-gray-800">Reports</span>
                     </div>
                     <div className="bg-white/65 backdrop-blur-[30px] border border-white/40 p-4 rounded-xl shadow-sm flex flex-col items-center justify-center aspect-square active:scale-95 transition-all cursor-pointer">
                        <span className="text-3xl mb-2">⚙️</span>
                        <span className="font-semibold text-gray-800">Settings</span>
                     </div>
                  </>
                )}
             </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-black shadow-2xl overflow-hidden flex flex-col relative border border-gray-800 rounded-[40px]">

        <div className="flex-1 flex flex-col items-center justify-center px-6">
           <h1 className="text-white text-2xl font-outfit font-bold tracking-widest mb-8">Enter PIN</h1>

           <div className="flex gap-4 mb-12">
             {[0, 1, 2, 3].map(i => (
               <div key={i} className={`w-4 h-4 rounded-full transition-all duration-200 ${pinInput.length > i ? 'bg-white' : 'border-2 border-gray-600'} ${error ? 'bg-red-500 border-red-500 animate-pulse' : ''}`} />
             ))}
           </div>

           <div className="grid grid-cols-3 gap-x-8 gap-y-6">
             {['1', '2', '3', '4', '5', '6', '7', '8', '9'].map(num => (
               <button
                 key={num}
                 onClick={() => handleKeyPress(num)}
                 className="w-20 h-20 rounded-full bg-gray-800/50 border border-gray-700 text-white text-3xl font-light hover:bg-gray-700 active:bg-gray-600 active:scale-95 transition-all flex items-center justify-center"
               >
                 {num}
               </button>
             ))}
             <div className="w-20 h-20"></div>
             <button
               onClick={() => handleKeyPress('0')}
               className="w-20 h-20 rounded-full bg-gray-800/50 border border-gray-700 text-white text-3xl font-light hover:bg-gray-700 active:bg-gray-600 active:scale-95 transition-all flex items-center justify-center"
             >
               0
             </button>
             <button
               onClick={handleBackspace}
               className="w-20 h-20 rounded-full bg-transparent text-gray-400 hover:text-white active:scale-95 transition-all flex items-center justify-center"
             >
               <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2M3 12l6.414 6.414a2 2 0 001.414.586H19a2 2 0 002-2V7a2 2 0 00-2-2h-8.172a2 2 0 00-1.414.586L3 12z" /></svg>
             </button>
           </div>
        </div>

        <div className="py-8 text-center border-t border-gray-800">
           <p className="text-gray-500 text-sm font-medium">Terminal offline sync active</p>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
