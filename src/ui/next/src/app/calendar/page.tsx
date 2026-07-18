'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function CalendarPage() {
  const [appointments, setAppointments] = useState<any[]>([]);
  const [morningBriefing, setMorningBriefing] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAppointment, setSelectedAppointment] = useState<any>(null);

  useEffect(() => {
    const tenantId = localStorage.getItem('tenant_id') || 'e2e-tenant';
    fetch(`/api/ui/bookings?tenant_id=${tenantId}`)
      .then(res => {
        if (!res.ok) {
           throw new Error('Failed to load bookings');
        }
        return res.json();
      })
      .then(data => {
        if (Array.isArray(data)) {
          const formattedAppointments = data.map((b: any) => {
            const startDate = new Date(b.start_time);
            const isPast = startDate < new Date();
            return {
              id: b.id,
              customer: b.customer_name || 'Customer',
              service: b.product_title || 'Service Booking',
              time: startDate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
              date: startDate.toLocaleDateString(),
              status: b.status || 'pending',
              ai_scheduled: true,
              link: '',
              isPast,
              rawDate: startDate,
              paymentStatus: b.status === 'confirmed' ? 'Paid' : 'Deposit Required',
              aiSummary: b.ai_summary || `AI Details for ${b.product_title || 'Service Booking'}`
            };
          });
          setAppointments(formattedAppointments);

          const futureAppointments = formattedAppointments.filter(a => !a.isPast);
          const unpaidAppointments = futureAppointments.filter(a => a.paymentStatus === 'Deposit Required').length;

          setMorningBriefing({
             message: `You have ${futureAppointments.length} appointments today. ${unpaidAppointments > 0 ? `${unpaidAppointments} client(s) still need to pay their deposit.` : 'All deposits are paid.'}`
          });
        }
        setIsLoading(false);
      })
      .catch(err => {
         console.error(err);
         setError("Failed to load appointments. Please try again later.");
         setIsLoading(false);
      });
  }, []);

  const [aiEnabled, setAiEnabled] = useState(true);

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4">
          <Link href="/dashboard" aria-label="Back to Dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Calendar & Bookings</h1>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-sm font-medium text-gray-700">AI Scheduling</span>
          <button
            aria-label="Toggle AI Scheduling"
            aria-pressed={aiEnabled}
            onClick={() => setAiEnabled(!aiEnabled)}
            className={`w-11 h-6 rounded-full relative transition-colors duration-300 focus:outline-none ${aiEnabled ? 'bg-[#34C759]' : 'bg-gray-300'}`}
          >
            <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${aiEnabled ? 'translate-x-5' : 'translate-x-0'}`} />
          </button>
        </div>
      </header>

      <main className="p-6 max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-6">

        {/* Appointments Column */}
        <div className="md:col-span-2 space-y-6">
          {/* AI Morning Briefing Card */}
          {morningBriefing && (
             <div className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-100 rounded-xl p-5 shadow-sm">
                <div className="flex items-center gap-2 mb-2">
                   <span className="text-lg">✨</span>
                   <h2 className="font-semibold text-blue-900 font-outfit">Morning Briefing</h2>
                </div>
                <p className="text-blue-800 text-sm">{morningBriefing.message}</p>
             </div>
          )}

          <section className="app-card shadow-sm p-6 bg-white" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
            <h2 className="text-xl font-semibold font-outfit mb-4 text-gray-900">Today</h2>
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
                <div
                  key={apt.id}
                  onClick={() => setSelectedAppointment(apt)}
                  className={`cursor-pointer flex flex-col sm:flex-row justify-between items-start sm:items-center p-4 border rounded-lg hover:shadow-md transition-shadow ${apt.isPast ? 'bg-gray-50 opacity-70 border-gray-100' : 'bg-white border-gray-200'}`}
                >
                  <div>
                    <h3 className={`font-semibold text-lg ${apt.isPast ? 'text-gray-500' : 'text-gray-900'}`}>{apt.service}</h3>
                    <p className="text-sm text-gray-500">{apt.date} at {apt.time} • {apt.customer}</p>
                  </div>
                  <div className="mt-2 sm:mt-0 flex items-center gap-3">
                    <span className={`px-3 py-1 rounded-full text-xs font-medium ${apt.status === 'confirmed' ? 'bg-green-100 text-green-700' : 'bg-orange-100 text-orange-700'}`}>
                      {apt.status.charAt(0).toUpperCase() + apt.status.slice(1)}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </div>

        {/* Appointment Details Column */}
        <div className="md:col-span-1 space-y-6">
          <section className="app-card shadow-sm p-6 bg-white sticky top-24" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
            <h2 className="text-xl font-semibold font-outfit mb-4 text-gray-900">Appointment Details</h2>
            {!selectedAppointment ? (
              <p className="text-sm text-gray-500 text-center py-8">Select an appointment to view details.</p>
            ) : (
               <div className="space-y-5 animate-in fade-in slide-in-from-bottom-2 duration-300">
                  <div className="flex items-center gap-3">
                     <div className="w-12 h-12 rounded-full bg-gray-200 flex items-center justify-center text-xl font-bold text-gray-600">
                        {selectedAppointment.customer.charAt(0)}
                     </div>
                     <div>
                        <h3 className="font-bold text-gray-900">{selectedAppointment.customer}</h3>
                        <p className="text-xs text-gray-500">{selectedAppointment.time} • {selectedAppointment.service}</p>
                     </div>
                  </div>

                  <div className="bg-blue-50 p-3 rounded-lg border border-blue-100">
                     <span className="text-xs font-bold text-blue-800 uppercase tracking-wider block mb-1">AI Summary</span>
                     <p className="text-sm text-blue-900">{selectedAppointment.aiSummary}</p>
                  </div>

                  <div className="flex justify-between items-center py-2 border-b border-gray-100">
                     <span className="text-sm text-gray-600">Payment Status</span>
                     <span className={`text-sm font-semibold ${selectedAppointment.paymentStatus === 'Paid' ? 'text-green-600' : 'text-orange-600'}`}>
                        {selectedAppointment.paymentStatus}
                     </span>
                  </div>

                  <div className="space-y-2 pt-2">
                     <button className="w-full py-2.5 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800 transition">
                        Message Client
                     </button>
                     {selectedAppointment.paymentStatus === 'Deposit Required' && (
                        <button className="w-full py-2.5 bg-white border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition">
                           Request Payment
                        </button>
                     )}
                     <button className="w-full py-2.5 bg-white border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition">
                        Reschedule
                     </button>
                  </div>
               </div>
            )}
          </section>
        </div>

      </main>
    </div>
  );
}
