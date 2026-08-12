"use client";

import React, { useState, useEffect } from "react";
import { useParams, useRouter } from "next/navigation";

export default function InteractiveQuotePage() {
    const params = useParams();
    const router = useRouter();
    const quoteId = typeof params.id === "string" ? params.id : "";
    const isQuoteId = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(quoteId);

    const [quote, setQuote] = useState<any>(null);
    const [loading, setLoading] = useState(true);
    const [selectedDate, setSelectedDate] = useState<string>("");
    const [paying, setPaying] = useState(false);
    const [paid, setPaid] = useState(false);
    const [paymentLink, setPaymentLink] = useState<string | null>(null);

    useEffect(() => {
        if (!isQuoteId) {
            setLoading(false);
            return;
        }

        const fetchQuote = async () => {
            try {
                const res = await fetch(`/api/v1/quotes/${quoteId}`);
                if (res.ok) {
                    const data = await res.json();
                    setQuote(data);
                } else {
                }
            } catch {
                // The not-found surface below is the user-facing error state.
            } finally {
                setLoading(false);
            }
        };

        fetchQuote();
    }, [isQuoteId, quoteId]);

    const handlePayDeposit = async () => {
        setPaying(true);
        try {
            const res = await fetch(`/api/v1/quotes/${quoteId}/accept`, {
                method: "POST",
            });
            if (res.ok) {
                const data = await res.json();
                if (data.stripe_payment_link) {
                    setPaymentLink(data.stripe_payment_link);
                } else {
                    setPaid(true);
                }
            }
        } catch {
            // The payment action remains on the quote so the customer can retry.
        } finally {
            setPaying(false);
        }
    };

    if (loading) {
        return (
            <div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center p-4">
                <div className="text-center text-gray-500 animate-pulse">Loading quote...</div>
            </div>
        );
    }

    if (!quote) {
        return (
            <div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center p-4">
                <div className="text-center text-red-500">Quote not found.</div>
            </div>
        );
    }

    if (paymentLink) {
        return (
            <div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center p-4">
                <div className="glassmorphism p-8 rounded-[32px] border border-white/40 shadow-xl max-w-sm w-full text-center">
                    <h2 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Quote accepted</h2>
                    <p className="text-gray-600 mb-6">Continue to the secure payment page to complete your deposit.</p>
                    <a
                        href={paymentLink}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="block w-full py-3 px-4 bg-black hover:bg-gray-800 text-white font-bold rounded-[16px] transition-colors"
                    >
                        Continue to payment
                    </a>
                </div>
            </div>
        );
    }

    if (paid) {
        return (
            <div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center p-4">
                <div className="glassmorphism p-8 rounded-[32px] border border-white/40 shadow-xl max-w-sm w-full text-center">
                    <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                        <span className="text-green-600 text-3xl">✓</span>
                    </div>
                    <h2 className="text-2xl font-bold text-gray-900 mb-2 font-outfit">Deposit Paid!</h2>
                    <p className="text-gray-600 mb-6">Your appointment is confirmed. The remaining balance will be invoiced after the work is completed.</p>
                    <button
                        onClick={() => window.location.reload()}
                        className="w-full py-3 px-4 bg-gray-100 hover:bg-gray-200 text-gray-800 font-bold rounded-[16px] transition-colors"
                    >
                        View Receipt
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-[#F5F5F7] p-4 flex flex-col items-center">
            {/* Mobile-first constraints max-w-[375px] */}
            <div className="w-full max-w-[375px] mx-auto pb-20">
                <header className="py-6 text-center">
                    <h1 className="text-xl font-bold font-outfit text-gray-900">Your Quote</h1>
                    <p className="text-sm text-gray-500">Ref: {quoteId?.toString().substring(0, 8)}</p>
                </header>

                <main className="space-y-4">
                    <div className="glassmorphism p-6 rounded-[24px] border border-white/40 shadow-md bg-white/60 backdrop-blur-md">
                        <h2 className="text-lg font-bold text-gray-900 mb-4">{quote.quote?.service_name || "Quote"}</h2>

                        <div className="space-y-3 mb-6">
                            {quote.line_items?.map((item: any, i: number) => (
                                <div key={i} className="flex justify-between items-center text-sm">
                                    <span className="text-gray-600">{item.description} (x{item.quantity})</span>
                                    <span className="font-semibold text-gray-900">${(item.unit_price_cents * item.quantity / 100).toFixed(2)}</span>
                                </div>
                            ))}
                            <div className="pt-3 border-t border-gray-200 flex justify-between items-center">
                                <span className="font-medium text-gray-900">Total Estimate</span>
                                <span className="font-bold text-gray-900">${(quote.quote?.total_amount_cents / 100).toFixed(2)}</span>
                            </div>
                        </div>

                        <div className="bg-blue-50/50 p-4 rounded-xl border border-blue-100">
                            <div className="flex justify-between items-center mb-1">
                                <span className="text-sm font-medium text-blue-900">Required Deposit</span>
                                <span className="text-lg font-bold text-blue-600">${(quote.quote?.required_deposit_cents / 100).toFixed(2)}</span>
                            </div>
                            <p className="text-xs text-blue-700/70">To secure your booking</p>
                        </div>
                    </div>

                    <div className="glassmorphism p-6 rounded-[24px] border border-white/40 shadow-md bg-white/60 backdrop-blur-md">
                        <h3 className="text-md font-bold text-gray-900 mb-3">Confirm Date</h3>
                        <input
                            type="datetime-local"
                            data-testid="quote-date-selector"
                            value={selectedDate}
                            onChange={(e) => setSelectedDate(e.target.value)}
                            className="w-full px-4 py-3 rounded-xl border border-gray-200 bg-white/80 focus:ring-2 focus:ring-blue-500 outline-none text-sm"
                        />
                    </div>
                </main>

                <div className="fixed bottom-0 left-0 right-0 p-4 bg-white/80 backdrop-blur-xl border-t border-gray-200 flex justify-center z-50">
                    <div className="w-full max-w-[375px]">
                        <button
                            data-testid="pay-deposit-button"
                            onClick={handlePayDeposit}
                            disabled={paying || !selectedDate}
                            className="w-full py-4 px-6 bg-black text-white font-bold rounded-[20px] transition-transform active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed flex justify-center items-center gap-2 shadow-lg"
                        >
                            {paying ? (
                                <span className="animate-pulse">Processing...</span>
                            ) : (
                                <>
                                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                                    </svg>
                                    Pay ${(quote.quote?.required_deposit_cents / 100).toFixed(2)} Deposit
                                </>
                            )}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}
