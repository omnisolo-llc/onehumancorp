'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function CalendarPage() {
  const [appointments, setAppointments] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const tenantId = localStorage.getItem('tenant_id') || 'e2e-tenant';
    fetch(`/api/v1/booking/list`, { headers: { 'x-tenant-id': tenantId } })
      .then(res => {
        if (!res.ok) {
           throw new Error('Failed to load bookings');
        }
        return res.json();
      })
      .then(data => {
        if (Array.isArray(data)) {
          setAppointments(data.map((b: any) => {
            const startDate = new Date(b.start_time);
            return {
              id: b.id,
              customer: b.customer_name || 'Customer',
              service: b.product_title || 'Service Booking',
              time: startDate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
              date: startDate.toLocaleDateString(),
              status: b.status || 'pending',
              ai_scheduled: true, // Assuming OHC Operations AI manages these
              link: '' // No link for physical bookings by default, could add if it's a virtual service
            };
          }));
        }
        setIsLoading(false);
      })
      .catch(err => {
         console.error(err);
         setError("Failed to load appointments. Please try again later.");
         setIsLoading(false);
      });
  }, []);


  const [selectedApt, setSelectedApt] = useState<any | null>(null);

  // Set dummy AI Activity data for the pre-meeting summary

  const [aiActivity, setAiActivity] = useState<any[]>([
    {
      id: "ai_1",
      time: "Just now",
      action: "Morning Briefing: You have 2 appointments today. 1 client still needs to pay their deposit. I've drafted a reminder."
    },
    {
      id: "ai_2",
      time: "30m ago",
      action: "Next: Guitar Lesson with Sarah. She struggled with chords last week. She hasn't paid her monthly invoice yet."
    }
  ]);

  const [aiEnabled, setAiEnabled] = useState(true);

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4 flex-wrap">
          <Link href="/dashboard" aria-label="Back to Dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Calendar & Bookings</h1>
        </div>
        <div className="flex items-center gap-3 flex-wrap mt-2 sm:mt-0">
          <span className="text-sm font-medium text-gray-700">AI Scheduling (Zero-Setup)</span>
          <button
            aria-label="Toggle AI Scheduling"
            aria-pressed={aiEnabled}
            onClick={() => setAiEnabled(!aiEnabled)}
            className={`w-11 h-6 rounded-full relative transition-colors duration-300 focus:outline-none ${aiEnabled ? 'bg-green-500' : 'bg-gray-300'}`}
          >
            <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${aiEnabled ? 'translate-x-5' : 'translate-x-0'}`} />
          </button>
        </div>
      </header>


      <main className="p-6 max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-6">

        {/* Appointment Detail Modal/Overlay */}
        {selectedApt && (
          <div className="fixed inset-0 bg-black/20 backdrop-blur-sm z-50 flex items-center justify-center p-4" onClick={() => setSelectedApt(null)} data-testid="appointment-modal">
            <div className="bg-white rounded-2xl shadow-xl w-full max-w-md overflow-hidden flex flex-col" onClick={e => e.stopPropagation()}>
              <div className="p-6 border-b border-gray-100 flex justify-between items-start">
                <div>
                  <h3 className="text-xl font-bold text-gray-900 mb-1">{selectedApt.service}</h3>
                  <p className="text-sm text-gray-500">{selectedApt.date} at {selectedApt.time}</p>
                </div>
                <button onClick={() => setSelectedApt(null)} className="text-gray-400 hover:text-gray-600">
                  <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>
              <div className="p-6 space-y-4 flex-1 overflow-y-auto">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 font-bold text-lg">
                    {selectedApt.customer.charAt(0)}
                  </div>
                  <div>
                    <p className="font-semibold text-gray-900">{selectedApt.customer}</p>
                    <p className="text-sm text-gray-500">Returning Client</p>
                  </div>
                </div>

                <div className="bg-blue-50 rounded-lg p-4 border border-blue-100">
                  <h4 className="text-sm font-semibold text-blue-900 mb-2 flex items-center gap-2"><span className="text-lg">✨</span> AI Preparation Brief</h4>
                  <p className="text-sm text-blue-800 leading-relaxed">
                    This is the 3rd lesson. Focus: Jazz scales. Client has struggled with chord transitions in the past.
                  </p>
                </div>

                <div className="flex items-center justify-between py-2 border-b border-gray-50">
                  <span className="text-sm text-gray-500">Payment Status</span>
                  <span className={`px-2.5 py-1 rounded-md text-xs font-medium ${selectedApt.status === 'confirmed' ? 'bg-green-100 text-green-700' : 'bg-orange-100 text-orange-700'}`}>
                    {selectedApt.status === 'confirmed' ? 'Paid' : 'Deposit Required'}
                  </span>
                </div>
              </div>

              <div className="p-4 bg-gray-50 border-t border-gray-100 flex gap-3">
                <button className="flex-1 px-4 py-2 bg-white border border-gray-200 rounded-lg text-sm font-semibold text-gray-700 hover:bg-gray-50 transition-colors shadow-sm">
                  Reschedule
                </button>
                <button className="flex-1 px-4 py-2 bg-[#0066FF] rounded-lg text-sm font-semibold text-white hover:bg-blue-600 transition-colors shadow-sm">
                  Message Client
                </button>
              </div>
            </div>
          </div>
        )}


        {/* AI Operations Activity Feed (Morning Briefing) - Moved to top/left for prominence per design */}
        <div className="md:col-span-3 space-y-6">
          <section className="app-card rounded-[16px] shadow-sm p-6" style={{ border: '1px solid rgba(0,0,0,0.05)', background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)' }}>
             <div className="flex items-center gap-2 mb-4">
              <span className="text-xl">🤖</span>
              <h2 className="text-xl font-semibold font-outfit text-gray-900">Morning Briefing</h2>
            </div>
            {aiActivity.length > 0 && (
               <p className="text-gray-800 text-lg">{aiActivity[0].action}</p>
            )}
          </section>
        </div>

        {/* Appointments Column */}
        <div className="md:col-span-2 space-y-6">
          <section className="app-card rounded-[16px] shadow-sm p-6" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
            <h2 className="text-xl font-semibold font-outfit mb-4 text-gray-900">Upcoming Appointments</h2>
            <div className="space-y-4">
              {isLoading ? (
                <div className="text-sm text-gray-500 p-4 border border-gray-100 rounded-lg text-center flex flex-col items-center justify-center gap-3">
                  <div className="w-6 h-6 border-2 border-gray-200 border-t-indigo-600 rounded-full animate-spin"></div>
                  Loading appointments...
                </div>
              ) : error ? (
                <div className="text-sm text-red-600 p-4 border border-red-100 bg-red-50 rounded-lg text-center">
                  {error}
                </div>
              ) : appointments.length === 0 ? (
                <div className="text-sm text-gray-500 p-4 border border-gray-100 rounded-lg text-center">No upcoming appointments.</div>
              ) : appointments.map(apt => (
                <div key={apt.id} className="flex flex-col sm:flex-row justify-between items-start sm:items-center p-4 border border-gray-100 rounded-lg hover:shadow-md transition-shadow cursor-pointer" onClick={() => setSelectedApt(apt)}>
                  <div>
                    <h3 className="font-semibold text-gray-900 text-lg">{apt.service}</h3>
                    <p className="text-sm text-gray-500">{apt.date} at {apt.time} • {apt.customer}</p>
                    {apt.ai_scheduled && (
                      <span className="inline-block mt-2 px-2 py-1 text-xs font-medium bg-blue-50 text-blue-600 rounded-md">✨ AI Scheduled</span>
                    )}
                  </div>
                  <div className="mt-2 sm:mt-0 flex flex-wrap items-center gap-2">
                    <span className={`px-3 py-1 rounded-full text-xs font-medium ${apt.status === 'confirmed' ? 'bg-green-100 text-green-700' : 'bg-orange-100 text-orange-700'}`}>
                      {apt.status.charAt(0).toUpperCase() + apt.status.slice(1)}
                    </span>
                    <button className="px-3 py-1.5 text-xs font-bold rounded bg-gray-100 text-gray-800 hover:bg-gray-200 transition border border-gray-200">
                        Message
                    </button>
                    {apt.link && (
                      <a href={apt.link} target="_blank" rel="noreferrer" className="px-3 py-1.5 text-xs font-bold rounded bg-blue-600 text-white hover:bg-blue-700 transition">
                        Join Meeting
                      </a>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </section>
        </div>

        {/* AI Operations Activity Feed */}
        <div className="md:col-span-1 space-y-6">
          <section className="app-card rounded-[16px] shadow-sm p-6" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
            <div className="flex items-center gap-2 mb-4">
              <span className="text-xl">⚡</span>
              <h2 className="text-xl font-semibold font-outfit text-gray-900">Operations Agent</h2>
            </div>
            <p className="text-sm text-gray-500 mb-6">Real-time activity of your AI managing bookings and inquiries.</p>

            <div className="space-y-4 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px  before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-slate-300 before:to-transparent">
              {aiActivity.length === 0 ? (
                <div className="text-sm text-gray-500 text-center py-4">No recent AI activity.</div>
              ) : aiActivity.slice(1).map((activity, idx) => (
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
