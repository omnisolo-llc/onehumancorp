'use client';
import { useState, useEffect } from 'react';

interface Appointment {
  id: string;
  job_name: string;
  status: string;
  customer_name: string;
}

export default function FieldOpsFeed() {
  const [appointments, setAppointments] = useState<Appointment[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/v1/field-ops/appointments')
      .then(res => res.json())
      .then(data => {
         if (data && data.appointments) {
             setAppointments(data.appointments);
         } else {
             setAppointments([]);
         }
         setLoading(false);
      })
      .catch(err => {
         console.error('Failed to fetch appointments', err);
         setAppointments([]);
         setLoading(false);
      });
  }, []);

  const approveQuote = async (id: string) => {
      setAppointments((prev) => prev.map(a => a.id === id ? { ...a, status: 'Approved' } : a));

      try {
          await fetch('/api/v1/field-ops/appointments', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ id, status: 'Approved' })
          });
      } catch (err) {
          console.error(err);
      }
  };

  if (loading) return <div className="p-4" id="loading">Loading Today's Work...</div>;

  return (
    <div className="min-h-screen bg-neutral-900 text-white p-4 font-sans max-w-md mx-auto">
      <header className="mb-6">
        <h1 className="text-2xl font-bold text-blue-400">Today's Work</h1>
        <p className="text-sm text-neutral-400">Your agentic field ops feed</p>
      </header>

      {appointments.length === 0 ? (
        <div className="text-neutral-500 text-center p-8 empty-state">No appointments or tasks scheduled for today.</div>
      ) : (
        <div className="space-y-4">
          {appointments.map((appt) => (
            <div key={appt.id} className="bg-neutral-800 p-4 rounded-xl border border-white/10 backdrop-blur-md appointment-card">
              <div className="flex justify-between items-start mb-2">
                <h2 className="text-lg font-semibold">{appt.job_name}</h2>
                <span className="text-xs px-2 py-1 bg-yellow-500/20 text-yellow-400 rounded-full status-badge">
                  {appt.status}
                </span>
              </div>
              <p className="text-sm text-neutral-300 mb-4 customer-name">Customer: {appt.customer_name}</p>

              {appt.status === 'Requested' || appt.status === 'Quote Pending' ? (
                <div className="flex gap-2 mt-4">
                  <button
                    className="flex-1 bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 px-4 rounded-lg transition approve-btn"
                    onClick={() => approveQuote(appt.id)}
                  >
                    Approve Quote
                  </button>
                  <button
                    className="flex-1 bg-neutral-700 hover:bg-neutral-600 text-white font-medium py-2 px-4 rounded-lg transition edit-btn"
                  >
                    Edit
                  </button>
                </div>
              ) : null}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
