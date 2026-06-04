'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function IntakeReviewsPage() {
  const [submissions, setSubmissions] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchSubmissions = async () => {
      try {
        const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
        const res = await fetch(`/api/questionnaires/submissions?tenant_id=${tenant}`, {
            headers: { 'X-Tenant-ID': tenant }
        });
        if (res.ok) {
            const data = await res.json();
            setSubmissions(data);
        }
      } catch (err) {
        console.error('Failed to fetch submissions', err);
      } finally {
        setLoading(false);
      }
    };
    fetchSubmissions();
  }, []);

  return (
    <div className="p-4 max-w-4xl mx-auto mt-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Intake Reviews</h1>
      </div>

      {loading ? (
        <p className="text-gray-500">Loading submissions...</p>
      ) : submissions.length === 0 ? (
        <div className="text-center text-gray-500 py-10 bg-gray-50 rounded-lg">
          No intake submissions to review yet.
        </div>
      ) : (
        <div className="space-y-4">
          {submissions.map((sub: any) => (
            <div key={sub.id} className="p-4 border rounded-xl bg-white shadow-sm flex flex-col md:flex-row md:items-center md:justify-between">
              <div>
                <h3 className="font-semibold text-lg">Intake #{sub.id.substring(0,6)}</h3>
                <p className="text-sm text-gray-600 mt-1">Status: <span className="font-medium px-2 py-1 rounded bg-blue-100 text-blue-800">{sub.status}</span></p>
                <div className="mt-2 text-gray-700 bg-gray-50 p-2 rounded text-sm">
                   {sub.summary}
                </div>
              </div>
              <div className="mt-4 md:mt-0 flex flex-col space-y-2">
                 <Link href={`/dashboard/intakes/${sub.id}`} className="bg-black text-white px-4 py-2 rounded-lg text-sm text-center hover:bg-gray-800 transition">
                    Review Quote
                 </Link>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
