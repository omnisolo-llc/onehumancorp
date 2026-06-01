"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{title: string, desc: string, link: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    fetch('/api/help')
      .then(res => res.json())
      .then(data => setArticles(data))
      .catch(console.error);
  }, []);

  const filteredArticles = articles.filter(article =>
    article.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    article.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
      <div className="w-full max-w-[375px] bg-[#F5F5F7] min-h-screen shadow-xl relative flex flex-col">
        {/* Header */}
        <header className="px-5 pt-10 pb-4 bg-white/70 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-200">
          <div className="flex justify-between items-center mb-4">
            <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors flex items-center justify-center min-w-[44px] min-h-[44px]">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <span className="text-xs font-bold text-blue-600 uppercase tracking-widest bg-blue-50 px-2.5 py-1 rounded-full">Help Center</span>
          </div>
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 tracking-tight">How can we help?</h1>
        </header>

        <main className="flex-1 p-5 overflow-y-auto pb-24 space-y-6">
          <div className="sticky top-0 z-10 pt-2 pb-4 bg-[#F5F5F7]">
            <input
              type="text"
              placeholder="Search for help articles..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full p-4 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm text-gray-800 text-sm"
            />
          </div>

          {filteredArticles.length === 0 ? (
            <p className="text-center text-gray-500 text-sm mt-8">No articles found matching "{searchQuery}"</p>
          ) : (
            <div className="space-y-4">
              {filteredArticles.map((article, idx) => (
                <Link key={idx} href={article.link} className="block">
                  <div className="bg-white/80 backdrop-blur-[30px] saturate-[210%] p-5 rounded-2xl shadow-sm border border-white/60 hover:shadow-md transition-all active:scale-[0.98] cursor-pointer h-full">
                    <h2 className="text-lg font-bold font-outfit text-gray-900 mb-1">{article.title}</h2>
                    <p className="text-sm text-gray-600">{article.desc}</p>
                  </div>
                </Link>
              ))}
            </div>
          )}
        </main>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
      `}} />
    </div>
  );
}
