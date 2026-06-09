"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../../../components/AppShell";
import { useRouter } from "next/navigation";

export default function JobDetailPage({ params }: { params: { id: string } }) {
  const router = useRouter();
  const [job, setJob] = useState<any>(null);
  const [notes, setNotes] = useState("");
  const [isOffline, setIsOffline] = useState(false);

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    const localJobsStr = localStorage.getItem("field_service_jobs");
    if (localJobsStr) {
      const jobs = JSON.parse(localJobsStr);
      const found = jobs.find((j: any) => j.id === params.id);
      if (found) {
        setJob(found);
        setNotes(found.notes || "");
      }
    }

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, [params.id]);

  const saveMutation = (status: string) => {
    const queueStr = localStorage.getItem("offline_sync_queue");
    let queue = queueStr ? JSON.parse(queueStr) : [];

    // update or add
    const existingIdx = queue.findIndex((m: any) => m.id === params.id);
    if (existingIdx >= 0) {
      queue[existingIdx] = { id: params.id, status, notes };
    } else {
      queue.push({ id: params.id, status, notes });
    }

    localStorage.setItem("offline_sync_queue", JSON.stringify(queue));

    // Optimistically update local view
    const localJobsStr = localStorage.getItem("field_service_jobs");
    if (localJobsStr) {
      let jobs = JSON.parse(localJobsStr);
      const idx = jobs.findIndex((j: any) => j.id === params.id);
      if (idx >= 0) {
        jobs[idx].status = status;
        jobs[idx].notes = notes;
        localStorage.setItem("field_service_jobs", JSON.stringify(jobs));
      }
    }
  };

  const syncNow = async () => {
    if (!navigator.onLine) return;
    const queueStr = localStorage.getItem("offline_sync_queue");
    if (!queueStr) return;
    try {
      const res = await fetch("/api/v1/field-service/sync", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": "default",
        },
        body: JSON.stringify({ mutations: JSON.parse(queueStr) }),
      });
      if (res.ok) {
        localStorage.removeItem("offline_sync_queue");
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleStart = () => {
    saveMutation("IN_PROGRESS");
    setJob({ ...job, status: "IN_PROGRESS" });
    if (!isOffline) syncNow();
  };

  const handleComplete = () => {
    saveMutation("COMPLETED");
    setJob({ ...job, status: "COMPLETED" });
    if (!isOffline) {
      syncNow().then(() => {
        router.push("/field-service/roster");
      });
    } else {
      router.push("/field-service/roster");
    }
  };

  if (!job) return <div className="p-8">Loading...</div>;

  return (
    <AppShell title="Job Details" subtitle={job.customer_name}>
      <div className="p-4 max-w-lg mx-auto">
        <button
          onClick={() => router.push("/field-service/roster")}
          className="mb-4 text-blue-600 font-medium flex items-center min-h-[44px] touch-manipulation"
        >
          <svg className="w-5 h-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Roster
        </button>

        <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6 mb-6">
          <div className="flex justify-between items-start mb-4">
            <h1 className="text-2xl font-bold">{job.service_requested}</h1>
            <span className={`text-xs px-2 py-1 rounded-full font-medium ${
              job.status === 'COMPLETED' ? 'bg-green-100 text-green-800' :
              job.status === 'IN_PROGRESS' ? 'bg-blue-100 text-blue-800' :
              'bg-gray-100 text-gray-800'
            }`}>
              {job.status}
            </span>
          </div>

          <div className="space-y-3 text-gray-600">
            <p><strong>Customer:</strong> {job.customer_name}</p>
            <p><strong>Scheduled:</strong> {new Date(job.scheduled_at).toLocaleString()}</p>
            {job.location && <p><strong>Location:</strong> {job.location}</p>}
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6 mb-6">
          <h2 className="text-lg font-bold mb-3">Job Notes</h2>
          <textarea
            className="w-full border border-gray-300 rounded-lg p-3 min-h-[120px] focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            placeholder="Add notes here (e.g. 'Customer needs new piping under sink. Send follow-up estimate.')"
            value={notes}
            onChange={(e) => {
              setNotes(e.target.value);
              saveMutation(job.status);
            }}
            disabled={job.status === 'COMPLETED'}
          />
          {isOffline && (
            <p className="text-xs text-orange-500 mt-2 flex items-center">
              <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" /></svg>
              Notes saved locally. Will sync when online.
            </p>
          )}
        </div>

        <div className="flex flex-col gap-3">
          {job.status === 'PENDING' && (
            <button
              onClick={handleStart}
              className="w-full min-h-[50px] bg-blue-600 text-white rounded-xl font-semibold text-lg touch-manipulation"
            >
              Start Job
            </button>
          )}

          {(job.status === 'PENDING' || job.status === 'IN_PROGRESS') && (
            <button
              onClick={handleComplete}
              className="w-full min-h-[50px] bg-green-600 text-white rounded-xl font-semibold text-lg touch-manipulation"
            >
              Complete Job
            </button>
          )}
        </div>
      </div>
    </AppShell>
  );
}
