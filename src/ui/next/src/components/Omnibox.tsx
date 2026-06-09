"use client";

import React, { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";

interface SearchResultItem {
  id: string;
  entity_type: string;
  title: string;
  subtitle?: string;
  url: string;
}

export function Omnibox() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResultItem[]>([]);
  const [loading, setLoading] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        setIsOpen((prev) => !prev);
      } else if (e.key === "Escape" && isOpen) {
        setIsOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen]);

  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    } else if (!isOpen) {
      setQuery("");
      setResults([]);
    }
  }, [isOpen]);

  useEffect(() => {
    if (!query) {
      setResults([]);
      return;
    }

    const fetchResults = async () => {
      setLoading(true);
      try {
        const res = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
        if (res.ok) {
          const data = await res.json();
          setResults(data.results || []);
        }
      } catch (err) {
        console.error("Search failed", err);
      } finally {
        setLoading(false);
      }
    };

    const debounce = setTimeout(fetchResults, 300);
    return () => clearTimeout(debounce);
  }, [query]);

  if (!isOpen) {
    return (
      <div className="hidden sm:block">
        <button
          onClick={() => setIsOpen(true)}
          className="flex items-center text-sm text-gray-400 bg-white/10 hover:bg-white/20 px-3 py-1.5 rounded-md border border-white/10 transition-colors"
          data-testid="omnibox-trigger"
        >
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
          Search... <span className="ml-4 text-xs">⌘K</span>
        </button>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-16 sm:pt-24 px-4 pb-20 text-center sm:block sm:p-0">
      <div
        className="fixed inset-0 bg-black/40 backdrop-blur-sm transition-opacity"
        onClick={() => setIsOpen(false)}
        aria-hidden="true"
        data-testid="omnibox-backdrop"
      ></div>

      <div className="relative inline-block align-bottom bg-white dark:bg-gray-900 rounded-xl text-left overflow-hidden shadow-2xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg w-full border border-gray-200 dark:border-gray-800">
        <div className="flex items-center px-4 py-3 border-b border-gray-100 dark:border-gray-800">
          <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
          <input
            ref={inputRef}
            type="text"
            className="flex-1 ml-3 bg-transparent border-0 focus:ring-0 text-gray-900 dark:text-white placeholder-gray-500 sm:text-sm"
            placeholder="Search customers, orders, messages..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            data-testid="omnibox-input"
          />
          <button
            onClick={() => setIsOpen(false)}
            className="text-gray-400 hover:text-gray-500 text-xs font-medium uppercase tracking-wider"
          >
            Esc
          </button>
        </div>

        <div className="max-h-96 overflow-y-auto p-2" data-testid="omnibox-results">
          {loading ? (
            <div className="px-4 py-8 text-center text-sm text-gray-500">Searching...</div>
          ) : results.length > 0 ? (
            <ul className="space-y-1">
              {results.map((result) => (
                <li key={result.id}>
                  <button
                    onClick={() => {
                      setIsOpen(false);
                      router.push(result.url);
                    }}
                    className="w-full flex items-center px-4 py-3 text-left rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 focus:bg-gray-100 dark:focus:bg-gray-800 focus:outline-none transition-colors"
                  >
                    <div>
                      <div className="text-sm font-medium text-gray-900 dark:text-white flex items-center">
                        <span className="bg-blue-100 text-blue-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300 uppercase tracking-wide">
                          {result.entity_type}
                        </span>
                        {result.title}
                      </div>
                      {result.subtitle && (
                        <div className="text-xs text-gray-500 mt-1">{result.subtitle}</div>
                      )}
                    </div>
                  </button>
                </li>
              ))}
            </ul>
          ) : query ? (
            <div className="px-4 py-8 text-center text-sm text-gray-500">No results found for "{query}"</div>
          ) : (
            <div className="px-4 py-8 text-center text-sm text-gray-500 flex flex-col items-center">
              <svg className="w-8 h-8 text-gray-300 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
              Start typing to search across your workspace
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
