"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';

export default function FinancePage() {
    const [invoices, setInvoices] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [showDraftModal, setShowDraftModal] = useState(false);
    const [draftInvoice, setDraftInvoice] = useState<any>(null);
    const [error, setError] = useState<string | null>(null);

    const fetchInvoices = async () => {
        try {
            const res = await fetch('/api/v1/invoices');
            if (!res.ok) throw new Error('Invoice data is unavailable.');
            const data = await res.json();
            if (!Array.isArray(data.invoices)) throw new Error('Invoice data is unavailable.');
            setInvoices(data.invoices);
        } catch {
            setError('Invoice data is unavailable.');
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchInvoices();
    }, []);

    return (
        <AppShell title="Finance">
            <main className="p-4 md:p-8 flex-1 w-full max-w-6xl mx-auto space-y-6 md:space-y-12 pb-24">
                <header className="mb-4">
                    <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">Finance & Invoicing</h1>
                    <p className="text-gray-500 mt-2 text-sm">Manage your cash flow, invoices, and deposits.</p>
                </header>
                {error && <p className="text-sm text-red-600" role="status">{error}</p>}

                {loading ? (
                    <div className="flex justify-center py-12">
                        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
                    </div>
                ) : (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {invoices.map((invoice, idx) => (
                            <div key={idx} className="bg-white dark:bg-gray-800 p-6 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm relative overflow-hidden group">
                                <div className="absolute top-0 right-0 p-4">
                                    <span className={`px-2 py-1 text-xs font-semibold rounded-full ${
                                        invoice.status === 'draft' ? 'bg-yellow-100 text-yellow-800' :
                                        invoice.status === 'paid' ? 'bg-green-100 text-green-800' :
                                        'bg-gray-100 text-gray-800'
                                    }`}>
                                        {invoice.status.toUpperCase()}
                                    </span>
                                </div>
                                <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-1">{invoice.client_name}</h3>
                                <p className="text-sm text-gray-500 mb-4">{invoice.id}</p>

                                <div className="flex justify-between items-end mt-4 pt-4 border-t border-gray-100 dark:border-gray-700">
                                    <div>
                                        <p className="text-xs text-gray-500">Amount Due</p>
                                        <p className="text-xl font-bold text-gray-900 dark:text-white">{invoice.base_currency === invoice.transaction_currency ? "$" + invoice.total_amount.toFixed(2) : `$${invoice.total_amount.toFixed(2)} (${invoice.transaction_currency})`}</p>
                                    </div>
                                    <button
                                        className="text-indigo-600 hover:text-indigo-800 text-sm font-medium"
                                        onClick={() => { if(invoice.status === 'draft') { setDraftInvoice(invoice); setShowDraftModal(true); } }}
                                    >
                                        {invoice.status === 'draft' ? 'Review & Send' : 'View Details'}
                                    </button>
                                </div>
                            </div>
                        ))}

                        <div className="bg-gray-50 dark:bg-gray-800/50 p-6 rounded-xl border-2 border-dashed border-gray-200 dark:border-gray-700 flex flex-col items-center justify-center text-center min-h-[200px]">
                            <div className="w-12 h-12 bg-indigo-100 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-2xl mb-3">
                                +
                            </div>
                            <h3 className="text-lg font-medium text-gray-900 dark:text-white">New Invoice</h3>
                            <p className="text-sm text-gray-500 mt-1">Invoice creation is unavailable until a client and line-item form is connected.</p>
                        </div>
                    </div>
                )}
            </main>

            {/* Translucent Glass Modal for Reviewing Draft Invoice */}
            {showDraftModal && draftInvoice && (
                <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40 backdrop-blur-[30px] saturate-[210%] p-0 sm:p-4">
                    <div className="bg-white/90 dark:bg-gray-900/90 backdrop-blur-[30px] saturate-[210%] w-full max-w-lg rounded-t-2xl sm:rounded-2xl shadow-2xl border border-white/20 dark:border-white/10 flex flex-col max-h-[90vh]">
                        <div className="p-6 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center sticky top-0 bg-white/90 dark:bg-gray-900/90 backdrop-blur-[30px] saturate-[210%] rounded-t-2xl z-10">
                            <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white">Review Invoice Draft</h2>
                            <button onClick={() => setShowDraftModal(false)} className="text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 p-2 rounded-full">
                                ✕
                            </button>
                        </div>

                        <div className="p-6 overflow-y-auto custom-scrollbar flex-1">
                            <div className="mb-6">
                                <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Billed To</label>
                                <input type="text" className="w-full bg-transparent border-b border-gray-300 dark:border-gray-600 pb-2 text-lg font-medium text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500 transition-colors" defaultValue={draftInvoice.client_name} />
                            </div>

                            <div className="mb-8">
                                <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-4">Line Items</label>
                                <div className="space-y-4">
                                    {draftInvoice.line_items?.map((item: any, idx: number) => (
                                        <div key={idx} className="flex items-center gap-4 bg-gray-50 dark:bg-gray-800/50 p-3 rounded-lg border border-gray-100 dark:border-gray-700">
                                            <div className="flex-1">
                                                <input type="text" className="w-full bg-transparent font-medium text-gray-900 dark:text-white text-sm focus:outline-none" defaultValue={item.description} />
                                            </div>
                                            <div className="w-16">
                                                <input type="number" className="w-full bg-transparent text-right text-sm text-gray-600 dark:text-gray-400 focus:outline-none" defaultValue={item.quantity} />
                                            </div>
                                            <div className="w-24 text-right">
                                                <span className="text-gray-500 text-sm">$</span>
                                                <input type="number" className="w-16 bg-transparent text-right text-sm font-medium text-gray-900 dark:text-white focus:outline-none" defaultValue={item.unit_price} />
                                            </div>
                                        </div>
                                    ))}

                                    <button className="text-indigo-600 text-sm font-medium flex items-center gap-1 hover:text-indigo-800">
                                        <span>+</span> Add Line Item
                                    </button>
                                </div>
                            </div>

                            <div className="flex justify-between items-center border-t border-gray-200 dark:border-gray-700 pt-6">
                                <span className="text-gray-500 font-medium">Total</span>
                                <span className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">${draftInvoice.total_amount?.toFixed(2)}</span>
                            </div>
                        </div>

                        <div className="p-6 border-t border-gray-200 dark:border-gray-700 bg-gray-50/50 dark:bg-gray-900/50 rounded-b-2xl">
                            <button
                                className="w-full py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all text-lg"
                                onClick={async () => {
                                    const res = await fetch(`/api/v1/invoices/${draftInvoice.id}/status`, {
                                        method: 'PUT',
                                        headers: { 'Content-Type': 'application/json' },
                                        body: JSON.stringify({ status: 'paid' })
                                    });
                                    if (res.ok) {
                                        alert("Invoice sent! Stripe payment link generated.");
                                        setShowDraftModal(false);
                                        fetchInvoices();
                                    }
                                }}
                            >
                                Approve & Send
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </AppShell>
    );
}
