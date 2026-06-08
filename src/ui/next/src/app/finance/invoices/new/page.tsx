"use client";

import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function NewInvoicePage() {
    const router = useRouter();
    const [customerName, setCustomerName] = useState('');
    const [serviceName, setServiceName] = useState('');
    const [price, setPrice] = useState('');
    const [statusMessage, setStatusMessage] = useState('');

    const [previewTax, setPreviewTax] = useState(0);
    const [previewTotal, setPreviewTotal] = useState(0);
    const [showPreview, setShowPreview] = useState(false);

    // Mock specific customer ID selection for localization test
    const getMockCustomerId = () => {
        if (customerName.toLowerCase().includes('leo') || customerName.toLowerCase().includes('uk')) return 'leo_student_uk';
        if (customerName.toLowerCase().includes('priya') || customerName.toLowerCase().includes('ca')) return 'priya_customer_ca';
        return 'other_customer';
    };

    const handlePreview = () => {
        const amount = parseFloat(price) || 0;
        let taxRate = 0.08;
        const customerId = getMockCustomerId();

        if (customerId === 'leo_student_uk') taxRate = 0.20;
        else if (customerId === 'priya_customer_ca') taxRate = 0.05;

        const tax = amount * taxRate;
        setPreviewTax(tax);
        setPreviewTotal(amount + tax);
        setShowPreview(true);
    };

    const handleSend = async () => {
        try {
            const amount = parseFloat(price);
            if (!amount || amount <= 0) {
                setStatusMessage('Please enter a valid price.');
                return;
            }

            const res = await fetch('/api/ledger/invoice/draft', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    tenant_id: 'tenant_1',
                    customer_id: getMockCustomerId(),
                    due_date: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
                    items: [
                        {
                            description: serviceName || 'General Service',
                            quantity: 1,
                            unit_price: amount,
                        }
                    ]
                })
            });

            if (res.ok) {
                setStatusMessage('Invoice Sent Successfully!');
            } else {
                setStatusMessage('Failed to send invoice.');
            }
        } catch (error) {
            setStatusMessage('Error sending invoice.');
        }
    };

    return (
        <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
            <div className="w-[375px] max-w-full space-y-6">
                <div className="bg-white/70 backdrop-blur-xl border border-white/20 shadow-xl rounded-3xl p-6 text-gray-800">
                    <h1 className="text-2xl font-semibold mb-6">New Invoice</h1>

                    <div className="space-y-4">
                        <div>
                            <label className="block text-sm font-medium mb-1 text-gray-600">Customer Name</label>
                            <input
                                type="text"
                                className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                                value={customerName}
                                onChange={(e) => setCustomerName(e.target.value)}
                                placeholder="e.g. Leo (UK)"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium mb-1 text-gray-600">Service</label>
                            <input
                                type="text"
                                className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                                value={serviceName}
                                onChange={(e) => setServiceName(e.target.value)}
                                placeholder="Guitar Lesson"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium mb-1 text-gray-600">Price (USD)</label>
                            <input
                                type="number"
                                className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                                value={price}
                                onChange={(e) => setPrice(e.target.value)}
                                placeholder="0.00"
                            />
                        </div>

                        <button
                            onClick={handlePreview}
                            className="w-full py-3 bg-gray-900 hover:bg-gray-800 text-white rounded-xl font-medium shadow-md transition-all active:scale-95"
                        >
                            Preview Invoice
                        </button>
                    </div>
                </div>

                {showPreview && (
                    <div className="bg-blue-50/70 backdrop-blur-xl border border-blue-100/50 shadow-2xl rounded-3xl p-6 transition-all duration-300 animate-in fade-in slide-in-from-bottom-4">
                        <div className="flex justify-between items-center mb-6">
                            <h2 className="text-xl font-semibold text-blue-900">Preview</h2>
                            <span className="text-xs font-semibold bg-blue-100 text-blue-800 px-3 py-1 rounded-full">Draft</span>
                        </div>

                        <div className="space-y-3 mb-6 bg-white/60 p-4 rounded-2xl border border-white/50">
                            <div className="flex justify-between text-sm">
                                <span className="text-gray-500">Billed to:</span>
                                <span className="font-medium">{customerName || 'N/A'}</span>
                            </div>
                            <div className="flex justify-between text-sm">
                                <span className="text-gray-500">Service:</span>
                                <span className="font-medium">{serviceName || 'N/A'}</span>
                            </div>
                            <hr className="border-gray-200" />
                            <div className="flex justify-between text-sm">
                                <span className="text-gray-500">Subtotal:</span>
                                <span className="font-medium">${(parseFloat(price) || 0).toFixed(2)}</span>
                            </div>
                            <div className="flex justify-between text-sm">
                                <span className="text-gray-500">Local Tax:</span>
                                <span className="font-medium text-amber-600">+${previewTax.toFixed(2)}</span>
                            </div>
                            <hr className="border-gray-200" />
                            <div className="flex justify-between text-lg font-bold">
                                <span>Total:</span>
                                <span>${previewTotal.toFixed(2)}</span>
                            </div>
                        </div>

                        <button
                            onClick={handleSend}
                            className="w-full py-4 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-semibold shadow-lg shadow-blue-500/30 transition-all active:scale-95"
                        >
                            Send & Record
                        </button>
                    </div>
                )}

                {statusMessage && (
                    <div className="text-center p-3 rounded-xl bg-white/50 backdrop-blur-sm border border-gray-100 text-sm font-medium text-gray-700">
                        {statusMessage}
                    </div>
                )}
            </div>
        </div>
    );
}
