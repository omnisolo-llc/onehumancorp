"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function SplitConfigurator() {
    const [subMerchant, setSubMerchant] = useState('');
    const [percentage, setPercentage] = useState(70);
    const router = useRouter();

    const handleConfirm = async () => {
        // Send config logic here
        alert(`Split confirmed: ${percentage}% to ${subMerchant}`);
        router.back();
    };

    return (
        <div style={{ padding: '20px', fontFamily: 'system-ui' }}>
            <div style={{ background: 'rgba(255,255,255,0.1)', backdropFilter: 'blur(20px)', borderRadius: '12px', padding: '20px', boxShadow: '0 4px 6px rgba(0,0,0,0.1)' }}>
                <h2>Split Configurator</h2>
                <label>
                    Who gets a cut?
                    <input
                        type="text"
                        value={subMerchant}
                        onChange={(e) => setSubMerchant(e.target.value)}
                        placeholder="Search or enter email"
                        style={{ display: 'block', width: '100%', marginBottom: '10px' }}
                    />
                </label>
                <label>
                    Percentage: {percentage}%
                    <input
                        type="range"
                        min="0"
                        max="100"
                        value={percentage}
                        onChange={(e) => setPercentage(parseInt(e.target.value))}
                        style={{ display: 'block', width: '100%', marginBottom: '10px' }}
                    />
                </label>
                <p><i>If this sells for $100, {subMerchant || 'they'} gets ${percentage}, you get ${100 - percentage}.</i></p>
                <button onClick={handleConfirm} style={{ padding: '10px 20px', fontSize: '16px', borderRadius: '8px', background: '#0070f3', color: '#fff', cursor: 'pointer' }}>
                    Confirm Split
                </button>
            </div>
        </div>
    );
}
