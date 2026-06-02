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
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 text-center">Help Center</h1>

        <div className="mb-8">
          <input
            type="text"
            placeholder="Search for help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm text-gray-800"
          />
        </div>

        {filteredArticles.length === 0 ? (
          <p className="text-center text-gray-500">No articles found matching "{searchQuery}"</p>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {filteredArticles.map((article, idx) => (
              <Link key={idx} href={article.link}>
                <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-6 rounded-xl shadow-sm border border-gray-100 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer h-full">
                  <h2 className="text-xl font-bold text-blue-600 mb-2">{article.title}</h2>
                  <p className="text-gray-600">{article.desc}</p>
                </div>
              </Link>
            ))}
          </div>
        )}

        {videos.length > 0 && (
          <div className="mt-16 pt-8 border-t border-gray-200">
            <h2 className="text-2xl font-bold text-gray-900 mb-6">Video Tutorials</h2>
            <div className="flex overflow-x-auto pb-4 space-x-4 snap-x hide-scrollbar">
              {videos.map((video) => (
                <div key={video.id} className="min-w-[160px] sm:min-w-[200px] bg-white/80 backdrop-blur-[20px] saturate-200 rounded-xl shadow-sm border border-gray-100 overflow-hidden snap-center flex-shrink-0 cursor-pointer hover:border-blue-300 transition-all">
                  <div className="bg-gray-200 aspect-[9/16] relative flex items-center justify-center">
                    <svg className="w-12 h-12 text-blue-600/80 hover:text-blue-600 transition-colors" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z" /></svg>
                    <div className="absolute bottom-2 right-2 bg-black/70 text-white text-xs px-2 py-1 rounded font-medium">
                      {video.duration}
                    </div>
                  </div>
                  <div className="p-3">
                    <h3 className="font-bold text-gray-900 text-sm leading-tight line-clamp-3">{video.title}</h3>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="mt-16 pt-8 border-t border-gray-200">
          <h2 className="text-xl font-bold text-gray-900 mb-2">Advanced Users</h2>
          <p className="text-gray-600 mb-4">
            Are you a developer looking to integrate directly with our systems? Check out our API reference.
          </p>
          <Link href="/api-docs">
            <span className="text-blue-600 font-bold hover:underline">View API Documentation &rarr;</span>
          </Link>
        </div>
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar {
          display: none;
        }
        .hide-scrollbar {
          -ms-overflow-style: none;
          scrollbar-width: none;
        }
      `}} />
    </div>
  );
}
