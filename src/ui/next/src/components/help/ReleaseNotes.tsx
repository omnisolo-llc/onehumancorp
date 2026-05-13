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
        <div className="release-notes">
            {notes.map(n => (
                <div key={n.version}>
                    <h3>Version {n.version}</h3>
                    <p>{n.date}</p>
                    <p>{n.content}</p>
                </div>
            ))}
        </div>
    );
}
