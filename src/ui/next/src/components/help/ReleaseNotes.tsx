"use client";
import React, { useState, useEffect } from 'react';

export default function ReleaseNotes() {
    const [notes, setNotes] = useState<{version: string, date: string, content: string}[]>([]);

    useEffect(() => {
        fetch('/api/help/changelog')
            .then(res => res.json())
            .then(data => setNotes(data))
            .catch(() => {});
    }, []);

    return (
        <div className="release-notes" style={{ padding: '20px' }}>
            <h1>Release Notes & Changelog</h1>
            {notes.map(n => (
                <div key={n.version} style={{ marginBottom: '20px', padding: '16px', borderLeft: '4px solid #0070f3', background: 'rgba(255,255,255,0.05)' }}>
                    <h3 style={{ margin: '0 0 8px 0' }}>Version {n.version} <span style={{ fontSize: '0.8rem', color: '#666', fontWeight: 'normal' }}>{n.date}</span></h3>
                    <p style={{ margin: 0 }}>{n.content}</p>
                </div>
            ))}
        </div>
    );
}
