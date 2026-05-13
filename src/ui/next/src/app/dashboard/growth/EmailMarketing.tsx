'use client';
import React, { useState } from 'react';

export default function EmailMarketing() {
    const [campaignName, setCampaignName] = useState('');
    const [prompt, setPrompt] = useState('');
    const [preview, setPreview] = useState<any>(null);
    const [status, setStatus] = useState('');

    const generateTemplate = async () => {
        const res = await fetch('/api/v1/growth/campaign/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: campaignName, contact_ids: [], prompt })
        });
        if (res.ok) {
            setPreview(await res.json());
        } else if (res.status === 402) {
            setStatus('Free tier limit reached. Please upgrade to send campaigns.');
        }
    };

    const sendCampaign = async () => {
        const res = await fetch('/api/v1/growth/campaign/send', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: campaignName, subject: preview.subject, body: preview.body, target_segment: 'all' })
        });
        if (res.ok) setStatus('Campaign successfully dispatched to contacts!');
    };

    return (
        <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">Email Marketing (Simple)</h2>
            <input className="w-full border p-2 mb-2 rounded" placeholder="Campaign Name (e.g. Flash Sale)" value={campaignName} onChange={e => setCampaignName(e.target.value)} />
            <textarea className="w-full border p-2 mb-4 rounded" placeholder="Prompt for AI Template..." value={prompt} onChange={e => setPrompt(e.target.value)} />
            <button onClick={generateTemplate} className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded">
                Generate AI Template
            </button>

            {preview && (
                <div className="mt-4 p-4 border rounded bg-gray-50">
                    <p className="font-bold">Subject: {preview.subject}</p>
                    <p className="mt-2 whitespace-pre-line text-sm">{preview.body}</p>
                    <button onClick={sendCampaign} className="mt-4 bg-green-600 text-white px-4 py-2 rounded">
                        Send Campaign
                    </button>
                </div>
            )}
            {status && <p className="mt-3 text-sm text-gray-800 font-medium">{status}</p>}
        </div>
    );
}
