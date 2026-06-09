"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';

export default function FieldOpsJobsPage() {
  const [isOffline, setIsOffline] = useState(false);
  const [jobs, setJobs] = useState([
    { id: 'job-1', customer: 'Alice Smith', address: '123 Main St', service: 'Plumbing Repair', status: 'pending', notes: '' },
    { id: 'job-2', customer: 'Bob Jones', address: '456 Oak Ave', service: 'Electrical Inspection', status: 'pending', notes: '' },
  ]);

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const handleStatusChange = (jobId: string, newStatus: string) => {
    setJobs(jobs.map(j => j.id === jobId ? { ...j, status: newStatus } : j));
  };

  const handleNotesChange = (jobId: string, notes: string) => {
    setJobs(jobs.map(j => j.id === jobId ? { ...j, notes } : j));
  };

  const handleComplete = (jobId: string) => {
    const job = jobs.find(j => j.id === jobId);
    if (!job) return;

    handleStatusChange(jobId, 'completed');

    if (job.notes) {
      SyncManager.getInstance().enqueue({
        id: `mutation-${Date.now()}`,
        type: 'draft_quote',
        notes: `Follow up quote requested by field op for job ${jobId}. Notes: ${job.notes}`
      });
    }
  };

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

      <div className="space-y-4">
        {jobs.map(job => (
          <div key={job.id} className="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
            <div className="p-5 border-b border-gray-100 bg-gray-50/50">
              <div className="flex justify-between items-start mb-2">
                <h3 className="font-bold text-lg text-gray-900">{job.customer}</h3>
                <span className={`px-2 py-1 text-xs font-semibold rounded-full ${job.status === 'completed' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'}`}>
                  {job.status.toUpperCase()}
                </span>
              </div>
              <p className="text-gray-600 text-sm mb-1">📍 {job.address}</p>
              <p className="text-gray-600 text-sm">🔧 {job.service}</p>
            </div>

            {job.status !== 'completed' && (
              <div className="p-5">
                <label className="block text-sm font-medium text-gray-700 mb-2">Service Notes & Potential Follow-ups</label>
                <textarea
                  className="w-full border border-gray-300 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-shadow min-h-[100px]"
                  placeholder="E.g., Fixed the leak but noticed the secondary pipe is corroded. Needs a replacement quote."
                  value={job.notes}
                  onChange={(e) => handleNotesChange(job.id, e.target.value)}
                />

                <div className="mt-4 flex gap-3">
                  <button
                    className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px]"
                    onClick={() => handleComplete(job.id)}
                  >
                    Complete Job
                  </button>
                </div>
              </div>
            )}
            {job.status === 'completed' && job.notes && (
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
