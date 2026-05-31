"use client";

import React, { useState, useEffect } from 'react';

interface MaterialAlert {
  id: string;
  name: string;
  current_stock: number;
  threshold: number;
  unit: string;
  suggested_order_qty: number;
  supplier_name: string;
  estimated_cost: number;
}

export default function InventoryDashboard() {
  const [alerts, setAlerts] = useState<MaterialAlert[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [successMsg, setSuccessMsg] = useState<string | null>(null);

  useEffect(() => {
    const fetchAlerts = async () => {
      try {
        const res = await fetch('/api/v1/ops/inventory/alerts', {
           headers: { 'X-Tenant-ID': 'tenant1' }
        });
        if (!res.ok) {
           throw new Error('Failed to fetch inventory alerts');
        }
        const data = await res.json();
        setAlerts(data.alerts || []);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };
    fetchAlerts();
  }, []);

  const handleApprovePO = async (materialId: string) => {
    setError(null);
    setSuccessMsg(null);
    try {
      const res = await fetch('/api/v1/ops/inventory/purchase-orders/approve', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          tenant_id: 'tenant1',
          purchase_order_id: `po_for_${materialId}`, // Simulated ID mapping
        }),
      });

      if (!res.ok) {
         setError('Failed to approve PO');
         return;
      }
      setSuccessMsg(`Approved Purchase Order for ${materialId}`);
      // Remove alert from list optimistically
      setAlerts(alerts.filter(a => a.id !== materialId));
    } catch (err) {
      setError('An error occurred');
    }
  };

  if (loading) {
     return <div className="p-8 text-center"><div className="w-10 h-10 border-4 border-[#0066FF] border-t-transparent rounded-full animate-spin mx-auto"></div></div>;
  }

  return (
    <div className="p-8 font-inter min-h-screen text-[#1D1D1F] dark:text-[#F5F5F7]">
      <header className="mb-8 flex justify-between items-end border-b border-gray-200 dark:border-white/10 pb-4">
        <div>
          <h1 className="text-3xl font-bold font-outfit tracking-tight">Inventory & Alerts</h1>
          <p className="text-gray-500 dark:text-[#A1A1A6]">Manage your raw materials and stock.</p>
        </div>
      </header>

      {error && (
        <div className="mb-6 p-4 bg-red-100/80 dark:bg-red-900/30 text-red-800 dark:text-red-200 rounded-[8px] border border-red-200 dark:border-red-800 backdrop-blur-md">
          {error}
        </div>
      )}

      {successMsg && (
        <div className="mb-6 p-4 bg-green-100/80 dark:bg-green-900/30 text-green-800 dark:text-green-200 rounded-[8px] border border-green-200 dark:border-green-800 backdrop-blur-md">
          {successMsg}
        </div>
      )}

      {alerts.length === 0 ? (
        <div className="text-center p-12 mac-glass-container">
           <h3 className="text-lg font-bold">All Good!</h3>
           <p className="text-gray-500 dark:text-gray-400 mt-2">Your inventory levels are healthy.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {alerts.map(mat => (
            <div key={mat.id} className="mac-glass-container p-5 shadow-lg flex flex-col" data-testid={`alert-card-${mat.id}`}>
              <div className="flex justify-between items-start mb-4">
                <div>
                  <h3 className="font-bold text-lg">{mat.name}</h3>
                  <span className="inline-block mt-1 px-2 py-0.5 bg-red-100 dark:bg-red-900/40 text-red-800 dark:text-red-300 text-xs rounded-full font-semibold">Low Stock</span>
                </div>
                <div className="text-right">
                  <div className="text-2xl font-black text-red-600 dark:text-red-400">{mat.current_stock} <span className="text-sm font-normal text-gray-500 dark:text-gray-400">{mat.unit}</span></div>
                  <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">Threshold: {mat.threshold}</div>
                </div>
              </div>

              <div className="bg-white/40 dark:bg-black/20 rounded-[8px] p-4 mt-auto border border-white/50 dark:border-white/10">
                 <p className="text-sm font-semibold mb-2">AI Suggested Action:</p>
                 <p className="text-sm text-gray-700 dark:text-gray-300 mb-4">
                    Order <strong>{mat.suggested_order_qty} {mat.unit}</strong> from {mat.supplier_name} for ~${mat.estimated_cost}.
                 </p>
                 <button
                    onClick={() => handleApprovePO(mat.id)}
                    className="w-full bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#1D1D1F] font-bold py-2.5 rounded-[8px] shadow-md hover:scale-[1.02] transition-transform active:scale-95"
                    data-testid={`approve-btn-${mat.id}`}
                 >
                    Approve Order
                 </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
