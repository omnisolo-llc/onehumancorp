"use client";

import React, { useEffect, useState } from 'react';
import DOMPurify from 'dompurify';
import { useParams, useRouter } from 'next/navigation';

export default function HelpArticlePage() {
  const { articleId } = useParams();
  const router = useRouter();

  const [articleData, setArticleData] = useState<{ title: string, contentHtml: string } | null>(null);
  const [error, setError] = useState(false);

  useEffect(() => {
    if (articleId) {
      fetch(`/api/help/${articleId}`)
        .then(res => {
          if (!res.ok) {
            setError(true);
            throw new Error('Not found');
          }
          return res.json();
        })
        .then(data => setArticleData(data))
        .catch(err => {
          console.error(err);
          setError(true);
        });
    }
  }, [articleId]);

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50/50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
        <div className="max-w-3xl mx-auto">
          <button onClick={() => router.push('/help')} className="mb-6 text-blue-600 hover:text-blue-800 font-medium flex items-center gap-2">
            &larr; Back to Help Center
          </button>
          <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-8 rounded-xl shadow-sm border border-gray-100/50 transition-all">
            <h1 className="text-3xl font-extrabold font-outfit text-gray-900 mb-6">Article Not Found</h1>
            <div className="prose prose-blue max-w-none text-gray-700">
              <p>We couldn't find the article you're looking for.</p>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (!articleData) {
    return (
      <div className="min-h-screen bg-gray-50/50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
        <div className="max-w-3xl mx-auto text-center py-20">
          <p className="text-gray-500">Loading...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50/50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <button onClick={() => router.push('/help')} className="mb-6 text-blue-600 hover:text-blue-800 font-medium flex items-center gap-2">
          &larr; Back to Help Center
        </button>
        <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-8 rounded-xl shadow-sm border border-gray-100/50 transition-all">
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 mb-6">{articleData.title}</h1>
          <div className="prose prose-blue max-w-none text-gray-700" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(articleData.contentHtml) }} />
        </div>
      </div>
    </div>
  );
}
