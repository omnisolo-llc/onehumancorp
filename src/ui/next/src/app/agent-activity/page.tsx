"use client";

import React, { useEffect, useState } from 'react';
import { AppShell } from '../components/AppShell';

interface Job {
  id: string;
  job_type: string;
  status: string;
  retry_count: number;
  created_at: string;
  updated_at: string;
}

export default function AgentActivityPage() {
  const [jobs, setJobs] = useState<Job[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchJobs = async () => {
      try {
        const res = await fetch('/api/ohc_job_queue');
        if (res.ok) {
          const data = await res.json();
          setJobs(data.jobs || []);
        }
      } catch (err) {
        console.error('Failed to fetch jobs', err);
      } finally {
        setLoading(false);
      }
    };
    fetchJobs();
    const interval = setInterval(fetchJobs, 5000);
    return () => clearInterval(interval);
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'PENDING': return 'bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400';
      case 'PROCESSING': return 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400';
      case 'COMPLETED': return 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400';
      case 'FAILED': return 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400';
      default: return 'bg-gray-100 dark:bg-gray-900/30 text-gray-700 dark:text-gray-400';
    }
  };

  const formatJobType = (type: string) => {
      // Very basic formatting mapping
      const mapping: Record<string, string> = {
          'invoice_agent': 'Invoicing Agent',
          'marketing_agent': 'Marketing Agent',
          'operations_agent': 'Operations Agent',
          'sync_inventory': 'Inventory Sync',
      };
      return mapping[type] || type.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  }

  const formatJobDesc = (type: string) => {
       const mapping: Record<string, string> = {
          'invoice_agent': 'Processing upcoming invoices and subscription billing.',
          'marketing_agent': 'Drafting and analyzing marketing campaigns.',
          'operations_agent': 'Synchronizing backend operations and updates.',
          'sync_inventory': 'Updating offline point-of-sale inventory records.',
      };
      return mapping[type] || 'Executing scheduled automated task...';
  }

  return (
    <AppShell title="Agent Activity">
      <div className="p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto space-y-8 animate-in fade-in duration-500">
        <header className="mb-8">
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Agent Activity</h1>
          <p className="text-[#86868B] dark:text-[#A1A1A6] text-lg mt-2 font-inter">Live view of automated assistant tasks running across the organization.</p>
        </header>

        <section className="space-y-4">
            <div className="glassmorphism p-6 border border-white/40 dark:border-white/10 shadow-sm">
                <div className="flex flex-col sm:flex-row justify-between gap-4 mb-6">
                    <h2 className="text-lg font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] flex items-center gap-2">
                        <span className="text-xl">⚙️</span> Active Operations
                    </h2>
                </div>

                <div className="flex flex-col gap-4">
                    {loading ? (
                         <div className="text-[#86868B] dark:text-[#A1A1A6] text-sm">Loading background tasks...</div>
                    ) : jobs.length === 0 ? (
                        <div className="text-[#86868B] dark:text-[#A1A1A6] text-sm py-4">No active or recent tasks found. Your agents are standing by.</div>
                    ) : (
                        jobs.map((job) => (
                            <div key={job.id} className={`glassmorphism p-4 border ${job.status === 'PROCESSING' ? 'border-blue-200 dark:border-blue-800' : 'border-white/40 dark:border-white/10'} shadow-md flex flex-col sm:flex-row gap-4 items-start sm:items-center relative overflow-hidden transition-shadow`}>
                                {job.status === 'PROCESSING' && (
                                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-[#0066FF] rounded-l-[16px]"></div>
                                )}
                                <div className={`w-16 flex flex-col items-center justify-center shrink-0 ${job.status === 'COMPLETED' ? 'text-[#34C759]' : job.status === 'FAILED' ? 'text-[#FF3B30]' : ''}`}>
                                    {job.status === 'PROCESSING' ? (
                                        <div className="animate-spin h-6 w-6 border-2 border-[#0066FF] border-t-transparent rounded-full"></div>
                                    ) : job.status === 'COMPLETED' ? (
                                        <svg xmlns="http://www.w3.org/2000/svg" className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                                        </svg>
                                    ) : job.status === 'PENDING' ? (
                                         <span className="text-2xl">⏳</span>
                                    ) : (
                                        <span className="text-2xl">⚠️</span>
                                    )}
                                </div>
                                <div className="flex-1">
                                    <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg">{formatJobType(job.job_type)}</h3>
                                    <p className="text-sm text-[#86868B] dark:text-[#A1A1A6]">{formatJobDesc(job.job_type)}</p>
                                    <p className="text-xs text-[#86868B] dark:text-[#A1A1A6] mt-1">
                                        {job.status === 'COMPLETED' ? `Completed at ${new Date(job.updated_at).toLocaleTimeString()}` : `Started at ${new Date(job.created_at).toLocaleTimeString()}`}
                                        {job.retry_count > 0 && ` • Retries: ${job.retry_count}`}
                                    </p>
                                </div>
                                <div className="flex flex-col gap-2 shrink-0 w-full sm:w-auto">
                                    <span className={`px-3 py-1 text-xs font-medium rounded-full self-start sm:self-end ${getStatusColor(job.status)}`}>
                                        {job.status}
                                    </span>
                                </div>
                            </div>
                        ))
                    )}
                </div>
            </div>
        </section>
      </div>
    </AppShell>
  );
}
