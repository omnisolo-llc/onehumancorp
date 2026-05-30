"use client";

import React, { useEffect, useState } from 'react';

type Video = {
  id: number;
  title: string;
  duration: string;
};

export default function VideosPage() {
  const [videos, setVideos] = useState<Video[]>([]);
  const [activeVideo, setActiveVideo] = useState<Video | null>(null);

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
        <p className="text-center text-gray-600 mb-12">Learn how to use OneHumanCorp with these short, easy-to-follow videos.</p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {videos.map((video) => (
            <div key={video.id} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex flex-col">
              <div className="w-full aspect-video bg-gray-200 rounded-lg mb-4 flex items-center justify-center relative overflow-hidden group">
                <div className="absolute inset-0 bg-black/20 group-hover:bg-black/40 transition-colors"></div>
                <div className="w-12 h-12 bg-white/90 backdrop-blur-md rounded-full flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
                  <svg className="w-6 h-6 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z" /></svg>
                </div>
                <div className="absolute bottom-2 right-2 bg-black/70 text-white text-xs px-2 py-1 rounded font-medium z-10">
                  {video.duration}
                </div>
              </div>
              <h2 className="text-lg font-bold text-gray-900 mb-2 font-outfit">{video.title}</h2>
              <button
                onClick={() => setActiveVideo(video)}
                className="mt-auto w-full py-2 bg-blue-50 text-blue-600 font-semibold rounded-lg hover:bg-blue-100 transition-colors text-sm"
              >
                Watch Video
              </button>
            </div>
          ))}
        </div>
      </div>

      {activeVideo && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/80 backdrop-blur-sm animate-fade-in">
          <div className="bg-black rounded-2xl overflow-hidden shadow-2xl max-w-sm w-full relative flex flex-col aspect-[9/16]">
            {/* Portrait-optimized player simulation */}
            <div className="flex-1 flex items-center justify-center bg-gray-900 relative">
               <video
                  autoPlay
                  loop
                  muted
                  playsInline
                  className="absolute inset-0 w-full h-full object-cover opacity-50"
                  src="https://www.w3schools.com/html/mov_bbb.mp4"
               />
               <div className="z-10 text-white text-center p-6">
                 <div className="w-16 h-16 bg-white/20 backdrop-blur-md rounded-full flex items-center justify-center mx-auto mb-4 animate-pulse">
                    <svg className="w-8 h-8 text-white ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z" /></svg>
                 </div>
                 <h3 className="font-outfit font-bold text-xl mb-2">{activeVideo.title}</h3>
                 <p className="text-gray-300 text-sm">Playing tutorial video...</p>
               </div>
            </div>
            <button
              onClick={() => setActiveVideo(null)}
              className="absolute top-4 right-4 text-white/70 hover:text-white bg-black/40 hover:bg-black/60 rounded-full p-2 backdrop-blur-md transition-all"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes fade-in {
          0% { opacity: 0; }
          100% { opacity: 1; }
        }
        .animate-fade-in { animation: fade-in 0.2s ease-out forwards; }
      `}} />
    </div>
  );
}
