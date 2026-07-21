"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

interface CreditFacility {
  id: string;
  tenant_id: string;
  approved_limit_usd: number;
  utilized_amount_usd: number;
  dynamic_score: number;
  underwriter_version?: string;
  updated_at?: string;
}

interface VendorRelation {
  id: string;
  tenant_id: string;
  vendor_name: string;
  vendor_email: string;
  current_terms: string;
  term_status: string;
  terms_granted_at?: string;
}

interface SupplierInvoice {
  id: string;
  tenant_id: string;
  vendor_relation_id: string;
  invoice_number: string;
  total_amount: number;
  currency: string;
  due_date?: string;
  status: string;
}

interface FactoringDiscount {
  id: string;
  tenant_id: string;
  client_invoice_id: string;
  invoice_amount: number;
  advance_rate: number;
  flat_fee_pct: number;
  advanced_amount_usd: number;
  factoring_status: string;
}

interface LedgerSweepConfig {
  id: string;
  supplier_invoice_id: string;
  daily_sweep_pct: number;
  maximum_sweep_usd?: number;
  accumulated_sweep_usd: number;
  last_sweep_run?: string;
}

export default function CreditHubPage() {
  const router = useRouter();

  // State variables
  const [activeTab, setActiveTab] = useState<'vendors' | 'factoring'>('vendors');
  const [capacity, setCapacity] = useState<CreditFacility | null>(null);
  const [vendors, setVendors] = useState<VendorRelation[]>([]);
  const [selectedVendorId, setSelectedVendorId] = useState<string>('');

  // Refinancing & Factoring states
  const [clientInvoiceId, setClientInvoiceId] = useState<string>('inv-client-999');
  const [invoiceAmount, setInvoiceAmount] = useState<number>(5000);
  const [factoringResult, setFactoringResult] = useState<FactoringDiscount | null>(null);

  // Sweep states
  const [sweepInvoiceId, setSweepInvoiceId] = useState<string>('inv-supplier-777');
  const [sweepSalesAmount, setSweepSalesAmount] = useState<number>(1000);
  const [sweepResult, setSweepResult] = useState<LedgerSweepConfig | null>(null);

  const [loading, setLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(false);

  // Auth headers helper
  const getAuthHeaders = () => {
    if (typeof window === 'undefined') return {};
    const token = localStorage.getItem('token') || '';
    const tenantId = localStorage.getItem('tenant_id') || 'default_tenant';
    return {
      'Authorization': `Bearer ${token}`,
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json'
    };
  };

  // Fetch initial data
  const fetchData = async () => {
    try {
      setLoading(true);
      const headers = getAuthHeaders();

      // Fetch capacity
      const capRes = await fetch('/api/v1/ui/credit/capacity', { headers });
      if (capRes.ok) {
        const capJson = await capRes.json();
        setCapacity(capJson);
      }

      // Fetch vendors
      const venRes = await fetch('/api/v1/ui/credit/vendors', { headers });
      if (venRes.ok) {
        const venJson = await venRes.json();
        setVendors(venJson);
        if (venJson.length > 0) {
          setSelectedVendorId(venJson[0].id);
        }
      }
    } catch (e) {
      console.error('Failed to fetch credit hub data', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  // AI-led Terms Negotiation
  const handleNegotiate = async () => {
    if (!selectedVendorId) return;
    setActionLoading(true);
    try {
      const response = await fetch('/api/v1/ui/credit/negotiate', {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify({ vendor_relation_id: selectedVendorId })
      });
      if (response.ok) {
        const updatedVendor = await response.json();
        // Update list
        setVendors(prev => prev.map(v => v.id === updatedVendor.id ? updatedVendor : v));
        alert(`AI Negotiation triggered! Status: ${updatedVendor.term_status}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setActionLoading(false);
    }
  };

  // Run Sales Sweep
  const handleSweep = async () => {
    setActionLoading(true);
    try {
      const response = await fetch('/api/v1/ui/credit/sweep', {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify({
          supplier_invoice_id: sweepInvoiceId,
          sales_amount: sweepSalesAmount
        })
      });
      if (response.ok) {
        const result = await response.json();
        setSweepResult(result);
        alert(`Sales Sweep run successfully! Accumulated reserve: $${result.accumulated_sweep_usd}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setActionLoading(false);
    }
  };

  // Submit Factoring Payout
  const handleFactor = async () => {
    setActionLoading(true);
    try {
      const response = await fetch('/api/v1/ui/credit/factor', {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify({
          client_invoice_id: clientInvoiceId,
          invoice_amount: invoiceAmount
        })
      });
      if (response.ok) {
        const result = await response.json();
        setFactoringResult(result);
        alert(`Invoice Factored successfully! Advanced Amount: $${result.advanced_amount_usd}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setActionLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="flex flex-col min-h-screen bg-gradient-to-br from-teal-50 via-white to-emerald-50 justify-center items-center p-4">
        <div className="flex flex-col items-center justify-center p-8 glass-card backdrop-blur-xl bg-white/10 border border-white/20 shadow-lg rounded-2xl w-full max-w-sm animate-pulse">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-teal-600"></div>
          <p className="mt-6 text-gray-600 font-medium">Underwriting credit lines...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-teal-50 via-white to-emerald-50 text-gray-900 w-full overflow-x-hidden max-w-[100vw]">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 shadow-sm w-full glass-panel backdrop-blur-xl bg-white/40 border-b border-white/20">
        <div className="flex items-center gap-3">
          <button
            id="back-button"
            onClick={() => router.push('/dashboard')}
            className="min-w-[44px] min-h-[44px] px-3 py-2 glass-card backdrop-blur-md bg-white/30 border border-white/30 shadow-sm rounded-xl text-sm font-medium text-gray-800 hover:-translate-y-0.5 hover:shadow-md transition-all duration-300 flex items-center justify-center">
            Back
          </button>
          <h1 className="text-xl md:text-2xl font-bold text-gray-900 tracking-tight">OHC Vendor Credit Hub</h1>
        </div>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        {/* Credit Capacity Pulse Card */}
        <section id="credit-capacity-card" className="glass-card backdrop-blur-2xl bg-white/40 border border-white/40 shadow-lg hover:shadow-2xl transition-all duration-300 p-6 rounded-2xl flex flex-col gap-4">
          <div className="flex justify-between items-start">
            <div>
              <span className="text-sm font-medium text-teal-700 uppercase tracking-wider">Business Credit Capacity</span>
              <h2 className="text-2xl md:text-3xl font-extrabold text-teal-900 mt-1">
                Your Capacity is Good (${capacity?.approved_limit_usd?.toLocaleString() || '7,500'})
              </h2>
            </div>
            <div className="bg-emerald-500/10 text-emerald-800 font-bold px-3 py-1.5 rounded-full text-sm flex items-center gap-1.5 border border-emerald-500/20 shadow-sm">
              <span className="h-2 w-2 rounded-full bg-emerald-500 animate-ping"></span>
              Score: {capacity?.dynamic_score?.toFixed(1) || '75.0'}
            </div>
          </div>

          {/* Utilization Progress Bar */}
          <div className="mt-4">
            <div className="flex justify-between text-sm font-medium text-gray-700 mb-1.5">
              <span>Utilized: ${capacity?.utilized_amount_usd?.toLocaleString() || '1,200'}</span>
              <span>Available: ${( (capacity?.approved_limit_usd || 7500) - (capacity?.utilized_amount_usd || 1200) ).toLocaleString()}</span>
            </div>
            <div className="w-full bg-teal-950/10 rounded-full h-3 overflow-hidden border border-white/30">
              <div
                className="bg-gradient-to-r from-teal-500 to-emerald-600 h-3 rounded-full transition-all duration-500"
                style={{ width: `${Math.min(100, (((capacity?.utilized_amount_usd || 1200) / (capacity?.approved_limit_usd || 7500)) * 100))}%` }}>
              </div>
            </div>
          </div>
        </section>

        {/* Tab Controls */}
        <section className="flex gap-2 p-1 bg-teal-950/5 rounded-xl border border-teal-950/10">
          <button
            id="tab-vendors"
            onClick={() => setActiveTab('vendors')}
            className={`flex-1 py-3 text-center text-sm font-medium rounded-lg transition-all min-h-[44px] ${activeTab === 'vendors' ? 'bg-white text-teal-900 shadow-md border border-white/20' : 'text-teal-800 hover:bg-white/10'}`}>
            Vendor Terms
          </button>
          <button
            id="tab-factoring"
            onClick={() => setActiveTab('factoring')}
            className={`flex-1 py-3 text-center text-sm font-medium rounded-lg transition-all min-h-[44px] ${activeTab === 'factoring' ? 'bg-white text-teal-900 shadow-md border border-white/20' : 'text-teal-800 hover:bg-white/10'}`}>
            Invoice Advance
          </button>
        </section>

        {/* Tab A Content: Vendor Terms & Sweeping */}
        {activeTab === 'vendors' && (
          <section id="vendors-tab" className="flex flex-col gap-6">
            <div className="glass-card backdrop-blur-xl bg-white/25 border border-white/30 p-6 rounded-2xl flex flex-col gap-4">
              <h3 className="text-lg font-bold text-teal-900">Request Net Terms via AI Assistant</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                The OHC Assistant negotiates Net-30/60 terms with your listed vendors, compiling a secure zero-knowledge proof of your transaction velocity. No sensitive private data is shared.
              </p>

              <div className="flex flex-col gap-3 mt-2">
                <label className="text-xs font-semibold text-teal-800 uppercase tracking-wide">Select Wholesale Vendor</label>
                <select
                  id="vendor-select"
                  value={selectedVendorId}
                  onChange={(e) => setSelectedVendorId(e.target.value)}
                  className="w-full min-h-[44px] px-3 py-2.5 bg-white/50 border border-teal-950/10 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-teal-500">
                  {vendors.map(v => (
                    <option key={v.id} value={v.id}>
                      {v.vendor_name} ({v.current_terms} - {v.term_status})
                    </option>
                  ))}
                </select>
              </div>

              <button
                id="negotiate-btn"
                onClick={handleNegotiate}
                disabled={actionLoading || !selectedVendorId}
                className="mt-4 min-h-[44px] w-full px-4 py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white rounded-xl font-semibold shadow-sm hover:shadow-md transition-all active:scale-95 duration-200 disabled:opacity-50">
                {actionLoading ? 'AI Negotiating...' : 'Let AI Negotiate Net-30 Terms'}
              </button>
            </div>

            {/* Daily Sweeping Simulation Card */}
            <div className="glass-card backdrop-blur-xl bg-white/25 border border-white/30 p-6 rounded-2xl flex flex-col gap-4">
              <h3 className="text-lg font-bold text-teal-900">Idempotent Daily Sales Sweep</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                Set aside 10% of daily sales dynamically to pay off supplier invoice <span className="font-semibold text-teal-900">{sweepInvoiceId}</span> smoothly.
              </p>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-2">
                <div className="flex flex-col gap-2">
                  <label className="text-xs font-semibold text-teal-800 uppercase tracking-wide">Supplier Invoice ID</label>
                  <input
                    id="sweep-invoice-input"
                    type="text"
                    value={sweepInvoiceId}
                    onChange={(e) => setSweepInvoiceId(e.target.value)}
                    className="min-h-[44px] px-3 py-2 bg-white/50 border border-teal-950/10 rounded-xl text-sm"
                  />
                </div>
                <div className="flex flex-col gap-2">
                  <label className="text-xs font-semibold text-teal-800 uppercase tracking-wide">Daily Sales Amount ($)</label>
                  <input
                    id="sweep-amount-input"
                    type="number"
                    value={sweepSalesAmount}
                    onChange={(e) => setSweepSalesAmount(parseFloat(e.target.value) || 0)}
                    className="min-h-[44px] px-3 py-2 bg-white/50 border border-teal-950/10 rounded-xl text-sm"
                  />
                </div>
              </div>

              <button
                id="sweep-btn"
                onClick={handleSweep}
                disabled={actionLoading}
                className="mt-4 min-h-[44px] w-full px-4 py-3 bg-emerald-600 hover:bg-emerald-700 text-white rounded-xl font-semibold shadow-sm hover:shadow-md transition-all active:scale-95 duration-200">
                {actionLoading ? 'Running Sweep...' : 'Simulate Daily Sweep'}
              </button>

              {sweepResult && (
                <div id="sweep-result" className="mt-4 p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-xl text-teal-950">
                  <h4 className="font-bold text-sm">Sweep Confirmed</h4>
                  <p className="text-xs mt-1">Accumulated Reserve: ${sweepResult.accumulated_sweep_usd.toFixed(2)}</p>
                  <p className="text-xs">Last Sweep: {sweepResult.last_sweep_run ? new Date(sweepResult.last_sweep_run).toLocaleString() : 'Just now'}</p>
                </div>
              )}
            </div>
          </section>
        )}

        {/* Tab B Content: Invoice Factoring */}
        {activeTab === 'factoring' && (
          <section id="factoring-tab" className="glass-card backdrop-blur-xl bg-white/25 border border-white/30 p-6 rounded-2xl flex flex-col gap-4">
            <h3 className="text-lg font-bold text-teal-900">Instant Invoice Factoring & Refinancing</h3>
            <p className="text-sm text-gray-700 leading-relaxed">
              Refinance outstanding client contracts up to 85% of face value instantly. Enjoy micro-payouts within minutes for a small flat 2% fee, while OHC handles collecting the remainder behind the scenes.
            </p>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-2">
              <div className="flex flex-col gap-2">
                <label className="text-xs font-semibold text-teal-800 uppercase tracking-wide">Client Invoice ID</label>
                <input
                  id="client-invoice-input"
                  type="text"
                  value={clientInvoiceId}
                  onChange={(e) => setClientInvoiceId(e.target.value)}
                  className="min-h-[44px] px-3 py-2 bg-white/50 border border-teal-950/10 rounded-xl text-sm"
                />
              </div>
              <div className="flex flex-col gap-2">
                <label className="text-xs font-semibold text-teal-800 uppercase tracking-wide">Invoice Amount ($)</label>
                <input
                  id="invoice-amount-input"
                  type="number"
                  value={invoiceAmount}
                  onChange={(e) => setInvoiceAmount(parseFloat(e.target.value) || 0)}
                  className="min-h-[44px] px-3 py-2 bg-white/50 border border-teal-950/10 rounded-xl text-sm"
                />
              </div>
            </div>

            <button
              id="factor-btn"
              onClick={handleFactor}
              disabled={actionLoading}
              className="mt-4 min-h-[44px] w-full px-4 py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white rounded-xl font-semibold shadow-sm hover:shadow-md transition-all active:scale-95 duration-200">
              {actionLoading ? 'Disbursing via ACH...' : 'Advance Funds Instantly'}
            </button>

            {factoringResult && (
              <div id="factoring-result" className="mt-4 p-4 bg-teal-500/10 border border-teal-500/20 rounded-xl text-teal-950 flex flex-col gap-1">
                <h4 className="font-bold text-sm text-teal-900">Advance Disbursed</h4>
                <p className="text-xs">Advanced Amount: <span className="font-semibold">${factoringResult.advanced_amount_usd.toLocaleString()}</span></p>
                <p className="text-xs">Advance Rate: {factoringResult.advance_rate * 100}%</p>
                <p className="text-xs">Flat Fee: {factoringResult.flat_fee_pct * 100}%</p>
                <p className="text-xs">Status: <span className="text-emerald-700 font-bold uppercase">{factoringResult.factoring_status}</span></p>
              </div>
            )}
          </section>
        )}

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
      `}} />
    </div>
  );
}
