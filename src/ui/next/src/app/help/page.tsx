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
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 text-center font-outfit">Help Center</h1>

        <div className="mb-10 max-w-xl mx-auto relative">
          <input
            type="text"
            placeholder="Search for answers..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full px-5 py-4 pl-12 rounded-2xl border border-gray-200 bg-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 transition-shadow text-gray-800 placeholder-gray-400 font-inter text-lg"
          />
          <svg className="w-6 h-6 text-gray-400 absolute left-4 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
        </div>

        {filteredArticles.length === 0 ? (
          <div className="text-center text-gray-500 py-12">No results found for "{searchQuery}". Try a different search term.</div>
        ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {filteredArticles.map((article, idx) => (
            <Link key={idx} href={article.link}>
              <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer h-full">
                <h2 className="text-xl font-bold text-blue-600 mb-2">{article.title}</h2>
                <p className="text-gray-600">{article.desc}</p>
              </div>
            </Link>
          ))}
        </div>
        )}
      </div>
    </div>
  );
}
