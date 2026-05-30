"use client";

import React, { useState, useEffect } from "react";
import { useRouter } from "next/navigation";

export default function CapitalPage() {
    const router = useRouter();
    const [offers, setOffers] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchOffers = async () => {
            try {
                // Mock API call since we need a working UI without the full backend connected yet
                const mockOffers = [
                    {
                        id: "offer-1",
                        amount_cents: 200000,
                        fee_cents: 15000,
                        sweep_percentage: 0.1,
                        status: "PENDING"
                    }
                ];
                setOffers(mockOffers);
            } catch (err) {
                console.error("Failed to fetch offers:", err);
            } finally {
                setLoading(false);
            }
        };

        fetchOffers();
    }, []);

    const acceptOffer = async (offerId: string) => {
        try {
            // Mock accept
            setOffers(offers.map(o => o.id === offerId ? { ...o, status: "ACCEPTED" } : o));
            alert("Offer accepted! Funds deposited to your wallet.");
        } catch (err) {
            console.error("Failed to accept offer:", err);
        }
    };

    if (loading) {
        return <div className="flex h-screen items-center justify-center">Loading...</div>;
    }

    return (
        <div className="flex flex-col h-screen bg-gray-50 font-inter">
            <div className="w-[375px] mx-auto h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200 mt-10 p-6 rounded-3xl" style={{
                background: "rgba(255, 255, 255, 0.45)",
                backdropFilter: "blur(40px) saturate(250%)",
                border: "1px solid rgba(255, 255, 255, 0.5)",
            }}>
                <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Capital Offers</h1>

                {offers.length === 0 ? (
                    <p className="text-gray-500">No active offers at this time.</p>
                ) : (
                    <div className="space-y-4">
                        {offers.map((offer) => (
                            <div key={offer.id} className="bg-white/80 p-5 rounded-2xl shadow-sm border border-gray-100">
                                <h2 className="text-xl font-semibold mb-2">Advance: ${(offer.amount_cents / 100).toFixed(2)}</h2>
                                <p className="text-gray-600 mb-1">One-time fee: ${(offer.fee_cents / 100).toFixed(2)}</p>
                                <p className="text-gray-600 mb-4">Repayment: {(offer.sweep_percentage * 100).toFixed(0)}% of daily sales until repaid</p>

                                {offer.status === "PENDING" ? (
                                    <button
                                        onClick={() => acceptOffer(offer.id)}
                                        className="w-full bg-blue-600 text-white font-semibold py-3 rounded-xl hover:bg-blue-700 transition-colors flex items-center justify-center gap-2"
                                    >
                                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" /></svg>
                                        Accept Offer
                                    </button>
                                ) : (
                                    <div className="w-full bg-green-100 text-green-800 font-semibold py-3 rounded-xl text-center">
                                        Accepted
                                    </div>
                                )}
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
}
