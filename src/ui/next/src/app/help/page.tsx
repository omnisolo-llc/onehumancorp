"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{title: string, desc: string, link: string}[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setIsLoading(true);
    fetch('/api/help')
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to fetch help articles');
        }
        return res.json();
      })
      .then(data => {
        setArticles(data);
        setError(null);
      })
      .catch(err => {
        console.error(err);
        setError("We couldn't load the help articles right now. Please try again later.");
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  const filteredArticles = articles.filter(article =>
    article.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    article.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 text-center">Help Center</h1>

        <div className="mb-8">
          <input
            type="text"
            placeholder="Search for help articles..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full p-4 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm text-gray-800"
          />
        </div>

        {isLoading ? (
          <div className="flex justify-center items-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          </div>
        ) : error ? (
          <div className="bg-red-50 p-4 rounded-xl text-red-800 border border-red-100 text-center">
            {error}
          </div>
        ) : filteredArticles.length === 0 ? (
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
