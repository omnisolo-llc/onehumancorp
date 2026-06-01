"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function VideosPage() {
  const [videos, setVideos] = useState<{id: number, title: string, duration: string}[]>([]);

  useEffect(() => {
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => setVideos(data))
      .catch(console.error);
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 text-center font-outfit">Video Tutorials</h1>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {videos.map((video) => (
            <div key={video.id} className="bg-white/80 backdrop-blur-[20px] saturate-200 p-0 rounded-xl shadow-sm border border-gray-100 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer overflow-hidden flex flex-col group">
              <div className="bg-gray-200 h-48 relative flex items-center justify-center">
                <div className="absolute inset-0 bg-black/10 group-hover:bg-black/20 transition-colors"></div>
                <svg className="w-16 h-16 text-white opacity-80 group-hover:opacity-100 group-hover:scale-110 transition-all z-10" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
              </div>
              <div className="p-4 flex flex-col flex-grow">
                <h2 className="text-lg font-bold text-gray-900 mb-2 font-outfit leading-tight">{video.title}</h2>
                <div className="mt-auto flex justify-between items-center text-sm">
                  <span className="bg-blue-50 text-blue-700 px-2.5 py-1 rounded-md font-semibold">{video.duration}</span>
                  <span className="text-gray-400 font-medium">Play video</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
