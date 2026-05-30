"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function HelpCenterPage() {
  const [articles, setArticles] = useState<{title: string, desc: string, link: string}[]>([]);

  useEffect(() => {
    fetch('/api/help')
      .then(res => res.json())
      .then(data => setArticles(data))
      .catch(console.error);
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8 text-center">Help Center</h1>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {articles.map((article, idx) => (
            <Link key={idx} href={article.link}>
              <div className="bg-white/70 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-sm border border-white/50 hover:border-blue-300 hover:shadow-md hover:-translate-y-1 transition-all cursor-pointer h-full">
                <h2 className="text-xl font-bold font-outfit text-blue-600 mb-2">{article.title}</h2>
                <p className="text-gray-600">{article.desc}</p>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
