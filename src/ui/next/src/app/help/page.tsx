"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState([]);
  const [search, setSearch] = useState("");

  useEffect(() => {
    fetch('/api/help')
      .then(res => res.json())
      .then(data => setArticles(data));
  }, []);

  const filteredArticles = articles.filter((a: any) =>
    a.title.toLowerCase().includes(search.toLowerCase()) ||
    a.desc.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-4xl font-bold text-gray-900 mb-8 text-center font-outfit">How can we help?</h1>

        <div className="mb-12">
          <div className="relative max-w-2xl mx-auto">
            <input
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search for answers..."
              className="w-full px-6 py-4 rounded-full border border-gray-200 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-lg"
            />
            <button className="absolute right-3 top-1/2 -translate-y-1/2 p-2 bg-blue-600 text-white rounded-full hover:bg-blue-700 transition-colors">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
            </button>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {filteredArticles.map((article: any, index: number) => (
            <Link key={index} href={article.link} className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 hover:shadow-md hover:border-blue-100 transition-all group">
              <h3 className="text-xl font-bold text-gray-900 mb-2 group-hover:text-blue-600 flex items-center justify-between">
                {article.title}
                <svg className="w-5 h-5 text-gray-400 group-hover:text-blue-500 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
              </h3>
              <p className="text-gray-600 text-sm leading-relaxed">{article.desc}</p>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
