"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";

interface HelpArticle {
  title: string;
  desc: string;
  link: string;
}

interface VideoTutorial {
  id: number;
  title: string;
  duration: string;
  video_url?: string;
}

function HelpCard({ title, description, href }: { title: string, description: string, href: string }) {
  return (
    <Link href={href} className="block group">
      <div className="backdrop-blur-[30px] saturate-[210%] bg-white/65 border border-white/40 shadow-[0_8px_32px_rgba(0,0,0,0.1)] rounded-2xl dark:bg-[#16161a]/70 dark:border-white/10 p-4 md:p-6 transition-transform hover:scale-[1.02] h-full flex flex-col min-h-[140px]">
        <h3 className="font-outfit text-lg font-bold text-[#1d1d1f] dark:text-white mb-2">{title}</h3>
        <p className="font-inter text-sm text-[#86868b] dark:text-gray-300 flex-grow">{description}</p>
      </div>
    </Link>
  );
}

function VideoCard({ video, onClick }: { video: VideoTutorial, onClick: () => void }) {
  return (
    <div onClick={onClick} className="backdrop-blur-[30px] saturate-[210%] bg-white/65 border border-white/40 shadow-[0_8px_32px_rgba(0,0,0,0.1)] rounded-2xl dark:bg-[#16161a]/70 dark:border-white/10 overflow-hidden flex flex-col cursor-pointer transition-transform hover:scale-[1.02]">
      <div className="w-full aspect-video bg-black/5 dark:bg-white/5 relative flex items-center justify-center group">
        <div className="absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-all"></div>
        <div className="w-10 h-10 rounded-full bg-white/90 backdrop-blur-sm flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
          <svg className="w-5 h-5 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
        </div>
      </div>
      <div className="p-4 md:p-6 flex-grow">
        <h3 className="font-outfit text-sm font-semibold text-[#1d1d1f] dark:text-white mb-1 line-clamp-2">{video.title}</h3>
        <span className="font-inter text-xs text-[#86868b] dark:text-gray-400">{video.duration}</span>
      </div>
    </div>
  );
}

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<HelpArticle[]>([]);
  const [videos, setVideos] = useState<VideoTutorial[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedVideo, setSelectedVideo] = useState<VideoTutorial | null>(null);

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

  const filteredVideos = videos.filter(video =>
    video.title.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#F5F5F7]/80 dark:bg-[#000000]/80 p-4 md:p-8 backdrop-blur-[20px] saturate-200 font-inter">
      <div className="max-w-4xl mx-auto space-y-8 md:space-y-12">
        <div className="text-center space-y-4">
          <h1 className="font-outfit text-3xl md:text-5xl font-extrabold text-[#1d1d1f] dark:text-white tracking-tight">Help Center</h1>
          <div className="relative max-w-2xl mx-auto">
             <input
               type="text"
               placeholder="Search for help articles and videos..."
               value={searchQuery}
               onChange={(e) => setSearchQuery(e.target.value)}
               className="w-full px-6 py-4 rounded-full border border-gray-200 dark:border-white/10 bg-white/80 dark:bg-black/50 backdrop-blur-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 font-inter text-base"
             />
             <svg className="absolute right-6 top-4 w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
          </div>
        </div>

        {articles.length === 0 && filteredVideos.length === 0 ? (
          <p className="text-center text-gray-500 font-medium backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 py-8 rounded-2xl border border-white/40 dark:border-white/10 w-full shadow-[0_8px_32px_rgba(0,0,0,0.1)]">
            No results found matching "{searchQuery}"
          </p>
        ) : (
          <div className="space-y-12">
            {articles.length > 0 && (
              <section>
                <div className="flex items-center mb-6">
                  <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">Articles</h2>
                  <div className="ml-4 flex-grow border-t border-gray-200/50 dark:border-white/10"></div>
                </div>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 md:gap-6">
                  {articles.map((article, idx) => (
                    <HelpCard key={idx} title={article.title} description={article.desc} href={article.link} />
                  ))}
                </div>
              </section>
            )}

            {filteredVideos.length > 0 && (
              <section>
                <div className="flex items-center mb-6">
                  <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">Video Tutorials</h2>
                  <div className="ml-4 flex-grow border-t border-gray-200/50 dark:border-white/10"></div>
                </div>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                  {filteredVideos.map((v) => (
                    <VideoCard key={v.id} video={v} onClick={() => setSelectedVideo(v)} />
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
