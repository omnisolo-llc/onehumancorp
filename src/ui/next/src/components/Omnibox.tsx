"use client";
import React, { useState, useEffect, useRef, useCallback } from 'react';
import { useRouter } from 'next/navigation';

export interface SearchResultItem {
  id: string;
  entity_type: string;
  title: string;
  subtitle: string;
  url: string;
  created_at_unix: number;
}

export const Omnibox: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResultItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsOpen((prev) => !prev);
      }
      if (e.key === 'Escape' && isOpen) {
        setIsOpen(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen]);

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 50);
      setQuery('');
      setResults([]);
    }
  }, [isOpen]);

  const fetchResults = useCallback(async (q: string) => {
    if (!q.trim()) {
      setResults([]);
      return;
    }
    setIsLoading(true);
    try {
      const res = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
      if (res.ok) {
        const data = await res.json();
        setResults(data.results || []);
      } else {
        setResults([]);
      }
    } catch (e) {
      console.error(e);
      setResults([]);
    } finally {
      setIsLoading(false);
      setSelectedIndex(0);
    }
  }, []);

  useEffect(() => {
    const delayDebounceFn = setTimeout(() => {
      fetchResults(query);
    }, 300);
    return () => clearTimeout(delayDebounceFn);
  }, [query, fetchResults]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev < results.length - 1 ? prev + 1 : prev));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (results[selectedIndex]) {
        router.push(results[selectedIndex].url);
        setIsOpen(false);
      }
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[1000] flex items-start justify-center pt-[10vh] px-4" onClick={() => setIsOpen(false)}>
      <div className="fixed inset-0 bg-black/40 backdrop-blur-sm" />
      <div
        className="relative w-full max-w-2xl bg-white dark:bg-[#1C1C1E]/80 backdrop-blur-xl border border-gray-200 dark:border-white/10 rounded-2xl shadow-2xl overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center px-4 border-b border-gray-200 dark:border-white/10">
          <svg className="w-5 h-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            ref={inputRef}
            className="w-full px-4 py-4 bg-transparent border-none text-gray-900 dark:text-white focus:outline-none placeholder-gray-400 text-lg"
            placeholder="Search customers, orders, messages, or type a command..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
          />
          <div className="px-2 py-1 text-xs text-gray-400 bg-gray-100 dark:bg-white/5 rounded border border-gray-200 dark:border-white/10">
            ESC
          </div>
        </div>

        <div className="max-h-[60vh] overflow-y-auto p-2">
          {isLoading ? (
            <div className="p-4 text-sm text-gray-500 text-center">Searching...</div>
          ) : results.length > 0 ? (
            <ul className="space-y-1">
              {results.map((result, idx) => (
                <li key={result.id}>
                  <button
                    className={`w-full text-left flex items-center p-3 rounded-xl transition-colors ${
                      idx === selectedIndex
                        ? 'bg-blue-500 text-white'
                        : 'hover:bg-gray-100 dark:hover:bg-white/5 text-gray-900 dark:text-white'
                    }`}
                    onClick={() => {
                      router.push(result.url);
                      setIsOpen(false);
                    }}
                    onMouseEnter={() => setSelectedIndex(idx)}
                  >
                    <div className={`flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center mr-3 ${
                      idx === selectedIndex ? 'bg-white/20 text-white' : 'bg-gray-100 dark:bg-white/10'
                    }`}>
                      {result.entity_type === 'customer' && '👤'}
                      {result.entity_type === 'order' && '📦'}
                      {result.entity_type === 'message' && '💬'}
                    </div>
                    <div>
                      <div className="font-medium">{result.title}</div>
                      <div className={`text-sm ${idx === selectedIndex ? 'text-blue-100' : 'text-gray-500'}`}>
                        {result.subtitle}
                      </div>
                    </div>
                  </button>
                </li>
              ))}
            </ul>
          ) : query.length > 0 ? (
            <div className="p-4 text-sm text-gray-500 text-center">No results found for "{query}"</div>
          ) : (
            <div className="p-4 text-sm text-gray-500 text-center">Type to start searching...</div>
          )}
        </div>
      </div>
    </div>
  );
};
