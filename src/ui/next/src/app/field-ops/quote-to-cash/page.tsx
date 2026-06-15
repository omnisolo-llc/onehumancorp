"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';

type QuoteItem = {
    description: string;
    amount: number;
}

// Local edge agent simulator for offline-first heuristic parsing
const parseQuote = (input: string) => {
    const services: QuoteItem[] = [];
    const materials: QuoteItem[] = [];
    const labor: QuoteItem[] = [];
    let total = 0;

    const lowerInput = input.toLowerCase();

    // Extract numbers and surrounding context for a simplistic edge parsing
    const regex = /(\d+)\s*(for\s+)?(parts|materials|labor|hours|service|fix|repair)/g;
    let match;
    let foundSpecifics = false;

    while ((match = regex.exec(lowerInput)) !== null) {
        foundSpecifics = true;
        const amount = parseInt(match[1]);
        const type = match[3];

        if (type === 'parts' || type === 'materials') {
            materials.push({ description: 'Materials', amount });
            total += amount;
        } else if (type === 'labor' || type === 'hours') {
            // simplistic assumption: if it's hours, assume $50/hr, else direct amount
            const cost = type === 'hours' ? amount * 50 : amount;
            labor.push({ description: type === 'hours' ? `${amount} Hours Labor` : 'Labor', amount: cost });
            total += cost;
        } else {
            services.push({ description: 'Service Call', amount });
            total += amount;
        }
    }

    // Fallback if no specific amounts are found but we have text
    if (!foundSpecifics && input.trim().length > 0) {
         services.push({ description: 'General Service Estimate', amount: 150 });
         total += 150;
    }

    return { services, materials, labor, total };
};

export default function FieldOpsQuoteToCashPage() {
    const [isOffline, setIsOffline] = useState(false);
    const [voiceInput, setVoiceInput] = useState('');
    const [draftQuote, setDraftQuote] = useState<{ services: QuoteItem[], materials: QuoteItem[], labor: QuoteItem[], total: number } | null>(null);
    const [isGenerating, setIsGenerating] = useState(false);
    const [isPaymentSaved, setIsPaymentSaved] = useState(false);

    useEffect(() => {
        setIsOffline(!navigator.onLine);
        const handleOnline = () => setIsOffline(false);
        const handleOffline = () => setIsOffline(true);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);

        return () => {
            window.removeEventListener('online', handleOnline);
            window.removeEventListener('offline', handleOffline);
        };
    }, []);

    const handleGenerateQuote = () => {
        setIsGenerating(true);
        // We use a small delay to mimic async processing, but use actual logic to parse the text
        setTimeout(() => {
            const quote = parseQuote(voiceInput);
            setDraftQuote(quote);
            setIsGenerating(false);
        }, 300);
    }

    const handleCollectDeposit = () => {
        if (!draftQuote) return;

        // 1. Sync the quote creation
        SyncManager.getInstance().enqueue({
            id: `draft_quote-${Date.now()}`,
            type: 'draft_quote',
            notes: voiceInput,
            total: draftQuote.total
        });

        // 2. Sync the deposit
        SyncManager.getInstance().enqueue({
            id: `tap_to_pay-${Date.now()}`,
            type: 'tap_to_pay',
            amount: draftQuote.total,
            currency: 'usd',
            product_id: 'draft_quote_deposit'
        });

        setIsPaymentSaved(true);
    };

    return (
        <div className="p-4 bg-gray-50 min-h-screen font-sans">
            <div className="flex justify-between items-center mb-6">
                <h1 className="text-2xl font-bold text-gray-900">New Quote</h1>
                {isOffline && (
                    <div className="flex items-center text-orange-600 bg-orange-50 px-3 py-1 rounded-full text-sm font-semibold border border-orange-200">
                        <span className="mr-2">☁️</span> Saved Offline
                    </div>
                )}
            </div>

            <div className="bg-white/70 backdrop-blur-md rounded-2xl shadow-sm border border-white/40 p-5 mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-2">Tell OHC about the job</label>
                <textarea
                    className="w-full bg-white/50 border border-gray-200 rounded-xl p-4 text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all min-h-[120px] shadow-inner"
                    placeholder="E.g., Needs a new sink installed, materials cost 50, labor 100..."
                    value={voiceInput}
                    onChange={(e) => setVoiceInput(e.target.value)}
                />
                <button
                    className="w-full mt-4 bg-gradient-to-r from-gray-800 to-gray-900 hover:from-gray-700 hover:to-gray-800 text-white font-semibold py-3 rounded-xl transition-all active:scale-[0.98] shadow-md disabled:opacity-50 disabled:cursor-not-allowed"
                    onClick={handleGenerateQuote}
                    disabled={isGenerating || !voiceInput}
                >
                    {isGenerating ? 'Generating...' : '🎤 Tell OHC about the job'}
                </button>
            </div>

            {draftQuote && (
                <div className="bg-white/80 backdrop-blur-lg rounded-2xl shadow-lg border border-white/60 p-6 mb-4 transform transition-all animate-in fade-in slide-in-from-bottom-4">
                    <h2 className="font-bold text-xl text-gray-900 mb-4 border-b border-gray-100 pb-3">Quote Draft</h2>
                    <div className="space-y-3 mb-6 text-sm text-gray-700">
                        {draftQuote.services.map((item, idx) => (
                            <div key={idx} className="flex justify-between items-center">
                                <span className="font-medium text-gray-800">{item.description}</span>
                                <span className="font-semibold">${item.amount}</span>
                            </div>
                        ))}
                        {draftQuote.materials.map((item, idx) => (
                            <div key={idx} className="flex justify-between items-center text-gray-600">
                                <span>+ {item.description}</span>
                                <span>${item.amount}</span>
                            </div>
                        ))}
                        {draftQuote.labor.map((item, idx) => (
                            <div key={idx} className="flex justify-between items-center text-gray-600">
                                <span>+ {item.description}</span>
                                <span>${item.amount}</span>
                            </div>
                        ))}
                    </div>
                    <div className="flex justify-between items-center font-bold text-xl text-gray-900 border-t border-gray-100 pt-4 mb-6">
                        <span>Total:</span>
                        <span>${draftQuote.total}</span>
                    </div>

                    {!isPaymentSaved ? (
                         <button
                            className="w-full bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white font-semibold py-4 rounded-xl transition-all active:scale-[0.98] shadow-md flex justify-center items-center gap-2"
                            onClick={handleCollectDeposit}
                        >
                            Collect Deposit
                        </button>
                    ) : (
                         <div className="w-full bg-green-50/80 backdrop-blur text-green-700 text-center font-semibold py-4 rounded-xl border border-green-200/50 shadow-sm transition-all animate-in zoom-in-95">
                            {isOffline ? 'Payment Saved Offline' : 'Payment Collected Successfully'}
                        </div>
                    )}

                </div>
            )}
        </div>
    );
}
