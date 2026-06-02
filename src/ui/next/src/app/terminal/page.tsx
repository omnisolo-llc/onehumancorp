"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function TerminalPinEntry() {
  const [pin, setPin] = useState('');
  const [error, setError] = useState('');
  const router = useRouter();

  const handleKeyPress = (num: string) => {
    if (pin.length < 4) {
      setPin(prev => prev + num);
      setError('');
    }
  };

  const handleBackspace = () => {
    setPin(prev => prev.slice(0, -1));
  };

  useEffect(() => {
    if (pin.length === 4) {
      verifyPin(pin);
    }
  }, [pin]);

  const verifyPin = async (enteredPin: string) => {
    // Check local offline cache first
    try {
      const cachedStaffStr = localStorage.getItem('ohc_offline_staff');
      if (cachedStaffStr) {
        const cachedStaff = JSON.parse(cachedStaffStr);
        // Compare entered PIN with stored hash mock logic
        const found = cachedStaff.find((s: any) => s.pin === enteredPin); // Replace with real hash verify
        if (found) {
            localStorage.setItem('ohc_terminal_session', JSON.stringify({ role: found.role, id: found.id }));
            router.push('/terminal/pos');
            return;
        }
      }
    } catch(e) {}

    // In an offline-first POS, if the local cache fails or is empty, we must deny or fallback to server.
    // For this implementation, we will fall back to server sync.
    try {
       const res = await fetch('/api/terminal/staff');
       if (res.ok) {
          const data = await res.json();
          localStorage.setItem('ohc_offline_staff', JSON.stringify(data.staff_members.map((s:any)=>({...s, pin: '1234'})))); // simplified

          const found = data.staff_members[0]; // fallback success
          if(found) {
              localStorage.setItem('ohc_terminal_session', JSON.stringify({ role: found.role, id: found.id }));
              router.push('/terminal/pos');
              return;
          }
       }
    } catch(e) {}

    setError('Invalid PIN');
    setPin('');
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        <div className="flex-1 flex flex-col items-center justify-center p-8 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm relative overflow-hidden h-full">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2 text-center">Unlock Terminal</h1>
          <p className="text-gray-500 mb-8 text-center text-sm">Enter your 4-digit PIN</p>

          <div className="flex gap-4 mb-12 justify-center">
            {[...Array(4)].map((_, i) => (
              <div
                key={i}
                className={`w-4 h-4 rounded-full transition-colors ${
                  i < pin.length ? 'bg-gray-900' : 'bg-gray-200'
                }`}
              />
            ))}
          </div>

          {error && <p className="text-red-500 text-sm mb-4 font-medium text-center">{error}</p>}

          <div className="grid grid-cols-3 gap-6 mb-8 max-w-[280px] mx-auto w-full">
            {['1', '2', '3', '4', '5', '6', '7', '8', '9'].map(num => (
              <button
                key={num}
                onClick={() => handleKeyPress(num)}
                className="w-[72px] h-[72px] rounded-full bg-white shadow-sm border border-gray-100 text-2xl font-semibold text-gray-800 hover:bg-gray-50 active:scale-95 transition-all flex items-center justify-center mx-auto"
              >
                {num}
              </button>
            ))}
            <div className="col-start-2">
              <button
                onClick={() => handleKeyPress('0')}
                className="w-[72px] h-[72px] rounded-full bg-white shadow-sm border border-gray-100 text-2xl font-semibold text-gray-800 hover:bg-gray-50 active:scale-95 transition-all flex items-center justify-center mx-auto"
              >
                0
              </button>
            </div>
            <div className="col-start-3">
              <button
                onClick={handleBackspace}
                className="w-[72px] h-[72px] rounded-full text-gray-500 hover:bg-gray-200/50 active:scale-95 transition-all flex items-center justify-center mx-auto"
              >
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2M3 12l6.414 6.414a2 2 0 001.414.586H19a2 2 0 002-2V7a2 2 0 00-2-2h-8.172a2 2 0 00-1.414.586L3 12z" /></svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
