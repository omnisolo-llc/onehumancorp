'use client';
import { useState, useEffect } from 'react';

export default function Videos() {
  const [videos, setVideos] = useState([]);

  useEffect(() => {
    fetch('/api/v1/docs/videos')
      .then(r => r.json())
      .then(d => {
        if (d.status === 'ok') setVideos(d.videos || []);
      })
      .catch(() => {});
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 p-6 font-inter">
      <h1 className="text-2xl font-bold font-outfit mb-6">Video Tutorials</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {videos.map((v: any) => (
          <div key={v.id} className="bg-white rounded-lg shadow-sm p-4 border border-gray-100">
            <div className="aspect-[9/16] md:aspect-video bg-gray-900 rounded flex items-center justify-center relative overflow-hidden">
               <span className="text-white text-sm">▶ Play</span>
               <div className="absolute inset-0 border-[4px] border-transparent hover:border-blue-500 transition-colors pointer-events-none rounded"></div>
            </div>
            <h2 className="mt-3 font-semibold text-gray-900">{v.title}</h2>
          </div>
        ))}
      </div>
    </div>
  );
}
