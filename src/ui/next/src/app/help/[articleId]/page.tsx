"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import { useParams } from 'next/navigation';

export default function HelpArticlePage() {
  const params = useParams();
  const articleId = params.articleId as string;
  const [article, setArticle] = useState<{title: string, contentHtml: string} | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!articleId) return;

    fetch(`/api/help/${articleId}`)
      .then(async (res) => {
        if (!res.ok) {
           throw new Error("Article not found");
        }
        return res.json();
      })
      .then(data => setArticle(data))
      .catch(e => setError(e.message));
  }, [articleId]);

  if (error) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
        <div className="max-w-3xl mx-auto text-center">
            <h1 className="text-3xl font-bold text-gray-900 mb-4 font-outfit">Oops!</h1>
            <p className="text-gray-600 mb-8">{error}</p>
            <Link href="/help" className="text-blue-600 font-medium hover:underline">← Back to Help Center</Link>
        </div>
      </div>
    );
  }

  if (!article) {
     return (
       <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter flex items-center justify-center">
          <p className="text-gray-500 font-medium animate-pulse">Loading article...</p>
       </div>
     )
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <div className="mb-8">
            <Link href="/help" className="text-blue-600 font-medium hover:underline inline-flex items-center">
                <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
                Back to Help Center
            </Link>
        </div>

        <article className="bg-white/80 backdrop-blur-[20px] saturate-200 p-8 sm:p-12 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/60">
            <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-8 tracking-tight">{article.title}</h1>
            <div
                className="prose prose-lg prose-blue max-w-none prose-headings:font-outfit prose-headings:font-bold prose-headings:text-gray-800 prose-p:text-gray-700 prose-p:leading-relaxed"
                dangerouslySetInnerHTML={{ __html: article.contentHtml }}
            />
        </article>
      </div>
    </div>
  );
}
