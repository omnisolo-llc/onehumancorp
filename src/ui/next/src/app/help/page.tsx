"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{category: string, title: string, desc: string, link: string}[]>([]);
  const [videos, setVideos] = useState<{id: number, title: string, duration: string, video_url: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedVideo, setSelectedVideo] = useState<{id: number, title: string, duration: string, video_url: string} | null>(null);

  useEffect(() => {
    const url = searchQuery.trim() ? `/api/help/search?q=${encodeURIComponent(searchQuery.trim())}` : '/api/help/search?q=';
    fetch(url)
      .then(res => res.json())
      .then(data => setArticles(data))
      .catch(console.error);
  }, [searchQuery]);

  useEffect(() => {
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => setVideos(data))
      .catch(console.error);
  }, []);

  const filteredArticles = articles;

  const filteredVideos = videos.filter(video =>
    video.title.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-8 text-center tracking-tight">Help Center</h1>

        <div className="mb-10 w-full sm:w-3/4 mx-auto">
          <input
            type="text"
            placeholder="Search for help articles and videos..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 rounded-2xl border border-white/50 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-[0_8px_32px_rgba(0,0,0,0.05)] text-gray-900 backdrop-blur-[20px] saturate-200 bg-white/70 hover:bg-white/80 min-h-[44px] text-base placeholder:text-gray-500 transition-all"
          />
        </div>

        {filteredArticles.length === 0 && filteredVideos.length === 0 ? (
          <div className="flex flex-col items-center justify-center bg-white/40 backdrop-blur-[20px] saturate-200 py-16 px-4 rounded-2xl border border-white/30 shadow-[0_4px_16px_rgba(0,0,0,0.02)]">
            <svg className="w-16 h-16 text-gray-400 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p className="text-center text-gray-600 font-medium text-lg">
              No results found matching <span className="text-gray-900 font-semibold">"{searchQuery}"</span>
            </p>
            <p className="text-center text-gray-500 mt-2 text-sm">
              Try adjusting your search terms or ask our AI assistant for help.
            </p>
          </div>
        ) : (
          <div className="space-y-12">
            {filteredArticles.length > 0 && (
              <div className="space-y-12">
                {Array.from(new Set(filteredArticles.map(a => a.category || "General"))).map((category) => (
                  <section key={category}>
                    <div className="flex items-center mb-6">
                      <h2 className="text-2xl font-bold font-outfit text-gray-900">{category}</h2>
                      <div className="ml-4 flex-grow border-t border-gray-200/50"></div>
                    </div>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
                      {filteredArticles.filter(a => (a.category || "General") === category).map((article, idx) => (
                        <Link key={idx} href={article.link} className="block group">
                          <div className="app-card backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_4px_16px_rgba(0,0,0,0.04)] border border-white/50 group-hover:border-blue-300 group-hover:shadow-[0_0_15px_rgba(59,130,246,0.5)] group-hover:-translate-y-1 hover:bg-white/80 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[140px]">
                            <h3 className="text-xl font-bold font-outfit text-blue-600 mb-3 group-hover:text-blue-700">{article.title}</h3>
                            <p className="text-gray-600 leading-relaxed flex-grow">{article.desc}</p>
                          </div>
                        </Link>
                      ))}
                    </div>
                  </section>
                ))}
              </div>
            )}

            {filteredVideos.length > 0 && (
              <section>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Video Tutorials</h2>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                  {filteredVideos.map((v) => (
                    <div key={v.id} onClick={() => setSelectedVideo(v)} className="aspect-[9/16] bg-gray-200 rounded-2xl flex items-center justify-center relative overflow-hidden group shadow-sm border border-white/30 cursor-pointer hover:shadow-md transition-all">
                      <div className="absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-all"></div>
                      <div className="w-10 h-10 bg-white/90 backdrop-blur-sm rounded-full flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
                        <svg className="w-5 h-5 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                      </div>
                      <div className="absolute bottom-2 left-2 right-2 z-10">
                        <p className="text-white text-xs font-bold drop-shadow-md line-clamp-2 leading-tight">{v.title}</p>
                        <p className="text-white/80 text-[10px] font-medium mt-0.5">{v.duration}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </section>
            )}
          </div>
        )}
      </div>

      {selectedVideo && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
          <div className="relative w-full max-w-4xl bg-black rounded-2xl overflow-hidden shadow-2xl">
            <div className="absolute top-4 right-4 z-10 flex space-x-2">
              <button
                aria-label="Close video"
                onClick={() => setSelectedVideo(null)}
                className="bg-white/10 hover:bg-white/20 text-white rounded-full p-2 backdrop-blur-md transition-all font-inter text-sm px-4 flex items-center gap-2 border border-white/20 shadow-lg"
              >
                Close video
              </button>
            </div>
            <div className="aspect-video w-full bg-gray-900 flex items-center justify-center relative">
              <video
                src={selectedVideo.video_url || ""}
                controls
                autoPlay
                className="w-full h-full object-contain"
              >
                Your browser does not support the video tag.
              </video>
            </div>
            <div className="absolute bottom-0 left-0 right-0 p-6 bg-gradient-to-t from-black/80 to-transparent">
              <h3 className="text-white font-outfit text-xl font-bold">{selectedVideo.title}</h3>
              <p className="text-white/80 font-inter text-sm">{selectedVideo.duration}</p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
