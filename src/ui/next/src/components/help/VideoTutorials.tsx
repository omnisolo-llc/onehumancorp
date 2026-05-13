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
        <div className="video-tutorials portrait-optimized">
            {videos.map(v => (
                <div key={v.id}>
                    <h3>{v.title}</h3>
                    <video src={v.url} controls />
                </div>
            ))}
        </div>
    );
}
