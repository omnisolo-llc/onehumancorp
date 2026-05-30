'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function PricingSettingsPage() {
    const router = useRouter();
    const [enabled, setEnabled] = useState(false);
    const [minPrice, setMinPrice] = useState(800);
    const [maxPrice, setMaxPrice] = useState(1200);
    const [strategies, setStrategies] = useState<string[]>(['ClearInventory', 'WeatherDemand']);
    const [status, setStatus] = useState('');

    const toggleStrategy = (strategy: string) => {
        setStrategies(prev =>
            prev.includes(strategy)
                ? prev.filter(s => s !== strategy)
                : [...prev, strategy]
        );
    };

    const handleSave = async () => {
        setStatus('Saving...');
        try {
            // using "default_product" as a placeholder for E2E purposes
            const res = await fetch('/api/v1/pricing/default_product/config', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    enabled,
                    min_price_cents: minPrice,
                    max_price_cents: maxPrice,
                    strategies
                })
            });
            if (res.ok) {
                setStatus('Settings saved successfully!');
                setTimeout(() => setStatus(''), 3000);
            } else {
                setStatus('Failed to save settings.');
            }
        } catch (e) {
            setStatus('Error occurred while saving.');
        }
    };

    return (
        <div className="p-8 max-w-xl mx-auto backdrop-blur-2xl bg-white/30 rounded-3xl border border-white/50 shadow-2xl mt-12 text-gray-900">
            <h1 className="text-3xl font-extrabold mb-2 tracking-tight">Smart Pricing</h1>
            <p className="mb-8 text-gray-600 font-medium">Enable AI-driven dynamic pricing to clear inventory and maximize revenue.</p>

            <div className="flex items-center justify-between mb-8 p-4 bg-white/40 rounded-2xl border border-white/60">
                <span className="font-semibold text-lg">Enable Dynamic Pricing</span>
                <label className="relative inline-flex items-center cursor-pointer">
                    <input
                        type="checkbox"
                        className="sr-only peer"
                        checked={enabled}
                        onChange={e => setEnabled(e.target.checked)}
                    />
                    <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
                </label>
            </div>

            <div className="mb-6 p-4 bg-white/40 rounded-2xl border border-white/60">
                <label className="block mb-4 font-semibold">Minimum Price Limit (cents)</label>
                <input
                    type="range"
                    min="500"
                    max="1000"
                    value={minPrice}
                    onChange={e => setMinPrice(Number(e.target.value))}
                    className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                />
                <div className="text-right text-sm font-medium text-gray-600 mt-2">{minPrice / 100} $</div>
            </div>

            <div className="mb-8 p-4 bg-white/40 rounded-2xl border border-white/60">
                <label className="block mb-4 font-semibold">Maximum Price Limit (cents)</label>
                <input
                    type="range"
                    min="1000"
                    max="2000"
                    value={maxPrice}
                    onChange={e => setMaxPrice(Number(e.target.value))}
                    className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                />
                <div className="text-right text-sm font-medium text-gray-600 mt-2">{maxPrice / 100} $</div>
            </div>

            <div className="mb-8 p-4 bg-white/40 rounded-2xl border border-white/60">
                <label className="block mb-4 font-semibold">Pricing Strategies</label>
                <div className="flex flex-col gap-3">
                    {['ClearInventory', 'MaximizeRevenue', 'FillSchedule', 'WeatherDemand'].map(strategy => (
                        <label key={strategy} className="flex items-center space-x-3 cursor-pointer">
                            <input
                                type="checkbox"
                                checked={strategies.includes(strategy)}
                                onChange={() => toggleStrategy(strategy)}
                                className="w-5 h-5 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
                            />
                            <span className="font-medium text-gray-800">{strategy}</span>
                        </label>
                    ))}
                </div>
            </div>

            <button
                onClick={handleSave}
                className="w-full py-4 bg-gradient-to-r from-blue-600 to-indigo-600 text-white rounded-2xl font-bold text-lg shadow-xl hover:from-blue-700 hover:to-indigo-700 transition-all active:scale-[0.98]"
            >
                Save Settings
            </button>

            {status && (
                <div className={`mt-4 text-center font-medium ${status.includes('successfully') ? 'text-green-600' : 'text-blue-600'}`}>
                    {status}
                </div>
            )}
        </div>
    );
}
