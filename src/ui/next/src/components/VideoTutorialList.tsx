"use client";

import React, { useEffect, useState } from 'react';

type VideoTutorial = {
  id: number;
  title: string;
  duration: string;
  video_url: string;
};

export function VideoTutorialList({
  videos: initialVideos,
  loading: externalLoading,
}: {
  videos?: VideoTutorial[];
  loading?: boolean;
} = {}) {
  const [fetchedVideos, setFetchedVideos] = useState<VideoTutorial[]>([]);
  const [fetchedLoading, setFetchedLoading] = useState(true);
  const [activeVideo, setActiveVideo] = useState<VideoTutorial | null>(null);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    if (initialVideos !== undefined) return;
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => {
        setFetchedVideos(data);
        setFetchedLoading(false);
      })
      .catch(err => {
        console.error('Failed to load video tutorials', err);
        setFetchedLoading(false);
      });
  }, [initialVideos]);

  const loading = externalLoading !== undefined ? externalLoading : fetchedLoading;
  const videos = initialVideos !== undefined ? initialVideos : fetchedVideos;

  const filteredVideos = videos.filter(video =>
    video.title.toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (loading) {
    return (
      <div className="flex justify-center items-center py-12 backdrop-filter backdrop-blur-[30px] saturate-[210%] bg-white/30">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto py-8 px-4 font-inter">
      <div className="flex flex-col sm:flex-row justify-between items-center mb-6 gap-4">
        <h2 className="text-2xl font-extrabold font-outfit text-gray-900 text-center sm:text-left">Video Tutorials</h2>
        <div className="w-full sm:w-64 relative">
          <input
            type="text"
            placeholder="Search videos..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all outline-none"
          />
          <svg className="w-5 h-5 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>
      </div>

      {videos.length === 0 ? (
        <div className="text-center py-12 text-gray-500">
          <p>No video tutorials available right now.</p>
        </div>
      ) : filteredVideos.length === 0 ? (
        <div className="text-center py-12 text-gray-500">
          <p>No video tutorials match your search.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredVideos.map(video => (
            <div key={video.id} onClick={() => setActiveVideo(video)} className="backdrop-blur-[30px] bg-white/80 dark:bg-black/40 saturate-[210%] rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.08)] border border-white/80 dark:border-white/20 overflow-hidden group hover:shadow-lg transition-all cursor-pointer flex flex-col hover:-translate-y-1">
            {/* Mock video player area (portrait optimized 9:16 approx for mobile shorts feel, or standard 16:9) */}
            <div className="w-full aspect-[9/16] sm:aspect-video bg-gray-900 relative flex items-center justify-center">
              {/* Play button overlay */}
              <div className="absolute inset-0 bg-black/20 group-hover:bg-black/10 transition-colors flex items-center justify-center">
                <div className="w-12 h-12 bg-white/30 backdrop-blur-[30px] saturate-[210%] rounded-full flex items-center justify-center group-hover:scale-110 transition-transform">
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
      )}
      {/* Video Player Modal */}
      {activeVideo && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center bg-gray-900/80 backdrop-blur-[30px] saturate-[210%] saturate-200 p-4 animate-fade-in">
          <div className="bg-black backdrop-blur-[30px] saturate-[210%] rounded-3xl shadow-2xl flex flex-col overflow-hidden border border-white/20 w-full max-w-[375px] mx-auto aspect-[9/16] relative animate-pop-in">
            {/* Header */}
            <div className="absolute top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/90 to-transparent z-10 flex justify-between items-start pt-6">
              <h3 className="text-white font-bold font-outfit text-base pr-4 line-clamp-2 drop-shadow-md leading-tight">{activeVideo.title}</h3>
              <button onClick={() => setActiveVideo(null)} className="text-white/80 hover:text-white bg-white/20 hover:bg-white/30 backdrop-blur-[30px] saturate-[210%] border border-white/20 rounded-full p-2 transition-all min-h-[44px] min-w-[44px] flex items-center justify-center flex-shrink-0" aria-label="Close video">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            {/* Real Video Player area */}
            <div className="flex-1 flex items-center justify-center relative bg-black">
               <video
                 controls
                 className="w-full h-full object-contain"
                 src={activeVideo.video_url || undefined}
                 autoPlay
               >
                 Your browser does not support the video tag.
               </video>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
