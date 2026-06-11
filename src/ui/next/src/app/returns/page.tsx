"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';

type ReturnRequest = {
  id: string;
  tenant_id: string;
  order_id: string;
  customer_id: string;
  product_id: string;
  reason: string;
  action_type: string;
  status: string;
  refund_amount_cents: number;
};

export default function ReturnsDashboard() {
  const [returns, setReturns] = useState<ReturnRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [actionStatus, setActionStatus] = useState("");

  const fetchReturns = async () => {
    try {
      setLoading(true);
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      const response = await fetch('/api/v1/returns', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
      if (response.ok) {
        const data = await response.json();
        setReturns(data);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchReturns();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      setActionStatus("Processing...");
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      const response = await fetch(`/api/v1/returns/${id}/approve`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
      if (response.ok) {
        setActionStatus("Return Approved Successfully");
        fetchReturns();
      } else {
        setActionStatus("Failed to approve return");
      }
    } catch (e) {
      console.error(e);
      setActionStatus("Error processing return");
    }
  };

  return (
    <AppShell>
      <div className="p-4 sm:p-8 max-w-4xl mx-auto min-h-screen">
        <h1 className="text-2xl font-bold mb-6 text-gray-900 dark:text-white">Omnichannel Returns & Exchanges</h1>

        {actionStatus && (
          <div className="mb-4 p-4 rounded-lg bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 border border-emerald-500/20 backdrop-blur-md">
            {actionStatus}
          </div>
        )}

        {loading ? (
          <div className="text-gray-500">Loading requests...</div>
        ) : returns.length === 0 ? (
          <div className="text-gray-500 p-8 text-center bg-white/5 backdrop-blur-md rounded-2xl border border-white/10 shadow-sm">
            No return requests found.
          </div>
        ) : (
          <div className="space-y-4">
            {returns.map((req) => (
              <div key={req.id} className="p-5 rounded-2xl bg-white/10 dark:bg-black/10 backdrop-blur-md border border-white/20 dark:border-white/10 shadow-lg" data-testid={`return-card-${req.id}`}>
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div>
                    <h3 className="font-semibold text-lg text-gray-900 dark:text-white">Order #{req.order_id}</h3>
                    <p className="text-gray-600 dark:text-gray-300 text-sm mt-1">Customer: {req.customer_id} • Product: {req.product_id}</p>
                    <p className="text-gray-600 dark:text-gray-300 text-sm mt-1">Reason: <span className="italic">{req.reason}</span></p>
                    <div className="mt-2 flex gap-2">
                      <span className="px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-500/10 text-blue-700 dark:text-blue-400 border border-blue-500/20">
                        {req.action_type.toUpperCase()}
                      </span>
                      <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium border ${req.status === 'pending' ? 'bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20' : 'bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 border-emerald-500/20'}`}>
                        {req.status.toUpperCase()}
                      </span>
                    </div>
                  </div>

                  <div className="flex flex-col items-end gap-3">
                    <span className="font-semibold text-gray-900 dark:text-white">
                      ${(req.refund_amount_cents / 100).toFixed(2)}
                    </span>
                    {req.status === 'pending' && (
                      <button
                        onClick={() => handleApprove(req.id)}
                        className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white text-sm font-medium rounded-xl shadow-sm transition-all active:scale-95"
                        data-testid={`approve-btn-${req.id}`}
                      >
                        Approve
                      </button>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </AppShell>
  );
}
