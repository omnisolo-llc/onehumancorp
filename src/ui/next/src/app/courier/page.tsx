'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

interface DeliveryJob {
  id: string;
  order_id: string;
  payout_cents: number;
  pickup_location_lat: number;
  pickup_location_lng: number;
  delivery_location_lat: number;
  delivery_location_lng: number;
}

export default function CourierJobList() {
  const [jobs, setJobs] = useState<DeliveryJob[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/v1/delivery/jobs?organization_id=test-org', {
      method: 'GET',
    })
      .then(res => {
        if (!res.ok) {
           return [];
        }
        return res.json();
      })
      .then((data: any) => {
        if (data && data.jobs) {
          setJobs(data.jobs);
        }
      })
      .catch(err => console.error("Failed to fetch jobs:", err))
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 pb-20">
      <header className="bg-white/80 backdrop-blur-md sticky top-0 z-10 border-b border-gray-200 px-4 py-4 flex justify-between items-center shadow-sm">
        <h1 className="text-xl font-bold text-gray-900 font-[Outfit]">Available Jobs</h1>
        <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 font-bold">C</div>
      </header>

      <main className="p-4 max-w-[375px] mx-auto">
        {loading ? (
          <div className="flex justify-center mt-10">
            <div className="animate-spin h-6 w-6 border-2 border-blue-500 rounded-full border-t-transparent"></div>
          </div>
        ) : jobs.length === 0 ? (
          <div className="text-center mt-10 text-gray-500 font-[Inter]">No jobs available right now.</div>
        ) : (
          <div className="space-y-4">
            {jobs.map((job) => (
              <Link key={job.id} href={`/courier/${job.id}`}>
                <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-4 active:scale-[0.98] transition-transform">
                  <div className="flex justify-between items-start mb-2">
                    <span className="text-xs font-semibold px-2 py-1 bg-green-100 text-green-700 rounded-md">New</span>
                    <span className="text-lg font-bold text-gray-900">${(job.payout_cents / 100).toFixed(2)}</span>
                  </div>
                  <div className="text-sm text-gray-600 font-[Inter] mb-1">
                    <span className="font-medium">Pickup:</span> Lat {job.pickup_location_lat.toFixed(4)}, Lng {job.pickup_location_lng.toFixed(4)}
                  </div>
                  <div className="text-sm text-gray-600 font-[Inter]">
                    <span className="font-medium">Dropoff:</span> Lat {job.delivery_location_lat.toFixed(4)}, Lng {job.delivery_location_lng.toFixed(4)}
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
