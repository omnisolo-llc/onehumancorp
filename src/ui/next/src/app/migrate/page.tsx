"use client";

import React, { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function MigratePage() {
    const [url, setUrl] = useState('');
    const [loading, setLoading] = useState(false);
    const [complete, setComplete] = useState(false);
    const router = useRouter();

    const handleMigrate = () => {
        setLoading(true);
        setTimeout(() => {
            setLoading(false);
            setComplete(true);
        }, 1500);
    };

    return (
        <div className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8 animate-fade-in font-inter">
            <h1 className="text-2xl font-bold mb-4 font-outfit text-gray-900">Migrate Existing Store</h1>
            <div className="p-6 shadow-sm border rounded-2xl mac-glass-container flex flex-col transition-all" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderColor: 'rgba(255, 255, 255, 0.4)' }}>
                {!loading && !complete && (
                    <div className="flex flex-col gap-4 max-w-md">
                        <label className="block text-sm font-semibold text-gray-700">Store URL</label>
                        <input
                            name="migration_url"
                            value={url}
                            onChange={(e) => setUrl(e.target.value)}
                            className="border p-3 rounded-lg focus:ring-2 focus:ring-[#0066FF] outline-none text-gray-800 transition-shadow duration-200 bg-white/50"
                            placeholder="e.g. mayas-cakes.myshopify.com"
                        />
                        <button onClick={handleMigrate} className="px-6 py-3 font-bold text-white bg-[#0066FF] hover:bg-[#0056b3] rounded-lg shadow-md transition-transform hover:scale-[1.02] active:scale-[0.98] mt-2">
                            Start Migration
                        </button>
                    </div>
                )}
                {loading && (
                    <div className="flex flex-col items-center justify-center py-12 animate-pulse">
                        <div className="w-8 h-8 border-4 border-blue-200 border-t-[#0066FF] rounded-full animate-spin mb-4"></div>
                        <p className="text-lg font-medium text-gray-700">Our AI is carefully moving your store...</p>
                    </div>
                )}
                {complete && (
                    <div className="flex flex-col items-center justify-center py-12 text-center animate-fade-in">
                        <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-4 text-3xl shadow-sm">✓</div>
                        <p className="text-xl font-bold font-outfit text-gray-900 mb-6">Migration Complete</p>
                        <button onClick={() => router.push('/products')} className="px-6 py-3 font-bold text-white bg-[#34C759] hover:bg-[#2eb350] rounded-lg shadow-md transition-transform hover:scale-[1.02] active:scale-[0.98] flex items-center justify-center gap-2">
                            Review & Publish
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
}
