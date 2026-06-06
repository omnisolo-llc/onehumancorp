"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{title: string, desc: string, link: string}[]>([]);
  const [videos, setVideos] = useState<{id: number, title: string, duration: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    fetch('/api/help')
      .then(res => res.json())
      .then(data => setArticles(data))
      .catch(console.error);

    fetch('/api/videos')
      .then(res => res.json())
      .then(data => setVideos(data))
      .catch(console.error);
  }, []);

  const filteredArticles = articles.filter(article =>
    article.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    article.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );

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
          <p className="text-center text-gray-500 font-medium bg-white/40 backdrop-blur-[20px] saturate-200 py-8 rounded-2xl border border-white/30 w-full">
            No results found matching "{searchQuery}"
          </p>
        ) : (
          <div className="space-y-12">
            {filteredArticles.length > 0 && (
              <section>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Articles</h2>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
                  {filteredArticles.map((article, idx) => (
                    <Link key={idx} href={article.link} className="block group">
                      <div className="bg-white/60 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_4px_16px_rgba(0,0,0,0.04)] border border-white/50 group-hover:border-blue-300 group-hover:shadow-[0_8px_32px_rgba(0,0,0,0.08)] group-hover:-translate-y-1 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[140px]">
                        <h2 className="text-xl font-bold font-outfit text-blue-600 mb-3 group-hover:text-blue-700">{article.title}</h2>
                        <p className="text-gray-600 leading-relaxed flex-grow">{article.desc}</p>
                      </div>
                    </Link>
                  ))}
                </div>
              </section>
            )}

            {filteredVideos.length > 0 && (
              <section>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Video Tutorials</h2>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                  {filteredVideos.map((v) => (
                    <div key={v.id} className="aspect-[9/16] bg-gray-200 rounded-2xl flex items-center justify-center relative overflow-hidden group shadow-sm border border-white/30 cursor-pointer hover:shadow-md transition-all">
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
    </div>
  );
}
