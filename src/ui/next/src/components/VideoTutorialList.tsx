"use client";

import React, { useEffect, useState } from 'react';

type VideoTutorial = {
  id: number;
  title: string;
  duration: string;
};

export function VideoTutorialList() {
  const [videos, setVideos] = useState<VideoTutorial[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => {
        setVideos(data);
        setLoading(false);
      })
      .catch(err => {
        console.error('Failed to load video tutorials', err);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <div className="flex justify-center items-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (videos.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        <p>No video tutorials available right now.</p>
      </div>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto py-8 px-4 font-inter">
      <h2 className="text-2xl font-extrabold font-outfit text-gray-900 mb-6 text-center sm:text-left">Video Tutorials</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {videos.map(video => (
          <div key={video.id} className="bg-white/80 backdrop-blur-[20px] saturate-200 rounded-2xl shadow-sm border border-gray-100 overflow-hidden group hover:shadow-md transition-all cursor-pointer flex flex-col">
            {/* Mock video player area (portrait optimized 9:16 approx for mobile shorts feel, or standard 16:9) */}
            <div className="w-full aspect-[9/16] sm:aspect-video bg-gray-900 relative flex items-center justify-center">
              {/* Play button overlay */}
              <div className="absolute inset-0 bg-black/20 group-hover:bg-black/10 transition-colors flex items-center justify-center">
                <div className="w-12 h-12 bg-white/30 backdrop-blur-md rounded-full flex items-center justify-center group-hover:scale-110 transition-transform">
                  <svg className="w-6 h-6 text-white ml-1" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M8 5v14l11-7z" />
                  </svg>
                </div>
              </div>
              {/* Duration badge */}
              <div className="absolute bottom-3 right-3 bg-black/70 text-white text-xs px-2 py-1 rounded-md font-medium">
                {video.duration}
              </div>
            </div>

            <div className="p-4 flex-grow">
              <h3 className="font-bold text-gray-900 leading-tight mb-2 group-hover:text-blue-600 transition-colors line-clamp-2">
                {video.title}
              </h3>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
