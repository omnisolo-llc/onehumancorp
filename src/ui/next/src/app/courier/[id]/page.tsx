'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import { useParams } from 'next/navigation';

export default function CourierJobDetail() {
  const params = useParams();
  const jobId = params?.id as string;
  const [status, setStatus] = useState<'AVAILABLE' | 'CLAIMED' | 'DELIVERED'>('AVAILABLE');
  const [loading, setLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(false);
  const [payout, setPayout] = useState(0);

  useEffect(() => {
    fetch(`/api/v1/delivery/jobs?organization_id=test-org`)
      .then(res => res.json())
      .then(data => {
        const job = data?.jobs?.find((j: any) => j.id === jobId);
        if (job) {
           setStatus(job.status as any);
           setPayout(job.payout_cents);
        }
      })
      .finally(() => setLoading(false));
  }, [jobId]);

  const handleAction = async () => {
    setActionLoading(true);
    const orgId = "test-org";
    const courierId = "test-courier-id";

    try {
      if (status === 'AVAILABLE') {
        const res = await fetch('/api/v1/delivery/jobs/claim', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ organization_id: orgId, job_id: jobId, courier_id: courierId })
        });
        if (res.ok) setStatus('CLAIMED');
      } else if (status === 'CLAIMED') {
        const res = await fetch('/api/v1/delivery/jobs/deliver', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ organization_id: orgId, job_id: jobId, courier_id: courierId })
        });
        if (res.ok) setStatus('DELIVERED');
      }
    } catch (err) {
      console.error(err);
    } finally {
      setActionLoading(false);
    }
  };

  if (loading) {
     return <div className="min-h-screen bg-gray-50 flex justify-center items-center">Loading...</div>;
  }

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col max-w-[375px] mx-auto relative">
      <header className="bg-white/80 backdrop-blur-md sticky top-0 z-10 border-b border-gray-200 px-4 py-4 flex items-center shadow-sm">
        <Link href="/courier" className="mr-3 text-blue-600">
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
        </Link>
        <h1 className="text-xl font-bold text-gray-900 font-[Outfit]">Job Details</h1>
      </header>

      <main className="p-4 flex-1">
        <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-5 mb-4">
          <h2 className="text-2xl font-bold text-gray-900 mb-1">${(payout / 100).toFixed(2)}</h2>
          <p className="text-gray-500 text-sm font-[Inter] mb-4">Delivery Task</p>

          <div className="relative pl-6 border-l-2 border-gray-200 space-y-6">
            <div className="relative">
              <div className="absolute -left-[31px] bg-white rounded-full p-1 border-2 border-blue-500 w-4 h-4"></div>
              <h3 className="font-bold text-gray-900">Pickup</h3>
              <p className="text-gray-600 text-sm font-[Inter]">Merchant Location</p>
            </div>
            <div className="relative">
              <div className="absolute -left-[31px] bg-white rounded-full p-1 border-2 border-green-500 w-4 h-4"></div>
              <h3 className="font-bold text-gray-900">Dropoff</h3>
              <p className="text-gray-600 text-sm font-[Inter]">Customer Location</p>
            </div>
          </div>
        </div>

        {status === 'DELIVERED' && (
          <div className="bg-green-50 border border-green-200 rounded-xl p-4 text-center">
            <h3 className="text-green-800 font-bold mb-1">Delivered!</h3>
            <p className="text-green-600 text-sm">Payout is being processed.</p>
          </div>
        )}
      </main>

      {status !== 'DELIVERED' && (
        <div className="p-4 bg-white border-t border-gray-200 pb-8">
          <button
            onClick={handleAction}
            disabled={actionLoading}
            className={`w-full py-4 rounded-xl font-bold text-white shadow-md transition-transform active:scale-[0.98] ${
              status === 'AVAILABLE' ? 'bg-blue-600 hover:bg-blue-700' : 'bg-green-600 hover:bg-green-700'
            }`}
          >
            {actionLoading ? 'Processing...' : status === 'AVAILABLE' ? 'Claim Job' : 'Mark Delivered'}
          </button>
        </div>
      )}
    </div>
  );
}
