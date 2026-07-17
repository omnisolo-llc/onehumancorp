"use client";

import React, { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function ProposalCalculatorContent() {
    const searchParams = useSearchParams();

    const [tenant, setTenant] = useState('demo');
    const [serviceName, setServiceName] = useState('Custom Proposal');
    const [basePrice, setBasePrice] = useState(0);
    const [unitName, setUnitName] = useState('Units');
    const [pricePerUnit, setPricePerUnit] = useState(0);
    const [theme, setTheme] = useState('light');
    const [quantity, setQuantity] = useState(1);

    useEffect(() => {
        const t = searchParams.get('tenant') || 'demo';
        setTenant(t);
        setServiceName(searchParams.get('service') || 'Custom Proposal');
        setBasePrice(Number(searchParams.get('basePrice')) || 0);
        setUnitName(searchParams.get('unitName') || 'Units');
        setPricePerUnit(Number(searchParams.get('pricePerUnit')) || 0);
        setTheme(searchParams.get('theme') || 'light');
    }, [searchParams]);

    const handleQuantityChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setQuantity(Number(e.target.value));
    };

    const handleRequestClick = () => {
        // Send a message to the parent frame
        if (window.parent && window.parent !== window) {
            window.parent.postMessage({ type: 'ohc-proposal-request', tenant }, '*');
        } else {
            // Open the work intake or contact page as fallback
            window.open(`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`, '_blank');
        }
    };

    const getThemeStyles = () => {
        return theme === 'dark'
            ? { background: '#111827', color: '#f9fafb' }
            : { background: '#ffffff', color: '#1f2937' };
    };

    const totalCost = basePrice + (quantity * pricePerUnit);

    return (
        <div className="min-h-screen flex items-center justify-center font-inter w-full" style={getThemeStyles()}>
            <div className="w-full h-full flex flex-col justify-between">
                <div className="p-6 flex-1">
                    <h3 className="text-xl font-bold mb-4 font-outfit text-center">{serviceName} Proposal</h3>

                    <div className="space-y-4">
                        {basePrice > 0 && (
                            <div className="flex justify-between items-center opacity-80">
                                <span>Base Price</span>
                                <span className="font-semibold">${basePrice.toFixed(2)}</span>
                            </div>
                        )}

                        <div className="space-y-2 mt-4">
                            <div className="flex justify-between items-center text-sm font-medium">
                                <label htmlFor="unit-slider">Number of {unitName}</label>
                                <span className="text-indigo-500 font-bold">{quantity}</span>
                            </div>
                            <input
                                id="unit-slider"
                                type="range"
                                min="1"
                                max="100"
                                value={quantity}
                                onChange={handleQuantityChange}
                                className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-indigo-600"
                            />
                            <div className="text-right text-xs opacity-70">
                                ${pricePerUnit.toFixed(2)} per {unitName.toLowerCase()}
                            </div>
                        </div>

                        <div className="pt-4 border-t mt-4" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                            <div className="flex justify-between items-end">
                                <span className="text-lg">Estimated Total</span>
                                <span className="text-3xl font-bold text-indigo-500">${totalCost.toFixed(2)}</span>
                            </div>
                        </div>

                        <button
                            onClick={handleRequestClick}
                            className="w-full mt-6 py-3 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl transition-colors shadow-md"
                        >
                            Request Proposal
                        </button>
                    </div>
                </div>

                <div className="mt-auto py-3 border-t w-full text-center" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb', backgroundColor: theme === 'dark' ? '#1f2937' : '#f9fafb' }}>
                    <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="text-xs font-semibold tracking-wide hover:underline opacity-70 hover:opacity-100 transition-opacity inline-flex items-center gap-1" style={{ color: '#6b7280' }}>
                        ⚡ Powered by OHC
                    </a>
                </div>
            </div>
            <style dangerouslySetInnerHTML={{__html: `
                @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
                .font-inter { font-family: 'Inter', sans-serif; }
                .font-outfit { font-family: 'Outfit', sans-serif; }
                html, body { margin: 0; padding: 0; height: 100%; width: 100%; overflow: hidden; }
                input[type=range] { -webkit-appearance: none; background: transparent; }
                input[type=range]::-webkit-slider-thumb { -webkit-appearance: none; height: 16px; width: 16px; border-radius: 50%; background: #4f46e5; cursor: pointer; margin-top: -4px; box-shadow: 0 1px 3px rgba(0,0,0,0.3); }
                input[type=range]::-webkit-slider-runnable-track { width: 100%; height: 8px; cursor: pointer; background: #e5e7eb; border-radius: 4px; }
            `}} />
        </div>
    );
}

export default function ProposalCalculatorPage() {
    return (
        <Suspense fallback={<div className="p-8 text-center text-gray-500 font-inter">Loading calculator...</div>}>
            <ProposalCalculatorContent />
        </Suspense>
    );
}
