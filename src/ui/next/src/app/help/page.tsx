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
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-8 text-center tracking-tight">Help Center</h1>

        <div className="mb-10 w-full sm:w-3/4 mx-auto">
          <input
            type="text"
            placeholder="Search for help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 rounded-2xl border border-white/50 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-[0_8px_32px_rgba(0,0,0,0.05)] text-gray-900 backdrop-blur-[20px] saturate-200 bg-white/70 hover:bg-white/80 min-h-[44px] text-base placeholder:text-gray-500 transition-all"
          />
        </div>

        {filteredArticles.length === 0 ? (
          <p className="text-center text-gray-500 font-medium bg-white/40 backdrop-blur-[20px] saturate-200 py-8 rounded-2xl border border-white/30 w-full">
            No articles found matching "{searchQuery}"
          </p>
        ) : (
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
        )}
      </div>
    </div>
  );
}
