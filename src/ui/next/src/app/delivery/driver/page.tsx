"use client";

import { useState } from 'react';

export default function DriverApp() {
  const [status, setStatus] = useState('in_transit');
  const [photoUploaded, setPhotoUploaded] = useState(false);

  const handleMarkDelivered = () => {
    setStatus('delivered');
    alert('Order marked as delivered. Syncing with dashboard...');
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter max-w-[375px] mx-auto border-x border-gray-200 relative overflow-hidden">
      {/* Map Header */}
      <div className="h-[50vh] bg-blue-50 relative flex items-center justify-center">
        <div className="absolute inset-0 opacity-50 bg-[url('https://maps.gstatic.com/mapfiles/maps_lite/images/2x/map.png')] bg-cover"></div>
        <div className="z-10 bg-white px-4 py-2 rounded-full shadow-sm font-medium text-sm text-gray-900 border border-gray-100 flex items-center gap-2">
          <div className="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></div>
          Navigating to Stop 1
        </div>
      </div>

      {/* Swipe Up Drawer (simulated) */}
      <div className="flex-1 bg-white rounded-t-3xl -mt-6 z-20 shadow-[0_-8px_30px_rgba(0,0,0,0.05)] p-6 flex flex-col justify-between">
        <div>
          <div className="w-12 h-1 bg-gray-200 rounded-full mx-auto mb-6"></div>
          <div className="flex justify-between items-start mb-2">
            <h2 className="text-2xl font-bold font-outfit text-gray-900">Drop off</h2>
            <span className="bg-green-100 text-green-700 px-2 py-1 rounded text-xs font-bold uppercase tracking-wider">Stop 1 of 3</span>
          </div>
          <p className="text-gray-500 mb-6">Vegan Chocolate Cake for Sarah</p>

          <div className="p-4 bg-gray-50 rounded-xl border border-gray-100 mb-6">
            <p className="font-medium text-gray-900">123 Main St</p>
            <p className="text-sm text-gray-500">Apartment 4B. Leave at door.</p>
          </div>
        </div>

        <div className="space-y-3">
          {status === 'in_transit' && (
            <>
              <button className="w-full py-4 bg-black text-white rounded-xl font-bold hover:bg-gray-800 transition-colors">
                Navigate
              </button>
              <button
                onClick={handleMarkDelivered}
                className="w-full py-4 bg-white text-gray-900 border-2 border-gray-200 rounded-xl font-bold hover:bg-gray-50 transition-colors flex items-center justify-center gap-2"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
                Mark Delivered
              </button>
            </>
          )}

          {status === 'delivered' && (
            <div className="w-full py-4 bg-green-100 text-green-800 rounded-xl font-bold text-center flex items-center justify-center gap-2">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
              Delivered Successfully
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
