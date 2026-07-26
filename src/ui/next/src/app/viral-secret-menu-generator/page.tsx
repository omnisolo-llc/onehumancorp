"use client";

import React, { useState, useEffect } from 'react';
import Head from 'next/head';

export default function SecretMenuGeneratorPage() {
    const [tenant, setTenant] = useState('demo-business');
    const [itemName, setItemName] = useState('');
    const [itemDesc, setItemDesc] = useState('');
    const [accessCode, setAccessCode] = useState('');
    const [sharesReq, setSharesReq] = useState('3');
    const [copied, setCopied] = useState(false);

    useEffect(() => {
        if (typeof window !== 'undefined') {
            const storedTenant = localStorage.getItem('business_display_name');
            if (storedTenant) {
                setTenant(storedTenant);
            }
        }
    }, []);

    const handleCopy = () => {
        navigator.clipboard.writeText(embedUrl);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    const origin = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
    const embedUrl = `${origin}/api/v1/growth/secret-menu/embed?tenant=${encodeURIComponent(tenant)}&item_name=${encodeURIComponent(itemName)}&item_desc=${encodeURIComponent(itemDesc)}&access_code=${encodeURIComponent(accessCode)}&shares_req=${encodeURIComponent(sharesReq)}`;

    return (
        <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-8">
            <Head>
                <title>Viral Secret Menu Generator | OHC Growth</title>
            </Head>

            <div className="max-w-4xl mx-auto">
                <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">Viral Secret Menu Generator 🤫</h1>
                <p className="text-gray-600 dark:text-gray-400 mb-8">
                    Create a hidden menu item or secret offer that unlocks only when customers share it with friends.
                </p>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                    {/* Configuration Form */}
                    <div className="bg-white dark:bg-gray-800 p-6 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-700">
                        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">Configure Widget</h2>

                        <div className="space-y-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Secret Item Name
                                </label>
                                <input
                                    id="itemName"
                                    type="text"
                                    value={itemName}
                                    onChange={(e) => setItemName(e.target.value)}
                                    placeholder="e.g. Double Smash Burger"
                                    className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md bg-transparent dark:text-white"
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Item Description
                                </label>
                                <input
                                    id="itemDesc"
                                    type="text"
                                    value={itemDesc}
                                    onChange={(e) => setItemDesc(e.target.value)}
                                    placeholder="e.g. Extra cheese, extra smash."
                                    className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md bg-transparent dark:text-white"
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Access Code (Unlocked upon sharing)
                                </label>
                                <input
                                    id="accessCode"
                                    type="text"
                                    value={accessCode}
                                    onChange={(e) => setAccessCode(e.target.value)}
                                    placeholder="e.g. SMASHX2"
                                    className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md bg-transparent dark:text-white"
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Shares Required
                                </label>
                                <input
                                    id="sharesReq"
                                    type="number"
                                    min="1"
                                    value={sharesReq}
                                    onChange={(e) => setSharesReq(e.target.value)}
                                    className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md bg-transparent dark:text-white"
                                />
                            </div>
                        </div>

                        <div className="mt-8">
                            <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-2">Embed & Share</h3>
                            <div className="flex items-center gap-2 mb-2 bg-gray-50 dark:bg-gray-900 p-2 rounded-lg border border-gray-200 dark:border-gray-700">
                                <span id="shareLink" className="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-xs text-gray-500">
                                    {embedUrl}
                                </span>
                                <button
                                    id="copyBtn"
                                    onClick={handleCopy}
                                    className="px-3 py-1 bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-medium rounded transition-colors"
                                >
                                    {copied ? 'Copied!' : 'Copy Link'}
                                </button>
                            </div>
                            <p className="text-xs text-gray-500 dark:text-gray-400">
                                Copy the URL above or embed it via an iframe on your storefront.
                            </p>
                        </div>
                    </div>

                    {/* Live Preview */}
                    <div className="bg-gray-100 dark:bg-gray-800/50 p-6 rounded-2xl border border-gray-200 dark:border-gray-700 flex flex-col">
                        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">Live Preview</h2>
                        <div className="flex-1 bg-white dark:bg-gray-900 rounded-xl overflow-hidden shadow-inner border border-gray-200 dark:border-gray-700">
                            <iframe
                                id="previewFrame"
                                src={embedUrl}
                                className="w-full h-full min-h-[500px]"
                                title="Secret Menu Widget Preview"
                                frameBorder="0"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
