"use client";

import React, { useState, useEffect } from 'react';

export default function TerminalPage() {
  const [pin, setPin] = useState('');
  const [unlocked, setUnlocked] = useState(false);
  const [role, setRole] = useState<string | null>(null);
  const [staffName, setStaffName] = useState<string | null>(null);
  const [clockedIn, setClockedIn] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    // Load staff state from local storage to support offline
    const staffStateStr = localStorage.getItem('ohc_staff_state');
    if (staffStateStr) {
      const staffState = JSON.parse(staffStateStr);
      setRole(staffState.role);
      setStaffName(staffState.name);
    }
  }, []);

  const handlePinSubmit = async () => {
    setLoading(true);
    try {
      // First try to authenticate online
      const response = await fetch('/api/staff', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'verify_pin', pin })
      });

      if (response.ok) {
        const data = await response.json();
        setUnlocked(true);
        setRole(data.user.role);
        setStaffName(data.user.name);

        // Cache credentials for offline use
        localStorage.setItem('ohc_staff_state', JSON.stringify({
          role: data.user.role,
          name: data.user.name,
          pin_hash: pin // In real app, this would be a proper hash, not plaintext
        }));
      } else {
        // Fallback to offline cache if online fails
        const staffStateStr = localStorage.getItem('ohc_staff_state');
        if (staffStateStr) {
           const staffState = JSON.parse(staffStateStr);
           if (staffState.pin_hash === pin) {
             setUnlocked(true);
             setRole(staffState.role);
             setStaffName(staffState.name);
           } else {
             alert('Invalid PIN');
             setPin('');
           }
        } else {
          alert('Invalid PIN and no offline cache available');
          setPin('');
        }
      }
    } catch (error) {
       // Network error, try offline
       const staffStateStr = localStorage.getItem('ohc_staff_state');
        if (staffStateStr) {
           const staffState = JSON.parse(staffStateStr);
           if (staffState.pin_hash === pin) {
             setUnlocked(true);
             setRole(staffState.role);
             setStaffName(staffState.name);
           } else {
             alert('Invalid PIN');
             setPin('');
           }
        } else {
          alert('Network offline and no credentials cached');
          setPin('');
        }
    } finally {
      setLoading(false);
    }
  };

  const handleClockIn = () => {
    setClockedIn(true);
    // Queue timecard event for offline sync
    const events = JSON.parse(localStorage.getItem('ohc_timecard_events') || '[]');
    events.push({
      id: crypto.randomUUID(),
      staff_id: 'staff-1',
      event_type: 'CLOCK_IN',
      event_time: new Date().toISOString()
    });
    localStorage.setItem('ohc_timecard_events', JSON.stringify(events));
  };

  const handleClockOut = () => {
    setClockedIn(false);
    // Queue timecard event for offline sync
    const events = JSON.parse(localStorage.getItem('ohc_timecard_events') || '[]');
    events.push({
      id: crypto.randomUUID(),
      staff_id: 'staff-1',
      event_type: 'CLOCK_OUT',
      event_time: new Date().toISOString()
    });
    localStorage.setItem('ohc_timecard_events', JSON.stringify(events));
    setUnlocked(false);
    setPin('');
  };

  if (!unlocked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter">
        <div className="w-[375px] max-w-[375px] min-h-[812px] bg-black text-white shadow-2xl flex flex-col justify-center relative border-x border-gray-800 p-8">
          <h1 className="text-2xl font-outfit text-center mb-8">Enter your PIN to unlock</h1>
          <div className="text-4xl text-center tracking-[1em] mb-12 h-12 flex items-center justify-center">
            {pin.padEnd(4, '•').substring(0, 4)}
          </div>
          <div className="grid grid-cols-3 gap-6 mb-12">
            {[1, 2, 3, 4, 5, 6, 7, 8, 9].map(num => (
              <button
                key={num}
                onClick={() => setPin(prev => prev.length < 4 ? prev + num : prev)}
                className="w-20 h-20 bg-gray-800 rounded-full text-3xl flex items-center justify-center active:bg-gray-700 mx-auto"
              >
                {num}
              </button>
            ))}
            <div></div>
            <button
              onClick={() => setPin(prev => prev.length < 4 ? prev + '0' : prev)}
              className="w-20 h-20 bg-gray-800 rounded-full text-3xl flex items-center justify-center active:bg-gray-700 mx-auto"
            >
              0
            </button>
            <button
              onClick={() => setPin(prev => prev.slice(0, -1))}
              className="w-20 h-20 bg-gray-800 rounded-full text-2xl flex items-center justify-center active:bg-gray-700 mx-auto"
            >
              ⌫
            </button>
          </div>
          {pin.length === 4 && (
            <button
              onClick={handlePinSubmit}
              disabled={loading}
              className={`w-full py-4 rounded-full text-xl font-bold transition-colors ${loading ? 'bg-blue-800' : 'bg-blue-600 active:bg-blue-700'}`}
            >
              {loading ? 'Unlocking...' : 'Unlock'}
            </button>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl flex flex-col relative border-x border-gray-200">
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex justify-between items-center">
           <div>
             <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Point of Sale</h1>
             <p className="text-gray-500 text-sm mt-1">{staffName || role} • {role}</p>
           </div>
           <div className={`w-3 h-3 rounded-full ${clockedIn ? 'bg-green-500' : 'bg-gray-400'}`}></div>
        </div>

        <div className="flex-1 px-4 py-6">
          <div className="grid grid-cols-2 gap-4">
            <button className="h-32 bg-white/65 backdrop-blur-[30px] border border-white/40 rounded-2xl shadow-sm text-lg font-semibold text-gray-800 active:scale-95 transition-transform">
              New Sale
            </button>
            <button className="h-32 bg-white/65 backdrop-blur-[30px] border border-white/40 rounded-2xl shadow-sm text-lg font-semibold text-gray-800 active:scale-95 transition-transform">
              Orders
            </button>
            {role !== 'Cashier' && (
              <button className="h-32 bg-white/65 backdrop-blur-[30px] border border-white/40 rounded-2xl shadow-sm text-lg font-semibold text-gray-800 active:scale-95 transition-transform">
                Reports
              </button>
            )}
            {role !== 'Cashier' && (
              <button className="h-32 bg-white/65 backdrop-blur-[30px] border border-white/40 rounded-2xl shadow-sm text-lg font-semibold text-gray-800 active:scale-95 transition-transform">
                Settings
              </button>
            )}
          </div>
        </div>

        <div className="p-6 border-t border-gray-200 bg-white/65 backdrop-blur-[30px]">
           {clockedIn ? (
             <button
               onClick={handleClockOut}
               className="w-full py-4 bg-red-600 text-white rounded-xl text-lg font-bold shadow-md active:bg-red-700 transition-colors"
             >
               Clock Out & Lock
             </button>
           ) : (
             <button
               onClick={handleClockIn}
               className="w-full py-4 bg-green-600 text-white rounded-xl text-lg font-bold shadow-md active:bg-green-700 transition-colors"
             >
               Clock In
             </button>
           )}
        </div>
      </div>
    </div>
  );
}
