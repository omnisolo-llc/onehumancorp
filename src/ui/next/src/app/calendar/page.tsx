'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function CalendarPage() {
  const [appointments, setAppointments] = useState([
    { id: '1', customer: 'Maya', service: 'Custom Cake Consultation', time: '10:00 AM', date: 'Today', status: 'confirmed', ai_scheduled: true },
    { id: '2', customer: 'Carlos', service: 'Pipe Fixing', time: '2:00 PM', date: 'Tomorrow', status: 'pending', ai_scheduled: true },
    { id: '3', customer: 'Priya', service: 'Styling Session', time: '4:00 PM', date: 'Tomorrow', status: 'confirmed', ai_scheduled: false },
  ]);

  const [aiActivity, setAiActivity] = useState([
    { id: '1', action: 'Proactively offered 3 time slots to Maya for Cake Consultation via IG DM.', time: '09:15 AM' },
    { id: '2', action: 'Automatically followed up with Carlos regarding Pipe Fixing inquiry.', time: '08:30 AM' },
  ]);

  const [aiEnabled, setAiEnabled] = useState(true);

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Calendar & Bookings</h1>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-sm font-medium text-gray-700">AI Scheduling (Zero-Setup)</span>
          <button
            onClick={() => setAiEnabled(!aiEnabled)}
            className={`w-11 h-6 rounded-full relative transition-colors duration-300 focus:outline-none ${aiEnabled ? 'bg-green-500' : 'bg-gray-300'}`}
          >
            <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${aiEnabled ? 'translate-x-5' : 'translate-x-0'}`} />
          </button>
        </div>
      </header>

      <main className="p-6 max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-6">

        {/* Appointments Column */}
        <div className="md:col-span-2 space-y-6">
          <section className="bg-white rounded-[16px] shadow-sm p-6" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
            <h2 className="text-xl font-semibold font-outfit mb-4 text-gray-900">Upcoming Appointments</h2>
            <div className="space-y-4">
              {appointments.map(apt => (
                <div key={apt.id} className="flex flex-col sm:flex-row justify-between items-start sm:items-center p-4 border border-gray-100 rounded-lg hover:shadow-md transition-shadow">
                  <div>
                    <h3 className="font-semibold text-gray-900 text-lg">{apt.service}</h3>
                    <p className="text-sm text-gray-500">{apt.date} at {apt.time} • {apt.customer}</p>
                    {apt.ai_scheduled && (
                      <span className="inline-block mt-2 px-2 py-1 text-xs font-medium bg-blue-50 text-blue-600 rounded-md">✨ AI Scheduled</span>
                    )}
                  </div>
                  <div className="mt-2 sm:mt-0">
                    <span className={`px-3 py-1 rounded-full text-xs font-medium ${apt.status === 'confirmed' ? 'bg-green-100 text-green-700' : 'bg-orange-100 text-orange-700'}`}>
                      {apt.status.charAt(0).toUpperCase() + apt.status.slice(1)}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </div>

        {/* AI Operations Activity Feed */}
        <div className="md:col-span-1 space-y-6">
          <section className="bg-white rounded-[16px] shadow-sm p-6" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
            <div className="flex items-center gap-2 mb-4">
              <span className="text-xl">🤖</span>
              <h2 className="text-xl font-semibold font-outfit text-gray-900">Operations Agent</h2>
            </div>
            <p className="text-sm text-gray-500 mb-6">Real-time activity of your AI managing bookings and inquiries.</p>

            <div className="space-y-4 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px  before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-slate-300 before:to-transparent">
              {aiActivity.map((activity, idx) => (
                <div key={activity.id} className="relative flex items-center justify-between  group is-active">
                  <div className="flex items-center justify-center w-10 h-10 rounded-full border border-white bg-blue-100 text-blue-500 shadow shrink-0  z-10">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                  </div>
                  <div className="w-[calc(100%-4rem)]  bg-white p-4 rounded border border-gray-100 shadow-sm ml-4 ">
                    <div className="flex items-center justify-between mb-1">
                      <span className="text-xs font-semibold text-gray-500">{activity.time}</span>
                    </div>
                    <p className="text-sm text-gray-800">{activity.action}</p>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </div>

      </main>
    </div>
  );
}
