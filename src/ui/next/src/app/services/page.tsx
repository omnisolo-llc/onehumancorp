'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function ServicesPage() {
  const [services, setServices] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/v1/booking/services', {
      headers: {
        'X-Tenant-ID': 'e2e-tenant'
      }
    })
      .then(res => res.json())
      .then(data => {
        setServices(data || []);
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return <div className="p-4 text-center">Loading services...</div>;
  }

  return (
    <div className="p-4 max-w-4xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Services</h1>
        <Link href="/services/new" className="bg-black text-white px-4 py-2 rounded hover:bg-gray-800">
          Add Service
        </Link>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {services.length === 0 ? (
          <div className="col-span-2 text-center py-10 text-gray-500 bg-white rounded shadow">
            No services found. Click "Add Service" to create one.
          </div>
        ) : (
          services.map((s) => (
            <div key={s.id} className="bg-white p-6 rounded shadow border">
              <h2 className="text-xl font-bold mb-2 text-black">{s.title}</h2>
              <p className="text-gray-600 mb-4">{s.description || 'No description provided.'}</p>
              <div className="flex justify-between items-center mt-4 pt-4 border-t">
                <span className="font-medium text-lg">${(s.price_cents / 100).toFixed(2)}</span>
                <Link href={`/book?serviceId=${s.id}`} className="text-blue-600 hover:text-blue-800 font-medium">
                  Book Now →
                </Link>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
