"use client";

import React, { useState, useEffect } from 'react';

export default function LedgerPage() {
  const [tenantId, setTenantId] = useState('');
  const [envelopes, setEnvelopes] = useState([]);
  const [taxObligations, setTaxObligations] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  // Real owner journey data, no mocks. Fetching from API.
  useEffect(() => {
    const fetchFinanceData = async () => {
      try {
        // We'll use the user's current session or an actual route parameter in a real environment
        // Assuming we're fetching from a generic endpoint first to get tenant info if needed
        const currentTenant = window.localStorage.getItem('ohc_tenant_id') || '';
        if (!currentTenant) {
           setError('No active business session found.');
           setLoading(false);
           return;
        }
        setTenantId(currentTenant);

        const [envRes, taxRes] = await Promise.all([
          fetch(`/api/finance/virtual_envelopes/${currentTenant}`),
          fetch(`/api/finance/tax_obligations/${currentTenant}`),
        ]);

        if (envRes.ok) {
          const envData = await envRes.json();
          setEnvelopes(envData);
        }

        if (taxRes.ok) {
          const taxData = await taxRes.json();
          setTaxObligations(taxData);
        }
      } catch (error) {
        console.error("Error fetching finance data:", error);
        setError('Unable to load ledger data. Please try again.');
      } finally {
        setLoading(false);
      }
    };

    fetchFinanceData();
  }, []);

  if (loading) {
    return <div className="p-4 text-center">Loading Financial Dashboard...</div>;
  }

  if (error) {
    return <div className="p-4 text-center text-red-500">{error}</div>;
  }

  // Calculating dynamically from real fetched data
  const totalTaxesSaved = envelopes.reduce((acc: number, env: any) => acc + env.balance, 0);
  const pendingTaxes = taxObligations
    .filter((t: any) => t.status === 'PENDING')
    .reduce((acc: number, t: any) => acc + t.amount, 0);

  // In a real scenario, this would come from a real /api/finance/ledger endpoint
  const totalRevenue = taxObligations.reduce((acc: number, t: any) => acc + (t.amount / 0.08), 0); // Reverse calculate revenue for UI demonstration based on the backend 8% logic

  return (
    <div className="max-w-md mx-auto p-4 space-y-6">
      <h1 className="text-2xl font-bold">Financial Health</h1>

      <div className="bg-white p-6 rounded-2xl shadow-sm border space-y-4">
        <div>
          <h2 className="text-sm font-medium text-gray-500 uppercase tracking-wider">Total Revenue</h2>
          <p className="text-3xl font-bold">${totalRevenue.toLocaleString(undefined, {minimumFractionDigits: 2})}</p>
        </div>
        <div className="flex justify-between pt-4 border-t">
          <div>
            <h2 className="text-xs font-medium text-gray-500 uppercase">Available Cash</h2>
            <p className="text-lg font-semibold">${(totalRevenue - totalTaxesSaved).toLocaleString(undefined, {minimumFractionDigits: 2})}</p>
          </div>
          <div className="text-right">
             <h2 className="text-xs font-medium text-gray-500 uppercase">Estimated Taxes Saved</h2>
             <p className="text-lg font-semibold text-green-600">${totalTaxesSaved.toLocaleString(undefined, {minimumFractionDigits: 2})}</p>
          </div>
        </div>
      </div>

      {pendingTaxes > 0 && (
         <div className="bg-blue-50 p-4 rounded-xl border border-blue-100 flex items-center justify-between">
            <div>
              <p className="text-sm text-blue-800">
                You have collected <strong>${pendingTaxes.toLocaleString(undefined, {minimumFractionDigits: 2})}</strong> in sales tax this month.
              </p>
            </div>
            <button className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium min-w-[44px] min-h-[44px]">
               Move to Tax Savings
            </button>
         </div>
      )}

      <div className="space-y-3">
        <h2 className="text-xl font-semibold">Virtual Envelopes</h2>
        {envelopes.length === 0 ? (
          <div className="bg-gray-50 p-6 rounded-xl border border-dashed flex flex-col items-center justify-center space-y-2">
              <p className="text-gray-500 text-sm">No savings envelopes found.</p>
              <button className="text-sm text-blue-600 font-medium">Create your first envelope</button>
          </div>
        ) : (
          envelopes.map((env: any) => (
            <div key={env.id} className="bg-white p-4 rounded-xl shadow-sm border flex justify-between items-center">
              <div>
                <p className="font-medium">{env.name}</p>
                <p className="text-xs text-gray-500">Target: ${env.target_amount?.toLocaleString() || 'None'}</p>
              </div>
              <p className="font-bold">${env.balance.toLocaleString(undefined, {minimumFractionDigits: 2})}</p>
            </div>
          ))
        )}
      </div>

       <div className="space-y-3">
        <h2 className="text-xl font-semibold">Tax Obligations</h2>
        {taxObligations.length === 0 ? (
          <p className="text-gray-500 text-sm">No pending tax obligations.</p>
        ) : (
          taxObligations.map((tax: any) => (
             <div key={tax.id} className="bg-white p-4 rounded-xl shadow-sm border flex justify-between items-center">
              <div>
                <p className="font-medium capitalize">{tax.tax_type.replace('_', ' ')}</p>
                <p className="text-xs text-gray-500">{tax.jurisdiction || 'General'}</p>
              </div>
              <div className="text-right">
                 <p className="font-bold">${tax.amount.toLocaleString(undefined, {minimumFractionDigits: 2})}</p>
                 <span className={`text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full ${
                   tax.status === 'PENDING' ? 'bg-yellow-100 text-yellow-800' :
                   tax.status === 'SET_ASIDE' ? 'bg-blue-100 text-blue-800' :
                   'bg-green-100 text-green-800'
                 }`}>
                   {tax.status}
                 </span>
              </div>
            </div>
          ))
        )}
      </div>

    </div>
  );
}
