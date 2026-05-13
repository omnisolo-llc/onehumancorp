'use client';

import React, { useState, useEffect } from 'react';
import { HelpChat } from './HelpChat';
import { VideoTutorial } from './VideoTutorial';

interface Article {
  id: string;
  category: string;
  title: string;
  content: string;
  readTime: string;
}

export function HelpCenterModal({ isOpen, onClose }: { isOpen: boolean; onClose: () => void }) {
  const [searchQuery, setSearchQuery] = useState('');
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<'articles' | 'videos' | 'chat'>('articles');

  useEffect(() => {
    if (isOpen) {
      fetchArticles();
    }
  }, [isOpen]);

  useEffect(() => {
    if (!searchQuery) {
      fetchArticles();
    } else {
      const delaySearch = setTimeout(() => {
        fetch(`/api/help/search?q=${encodeURIComponent(searchQuery)}`)
          .then(res => res.json())
          .then(data => setArticles(data));
      }, 300);
      return () => clearTimeout(delaySearch);
    }
  }, [searchQuery]);

  const fetchArticles = async () => {
    setLoading(true);
    const res = await fetch('/api/help/articles');
    const data = await res.json();
    setArticles(data);
    setLoading(false);
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm sm:p-6">
      <div className="relative w-full max-w-2xl max-h-full overflow-hidden bg-white/90 backdrop-blur-[20px] saturate-[200%] shadow-2xl rounded-2xl flex flex-col border border-white/40">

        {/* Header */}
        <div className="p-4 border-b border-slate-200/50 flex items-center justify-between bg-white/50">
          <h2 className="text-xl font-bold text-slate-800" style={{ fontFamily: 'Outfit, sans-serif' }}>
            How can we help you today?
          </h2>
          <button onClick={onClose} className="p-2 text-slate-400 hover:text-slate-600 rounded-full hover:bg-slate-100/50 transition-colors">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
          </button>
        </div>

        {/* Search */}
        <div className="p-4 bg-slate-50/50 border-b border-slate-200/50">
          <div className="relative">
            <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
            <input
              type="text"
              placeholder="Search for answers..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-10 pr-4 py-3 bg-white border border-slate-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 shadow-sm text-sm outline-none transition-shadow"
              style={{ fontFamily: 'Inter, sans-serif' }}
            />
          </div>
        </div>

        {/* Tabs */}
        <div className="flex border-b border-slate-200/50 px-4">
          {['articles', 'videos', 'chat'].map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab as any)}
              className={`px-4 py-3 text-sm font-medium capitalize border-b-2 transition-colors ${activeTab === tab ? 'border-blue-500 text-blue-600' : 'border-transparent text-slate-500 hover:text-slate-700'}`}
            >
              {tab}
            </button>
          ))}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-4 sm:p-6 min-h-[300px]">
          {activeTab === 'articles' && (
            <div className="space-y-4">
              {loading ? (
                <div className="flex justify-center py-8"><div className="w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div></div>
              ) : articles.length > 0 ? (
                articles.map(article => (
                  <article key={article.id} className="p-4 bg-white rounded-xl shadow-sm border border-slate-100 hover:shadow-md transition-shadow cursor-pointer group">
                    <div className="flex justify-between items-start mb-1">
                      <span className="text-xs font-semibold text-blue-600 uppercase tracking-wider">{article.category}</span>
                      <span className="text-xs text-slate-400 flex items-center"><svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>{article.readTime}</span>
                    </div>
                    <h3 className="text-lg font-semibold text-slate-800 mb-2 group-hover:text-blue-600 transition-colors" style={{ fontFamily: 'Outfit, sans-serif' }}>{article.title}</h3>
                    <p className="text-sm text-slate-600 line-clamp-2" style={{ fontFamily: 'Inter, sans-serif' }}>{article.content}</p>
                  </article>
                ))
              ) : (
                <div className="text-center py-8 text-slate-500">No articles found for "{searchQuery}".</div>
              )}
            </div>
          )}

          {activeTab === 'videos' && <VideoTutorial />}

          {activeTab === 'chat' && <div className="h-[400px] -m-4 sm:-m-6"><HelpChat inModal /></div>}
        </div>

        {/* Footer links */}
        <div className="p-4 border-t border-slate-200/50 bg-slate-50/80 flex justify-between items-center text-xs text-slate-500">
          <a href="/help/api-docs" className="hover:text-blue-600 transition-colors">Advanced: API Documentation</a>
          <a href="/help/release-notes" className="hover:text-blue-600 transition-colors">What's New (Changelog)</a>
        </div>
      </div>
    </div>
  );
}
