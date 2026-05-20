"use client";

import React, { useEffect, useState } from "react";
import DOMPurify from 'isomorphic-dompurify';
import { marked } from 'marked';

export default function ChangelogPage() {
  const [content, setContent] = useState<string>("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("/api/changelog")
      .then(res => res.json())
      .then(data => {
        const html = marked.parse(data.content);
        setContent(html as string);
        setLoading(false);
      })
      .catch(() => {
        setLoading(false);
      });
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <h1 className="text-4xl font-bold text-gray-900 mb-8 font-outfit tracking-tight">What's New</h1>

        {loading ? (
          <div className="flex justify-center p-12">
            <div className="inline-block w-8 h-8 rounded-full border-4 border-gray-200 border-t-blue-600 animate-spin"></div>
          </div>
        ) : (
          <div
            className="prose prose-blue max-w-none bg-white p-8 rounded-2xl shadow-sm border border-gray-100"
            dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(content) }}
          />
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
