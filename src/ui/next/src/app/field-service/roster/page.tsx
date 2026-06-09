"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../../components/AppShell";
import Link from "next/link";

interface Job {
  id: string;
  customer_name: string;
  service_requested: string;
  status: string;
  scheduled_at: string;
  location?: string;
  notes?: string;
}

export default function FieldServiceRosterPage() {
  const [jobs, setJobs] = useState<Job[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [syncStatus, setSyncStatus] = useState<string | null>(null);
  const [draftEstimates, setDraftEstimates] = useState<string[]>([]);

  const fetchJobs = async () => {
    try {
      const res = await fetch("/api/v1/field-service/roster", {
        headers: { "x-tenant-id": "default" },
      });
      if (res.ok) {
        const data = await res.json();
        setJobs(data.jobs || []);
        localStorage.setItem("field_service_jobs", JSON.stringify(data.jobs || []));

        // Check estimates
        for (const job of data.jobs || []) {
          if (job.status === "COMPLETED") {
             const checkRes = await fetch(`/api/v1/field-service/jobs/${job.id}/estimate-ready`, {
               headers: { "x-tenant-id": "default" }
             });
             if (checkRes.ok) {
               const checkData = await checkRes.json();
               if (checkData.is_ready) {
                 setDraftEstimates((prev) => Array.from(new Set([...prev, job.id])));
               }
             }
          }
        }

      }
    } catch (err) {
      console.error("Failed to fetch jobs:", err);
      // Fallback to local storage if API fails (e.g. offline)
      const localJobs = localStorage.getItem("field_service_jobs");
      if (localJobs) setJobs(JSON.parse(localJobs));
    }
  };

  const syncOfflineQueue = async () => {
    const queueStr = localStorage.getItem("offline_sync_queue");
    if (!queueStr) return;
    const queue = JSON.parse(queueStr);
    if (queue.length === 0) return;

    setSyncStatus("Syncing...");
    try {
      const res = await fetch("/api/v1/field-service/sync", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": "default",
        },
        body: JSON.stringify({ mutations: queue }),
      });
      if (res.ok) {
        localStorage.removeItem("offline_sync_queue");
        setSyncStatus("Synced successfully.");
        setTimeout(() => setSyncStatus(null), 3000);
        await fetchJobs();
      } else {
        setSyncStatus("Sync failed. Will retry later.");
      }
    } catch (err) {
      console.error("Sync error:", err);
      setSyncStatus("Offline. Changes queued.");
    }
  };

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const handleOnline = () => {
      setIsOffline(false);
      syncOfflineQueue();
    };
    const handleOffline = () => setIsOffline(true);

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    // Initial load
    if (navigator.onLine) {
      syncOfflineQueue().then(fetchJobs);
    } else {
      const localJobs = localStorage.getItem("field_service_jobs");
      if (localJobs) setJobs(JSON.parse(localJobs));
    }

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  return (
    <AppShell
      title="Daily Roster"
      subtitle="Today's AI-optimized service route."
    >
      <div className="p-4 max-w-lg mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">Today's Jobs</h1>
          {isOffline && (
            <div className="flex items-center text-orange-500 font-semibold text-sm">
              <svg className="w-5 h-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 15l-6-6m0 0l6-6m-6 6h12" /></svg>
              Offline Mode
            </div>
          )}
        </div>

        {syncStatus && (
          <div className="mb-4 p-3 bg-blue-50 text-blue-800 rounded-md text-sm">
            {syncStatus}
          </div>
        )}

        <div className="space-y-4">
          {jobs.length === 0 ? (
            <p className="text-gray-500 text-center py-8">No jobs for today.</p>
          ) : (
            jobs.map((job) => (
              <div key={job.id} className="relative block">
                {draftEstimates.includes(job.id) && (
                  <div className="mb-2 p-4 bg-green-50 border border-green-200 rounded-lg flex flex-col gap-2">
                    <h3 className="font-semibold text-green-900">Draft Estimate Ready</h3>
                    <p className="text-sm text-green-800">The Sales Agent has drafted an estimate based on your notes for {job.customer_name}.</p>
                    <button className="bg-green-600 text-white px-4 py-2 rounded-md font-medium mt-2 w-full touch-manipulation min-h-[44px]">Approve & Send</button>
                  </div>
                )}

                <Link href={`/field-service/jobs/${job.id}`} className="block">
                  <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-5 cursor-pointer active:bg-gray-50 transition-colors touch-manipulation min-h-[44px]">
                    <div className="flex justify-between items-start mb-2">
                      <h2 className="text-lg font-semibold text-gray-900">{job.customer_name}</h2>
                      <span className={`text-xs px-2 py-1 rounded-full font-medium ${
                        job.status === 'COMPLETED' ? 'bg-green-100 text-green-800' :
                        job.status === 'IN_PROGRESS' ? 'bg-blue-100 text-blue-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {job.status}
                      </span>
                    </div>
                    <p className="text-gray-600 font-medium mb-1">{job.service_requested}</p>
                    <div className="text-sm text-gray-500 flex flex-col gap-1 mt-3">
                      <div className="flex items-center">
                        <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        {new Date(job.scheduled_at).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                      </div>
                      {job.location && (
                        <div className="flex items-center">
                          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
                          {job.location}
                        </div>
                      )}
                    </div>
                  </div>
                </Link>
              </div>
            ))
          )}
        </div>
      </div>
    </AppShell>
  );
}
