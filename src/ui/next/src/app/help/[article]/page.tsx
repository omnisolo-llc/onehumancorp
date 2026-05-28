"use client";

import React from 'react';
import { useParams } from 'next/navigation';

export default function HelpArticlePage() {
  const { article } = useParams();

  const articles: Record<string, { title: string, content: React.ReactNode }> = {
    "getting-started": {
      title: "Getting Started",
      content: (
        <>
          <p>Welcome to OneHumanCorp! Let's get your store set up.</p>
          <h2 className="text-xl font-bold mt-4">1. Describe Your Business</h2>
          <p>Go to the Builder page and type a short description of what you sell. Our AI will do the rest!</p>
          <h2 className="text-xl font-bold mt-4">2. Launch Your Store</h2>
          <p>Click the "Launch Store" button to make it live for the world to see.</p>
        </>
      )
    },
    // add more articles...
  };

  const articleData = article && typeof article === 'string' && articles[article] ? articles[article] : { title: "Article Not Found", content: <p>We couldn't find the article you're looking for.</p> };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white p-8 rounded-xl shadow-sm border border-gray-100">
        <h1 className="text-3xl font-bold text-gray-900 mb-6">{articleData.title}</h1>
        <div className="prose prose-blue max-w-none text-gray-700">
          {articleData.content}
        </div>
      </div>
    </div>
  );
}
