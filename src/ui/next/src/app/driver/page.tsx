'use client';
import { useState, useEffect } from 'react';
import Head from 'next/head';

export default function DriverApp() {
  const [activeRoute, setActiveRoute] = useState<any>(null);

  return (
    <div className="flex flex-col h-screen bg-[#1D1D1F] font-inter text-white">
      <Head>
        <title>Driver - OHC</title>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0" />
      </Head>

      <header className="px-4 py-4 flex items-center justify-between border-b border-gray-800 bg-black/50 backdrop-blur-md sticky top-0 z-50">
        <h1 className="text-xl font-bold font-outfit">OHC Driver</h1>
        <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-full bg-green-500"></div>
            <span className="text-sm text-gray-300">Online</span>
        </div>
      </header>

      <main className="flex-1 overflow-y-auto p-4 space-y-4">
        {!activeRoute ? (
            <div className="flex flex-col items-center justify-center h-full text-center space-y-4">
                <div className="text-6xl">📍</div>
                <h2 className="text-2xl font-semibold font-outfit">Waiting for routes...</h2>
                <p className="text-gray-400">Stay online. We'll notify you when a delivery batch is ready.</p>
            </div>
        ) : (
            <div>
                {/* Active route view will go here */}
            </div>
        )}
      </main>
    </div>
  );
}
