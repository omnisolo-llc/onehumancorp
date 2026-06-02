"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function PosView() {
  const router = useRouter();
  const [role, setRole] = useState<string | null>(null);
  const [isClockedIn, setIsClockedIn] = useState(false);
  const [offlineQueue, setOfflineQueue] = useState<any[]>([]);

  useEffect(() => {
    const session = localStorage.getItem('ohc_terminal_session');
    if (session) {
      setRole(JSON.parse(session).role);
    } else {
      router.push('/terminal');
    }

    // Load initial offline queue state
    try {
      const q = JSON.parse(localStorage.getItem('ohc_offline_timecards') || '[]');
      setOfflineQueue(q);
    } catch (e) {}

    // Optionally handle sync online listener
    const handleOnline = () => {
       // A real implementation would trigger a sync here
       // fetch('/api/terminal/sync', { ... })
       console.log('Back online, triggering sync...');
    };
    window.addEventListener('online', handleOnline);
    return () => window.removeEventListener('online', handleOnline);
  }, [router]);

  const handleLogout = () => {
    localStorage.removeItem('ohc_terminal_session');
    router.push('/terminal');
  };

  const handleTimecard = (type: 'clock_in' | 'clock_out') => {
    const session = JSON.parse(localStorage.getItem('ohc_terminal_session') || '{}');
    const staffId = session.id;

    const event = {
      id: `evt_${Date.now()}_${Math.random().toString(36).substring(7)}`,
      staff_member_id: staffId,
      event_type: type,
      timestamp: new Date().toISOString()
    };

    // Queue offline
    const updatedQueue = [...offlineQueue, event];
    setOfflineQueue(updatedQueue);
    localStorage.setItem('ohc_offline_timecards', JSON.stringify(updatedQueue));
    setIsClockedIn(type === 'clock_in');

    alert(`Successfully ${type === 'clock_in' ? 'Clocked In' : 'Clocked Out'} (Saved Offline)`);
  };

  if (!role) return null;

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">Terminal</h1>
            <p className="text-gray-500 text-xs mt-0.5">Role: {role}</p>
          </div>
          <button
            onClick={handleLogout}
            className="text-sm font-medium text-red-600 hover:text-red-700 active:scale-95 transition-transform"
          >
            Lock
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-gray-50/50 hide-scrollbar flex flex-col gap-4">

          <div className="p-5 bg-white rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-4">
            <h2 className="text-lg font-bold font-outfit text-gray-900">Time & Attendance</h2>

            {!isClockedIn ? (
              <button
                onClick={() => handleTimecard('clock_in')}
                className="w-full py-3 bg-green-600 hover:bg-green-700 text-white rounded-xl font-semibold shadow-sm active:scale-[0.98] transition-all"
              >
                Clock In
              </button>
            ) : (
              <button
                onClick={() => handleTimecard('clock_out')}
                className="w-full py-3 bg-orange-600 hover:bg-orange-700 text-white rounded-xl font-semibold shadow-sm active:scale-[0.98] transition-all"
              >
                Clock Out
              </button>
            )}

            {offlineQueue.length > 0 && (
               <p className="text-xs text-orange-600 text-center font-medium bg-orange-50 py-2 rounded-lg">
                 {offlineQueue.length} offline records waiting to sync
               </p>
            )}
          </div>

          <div className="p-5 bg-white rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-4">
             <h2 className="text-lg font-bold font-outfit text-gray-900">Register</h2>
             <button
               className="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-xl font-semibold shadow-sm active:scale-[0.98] transition-all"
               onClick={() => alert("Open POS Register")}
             >
               New Sale
             </button>
          </div>

          {role === 'Manager' && (
             <div className="p-5 bg-white rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-4">
               <h2 className="text-lg font-bold font-outfit text-gray-900">Financial Reports</h2>
               <p className="text-sm text-gray-600">Total Sales Today: $1,245.00</p>
             </div>
          )}

        </div>
      </div>
    </div>
  );
}
