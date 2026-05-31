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
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8 text-center">Help Center</h1>

<<<<<<< HEAD
        <div className="mb-8">
          <input
            type="text"
            placeholder="Search for help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm text-gray-800"
          />
=======
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {articles.map((article, idx) => (
            <Link key={idx} href={article.link}>
              <div className="bg-white/70 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-sm border border-white/50 hover:border-blue-300 hover:shadow-md hover:-translate-y-1 transition-all cursor-pointer h-full">
                <h2 className="text-xl font-bold font-outfit text-blue-600 mb-2">{article.title}</h2>
                <p className="text-gray-600">{article.desc}</p>
              </div>
            </Link>
          ))}
>>>>>>> 80b76d0e (feat(docs): polish documentation pages and fix Next.js build issue)
        </div>

        {filteredArticles.length === 0 ? (
          <p className="text-center text-gray-500">No articles found matching "{searchQuery}"</p>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {filteredArticles.map((article, idx) => (
              <Link key={idx} href={article.link}>
                <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-6 rounded-xl shadow-sm border border-gray-100 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer h-full">
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
