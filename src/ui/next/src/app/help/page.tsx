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
      <div className="max-w-4xl mx-auto">
        <h1 className="text-4xl font-bold font-outfit text-gray-900 mb-8 text-center drop-shadow-sm">Help Center</h1>

        <div className="mb-10 max-w-2xl mx-auto">
          <input
            type="text"
            placeholder="Search for help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 rounded-xl border border-gray-200/80 bg-white/70 backdrop-blur-md focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm text-gray-800 transition-all placeholder-gray-400"
          />
        </div>

        {filteredArticles.length === 0 ? (
          <p className="text-center text-gray-500 mt-12 bg-white/50 backdrop-blur-md py-8 rounded-xl border border-gray-100">No articles found matching "{searchQuery}"</p>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {filteredArticles.map((article, idx) => (
              <Link key={idx} href={article.link}>
                <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-6 rounded-xl shadow-sm border border-gray-100/50 hover:border-blue-300 hover:shadow-md hover:-translate-y-1 transition-all cursor-pointer h-full flex flex-col justify-between">
                  <div>
                     <h2 className="text-xl font-bold font-outfit text-blue-600 mb-2">{article.title}</h2>
                     <p className="text-gray-600 text-sm leading-relaxed">{article.desc}</p>
                  </div>
                  <div className="mt-4 flex items-center text-blue-500 text-sm font-semibold">
                     Read more <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
