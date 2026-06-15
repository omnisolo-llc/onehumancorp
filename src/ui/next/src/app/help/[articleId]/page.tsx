"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import { useParams } from 'next/navigation';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

export default function HelpArticlePage() {
  const params = useParams();
  const articleId = params.articleId as string;
  const [article, setArticle] = useState<{title: string, contentHtml: string, contentMarkdown: string} | null>(null);
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

        <article className="app-card backdrop-blur-[20px] saturate-200 p-4 sm:p-8 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/50 bg-white/70">
            <h1 className="text-2xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-8 tracking-tight">{article.title}</h1>
            <div className="text-lg text-gray-700 leading-relaxed font-inter space-y-4">
               {article.contentMarkdown ? (
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={{
                      h1: ({node, ...props}) => <h1 className="text-2xl font-bold text-gray-900 mt-6 mb-4" {...props} />,
                      h2: ({node, ...props}) => <h2 className="text-xl font-bold text-gray-900 mt-6 mb-3" {...props} />,
                      h3: ({node, ...props}) => <h3 className="text-lg font-bold text-gray-900 mt-4 mb-2" {...props} />,
                      p: ({node, ...props}) => <p className="mb-4" {...props} />,
                      ul: ({node, ...props}) => <ul className="list-disc pl-6 mb-4" {...props} />,
                      ol: ({node, ...props}) => <ol className="list-decimal pl-6 mb-4" {...props} />,
                      li: ({node, ...props}) => <li className="mb-1" {...props} />,
                      a: ({node, ...props}) => <a className="text-blue-600 hover:underline" {...props} />,
                      blockquote: ({node, ...props}) => <blockquote className="border-l-4 border-gray-300 pl-4 italic my-4" {...props} />,
                      code: ({node, ...props}) => <code className="bg-gray-100 rounded px-1 py-0.5 text-sm font-mono text-gray-800" {...props} />,
                      pre: ({node, ...props}) => <pre className="bg-gray-100 rounded-lg p-4 overflow-x-auto mb-4" {...props} />,
                    }}
                  >
                    {article.contentMarkdown}
                  </ReactMarkdown>
               ) : (
                  <div dangerouslySetInnerHTML={{ __html: article.contentHtml }} />
               )}
            </div>
        </article>
      </div>
    </div>
  );
}
