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
    <div className="min-h-screen bg-gray-50/50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto bg-white/40 backdrop-blur-[20px] saturate-200 p-8 rounded-3xl shadow-xl border border-white/50">
        <h1 className="text-4xl font-extrabold text-gray-900 mb-10 text-center font-outfit tracking-tight">Help Center</h1>

        <div className="mb-10 relative">
          <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
            <svg className="h-6 w-6 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
          <input
            type="text"
            placeholder="Search for help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-12 p-4 rounded-2xl border border-white/60 bg-white/60 backdrop-blur-md focus:outline-none focus:ring-4 focus:ring-blue-500/30 shadow-inner text-gray-800 transition-all font-medium text-lg placeholder-gray-500"
          />
        </div>

        {filteredArticles.length === 0 ? (
          <div className="text-center py-12 bg-white/50 rounded-2xl border border-white/50">
            <p className="text-gray-500 text-lg">No articles found matching <span className="font-semibold text-gray-700">"{searchQuery}"</span></p>
            <button
              onClick={() => setSearchQuery("")}
              className="mt-4 text-blue-600 font-medium hover:text-blue-800 transition-colors"
            >
              Clear search
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {filteredArticles.map((article, idx) => (
              <Link key={idx} href={article.link} className="group block h-full">
                <div className="bg-white/70 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-sm border border-white/60 group-hover:border-blue-300/80 group-hover:shadow-lg group-hover:-translate-y-1 transition-all duration-300 h-full flex flex-col">
                  <h2 className="text-xl font-bold text-blue-700 mb-3 font-outfit group-hover:text-blue-800 flex items-center justify-between">
                    {article.title}
                    <svg className="w-5 h-5 opacity-0 group-hover:opacity-100 transform -translate-x-2 group-hover:translate-x-0 transition-all text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                    </svg>
                  </h2>
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
