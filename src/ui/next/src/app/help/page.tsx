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

  const filteredArticles = articles.filter(a =>
    a.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    a.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-4xl font-extrabold text-[#1D1D1F] mb-4 font-outfit tracking-tight">How can we help?</h1>
          <p className="text-lg text-[#86868B] font-medium">Search our guides or browse by category</p>
        </div>

        <div className="relative mb-12 max-w-2xl mx-auto">
          <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
            <svg className="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
          <input
            type="text"
            placeholder="Search for articles..."
            className="block w-full pl-11 pr-4 py-4 bg-white/70 backdrop-blur-[20px] saturate-200 border border-white/50 rounded-2xl shadow-sm focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 outline-none transition-all text-lg font-medium"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {filteredArticles.map((article, idx) => (
            <Link key={idx} href={article.link}>
              <div className="bg-white/70 backdrop-blur-[20px] saturate-200 p-8 rounded-3xl shadow-sm border border-white/50 hover:border-blue-300 hover:shadow-xl transition-all cursor-pointer h-full group flex flex-col justify-between">
                <div>
                  <h2 className="text-2xl font-bold text-[#1D1D1F] mb-3 font-outfit group-hover:text-blue-600 transition-colors">{article.title}</h2>
                  <p className="text-[#86868B] leading-relaxed mb-6 font-medium">{article.desc}</p>
                </div>
                <div className="text-blue-600 font-bold flex items-center gap-2">
                  Read Guide
                  <svg className="w-4 h-4 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                  </svg>
                </div>
              </div>
            </Link>
          ))}
        </div>

        {filteredArticles.length === 0 && (
          <div className="text-center py-20 bg-white/40 backdrop-blur-md rounded-3xl border border-white/50">
            <div className="text-5xl mb-4">🔍</div>
            <h3 className="text-xl font-bold text-[#1D1D1F] mb-2 font-outfit">No articles found</h3>
            <p className="text-[#86868B] font-medium">Try different keywords or browse our categories.</p>
          </div>
        )}

        <div className="mt-20 text-center bg-blue-600/10 backdrop-blur-md p-10 rounded-[40px] border border-blue-100">
          <h2 className="text-2xl font-bold text-[#1D1D1F] mb-3 font-outfit">Still need help?</h2>
          <p className="text-[#86868B] mb-8 font-medium">Our AI Help Agent is available 24/7 to answer your questions.</p>
          <button
             onClick={() => window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' })}
             className="bg-blue-600 text-white px-8 py-4 rounded-2xl font-bold hover:bg-blue-700 transition-all shadow-lg hover:shadow-blue-200 active:scale-95"
          >
            Ask AI Assistant
          </button>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
