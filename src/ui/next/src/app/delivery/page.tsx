"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

interface RouteStop {
    id: string;
    order_id: string;
    address: string;
    status: string;
    eta_ms: number;
}

interface DeliveryRoute {
    id: string;
    organization_id: string;
    driver_id: string;
    status: string;
    stops: RouteStop[];
}

export default function DeliveryTrackingPage() {
    const [route, setRoute] = useState<DeliveryRoute | null>(null);
    const [loading, setLoading] = useState(true);
    const [updating, setUpdating] = useState<string | null>(null);

    useEffect(() => {
        fetchRoute();
    }, []);

    async function fetchRoute() {
        try {
            // Ideally we'd hit the real gRPC endpoint, but since we are purely verifying E2E
            // we will proxy through a Next.js API that calls our backend's `GetDeliveryRoute`
            // Let's use our new real API route
            const res = await fetch('/api/v1/delivery?route_id=dummy-route');
            const data = await res.json();
            setRoute(data);
        } catch (err) {
            console.error("Failed to fetch route:", err);
        } finally {
            setLoading(false);
        }
    }

    async function updateStopStatus(stopId: string, status: string) {
        setUpdating(stopId);
        try {
            await fetch('/api/v1/delivery/update', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    route_id: route?.id,
                    stop_id: stopId,
                    status: status
                })
            });
            await fetchRoute(); // refresh
        } catch (err) {
            console.error("Failed to update status", err);
        } finally {
            setUpdating(null);
        }
    }

    if (loading) {
        return <div className="p-8 font-outfit text-center text-gray-500">Loading Route...</div>;
    }

    if (!route || !route.stops || route.stops.length === 0) {
        return <div className="p-8 font-outfit text-center text-gray-500">No active deliveries found.</div>;
    }

    const nextStop = route.stops.find(s => s.status === 'pending') || route.stops[0];

    return (
        <div className="min-h-screen bg-gray-50 dark:bg-[#121212] p-4 sm:p-8 font-outfit flex justify-center">
            <div className="w-full max-w-[375px] space-y-6">
                <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Delivery Dispatch</h1>

                {/* UniFi style translucent Glassmorphism card */}
                <div className="backdrop-blur-xl bg-white/60 dark:bg-black/40 border border-white/20 dark:border-gray-800/50 shadow-lg rounded-2xl p-6 relative overflow-hidden">
                    <div className="absolute top-0 right-0 w-32 h-32 bg-blue-500/10 rounded-full blur-2xl -mr-10 -mt-10"></div>
                    <div className="relative z-10">
                        <div className="flex items-center justify-between mb-4">
                            <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Next Stop</h2>
                            <span className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full font-medium">
                                {new Date(nextStop.eta_ms).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                            </span>
                        </div>

                        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-1">{nextStop.address}</h3>
                        <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">Order #{nextStop.order_id}</p>

                        <div className="grid grid-cols-2 gap-3 mb-4">
                            <a
                                href={`https://maps.apple.com/?daddr=${encodeURIComponent(nextStop.address)}`}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="block w-full py-3 px-4 bg-blue-600 hover:bg-blue-700 text-white text-center font-medium rounded-xl transition-colors shadow-md shadow-blue-500/20 text-sm"
                            >
                                Navigate
                            </a>
                            {nextStop.status === 'pending' && (
                                <button
                                    onClick={() => updateStopStatus(nextStop.id, 'out_for_delivery')}
                                    disabled={updating === nextStop.id}
                                    className="block w-full py-3 px-4 bg-gray-900 dark:bg-gray-700 hover:bg-black text-white text-center font-medium rounded-xl transition-colors shadow-md text-sm disabled:opacity-50"
                                >
                                    {updating === nextStop.id ? '...' : 'Start Leg'}
                                </button>
                            )}
                            {nextStop.status === 'out_for_delivery' && (
                                <button
                                    onClick={() => updateStopStatus(nextStop.id, 'completed')}
                                    disabled={updating === nextStop.id}
                                    className="block w-full py-3 px-4 bg-green-600 hover:bg-green-700 text-white text-center font-medium rounded-xl transition-colors shadow-md text-sm disabled:opacity-50"
                                >
                                    {updating === nextStop.id ? '...' : 'Complete'}
                                </button>
                            )}
                        </div>
                    </div>
                </div>

                <div className="pt-4">
                    <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-4 px-2">Itinerary</h2>
                    <div className="space-y-3">
                        {route.stops.map((stop, i) => (
                            <div key={stop.id} className={`flex items-center p-4 rounded-xl ${stop.status === 'completed' ? 'bg-gray-100 dark:bg-gray-800/50' : 'bg-white dark:bg-gray-900 shadow-sm border border-gray-100 dark:border-gray-800'}`}>
                                <div className={`w-8 h-8 rounded-full flex items-center justify-center mr-4 text-sm font-bold ${stop.status === 'completed' ? 'bg-green-100 text-green-700' : stop.status === 'out_for_delivery' ? 'bg-orange-100 text-orange-700' : 'bg-blue-100 text-blue-700'}`}>
                                    {stop.status === 'completed' ? '✓' : i + 1}
                                </div>
                                <div className="flex-1">
                                    <p className={`font-medium ${stop.status === 'completed' ? 'text-gray-500 line-through' : 'text-gray-900 dark:text-white'}`}>{stop.address}</p>
                                    {stop.status === 'out_for_delivery' && <p className="text-xs text-orange-600 font-medium mt-1">Driving...</p>}
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}