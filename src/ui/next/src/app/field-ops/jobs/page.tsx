"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';

type Appointment = {
  id: string;
  customer_id: string;
  customer_name: string;
  job_template_id: string;
  job_name: string;
  status: string;
  scheduled_start_time: string;
  scheduled_end_time: string;
  location_address: string;
  location_lat?: number;
  location_lng?: number;
  notes: string;
  actual_start_time?: string;
  actual_end_time?: string;
};

export default function FieldOpsJobsPage() {
  const [isOffline, setIsOffline] = useState(false);
  const [jobs, setJobs] = useState<Appointment[]>([]);
  const [agentSuggestion, setAgentSuggestion] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [delayModalJob, setDelayModalJob] = useState<Appointment | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [currentLocation, setCurrentLocation] = useState<{lat: number, lng: number} | null>(null);

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    // Get current location if possible
    if ("geolocation" in navigator) {
        navigator.geolocation.getCurrentPosition(
            (position) => {
                setCurrentLocation({
                    lat: position.coords.latitude,
                    lng: position.coords.longitude
                });
            },
            (error) => console.log("Geolocation error:", error),
            { timeout: 10000 }
        );
    }

    fetch('/api/v1/field-ops/appointments')
      .then(res => res.json())
      .then(data => {
        if (data.appointments) {
          setJobs(data.appointments);
        }
        setLoading(false);
      })
      .catch(err => {
        console.error("Failed to load appointments", err);
        setLoading(false);
      });

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const handleStatusChange = (jobId: string, newStatus: string) => {
    const now = new Date().toISOString();
    setJobs(currentJobs =>
      currentJobs.map(j => {
        if (j.id === jobId) {
          const updated = { ...j, status: newStatus };
          if (newStatus === 'In-Progress') updated.actual_start_time = now;
          if (newStatus === 'Completed') updated.actual_end_time = now;
          return updated;
        }
        return j;
      })
    );

    const updatedJobs = jobs.map(j => {
      if (j.id === jobId) {
        const updated = { ...j, status: newStatus };
        if (newStatus === 'In-Progress') updated.actual_start_time = now;
        if (newStatus === 'Completed') updated.actual_end_time = now;
        return updated;
      }
      return j;
    });

    if (newStatus === 'Completed' || newStatus === 'En-Route') {
       fetch('/api/v1/field-ops/optimize-route', {
         method: 'POST',
         headers: { 'Content-Type': 'application/json' },
         body: JSON.stringify({
           appointments: updatedJobs,
           currentLocationLat: currentLocation?.lat,
           currentLocationLng: currentLocation?.lng
         })
       })
       .then(async res => {
           if (!res.ok) {
               throw new Error(await res.text());
           }
           return res.json();
       })
       .then(data => {
         if (data.success) {
           setJobs(data.optimizedRoute);
           if (data.agentSuggestion) {
             setAgentSuggestion(data.agentSuggestion);
           }
         }
       })
       .catch(err => console.error("Optimization failed", err));
    }
  };

  const handleNotesChange = (jobId: string, notes: string) => {
    setJobs(jobs.map(j => j.id === jobId ? { ...j, notes } : j));
  };

  const handleComplete = (jobId: string) => {
    const job = jobs.find(j => j.id === jobId);
    if (!job) return;

    handleStatusChange(jobId, 'Completed');

    if (job.notes && !isOffline) {
      SyncManager.getInstance().enqueue({
        id: `mutation-${Date.now()}`,
        type: 'draft_quote',
        notes: `Follow up quote requested by field op for job ${jobId}. Notes: ${job.notes}`
      });
    }
  };

  const handleRunningLate = (job: Appointment) => {
    setDelayModalJob(job);
    setErrorMessage(null);
  };

  const approveDelay = () => {
    if (!delayModalJob) return;

    fetch('/api/v1/field-ops/delay', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          appointment_id: delayModalJob.id,
          delay_minutes: 30
        })
    })
    .then(async res => {
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error(err.error || 'Failed to update delay');
        }
        return res.json();
    })
    .then(data => {
        if (data.success && data.appointments) {
            setJobs(data.appointments);
            setDelayModalJob(null);
        } else {
            setErrorMessage('Failed to update delay');
        }
    })
    .catch(err => {
        console.error("Delay notification failed", err);
        setErrorMessage(err.message || 'Network error');
    });
  };

  const calculateAffectedClients = (delayJob: Appointment) => {
      const delayJobDate = new Date(delayJob.scheduled_start_time);
      const affected = jobs.filter(j => new Date(j.scheduled_start_time) > delayJobDate && j.status !== 'Completed' && j.status !== 'Cancelled');
      return affected.length;
  };

  if (loading) {
     return <div className="p-4 bg-gray-50 min-h-screen flex items-center justify-center">Loading schedule...</div>;
  }

  return (
    <div className="p-4 bg-gray-50 min-h-screen">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold font-outfit text-gray-900">Today's Route</h1>
        {isOffline && (
          <div className="flex items-center text-orange-600 bg-orange-50 px-3 py-1 rounded-full text-sm font-semibold">
            <span className="mr-2">☁️</span> Offline Mode
          </div>
        )}
      </div>

      {delayModalJob && (
          <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
              <div className="bg-white/80 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-xl rounded-2xl p-6 w-full max-w-sm">
                  <div className="flex gap-4 items-start mb-4">
                      <div className="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center text-2xl shrink-0">🤖</div>
                      <div>
                          <h3 className="text-lg font-bold text-gray-900">Operations Agent</h3>
                          <p className="text-sm text-gray-700 mt-1">
                              {calculateAffectedClients(delayModalJob) > 0
                                ? `Notify the next ${calculateAffectedClients(delayModalJob)} clients of a 30-minute delay?`
                                : "No subsequent clients affected. Adjust schedule?"
                              }
                          </p>
                      </div>
                  </div>
                  {errorMessage && (
                      <div className="mb-4 text-sm text-red-600 font-semibold text-center bg-red-50 p-2 rounded">
                          {errorMessage}
                      </div>
                  )}
                  <div className="flex gap-3 mt-6">
                      <button
                          onClick={() => { setDelayModalJob(null); setErrorMessage(null); }}
                          className="flex-1 py-3 px-4 rounded-xl font-semibold text-gray-700 bg-gray-100 hover:bg-gray-200 transition-colors"
                      >
                          Cancel
                      </button>
                      <button
                          onClick={approveDelay}
                          className="flex-1 py-3 px-4 rounded-xl font-semibold text-white bg-[#0071E3] hover:bg-blue-600 transition-colors shadow-md"
                      >
                          Approve & Send
                      </button>
                  </div>
              </div>
          </div>
      )}

      {agentSuggestion && (
        <div className="mb-6 p-4 bg-blue-50 border border-blue-200 rounded-xl shadow-sm relative">
          <button
            onClick={() => setAgentSuggestion(null)}
            className="absolute top-2 right-2 text-gray-400 hover:text-gray-600 p-1"
          >
            ✕
          </button>
          <div className="flex gap-3">
             <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center text-xl shrink-0">🤖</div>
             <div>
               <p className="text-sm font-medium text-gray-900 mb-2">{agentSuggestion}</p>
               <div className="flex gap-2">
                 <button
                   onClick={() => setAgentSuggestion(null)}
                   className="px-3 py-1.5 bg-blue-600 text-white text-xs font-semibold rounded-lg"
                 >
                   Yes, text them
                 </button>
                 <button
                   onClick={() => setAgentSuggestion(null)}
                   className="px-3 py-1.5 bg-white border border-gray-300 text-gray-700 text-xs font-semibold rounded-lg"
                 >
                   No, stick to schedule
                 </button>
               </div>
             </div>
          </div>
        </div>
      )}

      <div className="space-y-4">
        {jobs.map(job => (
          <div key={job.id} className="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
            <div className="p-5 border-b border-gray-100 bg-gray-50/50">
              <div className="flex justify-between items-start mb-2">
                <h3 className="font-bold text-lg text-gray-900">{job.customer_name}</h3>
                <span className={`px-2 py-1 text-xs font-semibold rounded-full ${
                    job.status === 'Completed' ? 'bg-green-100 text-green-700' :
                    job.status === 'In-Progress' ? 'bg-yellow-100 text-yellow-700' :
                    job.status === 'En-Route' ? 'bg-purple-100 text-purple-700' :
                    'bg-blue-100 text-blue-700'
                  }`}>
                  {job.status.toUpperCase()}
                </span>
              </div>
              <p className="text-gray-600 text-sm mb-1 flex items-center gap-2">📍 {job.location_address}</p>
              <p className="text-gray-600 text-sm flex items-center gap-2">🔧 {job.job_name}</p>
              <p className="text-gray-500 text-xs mt-2 font-medium">
                {job.scheduled_start_time && job.scheduled_end_time && (
                  <>⏱ {new Date(job.scheduled_start_time).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})} - {new Date(job.scheduled_end_time).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</>
                )}
              </p>
            </div>

            {job.status !== 'Completed' && (
              <div className="p-5">
                <label className="block text-sm font-medium text-gray-700 mb-2">Service Notes & Potential Follow-ups</label>
                <textarea
                  className="w-full border border-gray-300 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-shadow min-h-[80px]"
                  placeholder="E.g., Needs a replacement quote."
                  value={job.notes}
                  onChange={(e) => handleNotesChange(job.id, e.target.value)}
                />

                <div className="mt-4 flex flex-col gap-2 sm:flex-row sm:gap-3">
                  <button
                    className="flex-1 bg-[#FF9500]/10 hover:bg-[#FF9500]/20 text-[#FF9500] font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px]"
                    onClick={() => handleRunningLate(job)}
                  >
                    Running Late
                  </button>

                  {job.status === 'Requested' || job.status === 'Scheduled' ? (
                     <button
                        className="flex-1 bg-purple-100 hover:bg-purple-200 text-purple-700 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px]"
                        onClick={() => handleStatusChange(job.id, 'En-Route')}
                      >
                        Heading to Job
                      </button>
                  ) : job.status === 'En-Route' ? (
                      <button
                        className="flex-1 bg-yellow-100 hover:bg-yellow-200 text-yellow-700 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px]"
                        onClick={() => handleStatusChange(job.id, 'In-Progress')}
                      >
                        Start Work
                      </button>
                  ) : (
                      <button
                        className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px]"
                        onClick={() => handleComplete(job.id)}
                      >
                        Job Done
                      </button>
                  )}
                </div>
              </div>
            )}
            {job.status === 'Completed' && job.notes && (
                <div className="p-5 bg-blue-50/50">
                     <p className="text-sm font-medium text-gray-800 mb-1">Saved Notes:</p>
                     <p className="text-sm text-gray-600 italic">"{job.notes}"</p>
                     <p className="text-xs text-blue-600 font-semibold mt-2">✨ Sales Agent will draft an estimate based on these notes once online.</p>
                </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
