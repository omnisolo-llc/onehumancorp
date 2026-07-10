"use client";

import React, { useState } from 'react';
import Head from 'next/head';

export default function MultilingualOrderInterceptor() {
    const [isListening, setIsListening] = useState(false);
    const [rawInput, setRawInput] = useState("");
    const [interceptedOrder, setInterceptedOrder] = useState<any>(null);
    const [isProcessing, setIsProcessing] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleListen = () => {
        setIsListening(true);
        setError(null);
        // Simulate listening and getting a transcript
        setTimeout(() => {
            const simulatedText = "Quiero 3 tacos de pollo";
            setRawInput(simulatedText);
            setIsListening(false);
            processOrder(simulatedText);
        }, 2000);
    };

    const processOrder = async (input: string) => {
        setIsProcessing(true);
        setError(null);
        try {
            const res = await fetch('/api/v1/agents/order-interceptor', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ raw_input: input })
            });
            if (!res.ok) throw new Error('Failed to process order');
            const data = await res.json();
            setInterceptedOrder(data);
        } catch (err: any) {
            setError(err.message || 'An error occurred');
        } finally {
            setIsProcessing(false);
        }
    };

    const handleConfirm = async () => {
        if (!interceptedOrder) return;
        try {
            const res = await fetch('/api/agent-feed', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    event_source: 'multilingual_walk_up',
                    context_payload: interceptedOrder
                })
            });
            if (!res.ok) throw new Error('Failed to add to list');

            setInterceptedOrder(null);
            setRawInput("");
            alert("Order confirmed and added to list!");
        } catch (err: any) {
            setError(err.message || 'Failed to add to list');
        }
    };

    return (
        <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
            <Head>
                <title>Multilingual Order Interceptor | OHC</title>
            </Head>

            <div className="w-full max-w-[375px] bg-white rounded-2xl shadow-xl overflow-hidden relative font-sans min-h-[600px] flex flex-col">
                <div className="p-6 flex-1 flex flex-col">
                    <h1 className="text-2xl font-bold text-gray-900 mb-2">Walk-up Order</h1>
                    <p className="text-sm text-gray-500 mb-8">Listen to customer or type</p>

                    {error && (
                        <div className="bg-red-50 text-red-600 p-4 rounded-xl mb-4 text-sm font-medium">
                            {error}
                        </div>
                    )}

                    {!interceptedOrder ? (
                        <div className="flex-1 flex flex-col items-center justify-center">
                            <div className="w-full mb-8">
                                <textarea
                                    value={rawInput}
                                    onChange={(e) => setRawInput(e.target.value)}
                                    placeholder="Type order here or tap mic..."
                                    className="w-full p-4 border border-gray-200 rounded-xl bg-gray-50 text-gray-800 resize-none h-32 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    disabled={isListening || isProcessing}
                                />
                                {rawInput && !isListening && !isProcessing && (
                                     <button
                                         onClick={() => processOrder(rawInput)}
                                         className="mt-4 w-full py-4 bg-blue-600 text-white rounded-xl font-semibold shadow-md hover:bg-blue-700 transition-colors"
                                     >
                                         Process Text Order
                                     </button>
                                )}
                            </div>

                            <div className="relative">
                                {isListening && (
                                    <div className="absolute inset-0 bg-blue-100 rounded-full animate-ping opacity-75"></div>
                                )}
                                <button
                                    onClick={handleListen}
                                    disabled={isListening || isProcessing}
                                    className={`relative z-10 w-24 h-24 rounded-full flex items-center justify-center transition-all shadow-lg
                                        ${isListening ? 'bg-blue-600 text-white scale-110' : 'bg-white text-blue-600 border-2 border-blue-100 hover:border-blue-300'}`}
                                >
                                    {isProcessing ? (
                                        <div className="w-8 h-8 border-4 border-blue-200 border-t-blue-600 rounded-full animate-spin"></div>
                                    ) : (
                                        <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
                                        </svg>
                                    )}
                                </button>
                            </div>
                            <p className="mt-6 text-sm text-gray-500 font-medium">
                                {isListening ? 'Listening...' : isProcessing ? 'Translating & Structuring...' : 'Tap to speak'}
                            </p>
                        </div>
                    ) : (
                        <div className="flex-1 flex flex-col">
                            <div className="bg-blue-50 rounded-2xl p-6 mb-6">
                                <div className="flex items-center justify-between mb-4">
                                    <span className="text-xs font-bold uppercase tracking-wider text-blue-600 bg-blue-100 px-3 py-1 rounded-full">
                                        Translated from {interceptedOrder.language}
                                    </span>
                                </div>
                                <h3 className="text-xl font-bold text-gray-900 mb-4">{interceptedOrder.intent}</h3>
                                <div className="space-y-3">
                                    {interceptedOrder.items.map((item: any, idx: number) => (
                                        <div key={idx} className="flex justify-between items-center bg-white p-4 rounded-xl shadow-sm">
                                            <span className="font-semibold text-gray-800 text-lg">{item.item}</span>
                                            <span className="bg-gray-100 text-gray-800 font-bold px-4 py-2 rounded-lg text-lg">x{item.quantity}</span>
                                        </div>
                                    ))}
                                </div>
                            </div>

                            <div className="mt-auto space-y-3">
                                <button
                                    onClick={handleConfirm}
                                    className="w-full py-4 bg-[#0071E3] text-white rounded-xl font-bold text-lg shadow-md hover:bg-blue-700 transition-colors"
                                >
                                    Confirm & Add to List
                                </button>
                                <button
                                    onClick={() => { setInterceptedOrder(null); setRawInput(""); }}
                                    className="w-full py-4 bg-gray-100 text-gray-700 rounded-xl font-bold hover:bg-gray-200 transition-colors"
                                >
                                    Cancel
                                </button>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}
