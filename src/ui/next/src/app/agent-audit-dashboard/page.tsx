'use client';
import React from 'react';
import Link from 'next/link';

export default function AgentAuditDashboard() {
  return (
    <div className="min-h-screen bg-gray-50 text-gray-900 p-8 font-inter">
      <header className="mb-8 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link href="/inbox" className="text-blue-500 hover:text-blue-700">
            &lt; Back to Inbox
          </Link>
          <h1 className="text-3xl font-bold font-outfit text-gray-900">Agent Audit Dashboard</h1>
        </div>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="md:col-span-2 space-y-6">
          <section className="shadow-lg rounded-[16px] p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h2 className="text-xl font-bold font-outfit mb-4">Cost Tracker</h2>
            <div className="text-4xl font-bold text-gray-900">$1,245.00</div>
            <div className="text-sm text-gray-500 mt-2">Total organizational spend</div>
          </section>

          <section className="shadow-lg rounded-[16px] p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h2 className="text-xl font-bold font-outfit mb-4">Operations</h2>
            <div className="flex items-center gap-4">
               <div className="w-3 h-3 rounded-full" style={{ backgroundColor: '#34C759' }}></div>
               <div>Agent Health: Optimal</div>
            </div>
          </section>

          <section className="shadow-lg rounded-[16px] p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h2 className="text-xl font-bold font-outfit mb-4">Marketing & Advertising</h2>
            <div className="flex items-center gap-4">
               <div className="w-3 h-3 rounded-full" style={{ backgroundColor: '#34C759' }}></div>
               <div>Campaigns Sync: Active</div>
            </div>
          </section>
        </div>

        <div className="md:col-span-1">
          <section className="shadow-lg rounded-[16px] p-6 h-full" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h2 className="text-xl font-bold font-outfit text-red-600 mb-4">Violation Feed</h2>
            <div className="space-y-4">
              <div className="p-3 bg-red-100 rounded-[8px] text-sm text-red-800">
                 [10:45 AM] Sandbox memory limit exceeded in Agent #452
              </div>
              <div className="p-3 bg-red-100 rounded-[8px] text-sm text-red-800">
                 [09:12 AM] Unauthorized network access attempt blocked
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}
