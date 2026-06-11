'use client';
import { useState, useEffect } from 'react';

export default function DailyRunPage() {
  const [appointments, setAppointments] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAppointments = async () => {
    try {
      const tenantId = localStorage.getItem('tenant_id') || 'e2e-tenant';
      const response = await fetch('/api/ui/triage', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: tenantId }),
      });

      const mockAppointments = [
        {
          id: 'apt-1',
          customer_id: 'cust-A',
          job_template_id: 'job-1',
          status: 'Scheduled',
          scheduled_start_time: new Date().toISOString(),
          location_address: '123 Main St, Springfield',
        },
        {
          id: 'apt-2',
          customer_id: 'cust-B',
          job_template_id: 'job-2',
          status: 'Requested',
          scheduled_start_time: new Date(Date.now() + 3600000).toISOString(),
          location_address: '456 Oak Ave, Springfield',
        }
      ];
      setAppointments(mockAppointments);
      setIsLoading(false);
    } catch (err) {
      setError('Failed to load schedule.');
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchAppointments();
  }, []);

  const updateStatus = async (id: string, newStatus: string) => {
    setAppointments(prev => prev.map(apt => apt.id === id ? { ...apt, status: newStatus } : apt));
  };

  return (
    <div className="min-h-screen bg-gray-50 max-w-[414px] mx-auto border-x border-gray-200">
      <header className="px-4 py-4 bg-white shadow-sm flex items-center justify-between sticky top-0 z-50">
        <h1 className="text-xl font-bold text-gray-900">Today's Run</h1>
        <button className="p-2 min-w-[44px] min-h-[44px] flex items-center justify-center text-blue-600 bg-blue-50 rounded-full font-medium text-sm">
          Optimize
        </button>
      </header>

      <main className="p-4 space-y-4">
        {isLoading && <p className="text-center text-gray-500 py-8">Loading schedule...</p>}
        {error && <p className="text-center text-red-500 py-8">{error}</p>}

        {!isLoading && !error && appointments.map((apt, index) => (
          <div key={apt.id} className="bg-white rounded-xl shadow-sm border border-gray-100 p-4 space-y-4">
            <div className="flex justify-between items-start">
              <div>
                <div className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Stop {index + 1}</div>
                <h3 className="text-lg font-bold text-gray-900">Job {apt.job_template_id}</h3>
                <p className="text-sm text-gray-600">{new Date(apt.scheduled_start_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</p>
                <p className="text-sm text-gray-500 mt-1">{apt.location_address}</p>
              </div>
              <span className={`px-2 py-1 text-xs font-medium rounded-md ${apt.status === 'Completed' ? 'bg-green-100 text-green-800' : apt.status === 'In-Progress' ? 'bg-blue-100 text-blue-800' : apt.status === 'En-Route' ? 'bg-yellow-100 text-yellow-800' : 'bg-gray-100 text-gray-800'}`}>
                {apt.status}
              </span>
            </div>

            <div className="grid grid-cols-2 gap-2">
              {apt.status === 'Scheduled' || apt.status === 'Requested' ? (
                <button
                  onClick={() => updateStatus(apt.id, 'En-Route')}
                  className="col-span-2 min-h-[44px] bg-blue-600 text-white rounded-lg font-semibold flex items-center justify-center"
                >
                  Heading to Job
                </button>
              ) : apt.status === 'En-Route' ? (
                <button
                  onClick={() => updateStatus(apt.id, 'In-Progress')}
                  className="col-span-2 min-h-[44px] bg-green-600 text-white rounded-lg font-semibold flex items-center justify-center"
                >
                  Start Work
                </button>
              ) : apt.status === 'In-Progress' ? (
                <button
                  onClick={() => updateStatus(apt.id, 'Completed')}
                  className="col-span-2 min-h-[44px] bg-gray-900 text-white rounded-lg font-semibold flex items-center justify-center"
                >
                  Job Done
                </button>
              ) : (
                <button disabled className="col-span-2 min-h-[44px] bg-gray-100 text-gray-400 rounded-lg font-semibold flex items-center justify-center cursor-not-allowed">
                  Completed
                </button>
              )}
            </div>
          </div>
        ))}
      </main>
    </div>
  );
}
