'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function SubscriptionsPage() {
  const [plans, setPlans] = useState<any[]>([]);
  const [subscribers, setSubscribers] = useState<any[]>([]);
  const [batches, setBatches] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);

  useEffect(() => {
    // Basic offline detection
    setIsOffline(!navigator.onLine);
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    const loadData = async () => {
      setLoading(true);
      try {
        if (!navigator.onLine) {
           // Load from indexedDB or local storage
           const cached = localStorage.getItem('ohc_subscriptions_cache');
           if (cached) {
               const data = JSON.parse(cached);
               setPlans(data.plans || []);
               setSubscribers(data.subscribers || []);
               setBatches(data.batches || []);
           }
        } else {
           const res = await fetch('/api/subscriptions');
           const data = await res.json();
           setPlans(data.plans || []);
           setSubscribers(data.subscribers || []);
           setBatches(data.batches || []);
           // Update cache
           localStorage.setItem('ohc_subscriptions_cache', JSON.stringify(data));
        }
      } catch (err) {
        console.error('Failed to fetch subscriptions:', err);
      } finally {
        setLoading(false);
      }
    };

    loadData();

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const handleSync = async () => {
      setSyncing(true);
      try {
          const res = await fetch('/api/subscriptions/sync-offline', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ events: [] }) // In a real app, pull pending events from local cache
          });
          if (res.ok) {
              alert('Sync complete!');
          } else {
              alert('Sync failed!');
          }
      } catch (e) {
          console.error(e);
          alert('Sync failed!');
      } finally {
          setSyncing(false);
      }
  };

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter pb-20">
      <div className="flex items-center justify-between mb-6 border-b border-gray-200 pb-4">
        <div className="flex items-center">
            <Link href="/dashboard" className="text-blue-500 font-semibold mr-4">&lt; Back</Link>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Subscriptions</h1>
        </div>
        {isOffline && <span className="text-xs font-bold px-2 py-1 rounded-full bg-red-100 text-red-700">Offline Mode</span>}
      </div>

      <div className="mb-6 flex justify-end">
          <button
              onClick={handleSync}
              disabled={isOffline || syncing}
              className={`px-4 py-2 rounded-xl font-bold text-sm shadow-sm transition-colors ${
                  isOffline ? 'bg-gray-200 text-gray-400' : 'bg-blue-600 text-white hover:bg-blue-700'
              }`}
          >
              {syncing ? 'Syncing...' : 'Force Sync'}
          </button>
      </div>

      <div className="mb-6">
        <h2 className="text-lg font-bold text-gray-900 mb-3">Active Plans</h2>
        {plans.map(p => (
          <div key={p.id} className="p-4 rounded-xl shadow-sm mb-3" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h3 className="font-bold text-gray-900">{p.name}</h3>
            <p className="text-sm text-gray-500">${(p.price_cents / 100).toFixed(2)} / {p.frequency}</p>
          </div>
        ))}
      </div>

      <div className="mb-6">
        <h2 className="text-lg font-bold text-gray-900 mb-3">Subscribers ({subscribers.length})</h2>
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
          {subscribers.map((s, i) => (
            <div key={s.id} className={`p-4 flex justify-between items-center ${i !== subscribers.length - 1 ? 'border-b border-gray-100' : ''}`}>
              <span className="font-medium text-gray-800">Customer #{s.customer_id.substring(0,6)}</span>
              <span className="text-xs font-bold px-2 py-1 rounded-full bg-green-100 text-green-700">{s.status}</span>
            </div>
          ))}
        </div>
      </div>

      <div>
        <h2 className="text-lg font-bold text-gray-900 mb-3">Upcoming Fulfillments</h2>
        {batches.map(b => (
          <div key={b.id} className="p-4 rounded-xl shadow-sm mb-3" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <div className="flex justify-between items-start mb-2">
              <h3 className="font-bold text-gray-900">Ship on {b.fulfillment_date}</h3>
              <span className="text-xs font-bold px-2 py-1 rounded-full bg-blue-100 text-blue-700">{b.subscriber_count} boxes</span>
            </div>
            <button
              className="w-full mt-2 py-2 bg-gray-900 text-white rounded-lg font-bold shadow-sm hover:bg-black transition-colors text-sm"
              onClick={() => alert('Printing labels...')}
            >
              Print Labels
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
