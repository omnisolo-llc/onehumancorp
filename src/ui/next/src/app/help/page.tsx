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

  return (
    <div className="min-h-screen bg-gray-50/50 py-12 px-4 sm:px-6 lg:px-8 font-inter flex justify-center">
      <div className="w-full max-w-[375px] md:max-w-4xl mx-auto flex flex-col bg-[#F5F5F7] md:bg-transparent min-h-screen md:min-h-0 shadow-xl md:shadow-none relative">
        <header className="px-5 pt-10 pb-4 bg-white/70 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-200">
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 tracking-tight text-center">Help Center</h1>
        </header>

        <div className="p-5 flex-1">
          <div className="mb-8 sticky top-[80px] z-10 bg-[#F5F5F7]/90 md:bg-transparent backdrop-blur-md pt-2 pb-4">
            <input
              type="text"
              placeholder="Search for help articles..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full p-4 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm text-gray-800 bg-white/80 backdrop-blur-[20px] saturate-200"
            />
          </div>

          {filteredArticles.length === 0 ? (
            <p className="text-center text-gray-500">No articles found matching "{searchQuery}"</p>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {filteredArticles.map((article, idx) => (
                <Link key={idx} href={article.link}>
                  <div className="bg-white/80 backdrop-blur-[30px] saturate-[210%] p-6 rounded-2xl shadow-sm border border-white/60 hover:shadow-md transition-all active:scale-[0.98] cursor-pointer h-full flex flex-col">
                    <h2 className="font-outfit font-bold text-gray-900 text-lg mb-2 leading-tight">{article.title}</h2>
                    <p className="text-sm text-gray-500 mb-4 flex-grow">{article.desc}</p>
                    <div className="text-blue-600 font-semibold text-xs flex items-center gap-1 mt-auto">
                      Read more
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          )}

          <h2 className="text-2xl font-bold font-outfit text-gray-900 mt-12 mb-6">Video Tutorials</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pb-12">
            {videos.map(video => (
              <div key={video.id} className="bg-white/80 backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-white/60 flex items-center gap-4 cursor-pointer hover:shadow-md transition-all">
                 <div className="w-16 h-12 bg-gray-200 rounded-lg flex items-center justify-center relative overflow-hidden flex-shrink-0">
                    <svg className="w-6 h-6 text-gray-500 z-10 relative" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                 </div>
                 <div>
                   <h3 className="font-semibold text-gray-900 text-sm">{video.title}</h3>
                   <p className="text-xs text-gray-500">{video.duration}</p>
                 </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
