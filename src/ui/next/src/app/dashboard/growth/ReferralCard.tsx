'use client';
import React, { useState, useEffect } from 'react';

export default function ReferralCard() {
    const [referralLink, setReferralLink] = useState('');
    const [stats, setStats] = useState([]);

    useEffect(() => {
        fetch('/api/v1/growth/referral/dashboard')
            .then(res => res.json())
            .then(data => setStats(data.referrals || []));
    }, []);

    const generateLink = async () => {
        const res = await fetch('/api/v1/growth/referral/generate', { method: 'POST' });
        if (res.ok) {
            const data = await res.json();
            setReferralLink(data.link);
        }
    };

    return (
        <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">Referral Program</h2>
            <p className="mb-4 text-sm text-gray-500">Share OHC with a friend, both get 1 month free Pro.</p>
            <button onClick={generateLink} className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow-sm">
                Generate Referral Link
            </button>
            {referralLink && (
                <div className="mt-4 p-3 bg-gray-100 rounded border">
                    <code className="text-sm select-all">{referralLink}</code>
                    <button className="mt-2 text-blue-500 text-xs block" onClick={() => navigator.clipboard.writeText(referralLink)}>Copy</button>
                </div>
            )}
            <div className="mt-6">
                <h3 className="font-semibold text-gray-700">Invite Tracking</h3>
                <ul className="mt-2 space-y-2">
                    {stats.map((s: any, i) => (
                        <li key={i} className="flex justify-between text-sm">
                            <span>Code: {s.code}</span>
                            <span>Clicks: {s.clicks} | Conversions: {s.conversions}</span>
                        </li>
                    ))}
                </ul>
            </div>
        </div>
    );
}
