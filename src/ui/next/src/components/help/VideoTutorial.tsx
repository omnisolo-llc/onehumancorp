'use client';

import React, { useState, useEffect } from 'react';

interface Video {
  id: string;
  title: string;
  url: string;
  duration: number;
  thumbnail: string;
}

export function VideoTutorial() {
  const [videos, setVideos] = useState<Video[]>([]);
  const [playing, setPlaying] = useState<string | null>(null);

  useEffect(() => {
    fetch('/api/help/videos')
      .then(res => res.json())
      .then(data => setVideos(data));
  }, []);

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
      {videos.map(video => (
        <div key={video.id} className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden group flex flex-col">
          {playing === video.id ? (
             <div className="relative aspect-video bg-black">
               <video
                 src={video.url}
                 controls
                 autoPlay
                 className="w-full h-full object-contain"
                 onEnded={() => setPlaying(null)}
               />
             </div>
          ) : (
            <div
              className="relative aspect-video bg-slate-100 cursor-pointer overflow-hidden"
              onClick={() => setPlaying(video.id)}
            >
              <div className="absolute inset-0 bg-black/10 group-hover:bg-black/20 transition-colors" />
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="w-12 h-12 bg-white/90 backdrop-blur-sm rounded-full flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform">
                  <svg className="w-6 h-6 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z" /></svg>
                </div>
              </div>
              <div className="absolute bottom-2 right-2 px-2 py-1 bg-black/70 text-white text-xs font-medium rounded backdrop-blur-sm">
                0:{video.duration.toString().padStart(2, '0')}
              </div>
            </div>
          )}
          <div className="p-3 bg-white">
            <h4 className="font-semibold text-sm text-slate-800 line-clamp-1" style={{ fontFamily: 'Outfit, sans-serif' }}>{video.title}</h4>
            <p className="text-xs text-slate-500 mt-1">Short Tutorial • Under 90s</p>
          </div>
        </div>
      ))}
    </div>
  );
}
