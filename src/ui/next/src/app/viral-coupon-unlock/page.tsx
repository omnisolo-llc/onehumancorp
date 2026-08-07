"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralCouponUnlockPage() {
    const router = useRouter();
    const [headline, setHeadline] = useState('20% Off Your First Order');
    const [couponCode, setCouponCode] = useState('WELCOME20');
    const [requiredShares, setRequiredShares] = useState(3);
    const [description, setDescription] = useState('Share this link with friends to unlock your exclusive discount code!');
    const [tenant, setTenant] = useState('demo');
    const [copied, setCopied] = useState(false);
    const [isClient, setIsClient] = useState(false);

    useEffect(() => {
        setIsClient(true);
        const stored = localStorage.getItem('business_display_name') || localStorage.getItem('business_display_name') || 'demo';
        setTenant(stored);
    }, []);

    const handleCopy = () => {
        const url = `${window.location.origin}/unlock/${tenant}?c=${btoa(couponCode)}&s=${requiredShares}`;
        navigator.clipboard.writeText(url);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    return (
        <div className="min-h-screen bg-gray-50 flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8 font-inter">
            <div className="w-full max-w-5xl space-y-8">
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-3xl font-bold font-outfit text-gray-900">Share-to-Unlock Coupon 🎁</h1>
                        <p className="mt-2 text-gray-600">Create a viral loop by requiring users to share before they get a discount.</p>
                    </div>
                    <button onClick={() => router.push('/dashboard')} className="text-sm font-medium text-indigo-600 hover:text-indigo-500">
                        &larr; Back to Dashboard
                    </button>
                </div>

                <div className="flex flex-col lg:flex-row gap-8">
                    {/* Editor */}
                    <div className="flex-1 bg-white p-6 rounded-2xl shadow-sm border border-gray-100 space-y-6">
                        <h2 className="text-xl font-semibold text-gray-900 font-outfit border-b pb-4">Coupon Settings</h2>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Headline</label>
                            <input
                                type="text"
                                value={headline}
                                onChange={(e) => setHeadline(e.target.value)}
                                placeholder="e.g. 20% Off Your First Order"
                                className="w-full border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-2.5 border"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                            <textarea
                                value={description}
                                onChange={(e) => setDescription(e.target.value)}
                                className="w-full border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-2.5 border h-20 resize-none"
                            />
                        </div>

                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Coupon Code</label>
                                <input
                                    type="text"
                                    value={couponCode}
                                    onChange={(e) => setCouponCode(e.target.value)}
                                    placeholder="e.g. WELCOME20"
                                    className="w-full border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-2.5 border font-mono uppercase"
                                />
                            </div>
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Required Shares</label>
                                <input
                                    type="number"
                                    min="1"
                                    max="10"
                                    value={requiredShares}
                                    onChange={(e) => setRequiredShares(parseInt(e.target.value) || 1)}
                                    className="w-full border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-2.5 border"
                                />
                            </div>
                        </div>

                        <div className="pt-6 border-t border-gray-100">
                            <button
                                onClick={handleCopy}
                                className={`w-full py-3 px-4 rounded-xl text-sm font-semibold text-white transition-all shadow-sm ${copied ? 'bg-green-500 hover:bg-green-600' : 'bg-indigo-600 hover:bg-indigo-700'}`}
                            >
                                {copied ? 'Copied!' : 'Copy Link'}
                            </button>
                        </div>
                    </div>

                    {/* Preview */}
                    <div className="flex-1">
                        <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-4 ml-2">Preview: Unlock Page</h2>
                        <div className="bg-gradient-to-br from-indigo-50 via-white to-purple-50 p-8 rounded-3xl shadow-sm border border-indigo-100 min-h-[500px] flex flex-col items-center justify-center text-center relative overflow-hidden">

                            <div className="absolute top-0 right-0 w-64 h-64 bg-purple-200 rounded-full mix-blend-multiply filter blur-3xl opacity-30 animate-blob"></div>
                            <div className="absolute top-0 left-0 w-64 h-64 bg-indigo-200 rounded-full mix-blend-multiply filter blur-3xl opacity-30 animate-blob animation-delay-2000"></div>

                            <div className="relative z-10 w-full max-w-sm">
                                <div className="bg-white p-6 rounded-2xl shadow-xl border border-gray-50 mb-6">
                                    <div className="w-16 h-16 bg-indigo-100 rounded-full flex items-center justify-center mx-auto mb-4">
                                        <span className="text-3xl">🎁</span>
                                    </div>
                                    <h3 className="text-2xl font-bold font-outfit text-gray-900 mb-2">{headline}</h3>
                                    <p className="text-sm text-gray-500 mb-6">{description}</p>

                                    <div className="bg-gray-50 border-2 border-dashed border-gray-200 rounded-xl p-4 mb-6 relative overflow-hidden group">
                                        <div className="absolute inset-0 backdrop-blur-[4px] bg-white/40 z-10 flex flex-col items-center justify-center transition-all">
                                            <span className="text-xs font-bold text-gray-800 bg-white px-3 py-1 rounded-full shadow-sm mb-1">Locked</span>
                                        </div>
                                        <span className="font-mono text-xl font-bold text-gray-400 select-none filter blur-[2px]">{couponCode}</span>
                                    </div>

                                    <div className="space-y-3">
                                        <div className="flex justify-between items-center text-xs font-medium text-gray-500 mb-1 px-1">
                                            <span>Progress</span>
                                            <span>1 / {requiredShares}</span>
                                        </div>
                                        <div className="w-full bg-gray-100 rounded-full h-2 mb-4">
                                            <div className="bg-indigo-600 h-2 rounded-full transition-all duration-500" style={{ width: `${(1 / requiredShares) * 100}%` }}></div>
                                        </div>
                                        <button className="w-full py-3 bg-gray-900 text-white rounded-xl text-sm font-bold shadow-md hover:bg-gray-800 transition-colors flex items-center justify-center gap-2">
                                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" /></svg>
                                            Share to Unlock ({requiredShares} Shares)
                                        </button>
                                    </div>
                                </div>
                                <div className="text-center opacity-60 hover:opacity-100 transition-opacity">
                                    {isClient && <PoweredByOHC tenantId={tenant} />}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
