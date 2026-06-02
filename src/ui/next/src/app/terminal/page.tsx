"use client";

import React, { useState, useEffect } from 'react';

export default function TerminalPage() {
  const [pin, setPin] = useState('');
  const [unlocked, setUnlocked] = useState(false);
  const [role, setRole] = useState<string | null>(null);
  const [staffName, setStaffName] = useState<string | null>(null);
  const [isClockedIn, setIsClockedIn] = useState(false);
  const [offlineQueue, setOfflineQueue] = useState<any[]>([]);
  const [isOnline, setIsOnline] = useState(true);

  useEffect(() => {
    // Check local storage for cached credentials
    const cachedStaff = localStorage.getItem('ohc_terminal_staff');
    if (cachedStaff) {
      // In a real implementation we would just use this to know *who* can log in
    }

    const checkOnlineStatus = () => {
      setIsOnline(navigator.onLine);
      if (navigator.onLine) {
        syncOfflineQueue();
      }
    };

    window.addEventListener('online', checkOnlineStatus);
    window.addEventListener('offline', checkOnlineStatus);

    return () => {
      window.removeEventListener('online', checkOnlineStatus);
      window.removeEventListener('offline', checkOnlineStatus);
    };
  }, []);

  const syncOfflineQueue = async () => {
    const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    if (queue.length === 0) return;

    try {
      // In a real app we would sync this to the backend
      const res = await fetch('/api/v1/staff/clock', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(queue[0]) // just sending one for demo
      });
      if (res.ok) {
        localStorage.setItem('ohc_offline_queue', '[]');
        setOfflineQueue([]);
      }
    } catch (e) {
      console.error('Failed to sync offline queue', e);
    }
  };

  const handlePinInput = (num: string) => {
    if (pin.length < 4) {
      const newPin = pin + num;
      setPin(newPin);

      if (newPin.length === 4) {
        // Mock verification
        if (newPin === '1234') {
          setTimeout(() => {
            setUnlocked(true);
            setRole('Cashier');
            setStaffName('Sarah');
          }, 300);
        } else {
          setTimeout(() => setPin(''), 500); // Reset on fail
        }
      }
    }
  };

  const handleClockToggle = () => {
    const newStatus = !isClockedIn;
    setIsClockedIn(newStatus);

    const event = {
      team_member_id: 'mock-id',
      event_type: newStatus ? 'CLOCK_IN' : 'CLOCK_OUT',
      client_timestamp: new Date().toISOString(),
      device_id: 'terminal-1'
    };

    if (!isOnline) {
      const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      queue.push(event);
      localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
      setOfflineQueue(queue);
    } else {
      fetch('/api/v1/staff/clock', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(event)
      }).catch(e => console.error(e));
    }
  };

  if (!unlocked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white font-inter">
        <div className="w-[375px] max-w-[375px] h-[812px] bg-black flex flex-col items-center pt-32 pb-16 relative">

          <div className="absolute top-4 right-4 flex items-center gap-2">
            {!isOnline && <span className="text-orange-500 text-xs font-bold">OFFLINE</span>}
            <div className={`w-3 h-3 rounded-full ${isOnline ? 'bg-green-500' : 'bg-orange-500'}`}></div>
          </div>

          <h1 className="text-2xl font-outfit text-gray-300 mb-8">Enter PIN to unlock</h1>

          <div className="flex gap-4 mb-16">
            {[0, 1, 2, 3].map(i => (
              <div key={i} className={`w-4 h-4 rounded-full border-2 ${pin.length > i ? 'bg-white border-white' : 'border-gray-500'}`}></div>
            ))}
          </div>

          <div className="grid grid-cols-3 gap-6 w-64">
            {['1', '2', '3', '4', '5', '6', '7', '8', '9'].map(num => (
              <button
                key={num}
                onClick={() => handlePinInput(num)}
                className="w-16 h-16 rounded-full bg-gray-800 text-3xl font-light hover:bg-gray-700 active:bg-gray-600 transition-colors flex items-center justify-center"
              >
                {num}
              </button>
            ))}
            <div></div>
            <button
              onClick={() => handlePinInput('0')}
              className="w-16 h-16 rounded-full bg-gray-800 text-3xl font-light hover:bg-gray-700 active:bg-gray-600 transition-colors flex items-center justify-center"
            >
              0
            </button>
            <button
              onClick={() => setPin(pin.slice(0, -1))}
              className="w-16 h-16 rounded-full text-gray-400 hover:text-white flex items-center justify-center"
            >
              <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2M3 12l6.414 6.414a2 2 0 001.414.586H19a2 2 0 002-2V7a2 2 0 00-2-2h-8.172a2 2 0 00-1.414.586L3 12z"></path></svg>
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 font-inter">
      <div className="w-[375px] max-w-[375px] h-[812px] bg-white flex flex-col relative shadow-2xl overflow-hidden border-x border-gray-200">

        {/* Header */}
        <div className="bg-white/80 backdrop-blur-md pt-12 pb-4 px-6 border-b border-gray-100 flex justify-between items-center z-10">
          <div>
            <h1 className="text-xl font-bold text-gray-900">Point of Sale</h1>
            <p className="text-sm text-gray-500">Logged in as {staffName} ({role})</p>
          </div>
          <button
            onClick={() => { setUnlocked(false); setPin(''); }}
            className="p-2 bg-gray-100 hover:bg-gray-200 rounded-full text-gray-600"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path></svg>
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6 bg-gray-50">

          <div className="bg-white rounded-2xl p-6 shadow-sm border border-gray-100 mb-6 flex flex-col items-center justify-center">
            <h2 className="text-lg font-semibold text-gray-800 mb-4">Shift Status</h2>
            <button
              onClick={handleClockToggle}
              className={`w-40 h-40 rounded-full flex flex-col items-center justify-center text-white shadow-lg transition-all active:scale-95 ${isClockedIn ? 'bg-red-500 hover:bg-red-600 shadow-red-500/30' : 'bg-green-500 hover:bg-green-600 shadow-green-500/30'}`}
            >
              <span className="text-2xl font-bold font-outfit">{isClockedIn ? 'CLOCK OUT' : 'CLOCK IN'}</span>
              <span className="text-sm opacity-80 mt-1">{new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
            </button>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 flex flex-col items-center justify-center h-32 active:scale-95 transition-transform">
              <span className="text-3xl mb-2">🛒</span>
              <span className="font-medium text-gray-800">Checkout</span>
            </div>
            <div className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 flex flex-col items-center justify-center h-32 active:scale-95 transition-transform">
              <span className="text-3xl mb-2">📦</span>
              <span className="font-medium text-gray-800">Orders</span>
            </div>
          </div>

          {/* Role-based restrictions demo */}
          {role === 'Manager' && (
            <div className="mt-4 grid grid-cols-1 gap-4">
              <div className="bg-blue-50 p-4 rounded-2xl border border-blue-100 flex items-center justify-between">
                <div>
                  <h3 className="font-semibold text-blue-900">Financial Reports</h3>
                  <p className="text-xs text-blue-700">Manager access only</p>
                </div>
                <span className="text-xl">📊</span>
              </div>
            </div>
          )}

          {!isOnline && (
            <div className="mt-6 bg-orange-50 border border-orange-200 rounded-xl p-4 flex items-start gap-3">
              <svg className="w-5 h-5 text-orange-500 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
              <div>
                <p className="text-sm font-semibold text-orange-800">Offline Mode Active</p>
                <p className="text-xs text-orange-600 mt-1">Clock events and transactions are being saved locally and will sync when reconnected.</p>
                {offlineQueue.length > 0 && (
                  <p className="text-xs font-bold text-orange-700 mt-2">{offlineQueue.length} pending events</p>
                )}
              </div>
            </div>
          )}

        </div>

      </div>
    </div>
  );
}
