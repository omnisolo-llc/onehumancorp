"use client";

import { useState, useEffect } from "react";

export default function AutoDreamMemory() {
    const [consolidating, setConsolidating] = useState(false);
    const [lastConsolidated, setLastConsolidated] = useState<string | null>(null);
    const [memoryCount, setMemoryCount] = useState(0);

    useEffect(() => {
        // Poll for status
        const fetchStatus = async () => {
            try {
                // Mock API for status since we only implemented the worker backend
                // In a real app, this would be a real endpoint
                setMemoryCount(Math.floor(Math.random() * 100) + 50);
                setLastConsolidated(new Date().toLocaleTimeString());
            } catch (e) {
                console.error("Failed to fetch AutoDream status", e);
            }
        };

        fetchStatus();
        const interval = setInterval(fetchStatus, 30000);
        return () => clearInterval(interval);
    }, []);

    const handleManualConsolidate = async () => {
        setConsolidating(true);
        // Simulate consolidation
        await new Promise(resolve => setTimeout(resolve, 2000));
        setConsolidating(false);
        setLastConsolidated(new Date().toLocaleTimeString());
        setMemoryCount(prev => prev + 1);
    };

    return (
        <section className="mb-6">
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Memory Consolidation</h2>
                <div className="flex items-center gap-2 px-3 py-1 bg-blue-50 rounded-full border border-blue-100">
                    <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#0066FF' }}></div>
                    <span className="text-xs font-medium" style={{ color: '#0066FF' }}>AutoDream Active</span>
                </div>
            </div>

            <div
                className="p-6 shadow-md flex flex-col md:flex-row items-center justify-between gap-6"
                style={{
                    background: 'rgba(255, 255, 255, 0.65)',
                    backdropFilter: 'blur(30px) saturate(210%)',
                    border: '1px solid rgba(255, 255, 255, 0.4)',
                    borderRadius: '16px'
                }}
            >
                <div className="flex items-center gap-5">
                    <div className="w-14 h-14 rounded-2xl bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-2xl shadow-lg">
                        🧠
                    </div>
                    <div>
                        <h3 className="font-bold text-lg font-outfit text-gray-900">Consolidated Intelligence</h3>
                        <p className="text-sm text-gray-600 font-inter">
                            AutoDream is organizing your business data into long-term memory.
                        </p>
                        <div className="flex gap-4 mt-2">
                            <span className="text-xs font-medium text-gray-500">
                                Total Memories: <span className="text-gray-900 font-bold">{memoryCount}</span>
                            </span>
                            <span className="text-xs font-medium text-gray-500">
                                Last Updated: <span className="text-gray-900 font-bold">{lastConsolidated || 'Never'}</span>
                            </span>
                        </div>
                    </div>
                </div>

                <button
                    onClick={handleManualConsolidate}
                    disabled={consolidating}
                    className={`px-6 py-2.5 font-bold rounded-xl transition-all duration-300 shadow-md flex items-center gap-2 ${
                        consolidating
                        ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
                        : 'bg-white text-blue-600 hover:bg-blue-50 hover:shadow-lg active:scale-95 border border-blue-100'
                    }`}
                >
                    {consolidating && <div className="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>}
                    {consolidating ? 'Dreaming...' : 'Dream Now'}
                </button>
            </div>
        </section>
    );
}
