'use client';
import { useState, useEffect, useRef } from 'react';
import Link from 'next/link';

export function Omnibox() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(false);
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
  }, [isOpen]);

  useEffect(() => {
    if (!query) {
      setResults([]);
      return;
    }

    const timer = setTimeout(async () => {
      setIsLoading(true);
      try {
        const res = await fetch(`/api/v1/search?q=${encodeURIComponent(query)}`);
        if (res.ok) {
          const data = await res.json();
          setResults(data);
        }
      } catch (err) {
        console.error("Failed to search", err);
      } finally {
        setIsLoading(false);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [query]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-start justify-center pt-20 px-4 sm:px-0">
      <div className="fixed inset-0 bg-black/40 backdrop-blur-sm transition-opacity" onClick={() => setIsOpen(false)} />
      <div className="relative w-full max-w-2xl bg-white/90 dark:bg-gray-900/90 backdrop-blur-md rounded-2xl shadow-2xl border border-gray-200 dark:border-gray-800 overflow-hidden">
        <div className="flex items-center px-4 py-3 border-b border-gray-200 dark:border-gray-800">
          <svg className="w-6 h-6 text-gray-400 mr-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            ref={inputRef}
            type="text"
            className="w-full bg-transparent border-none text-xl focus:ring-0 text-gray-900 dark:text-gray-100 placeholder-gray-500 outline-none"
            placeholder="Search customers, orders, messages... (Cmd+K)"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          {isLoading && (
            <div className="w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin ml-2"></div>
          )}
        </div>

        <div className="max-h-[60vh] overflow-y-auto p-2">
          {query.toLowerCase().startsWith('create ') && (
            <div className="p-2 mb-2">
              <Link
                href="/triage"
                onClick={() => setIsOpen(false)}
                className="flex items-center p-3 rounded-xl bg-blue-50 dark:bg-blue-900/30 hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors"
              >
                <div className="w-10 h-10 rounded-full bg-blue-500 text-white flex items-center justify-center mr-4">
                  ✨
                </div>
                <div>
                  <div className="font-semibold text-blue-900 dark:text-blue-100">Ask AI Assistant</div>
                  <div className="text-sm text-blue-700 dark:text-blue-300">"{query}"</div>
                </div>
              </Link>
            </div>
          )}

          {results.length === 0 && query && !isLoading && !query.toLowerCase().startsWith('create ') && (
            <div className="p-8 text-center text-gray-500">
              No results found for "{query}"
            </div>
          )}

          {results.map((r, i) => (
            <Link
              key={`${r.id}-${i}`}
              href={r.url}
              onClick={() => setIsOpen(false)}
              className="flex items-center p-3 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-colors mb-1"
            >
              <div className="w-10 h-10 rounded-full bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 flex items-center justify-center mr-4 text-lg">
                {r.type === 'customer' && '👤'}
                {r.type === 'order' && '📦'}
                {r.type === 'message' && '💬'}
                {r.type === 'booking' && '📅'}
              </div>
              <div className="flex-1 min-w-0">
                <div className="font-medium text-gray-900 dark:text-gray-100 truncate">{r.title}</div>
                {r.subtitle && <div className="text-sm text-gray-500 truncate">{r.subtitle}</div>}
              </div>
              <div className="text-xs text-gray-400 capitalize bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">
                {r.type}
              </div>
            </Link>
          ))}
        </div>

        <div className="px-4 py-2 border-t border-gray-100 dark:border-gray-800 flex justify-between items-center text-xs text-gray-500">
          <div><kbd className="bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded border border-gray-200 dark:border-gray-700">↑↓</kbd> to navigate</div>
          <div><kbd className="bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded border border-gray-200 dark:border-gray-700">esc</kbd> to close</div>
        </div>
      </div>
    </div>
  );
}
