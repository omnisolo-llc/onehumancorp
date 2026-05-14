"use client";
import React, { useState, useEffect } from 'react';

export default function VideoTutorials() {
    const [videos, setVideos] = useState<{id: string, title: string, url: string}[]>([]);

    useEffect(() => {
        fetch('/api/help/videos')
            .then(res => res.json())
            .then(data => setVideos(data))
            .catch(() => {});
    }, []);

    return (
        <div className="video-tutorials portrait-optimized" style={{ padding: '20px' }}>
            <h1>Video Tutorials</h1>
            <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
                {videos.map(v => (
                    <div key={v.id} style={{ border: '1px solid #ccc', borderRadius: '8px', padding: '16px', background: 'rgba(255,255,255,0.05)' }}>
                        <h3>{v.title}</h3>
                        <video src={v.url} controls style={{ maxWidth: '300px', borderRadius: '4px' }} />
                    </div>
                ))}
            </div>
        </div>
    );
}
