"use client";

import React, { useEffect, useState } from 'react';
import { WithTooltip } from '../../components/TooltipRegistry';
import Link from 'next/link';
import { VideoTutorialList } from '../../components/VideoTutorialList';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{category: string, title: string, desc: string, link: string}[]>([]);
  const [videos, setVideos] = useState<{id: number, title: string, duration: string, video_url: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [debouncedSearchQuery, setDebouncedSearchQuery] = useState("");

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearchQuery(searchQuery);
    }, 300);
    return () => clearTimeout(handler);
  }, [searchQuery]);

  useEffect(() => {
    const url = debouncedSearchQuery.trim() ? `/api/help/search?q=${encodeURIComponent(debouncedSearchQuery.trim())}` : '/api/help';
    fetch(url)
      .then(res => res.json())
      .then(data => setArticles(Array.isArray(data) ? data : []))
      .catch((err) => {
        console.error(err);
        setArticles([]);
      });
  }, [debouncedSearchQuery]);

  useEffect(() => {
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => setVideos(Array.isArray(data) ? data : []))
      .catch((err) => {
        console.error(err);
        setVideos([]);
      });
  }, []);

  const filteredArticles = articles.filter(a => a.category !== "Advanced");
  const advancedArticles = articles.filter(a => a.category === "Advanced");

  const filteredVideos = videos.filter(video =>
    video.title.toLowerCase().includes(debouncedSearchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-6 sm:py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 data-testid="help-center-title" className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-6 sm:mb-8 text-center tracking-tight">In-App Help Center</h1>

        <div className="mb-8 sm:mb-10 w-full sm:w-3/4 mx-auto block">
          <div className="w-full block">
            <WithTooltip id="help-search-tooltip">
              <input
                data-testid="help-search-input"
                type="text"
                placeholder="Search for help articles and videos..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full p-4 focus:outline-none focus:ring-2 focus:ring-blue-600 text-gray-900 backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] hover:bg-white/90 min-h-[50px] text-base placeholder:text-gray-500 transition-all rounded-[24px]"
              />
            </WithTooltip>
          </div>
        </div>

        {filteredArticles.length === 0 && filteredVideos.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-16 px-4 backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] rounded-3xl min-h-[300px] w-full max-w-[400px] mx-auto transition-all">
            <svg className="w-16 h-16 max-w-[64px] max-h-[64px] text-gray-400 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
              className="mt-6 px-6 py-3 bg-blue-600/95 hover:bg-blue-700 text-white font-semibold rounded-full shadow-lg backdrop-blur-md saturate-[210%] transition-all min-h-[44px] hover:scale-105 active:scale-95"
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
          <div className="space-y-10 sm:space-y-12">
            {filteredArticles.length > 0 && (
              <div className="space-y-10 sm:space-y-12 flex flex-col">
                {Array.from(new Set(filteredArticles.map(a => a.category || "General"))).map((category) => (
                  <section key={category} className="flex flex-col">
                    <div className="flex items-center mb-4 sm:mb-6">
                      <h2 className="text-xl sm:text-2xl font-bold font-outfit text-gray-900">{category}</h2>
                      <div className="ml-4 flex-grow border-t border-gray-200/50"></div>
                    </div>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 flex-col">
                      {filteredArticles.filter(a => (a.category || "General") === category).map((article, idx) => (
                        <Link key={idx} href={article.link} className="block group">
                          <div className="backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 p-5 sm:p-6 rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.08)] group-hover:border-blue-300 group-hover:shadow-[0_0_15px_rgba(59,130,246,0.5)] group-hover:-translate-y-1 hover:bg-white/90 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[120px] sm:min-h-[140px]">
                            <h3 className="text-lg sm:text-xl font-bold font-outfit text-blue-600 mb-2 sm:mb-3 group-hover:text-blue-700">{article.title}</h3>
                            <p className="text-sm sm:text-base text-gray-600 leading-relaxed flex-grow">{article.desc}</p>
                          </div>
                        </Link>
                      ))}
                    </div>
                  </section>
                ))}
              </div>
            )}

            {filteredVideos.length > 0 && (
              <div className="pt-4">
                 <VideoTutorialList videos={filteredVideos} loading={false} />
              </div>
            )}

            {advancedArticles.length > 0 && (
              <div className="space-y-10 sm:space-y-12 flex flex-col pt-8">
                {Array.from(new Set(advancedArticles.map(a => a.category || "General"))).map((category) => (
                  <section key={category} className="flex flex-col">
                    <div className="flex items-center mb-4 sm:mb-6">
                      <h2 className="text-xl sm:text-2xl font-bold font-outfit text-gray-900">{category}</h2>
                      <div className="ml-4 flex-grow border-t border-gray-200/50"></div>
                    </div>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 flex-col">
                      {advancedArticles.filter(a => (a.category || "General") === category).map((article, idx) => (
                        <Link key={idx} href={article.link} className="block group">
                          <div className="backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-white/10 border border-white/40 p-5 sm:p-6 rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.08)] group-hover:border-blue-300 group-hover:shadow-[0_0_15px_rgba(59,130,246,0.5)] group-hover:-translate-y-1 hover:bg-white/80 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[120px] sm:min-h-[140px]">
                            <h3 className="text-lg sm:text-xl font-bold font-outfit text-blue-600 mb-2 sm:mb-3 group-hover:text-blue-700">{article.title}</h3>
                            <p className="text-sm sm:text-base text-gray-600 leading-relaxed flex-grow">{article.desc}</p>
                          </div>
                        </Link>
                      ))}
                    </div>
                  </section>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
