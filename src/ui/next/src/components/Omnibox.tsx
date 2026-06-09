'use client';
import React, { useState, useEffect, useRef } from 'react';

type SearchResult = {
  id: string;
  entity_type: string;
  title: string;
  subtitle?: string;
};

export const Omnibox = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsOpen((prev) => !prev);
      }
      if (e.key === 'Escape') {
        setIsOpen(false);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    }
    if (!isOpen) {
      setQuery('');
      setResults([]);
    }
  }, [isOpen]);

  useEffect(() => {
    if (query.trim().length === 0) {
      setResults([]);
      return;
    }

    const timer = setTimeout(() => {
      setLoading(true);
      fetch(`/api/v1/search?q=${encodeURIComponent(query)}`)
        .then((res) => res.json())
        .then((data) => {
          setResults(data.results || []);
        })
        .catch(console.error)
        .finally(() => setLoading(false));
    }, 300);

    return () => clearTimeout(timer);
  }, [query]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-[10vh] bg-black/40 backdrop-blur-sm" onClick={() => setIsOpen(false)}>
      <div
        className="w-full max-w-2xl bg-white/80 backdrop-blur-md rounded-2xl shadow-2xl overflow-hidden border border-white/40 mx-4"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="p-4 border-b border-gray-200/50 flex items-center gap-3">
          <svg className="w-6 h-6 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            ref={inputRef}
            type="text"
            className="w-full bg-transparent border-none outline-none text-xl text-gray-800 placeholder-gray-400"
            placeholder="Search customers, orders, messages..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>

        {loading && <div className="p-4 text-sm text-gray-500 text-center">Searching...</div>}

        {!loading && results.length > 0 && (
          <ul className="max-h-[60vh] overflow-y-auto p-2">
            {results.map((result, i) => (
              <li
                key={`${result.entity_type}-${result.id}-${i}`}
                className="flex items-center gap-3 p-3 hover:bg-gray-100/50 rounded-xl cursor-pointer transition-colors"
                onClick={() => {
                  window.location.href = `/${result.entity_type}s/${result.id}`;
                }}
              >
                <div className="flex-1 min-w-0">
                  <div className="text-base font-medium text-gray-900 truncate">{result.title}</div>
                  {result.subtitle && <div className="text-sm text-gray-500 truncate">{result.subtitle}</div>}
                </div>
                <div className="px-2 py-1 text-xs font-medium text-gray-500 bg-gray-100 rounded-md capitalize">
                  {result.entity_type}
                </div>
              </li>
            ))}
          </ul>
        )}

        {!loading && query && results.length === 0 && (
          <div className="p-8 text-center text-gray-500">
            No results found for "{query}"
          </div>
        )}

        {!query && (
          <div className="p-4 bg-gray-50/50 border-t border-gray-100">
            <div className="flex justify-between items-center text-xs text-gray-500">
              <div className="flex gap-4">
                <span><kbd className="font-sans px-1.5 py-0.5 bg-gray-100 rounded border border-gray-200 shadow-sm">↑↓</kbd> to navigate</span>
                <span><kbd className="font-sans px-1.5 py-0.5 bg-gray-100 rounded border border-gray-200 shadow-sm">Enter</kbd> to select</span>
              </div>
              <div><kbd className="font-sans px-1.5 py-0.5 bg-gray-100 rounded border border-gray-200 shadow-sm">ESC</kbd> to close</div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
