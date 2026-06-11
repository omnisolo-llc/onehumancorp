"use client";

import React, { useEffect, useState } from 'react';
import { WithTooltip } from '../../components/TooltipRegistry';
import Link from 'next/link';
import { VideoTutorialList } from '../../components/VideoTutorialList';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{category: string, title: string, desc: string, link: string}[]>([]);
  const [videos, setVideos] = useState<{id: number, title: string, duration: string, video_url: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    const url = searchQuery.trim() ? `/api/help/search?q=${encodeURIComponent(searchQuery.trim())}` : '/api/help';
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
            className="w-full p-4 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-[0_8px_32px_rgba(0,0,0,0.05)] text-gray-900 glassmorphism hover:bg-white/75 min-h-[44px] text-base placeholder:text-gray-500 transition-all"
          />
        </div>

        {filteredArticles.length === 0 && filteredVideos.length === 0 ? (
          <div className="flex flex-col items-center justify-center glassmorphism py-16 px-4 shadow-[0_4px_16px_rgba(0,0,0,0.02)]">
            <svg className="w-16 h-16 text-gray-400 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p className="text-center text-gray-600 font-medium text-lg">
              No results found matching <span className="text-gray-900 font-semibold">"{searchQuery}"</span>
            </p>
            <p className="text-center text-gray-500 mt-2 text-sm">
              Try adjusting your search terms or ask our AI assistant for help.
            </p>
            <WithTooltip id="ask-ai-tooltip" defaultText="Open AI Help Chat to get answers instantly.">
            <button
              className="mt-6 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-full shadow-md transition-all min-h-[44px]"
              onClick={() => {
                const event = new CustomEvent('open-help-chat');
                window.dispatchEvent(event);
              }}
            >
              Ask AI Support Agent
            </button>
            </WithTooltip>
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
                          <div className="backdrop-blur-xl bg-white/50 border border-white/20 p-6 shadow-[0_4px_16px_rgba(0,0,0,0.04)] group-hover:border-blue-300 group-hover:shadow-[0_0_15px_rgba(59,130,246,0.5)] group-hover:-translate-y-1 hover:bg-white/75 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[140px]">
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
              <VideoTutorialList videos={filteredVideos} loading={false} />
            )}

            <section className="mt-12 pt-8 border-t border-gray-200/50">
              <div className="bg-yellow-50/50 glassmorphism p-6 border-yellow-200/50 shadow-sm flex flex-col sm:flex-row items-center justify-between gap-4">
                <div>
                  <h3 className="text-lg font-bold font-outfit text-yellow-900">Advanced Users</h3>
                  <p className="text-yellow-800/80 text-sm mt-1">For users who want to use OHC's APIs directly (e.g., connect a custom checkout).</p>
                </div>
                <Link href="/api-docs" className="shrink-0 px-6 py-2.5 bg-yellow-100 hover:bg-yellow-200 text-yellow-900 font-semibold rounded-xl shadow-sm transition-all border border-yellow-300/50">
                  API Documentation
                </Link>
              </div>
            </section>
          </div>
        )}
      </div>
    </div>
  );
}
