"use client";

import React, { useState, useEffect } from 'react';

export function OfflineStatus() {
  const [isOffline, setIsOffline] = useState(false);

  useEffect(() => {
    setIsOffline(!navigator.onLine);

    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  if (!isOffline) return null;

  return (
    <div className="fixed top-0 left-0 right-0 z-[100] flex justify-center pointer-events-none mt-4">
      <div className="pointer-events-auto bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-3xl saturate-[2.1] border border-white/40 dark:border-white/10 shadow-[0_4px_24px_rgba(0,0,0,0.08)] px-4 py-2 rounded-full flex items-center gap-2 transform transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
        <span className="w-2 h-2 rounded-full bg-orange-500 animate-pulse"></span>
        <span className="text-sm font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Working Offline</span>
      </div>
    </div>
  );
}
