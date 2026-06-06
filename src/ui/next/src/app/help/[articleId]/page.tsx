"use client";

import React, { useEffect, useState } from 'react';
import { useRouter, useParams } from 'next/navigation';

export default function HelpArticlePage() {
  const router = useRouter();
  const params = useParams();
  const articleId = params.articleId as string;

  const [article, setArticle] = useState<{ title: string, contentHtml: string } | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    if (!articleId) return;

    setLoading(true);
    fetch(`/api/help/${articleId}`)
      .then(res => {
        if (!res.ok) throw new Error('Article not found');
        return res.json();
      })
      .then(data => {
        setArticle(data);
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setError(true);
        setLoading(false);
      });
  }, [articleId]);

  if (loading) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] flex items-center justify-center font-inter">
        <p className="text-gray-500 font-medium">Loading article...</p>
      </div>
    );
  }

  if (error || !article) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 flex flex-col items-center justify-center font-inter">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-4">Oops!</h1>
        <p className="text-gray-600 mb-8">Article not found</p>
        <button
          onClick={() => router.push('/help')}
          className="inline-flex items-center px-6 py-3 bg-white hover:bg-gray-50 text-gray-900 font-bold rounded-xl border border-gray-200 shadow-sm transition-all"
        >
          Back to Help Center
        </button>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="bg-white/60 backdrop-blur-[20px] saturate-200 p-8 sm:p-12 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/50">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-6 tracking-tight">
            {article.title}
          </h1>

          <div
            className="prose prose-blue max-w-none text-gray-700"
            dangerouslySetInnerHTML={{ __html: article.contentHtml }}
          />

          <div className="pt-8 mt-10 border-t border-gray-200/50">
            <button
              onClick={() => router.push('/help')}
              className="inline-flex items-center px-6 py-3 bg-white hover:bg-gray-50 text-gray-900 font-bold rounded-xl border border-gray-200 shadow-sm transition-all active:scale-95"
            >
              <svg className="w-5 h-5 mr-2 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
              Back to Help Center
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
