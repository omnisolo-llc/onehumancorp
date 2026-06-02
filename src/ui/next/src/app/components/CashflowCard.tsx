"use client";

import { useState, useEffect } from "react";

export function CashflowCard() {
    const [forecast, setForecast] = useState<any>(null);
    const [loading, setLoading] = useState(true);
    const [resolving, setResolving] = useState(false);
    const [resolvedMessage, setResolvedMessage] = useState("");

    useEffect(() => {
        fetch("/api/finance/forecast")
            .then(res => res.json())
            .then(data => {
                setForecast(data);
                setLoading(false);
            });
    }, []);

    const handleResolve = async (action: string) => {
        setResolving(true);
        const res = await fetch("/api/finance/resolve-gap", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ action })
        });
        const data = await res.json();
        setResolvedMessage(data.message);
        setResolving(false);
        setForecast({ ...forecast, type: "surplus", alert_message: null, forecast_cents: forecast.forecast_cents + 50000 });
    };

    if (loading) {
        return (
            <div className="bg-white/40 backdrop-blur-md rounded-3xl p-6 shadow-sm animate-pulse">
                <div className="h-6 bg-white/50 rounded w-1/3 mb-4"></div>
                <div className="h-10 bg-white/50 rounded w-1/4 mb-2"></div>
                <div className="h-4 bg-white/50 rounded w-2/3"></div>
            </div>
        );
    }

    if (!forecast) return null;

    return (
        <div data-testid="cashflow-card" className="bg-white/60 backdrop-blur-xl border border-white/40 shadow-xl rounded-3xl p-6 transition-all duration-300 relative overflow-hidden">
            <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-400 to-indigo-500"></div>

            <h3 className="font-semibold text-lg font-outfit text-gray-900 mb-2">30-Day Cashflow Forecast</h3>

            <div className="flex items-end gap-3 mb-4">
                <span className={`text-4xl font-bold font-outfit ${forecast.type === 'shortfall' ? 'text-rose-600' : 'text-gray-900'}`}>
                    ${(forecast.forecast_cents / 100).toFixed(2)}
                </span>
                <span className="text-gray-500 pb-1 font-inter">predicted balance</span>
            </div>

            {forecast.alert_message && !resolvedMessage && (
                <div className="bg-rose-50/80 border border-rose-100 rounded-2xl p-4 mb-4">
                    <div className="flex gap-3">
                        <span className="text-rose-500 text-xl">⚠️</span>
                        <div>
                            <p className="text-rose-800 font-medium font-inter">{forecast.alert_message}</p>

                            <div className="mt-3 flex flex-wrap gap-2">
                                <button
                                    onClick={() => handleResolve("send_reminders")}
                                    disabled={resolving}
                                    className="bg-rose-600 text-white text-sm px-4 py-2 rounded-full font-medium hover:bg-rose-700 transition shadow-sm disabled:opacity-50"
                                >
                                    Send Invoice Reminders
                                </button>
                                <button
                                    onClick={() => handleResolve("take_advance")}
                                    disabled={resolving}
                                    className="bg-white text-gray-800 text-sm px-4 py-2 rounded-full font-medium border border-gray-200 hover:bg-gray-50 transition shadow-sm disabled:opacity-50"
                                >
                                    Take Cash Advance
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {resolvedMessage && (
                <div className="bg-green-50/80 border border-green-100 rounded-2xl p-4 mb-4">
                    <p className="text-green-800 font-medium font-inter">✅ {resolvedMessage}</p>
                </div>
            )}

            {!forecast.alert_message && !resolvedMessage && (
                <p className="text-gray-600 font-inter text-sm">Your business is looking healthy for the upcoming month.</p>
            )}
        </div>
    );
}
