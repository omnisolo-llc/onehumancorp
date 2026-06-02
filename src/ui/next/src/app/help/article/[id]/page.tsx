"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import DOMPurify from 'dompurify';

export default function ArticlePage({ params }: { params: { id: string } }) {
  const [article, setArticle] = useState<{ title: string; contentHtml: string } | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    fetch(`/api/help/${params.id}`)
      .then(res => {
        if (!res.ok) throw new Error("Not found");
        return res.json();
      })
      .then(data => {
        setArticle(data);
        setLoading(false);
      })
      .catch(() => {
        setError(true);
        setLoading(false);
      });
  }, [params.id]);

  if (loading) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 flex items-center justify-center font-inter">
        <p className="text-gray-500 text-lg">Loading...</p>
      </div>
    );
  }

  if (error || !article) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 flex flex-col items-center justify-center font-inter">
        <p className="text-gray-900 text-xl font-bold font-outfit mb-4">Article not found</p>
        <Link href="/help" className="text-blue-600 hover:underline">
          &larr; Back to Help Center
        </Link>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <Link href="/help" className="inline-flex items-center text-blue-600 hover:text-blue-700 font-medium mb-8 transition-colors">
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
          Back to Help Center
        </Link>

        <article className="bg-white/60 backdrop-blur-[20px] saturate-200 p-8 sm:p-10 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/50">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 mb-8 tracking-tight">{article.title}</h1>
          <div
            className="prose prose-blue max-w-none text-gray-700"
            dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(article.contentHtml) }}
          />
        </article>
      </div>
    </div>
  );
}
