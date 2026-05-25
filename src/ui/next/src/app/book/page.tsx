'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function BookServicePage() {
  const router = useRouter();
  const [services, setServices] = useState<any[]>([]);
  const [selectedService, setSelectedService] = useState<string>('');
  const [date, setDate] = useState('');
  const [time, setTime] = useState('');
  const [status, setStatus] = useState('');

  useEffect(() => {
    fetch('/api/v1/booking/services', {
      headers: {
        'X-Tenant-ID': 'e2e-tenant'
      }
    })
      .then(res => res.json())
      .then(data => {
        setServices(data || []);
      })
      .catch(err => console.error(err));
  }, []);

  const handleBook = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedService || !date || !time) return;

    setStatus('Booking...');

    // Combine date and time to ISO string
    const startDateTime = new Date(`${date}T${time}`).toISOString();

    // Default 1 hour duration

    const durationSelect = document.getElementById('duration') as HTMLSelectElement;
    const durationHours = durationSelect ? parseFloat(durationSelect.value) : 1;
    const endDate = new Date(new Date(`${date}T${time}`).getTime() + durationHours * 60 * 60 * 1000);

    const endDateTime = endDate.toISOString();

    const booking = {
      id: crypto.randomUUID(),
      tenant_id: 'e2e-tenant',
      customer_id: 'customer-123',
      order_id: selectedService, // backend uses order_id, but the struct name is order_id
      start_time: startDateTime,
      end_time: endDateTime,
      status: 'scheduled'
    };

    try {
      const res = await fetch('/api/v1/booking/records', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': 'e2e-tenant'
        },
        body: JSON.stringify(booking),
      });

      if (res.ok) {
        setStatus('Successfully booked!');
        setTimeout(() => router.push('/calendar'), 2000);
      } else {
        setStatus('Failed to book.');
      }
    } catch (err) {
      console.error(err);
      setStatus('Failed to book.');
    }
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded shadow">
      <h1 className="text-2xl font-bold mb-6 text-black">Book a Service</h1>
      <form onSubmit={handleBook} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700">Select Service</label>
          <select
            className="mt-1 block w-full rounded border-gray-300 p-2 text-black border"
            value={selectedService}
            onChange={(e) => setSelectedService(e.target.value)}
            required
          >
            <option value="">-- Choose a Service --</option>
            {services.map(s => (
              <option key={s.id} value={s.id}>{s.title}</option>
            ))}
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">Date</label>
          <input
            type="date"
            className="mt-1 block w-full rounded border-gray-300 p-2 text-black border"
            value={date}
            onChange={(e) => setDate(e.target.value)}
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">Duration</label>
          <select
            id="duration"
            className="mt-1 block w-full rounded border-gray-300 p-2 text-black border mb-4"
            defaultValue="1"
          >
            <option value="0.5">30 Minutes</option>
            <option value="1">1 Hour</option>
            <option value="1.5">1.5 Hours</option>
            <option value="2">2 Hours</option>
            <option value="3">3 Hours</option>
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700">Time</label>
          <input
            type="time"
            className="mt-1 block w-full rounded border-gray-300 p-2 text-black border"
            value={time}
            onChange={(e) => setTime(e.target.value)}
            required
          />
        </div>

        <button
          type="submit"
          className="w-full bg-blue-600 text-white p-2 rounded hover:bg-blue-700"
        >
          Confirm Booking
        </button>
      </form>

      {status && (
        <div className="mt-4 text-center text-sm font-medium text-gray-800">
          {status}
        </div>
      )}
    </div>
  );
}
