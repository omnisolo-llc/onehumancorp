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
  const [activeVideo, setActiveVideo] = useState<VideoTutorial | null>(null);

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
          <div key={video.id} onClick={() => setActiveVideo(video)} className="bg-white/80 backdrop-blur-[20px] saturate-200 rounded-2xl shadow-sm border border-gray-100 overflow-hidden group hover:shadow-md transition-all cursor-pointer flex flex-col">
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
      {/* Video Player Modal */}
      {activeVideo && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center bg-gray-900/80 backdrop-blur-[20px] saturate-200 p-4 animate-fade-in">
          <div className="bg-black backdrop-blur-[30px] rounded-3xl shadow-2xl flex flex-col overflow-hidden border border-white/20 w-full max-w-[375px] mx-auto aspect-[9/16] relative animate-pop-in">
            {/* Header */}
            <div className="absolute top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/90 to-transparent z-10 flex justify-between items-start pt-6">
              <h3 className="text-white font-bold font-outfit text-base pr-4 line-clamp-2 drop-shadow-md leading-tight">{activeVideo.title}</h3>
              <button onClick={() => setActiveVideo(null)} className="text-white/80 hover:text-white bg-white/20 hover:bg-white/30 backdrop-blur-md border border-white/20 rounded-full p-2 transition-all min-h-[44px] min-w-[44px] flex items-center justify-center flex-shrink-0" aria-label="Close video">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            {/* Fake Video Player area */}
            <div className="flex-1 flex items-center justify-center relative bg-gradient-to-br from-gray-800 to-black">
               {/* Simulating a video placeholder background */}
               <div className="absolute inset-0 bg-blue-500/10 blur-3xl rounded-full scale-150 mix-blend-screen pointer-events-none"></div>
               <button className="w-16 h-16 bg-white/20 hover:bg-white/30 backdrop-blur-[20px] saturate-200 border border-white/30 rounded-full flex items-center justify-center shadow-[0_8px_32px_rgba(0,0,0,0.5)] transition-all active:scale-95 group z-20">
                  <svg className="w-8 h-8 text-white ml-1 group-hover:scale-110 transition-transform" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
               </button>
            </div>

            {/* Controls */}
            <div className="absolute bottom-0 left-0 right-0 p-4 bg-gradient-to-t from-black/80 to-transparent z-10 flex flex-col gap-3">
              <div className="flex items-center gap-3">
                <button className="text-white/80 hover:text-white transition-colors">
                   <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                </button>
                <div className="h-1.5 flex-1 bg-white/20 backdrop-blur-sm rounded-full overflow-hidden cursor-pointer relative group">
                  <div className="h-full bg-blue-500 w-1/3 relative shadow-[0_0_10px_rgba(59,130,246,0.8)]">
                    <div className="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-2 bg-white rounded-full scale-0 group-hover:scale-100 transition-transform shadow-md"></div>
                  </div>
                </div>
                <div className="text-white/80 text-[10px] font-medium font-inter tabular-nums">
                  0:00 / {activeVideo.duration}
                </div>
                <button className="text-white/80 hover:text-white transition-colors">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" /></svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
