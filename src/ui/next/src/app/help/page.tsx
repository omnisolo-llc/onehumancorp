"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{title: string, desc: string, link: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [videos, setVideos] = useState<{id: number, title: string, duration: string}[]>([]);
  const [activeVideo, setActiveVideo] = useState<any>(null);

  useEffect(() => {
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => setVideos(data))
      .catch(console.error);

    fetch('/api/help')
      .then(res => res.json())
      .then(data => setArticles(data))
      .catch(console.error);
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-6 text-center">Help Center</h1>

        <div className="mb-8 relative">
          <input
            type="text"
            placeholder="Search help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 pl-12 bg-white rounded-xl shadow-sm border border-gray-200 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all text-gray-900"
          />
          <svg className="w-5 h-5 absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {articles.filter(a => a.title.toLowerCase().includes(searchQuery.toLowerCase()) || a.desc.toLowerCase().includes(searchQuery.toLowerCase())).map((article, idx) => (
            <Link key={idx} href={article.link}>
              <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer h-full">
                <h2 className="text-xl font-bold text-blue-600 mb-2">{article.title}</h2>
                <p className="text-gray-600">{article.desc}</p>
              </div>
            </Link>
          ))}
        </div>

        {/* Video Tutorials Section */}
        <div className="mt-16">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 text-center">Video Tutorials</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {videos.map((v) => (
              <div key={v.id} onClick={() => setActiveVideo(v)} className="aspect-[9/16] bg-gray-200 rounded-xl flex items-center justify-center relative overflow-hidden group cursor-pointer shadow-sm border border-gray-100">
                <div className="absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-all"></div>
                <div className="w-12 h-12 bg-white/90 backdrop-blur-sm rounded-full flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
                  <svg className="w-6 h-6 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                </div>
                <div className="absolute bottom-3 left-3 right-3 z-10">
                  <p className="text-white text-sm font-bold drop-shadow-md line-clamp-2 leading-tight">{v.title}</p>
                  <p className="text-white/80 text-xs font-medium mt-1">{v.duration}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Video Player Modal */}

        {/* Additional Resources */}
        <div className="mt-16 text-center border-t border-gray-200 pt-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6">Additional Resources</h2>
          <div className="flex justify-center gap-4">
             <Link href="/changelog">
               <span className="px-6 py-3 bg-white text-blue-600 font-bold rounded-xl shadow-sm border border-blue-100 hover:bg-blue-50 transition-colors inline-block cursor-pointer">
                 Release Notes & Changelog
               </span>
             </Link>
             <Link href="/api-docs">
               <span className="px-6 py-3 bg-white text-gray-600 font-bold rounded-xl shadow-sm border border-gray-200 hover:bg-gray-50 transition-colors inline-block cursor-pointer">
                 Advanced API Documentation
               </span>
             </Link>
          </div>
        </div>

        {activeVideo && (
          <div className="fixed inset-0 z-[100] flex items-center justify-center bg-gray-900/60 backdrop-blur-[20px] saturate-200 p-4">
            <div className="bg-black/90 backdrop-blur-md rounded-2xl shadow-2xl flex flex-col overflow-hidden border border-white/20 w-full max-w-sm aspect-[9/16] relative animate-pop-in">
              <div className="absolute top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/80 to-transparent z-10 flex justify-between items-start">
                <h3 className="text-white font-bold text-sm line-clamp-2 drop-shadow-md">{activeVideo.title}</h3>
                <button onClick={() => setActiveVideo(null)} className="text-white/80 hover:text-white bg-white/20 backdrop-blur-md border border-white/10 rounded-full p-1.5 transition-colors">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              <div className="flex-1 flex items-center justify-center relative bg-gradient-to-br from-gray-800 to-black">
                 <div className="absolute inset-0 bg-blue-500/10 blur-3xl rounded-full scale-150 mix-blend-screen pointer-events-none"></div>
                 <button className="w-16 h-16 bg-white/20 hover:bg-white/30 backdrop-blur-[20px] saturate-200 border border-white/30 rounded-full flex items-center justify-center shadow-[0_8px_32px_rgba(0,0,0,0.5)] transition-all active:scale-95 group z-20">
                    <svg className="w-8 h-8 text-white ml-1 group-hover:scale-110 transition-transform" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                 </button>
              </div>

              <div className="absolute bottom-0 left-0 right-0 p-4 bg-gradient-to-t from-black/80 to-transparent z-10 flex flex-col gap-3">
                <div className="flex items-center gap-3">
                  <button className="text-white/80 hover:text-white transition-colors">
                     <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
                  </button>
                  <div className="h-1.5 flex-1 bg-white/20 backdrop-blur-sm rounded-full overflow-hidden cursor-pointer relative group">
                    <div className="h-full bg-blue-500 w-1/3 relative shadow-[0_0_10px_rgba(59,130,246,0.8)]">
                      <div className="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-2 bg-white rounded-full scale-0 group-hover:scale-100 transition-transform shadow-md"></div>
                    </div>
                  </div>
                  <div className="text-white/80 text-[10px] font-medium font-inter tabular-nums">
                    0:00 / {activeVideo.duration}
                  </div>
                  <button className="text-white/80 hover:text-white transition-colors">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" /></svg>
                  </button>
                </div>
              </div>
            </div>

            <style dangerouslySetInnerHTML={{__html: `
              @keyframes pop-in {
                0% { opacity: 0; transform: scale(0.9); }
                100% { opacity: 1; transform: scale(1); }
              }
              .animate-pop-in { animation: pop-in 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards; }
            `}} />
          </div>
        )}
      </div>
    </div>
  );

}
