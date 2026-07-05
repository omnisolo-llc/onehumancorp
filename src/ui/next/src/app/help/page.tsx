"use client";

import React, { useEffect, useState } from 'react';
import { motion } from "framer-motion";
import { WithTooltip } from '../../components/TooltipRegistry';
import Link from 'next/link';
import { VideoTutorialList } from '../../components/VideoTutorialList';

function ArticleSections({ articles, hoverBg }: { articles: { category: string, title: string, desc: string, link: string }[], hoverBg: string }) {
  return (
    <div className="space-y-10 sm:space-y-12 flex flex-col">
      {Array.from(new Set(articles.map(a => a.category || "General"))).map((category) => (
        <section key={category} className="flex flex-col">
          <div className="flex items-center mb-4 sm:mb-6">
            <h2 className="text-xl sm:text-2xl font-bold font-outfit text-gray-900">{category}</h2>
            <div className="ml-4 flex-grow border-t border-gray-200/50"></div>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 flex-col">
            {articles.filter(a => (a.category || "General") === category).map((article, idx) => (
              <Link key={idx} href={article.link} className="block group">
                <div className={`backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 p-5 sm:p-6 rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.08)] group-hover:border-blue-300 group-hover:shadow-[0_0_15px_rgba(59,130,246,0.5)] group-hover:-translate-y-1 transition-all duration-300 cursor-pointer h-full flex flex-col min-h-[120px] sm:min-h-[140px] ${hoverBg}`}>
                  <h3 className="text-lg sm:text-xl font-bold font-outfit text-blue-600 mb-2 sm:mb-3 group-hover:text-blue-700">{article.title}</h3>
                  <p className="text-sm sm:text-base text-gray-600 leading-relaxed flex-grow">{article.desc}</p>
                </div>
              </Link>
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{category: string, title: string, desc: string, link: string}[]>([]);
  const [videos, setVideos] = useState<{id: number, title: string, duration: string, video_url: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [debouncedSearchQuery, setDebouncedSearchQuery] = useState("");
  const [isAdvancedOpen, setIsAdvancedOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearchQuery(searchQuery);
    }, 300);
    return () => clearTimeout(handler);
  }, [searchQuery]);

  useEffect(() => {
    const url = debouncedSearchQuery.trim() ? `/api/help/search?q=${encodeURIComponent(debouncedSearchQuery.trim())}` : '/api/help';
    setIsLoading(true); setIsError(false); fetch(url)
      .then(res => res.json())
      .then(data => { setArticles(Array.isArray(data) ? data : []); setIsLoading(false); })
      .catch((err) => {
        console.error(err);
        setArticles([]); setIsError(true); setIsLoading(false);
      });
  }, [debouncedSearchQuery]);

  useEffect(() => {
    const isMobile = typeof window !== 'undefined' && window.innerWidth < 768;
    const fetchUrl = isMobile ? '/api/videos?mobile_optimized=true' : '/api/videos';
    fetch(fetchUrl)
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

        <div className="mb-8 sm:mb-10 w-full max-w-2xl mx-auto block">
          <div className="w-full relative block">
            <WithTooltip id="help-search-tooltip">
              <input
                data-testid="help-search-input"
                type="text"
                placeholder="Search for help articles and videos..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)} onInput={(e) => setSearchQuery(e.currentTarget.value)}
                className="w-full pl-12 pr-4 py-4 focus:outline-none focus:ring-2 focus:ring-blue-600 text-gray-900 backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] hover:bg-white/90 min-h-[50px] text-base placeholder:text-gray-500 transition-all rounded-[24px]"
              />
              <svg className="w-6 h-6 text-gray-400 absolute left-4 top-1/2 -translate-y-1/2 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </WithTooltip>
          </div>
        </div>

        {isLoading ? (<div className="flex justify-center py-12"><div className="animate-spin rounded-full h-8 w-8 border-b-2 border-[#0071E3]"></div></div>) : filteredArticles.length === 0 && filteredVideos.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-24 px-8 backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] rounded-3xl min-h-[400px] w-full max-w-2xl mx-auto transition-all">
            <svg className="w-20 h-20 max-w-[80px] max-h-[80px] text-gray-400 mb-6 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p className="text-center text-gray-700 dark:text-gray-300 font-medium text-xl md:text-2xl mb-2">
              {searchQuery ? (<>No results found matching <span className="text-gray-900 dark:text-gray-100 font-semibold">"{searchQuery}"</span></>) : (<>No help articles available right now.</>)}
            </p>
            <p className="text-center text-gray-500 dark:text-gray-400 text-base md:text-lg mb-8 max-w-md">
              Try adjusting your search terms or ask our AI assistant for help.
            </p>
            <WithTooltip id="ask-ai-tooltip" defaultText="Open AI Help Chat to get answers instantly.">
            <button
              className="px-8 py-4 bg-[#0071E3] hover:bg-[#0077ED] text-white font-semibold rounded-full shadow-lg backdrop-blur-md saturate-[210%] transition-all min-h-[44px] hover:-translate-y-1 active:scale-95 text-lg"
              onClick={() => {
                const event = new CustomEvent('open-help-chat');
                window.dispatchEvent(event);
              }}
            >
              Ask anything
            </button>
            </WithTooltip>
          </div>
        ) : (
          <div className="space-y-10 sm:space-y-12">
            {filteredArticles.length > 0 && (
              <ArticleSections articles={filteredArticles} hoverBg="hover:bg-white/90" />
            )}

            {filteredVideos.length > 0 && (
              <div className="pt-4">
                 <VideoTutorialList videos={filteredVideos} loading={false} />
              </div>
            )}

            {advancedArticles.length > 0 && (
              <div className="pt-8">
                <div className="border-t border-gray-200/50 pt-8 mt-4"><button onClick={() => setIsAdvancedOpen(!isAdvancedOpen)} className="flex items-center text-gray-500 hover:text-gray-900 transition-colors duration-200"><span className="text-lg font-bold font-outfit mr-2">Advanced</span><svg className={`w-5 h-5 transform transition-transform duration-200 ${isAdvancedOpen ? 'rotate-180' : ' '}`} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" /></svg></button>{isAdvancedOpen && (<motion.div className="pt-6" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.3 }}><ArticleSections articles={advancedArticles} hoverBg="hover:bg-white/80" /></motion.div>)}</div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
