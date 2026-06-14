"use client";

import React, { useEffect, useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import Link from 'next/link';
import { PoweredByOHC } from '../../components/PoweredByOHC';

function InvoiceContent() {
  const searchParams = useSearchParams();
  const [invoiceData, setInvoiceData] = useState<{
    tenant: string;
    clientName: string;
    projectDetails: string;
    amount: string;
  } | null>(null);
  const [error, setError] = useState(false);

  // Use a deterministic seed to generate a stable invoice ID based on client name
  const [invoiceId, setInvoiceId] = useState('');

  useEffect(() => {
    const dataParam = searchParams.get('data');
    if (dataParam) {
      try {
        // Base64Url decode logic
        let base64Str = dataParam.replace(/-/g, '+').replace(/_/g, '/');
        while (base64Str.length % 4) {
          base64Str += '=';
        }
        const utf8Encoded = escape(atob(base64Str));
        const decodedData = JSON.parse(decodeURIComponent(utf8Encoded));
        setInvoiceData(decodedData);

        // Generate stable ID
        let hash = 0;
        const hashStr = decodedData.clientName + decodedData.amount;
        for (let i = 0; i < hashStr.length; i++) {
          hash = ((hash << 5) - hash) + hashStr.charCodeAt(i);
          hash |= 0;
        }
        setInvoiceId(Math.abs(hash).toString().substring(0, 6).padStart(4, '0'));
      } catch (err) {
        console.error("Failed to parse invoice data", err);
        setError(true);
      }
    } else {
        setError(true);
    }
  }, [searchParams]);

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7] font-inter">
        <p className="text-red-500 bg-red-50 p-4 rounded-xl">Error: Invalid or corrupted invoice data.</p>
      </div>
    );
  }

  if (!invoiceData) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7] font-inter">
        <p className="text-gray-500 animate-pulse">Loading invoice...</p>
      </div>
    );
  }

  const { tenant, clientName, projectDetails, amount } = invoiceData;
  const currentDate = new Date().toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <main className="p-4 md:p-8 flex-1 w-full max-w-3xl mx-auto flex flex-col justify-center gap-8">
        <section className="bg-white p-8 md:p-12 shadow-xl border border-gray-100 relative" style={{ borderRadius: '16px' }}>

          <div className="flex justify-between items-start mb-10 border-b border-gray-100 pb-8">
            <div>
              <h1 className="text-3xl font-bold font-outfit text-gray-900 uppercase tracking-wider mb-2">INVOICE</h1>
              <p className="text-sm text-gray-500">Invoice Date: {currentDate}</p>
              <p className="text-sm text-gray-500">Invoice ID: #{invoiceId}</p>
            </div>
            <div className="text-right">
              <h2 className="text-xl font-bold text-indigo-600 mb-1">{tenant}</h2>
              <p className="text-sm text-gray-500">Automated Billing Dept</p>
            </div>
          </div>

          <div className="mb-10">
            <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Billed To</h3>
            <p className="text-lg font-medium text-gray-900">{clientName}</p>
          </div>

          <div className="mb-10">
            <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Description</h3>
            <p className="text-md text-gray-700 whitespace-pre-wrap">{projectDetails}</p>
          </div>

          <div className="flex justify-between items-center border-t border-gray-100 pt-6 mt-8">
            <span className="text-xl font-bold text-gray-900 font-outfit">Total Due</span>
            <span className="text-3xl font-bold text-indigo-600 font-outfit">${parseFloat(amount).toFixed(2)}</span>
          </div>

          <button className="w-full mt-10 py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all text-lg flex items-center justify-center gap-2">
            Pay Invoice Securely
          </button>
        </section>

        {/* Viral Growth Loop Footer */}
        <div className="text-center pb-8 animate-fade-in flex flex-col items-center">
          <PoweredByOHC tenantId={tenant} />
          <Link
            href={`/onboarding?ref=${tenant}&source=invoice_generator`}
            target="_blank"
            className="inline-flex flex-col items-center gap-1 group mt-2"
          >
            <span className="text-sm font-medium text-indigo-600 group-hover:text-indigo-700 transition-colors">
              Create your own professional invoices for free
            </span>
          </Link>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
      `}} />
    </div>
  );
}

export default function InvoiceViewPage() {
  return (
    <Suspense fallback={
      <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7] font-inter">
        <p className="text-gray-500 animate-pulse">Loading invoice component...</p>
      </div>
    }>
      <InvoiceContent />
    </Suspense>
  );
}
