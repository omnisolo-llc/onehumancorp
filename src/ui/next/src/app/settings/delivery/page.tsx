"use client";
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { WithTooltip } from '../../../../components/TooltipRegistry';

export default function DeliverySettingsPage() {
  const router = useRouter();

  const [enabled, setEnabled] = useState(false);
  const [radius, setRadius] = useState(5);
  const [fee, setFee] = useState(7.50);
  const [prepTime, setPrepTime] = useState(15);
  const [isSaving, setIsSaving] = useState(false);
  const [isSaved, setIsSaved] = useState(false);

  useEffect(() => {
    // Load from local storage for mock persistence
    if (typeof localStorage !== 'undefined') {
      const stored = localStorage.getItem('ohc_delivery_settings');
      if (stored) {
        const parsed = JSON.parse(stored);
        setEnabled(parsed.enabled ?? false);
        setRadius(parsed.radius ?? 5);
        setFee(parsed.fee ?? 7.50);
        setPrepTime(parsed.prepTime ?? 15);
      }
    }
  }, []);

  const handleSave = () => {
    setIsSaving(true);
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('ohc_delivery_settings', JSON.stringify({
        enabled,
        radius,
        fee,
        prepTime
      }));
    }
    setTimeout(() => {
      setIsSaving(false);
      setIsSaved(true);
      setTimeout(() => setIsSaved(false), 2000);
    }, 600);
  };

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4">
          <button onClick={() => router.push('/settings')} className="text-gray-500 hover:text-gray-900">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </button>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Local Delivery Settings</h1>
        </div>
      </header>

      <main className="p-6 md:p-8 max-w-3xl mx-auto w-full flex flex-col gap-6">
        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 space-y-6">
          <div className="flex items-start justify-between border-b border-gray-100 pb-6">
            <div>
              <h2 className="text-xl font-bold font-outfit text-gray-900">DoorDash Drive White-Label</h2>
              <p className="text-sm text-gray-500 mt-1 max-w-lg">
                Automatically dispatch local Dashers to deliver orders to your customers. Customers order on your website, DoorDash handles the delivery. You keep your customer data and pay zero commission.
              </p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" className="sr-only peer" checked={enabled} onChange={(e) => setEnabled(e.target.checked)} />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
            </label>
          </div>

          <div className={`space-y-6 transition-opacity ${enabled ? 'opacity-100' : 'opacity-50 pointer-events-none'}`}>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
               <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Delivery Radius (Miles)</label>
                <div className="flex items-center gap-2">
                  <input
                    type="range"
                    min="1"
                    max="15"
                    step="1"
                    value={radius}
                    onChange={(e) => setRadius(parseInt(e.target.value))}
                    className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                  />
                  <span className="text-sm font-medium text-gray-900 w-12 text-right">{radius} mi</span>
                </div>
                <p className="text-xs text-gray-500 mt-2">Maximum distance customers can be from your location to request delivery.</p>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Delivery Fee Charged to Customer</label>
                <div className="relative">
                  <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    <span className="text-gray-500 sm:text-sm">$</span>
                  </div>
                  <input
                    type="number"
                    min="0"
                    step="0.5"
                    value={fee}
                    onChange={(e) => setFee(parseFloat(e.target.value))}
                    className="border border-gray-300 rounded-lg pl-7 pr-3 py-2 w-full text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                  />
                </div>
                <p className="text-xs text-gray-500 mt-2">DoorDash charges a flat ~$7.00 per delivery. You can subsidize this or charge a markup.</p>
              </div>
            </div>

            <div>
               <label className="block text-sm font-semibold text-gray-700 mb-2">Default Prep Time (Minutes)</label>
               <input
                 type="number"
                 min="0"
                 step="5"
                 value={prepTime}
                 onChange={(e) => setPrepTime(parseInt(e.target.value))}
                 className="border border-gray-300 rounded-lg px-3 py-2 w-full max-w-[200px] text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500"
               />
               <p className="text-xs text-gray-500 mt-2">How long it takes you to prepare an order. We'll automatically request the Dasher to arrive after this time.</p>
            </div>
          </div>

          <div className="pt-4 border-t border-gray-100 flex justify-end">
            <button
              onClick={handleSave}
              className="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-6 rounded-lg transition-colors flex items-center gap-2"
            >
              {isSaving ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                  Saving...
                </>
              ) : isSaved ? (
                <>
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  Saved!
                </>
              ) : (
                'Save Settings'
              )}
            </button>
          </div>
        </div>
      </main>

       <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
