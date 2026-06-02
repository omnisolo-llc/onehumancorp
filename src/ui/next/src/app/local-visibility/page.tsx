"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

export default function LocalVisibilitySettings() {
    const [isGbpConnected, setIsGbpConnected] = useState<boolean>(false);
    const [isSyncing, setIsSyncing] = useState<boolean>(false);
    const [lastSynced, setLastSynced] = useState<string | null>(null);

    useEffect(() => {
        const connected = typeof localStorage !== 'undefined' ? localStorage.getItem('gbp_connected') === 'true' : false;
        setIsGbpConnected(connected);
    }, []);

    const handleManualSync = async () => {
        setIsSyncing(true);
        try {
            const res = await fetch('/api/v1/local-seo/sync', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ entities: ['hours', 'menu', 'services'] })
            });
            if (res.ok) {
                const data = await res.json();
                setLastSynced(data.synced_at);
            }
        } catch (error) {
            console.error('Failed to sync', error);
        } finally {
            setIsSyncing(false);
        }
    };

    return (
        <div className="min-h-screen bg-[#F5F5F7] p-4 sm:p-8">
            <div className="max-w-2xl mx-auto">
                <Link href="/dashboard" className="text-[#0066FF] text-sm font-semibold mb-6 inline-block hover:underline">
                    &larr; Back to Dashboard
                </Link>

                <h1 className="text-3xl font-bold text-[#1D1D1F] font-outfit mb-2">Local Visibility</h1>
                <p className="text-[#86868B] mb-8 font-inter">
                    Manage how your business appears across local directories like Google Maps.
                </p>

                <div className="glass-panel p-6 rounded-2xl border border-[#D2D2D7] bg-white/65 backdrop-blur-xl mb-6">
                    <h2 className="text-lg font-bold text-[#1D1D1F] mb-4">Google Business Profile</h2>

                    <div className="flex items-center justify-between">
                        <div>
                            <p className="font-semibold text-[#1D1D1F]">Status</p>
                            <p className="text-sm text-[#86868B]">
                                {isGbpConnected ? 'Connected and syncing automatically.' : 'Not connected.'}
                            </p>
                        </div>
                        <div className={`px-3 py-1 rounded-full text-sm font-semibold ${isGbpConnected ? 'bg-[#34C759]/10 text-[#34C759]' : 'bg-[#E5E5EA] text-[#86868B]'}`}>
                            {isGbpConnected ? 'Connected' : 'Disconnected'}
                        </div>
                    </div>

                    {isGbpConnected && (
                        <div className="mt-6 pt-6 border-t border-[#E5E5EA]">
                            <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                                <div>
                                    <p className="font-semibold text-[#1D1D1F]">Catalog & Hours Sync</p>
                                    <p className="text-sm text-[#86868B]">
                                        Last synced: {lastSynced ? new Date(lastSynced).toLocaleString() : 'Never'}
                                    </p>
                                </div>
                                <button
                                    onClick={handleManualSync}
                                    disabled={isSyncing}
                                    className="px-4 py-2 bg-white border border-[#D2D2D7] text-[#1D1D1F] rounded-lg text-sm font-semibold hover:bg-[#F5F5F7] disabled:opacity-50"
                                >
                                    {isSyncing ? 'Syncing...' : 'Sync Now'}
                                </button>
                            </div>
                        </div>
                    )}
                </div>

                <div className="glass-panel p-6 rounded-2xl border border-[#D2D2D7] bg-white/65 backdrop-blur-xl">
                    <h2 className="text-lg font-bold text-[#1D1D1F] mb-4">AI Review Drafting</h2>
                    <p className="text-sm text-[#86868B] mb-4">
                        "The Ambassador" AI automatically drafts personalized responses to your customer reviews.
                        You always get to review and approve them before they are published.
                    </p>
                    <div className="flex items-center justify-between p-4 bg-[#F5F5F7] rounded-xl">
                        <span className="font-semibold text-[#1D1D1F]">Auto-draft responses</span>
                        <div className="w-12 h-6 bg-[#34C759] rounded-full relative cursor-pointer">
                            <div className="w-5 h-5 bg-white rounded-full absolute right-0.5 top-0.5 shadow-sm"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
