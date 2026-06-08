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
 <section className="app-panel">
 <div className="app-panel-header">
 <h2 className="app-panel-title">Cost Tracker</h2>
 </div>
 <div className="app-panel-body">
 <div className="text-4xl font-bold text-gray-900">$1,245.00</div>
 <div className="text-sm text-gray-500 mt-2">Total organizational spend</div>
 </div>
 </section>

 <section className="app-card">
 <h2 className="text-xl font-bold font-outfit mb-4">Operations</h2>
 <div className="flex items-center gap-4">
 <div className="w-3 h-3 rounded-full bg-green-500" ></div>
 <div>Agent Health: Optimal</div>
 </div>
 </section>

 <section className="app-card">
 <h2 className="text-xl font-bold font-outfit mb-4">Marketing & Advertising</h2>
 <div className="flex items-center gap-4">
 <div className="w-3 h-3 rounded-full bg-green-500" ></div>
 <div>Campaigns Sync: Active</div>
 </div>
 </section>
 </div>

 <div className="md:col-span-1">
 <section className="app-panel h-full">
 <div className="app-panel-header">
 <h2 className="app-panel-title text-red-600">Violation Feed</h2>
 </div>
 <div className="app-panel-body space-y-4">
 <div className="p-3 bg-red-100 rounded-lg text-sm text-red-800">
 [10:45 AM] Sandbox memory limit exceeded in Agent #452
 </div>
 <div className="p-3 bg-red-100 rounded-lg text-sm text-red-800">
 [09:12 AM] Unauthorized network access attempt blocked
 </div>
 </div>
 </section>
 </div>
 </div>
 </div>
 );
}
