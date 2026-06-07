"use client";

import React, { useEffect, useState } from 'react';
import { useRouter, useParams } from 'next/navigation';

export default function CustomerProfilePage() {
  const router = useRouter();
  const params = useParams();
  const [customer, setCustomer] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  // Dynamic tenant retrieval
  const [tenantId, setTenantId] = useState('e2e-tenant');
  const customerId = params.id;

  useEffect(() => {
    // Determine tenant dynamically on client side
    const storedTenant = localStorage.getItem('current_tenant_id') || localStorage.getItem('tenant_id');
    if (storedTenant) {
      setTenantId(storedTenant);
    }
  }, []);

  useEffect(() => {
    async function fetchCustomer() {
      if (!customerId) return;
      try {
        const token = localStorage.getItem('token') || '';
        const res = await fetch(`/api/v1/growth/customers/${tenantId}/${customerId}/customer360`, {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });

        const json = await res.json();
        if (res.ok && json.success) {
          setCustomer(json.data);
        } else {
          setError(json.error || 'Failed to load customer profile.');
        }
      } catch (err) {
        setError('An unexpected error occurred.');
      } finally {
        setLoading(false);
      }
    }
    fetchCustomer();
  }, [customerId, tenantId]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] glassmorphism rounded-[16px] shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button aria-label="Back" onClick={() => router.back()} className="text-gray-500">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          </button>
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Customer Profile</h1>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6 space-y-6">
          {loading && <div className="text-center text-gray-500 text-sm mt-10">Loading profile...</div>}
          {error && <div className="text-center text-red-500 text-sm mt-10">{error}</div>}

          {!loading && !error && customer && (
            <div className="bg-white/60 backdrop-blur-md border border-gray-200 rounded-xl p-6 shadow-sm">
              <div className="flex items-center gap-4 mb-6">
                <div className="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 text-2xl font-bold text-blue-600 uppercase">
                   {(customer.name || customer.customer_id || 'C').charAt(0)}
                </div>
                <div>
                  <h2 className="text-lg font-bold text-gray-900">{customer.name || 'Anonymous Customer'}</h2>
                  <p className="text-sm text-gray-500">{customer.email || 'No email provided'}</p>
                </div>
              </div>

              <div className="space-y-4">
                <div>
                  <h3 className="text-sm font-semibold text-gray-700 uppercase tracking-wider mb-2">Known Preferences</h3>
                  {customer.preferences && customer.preferences.length > 0 ? (
                    <div className="flex flex-wrap gap-2">
                      {customer.preferences.map((pref: string, idx: number) => (
                        <span key={idx} className="bg-blue-50 text-blue-700 text-xs font-semibold px-3 py-1 rounded-full border border-blue-200">
                          {pref}
                        </span>
                      ))}
                    </div>
                  ) : (
                    <p className="text-sm text-gray-500 italic">No preferences recorded yet.</p>
                  )}
                </div>

                {customer.mood && (
                  <div>
                    <h3 className="text-sm font-semibold text-gray-700 uppercase tracking-wider mb-2">Current Mood</h3>
                    <p className="text-sm text-gray-900 capitalize">{customer.mood}</p>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
