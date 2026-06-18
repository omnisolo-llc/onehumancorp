'use client';

import React, { useState, useEffect } from 'react';

export default function P2PJoinPrompt() {
  const [showPrompt, setShowPrompt] = useState(false);
  const [detectedHost, setDetectedHost] = useState<string | null>(null);

  useEffect(() => {
    // Simulate detecting a nearby local register via Bluetooth LE/mDNS
    const timer = setTimeout(() => {
      setDetectedHost("Fatima's Register");
      setShowPrompt(true);
    }, 3000);
    return () => clearTimeout(timer);
  }, []);

  const handleJoin = () => {
    setShowPrompt(false);
    // In a real app, this would call the rust backend via Tauri commands to join the mesh
    console.log("Joined P2P mesh register network");
    // Show a temporary success message
    alert("Successfully joined the local register network! Inventory is now synchronized.");
  };

  if (!showPrompt) return null;

  return (
    <div className="fixed inset-x-0 bottom-0 p-4 z-50">
      <div className="bg-white/90 backdrop-blur-md border border-gray-200 rounded-2xl shadow-xl p-5 w-full max-w-sm mx-auto flex flex-col gap-3 transition-all duration-300 transform translate-y-0">
        <div className="flex items-start gap-3">
          <div className="bg-indigo-100 text-indigo-600 rounded-full p-2 mt-1">
             <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </div>
          <div>
            <h3 className="font-bold text-gray-900 text-[15px]">Nearby Register Detected</h3>
            <p className="text-gray-600 text-sm mt-0.5 leading-relaxed">
              We found <strong>{detectedHost}</strong> nearby. Join their local network to sync offline inventory instantly without internet.
            </p>
          </div>
        </div>
        <div className="flex gap-2 mt-2">
          <button
            onClick={() => setShowPrompt(false)}
            className="flex-1 py-2 px-3 text-sm font-medium text-gray-700 bg-gray-100 rounded-xl hover:bg-gray-200 transition-colors"
          >
            Dismiss
          </button>
          <button
            onClick={handleJoin}
            className="flex-1 py-2 px-3 text-sm font-bold text-white bg-indigo-600 rounded-xl shadow-sm hover:bg-indigo-700 transition-colors shadow-indigo-200"
          >
            Join Network
          </button>
        </div>
      </div>
    </div>
  );
}
