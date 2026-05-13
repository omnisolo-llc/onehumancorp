'use client';
import React, { useState } from 'react';

export default function StorefrontShare() {
    const [preview, setPreview] = useState<any>(null);

    const fetchStorefront = async () => {
        const res = await fetch('/api/v1/growth/storefront/render', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: 'system' })
        });
        if (res.ok) {
            setPreview(await res.json());
        }
    };

    return (
        <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">Business Share & Embed</h2>
            <p className="text-sm text-gray-500 mb-4">Shareable OpenGraph link card for your business.</p>
            <button onClick={fetchStorefront} className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded">
                View Storefront Embed
            </button>
            {preview && (
                <div className="mt-4 p-4 border rounded bg-gray-50 overflow-hidden">
                    <h3 className="font-bold text-gray-700 mb-2">HTML Output</h3>
                    <pre className="text-xs text-gray-600 whitespace-pre-wrap">{preview.html}</pre>
                    {preview.viral_badge && (
                        <div className="mt-4 p-2 bg-yellow-100 text-yellow-800 text-xs rounded">
                            Includes viral loop: "Built with OHC". Upgrade to Starter/Pro to remove.
                        </div>
                    )}
                </div>
            )}
        </div>
    );
}
