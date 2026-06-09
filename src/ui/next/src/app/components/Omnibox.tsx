"use client";

import React, { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";

interface SearchResult {
  id: string;
  entity_type: string;
  title: string;
  subtitle: string;
  route: string;
}

export function Omnibox() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const router = useRouter();
  const inputRef = useRef<HTMLInputElement>(null);

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
    if (!query.trim()) {
      setResults([]);
      return;
    }

    const timer = setTimeout(async () => {
      setLoading(true);
      try {
        const response = await fetch(`/api/v1/search?q=${encodeURIComponent(query)}`);
        const data = await response.json();
        if (data.success && data.results) {
          setResults(data.results);
        } else {
          setResults([]);
        }
      } catch (err) {
        console.error("Search error", err);
      } finally {
        setLoading(false);
      }
    }, 300); // 300ms debounce

    return () => clearTimeout(timer);
  }, [query]);

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-[100] flex items-start justify-center pt-[10vh] px-4"
      onClick={() => setIsOpen(false)}
      style={{
        backgroundColor: "rgba(0, 0, 0, 0.2)",
        backdropFilter: "blur(4px)",
      }}
    >
      <div
        className="w-full max-w-2xl bg-white dark:bg-[#1c1c1e] rounded-xl shadow-2xl overflow-hidden"
        style={{
          border: "1px solid rgba(255,255,255,0.1)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center p-4 border-b border-gray-200 dark:border-gray-800">
          <svg
            className="w-5 h-5 text-gray-400 mr-3"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <input
            ref={inputRef}
            type="text"
            className="w-full bg-transparent border-none outline-none text-lg text-gray-900 dark:text-gray-100 placeholder-gray-500"
            placeholder="Search customers, orders, messages... (Cmd+K)"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          {loading && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-gray-900 dark:border-white ml-2"></div>
          )}
        </div>

        <div className="max-h-[60vh] overflow-y-auto">
          {results.length > 0 ? (
            <div className="py-2">
              {results.map((item, idx) => (
                <div
                  key={`${item.entity_type}-${item.id}-${idx}`}
                  className="px-4 py-3 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex flex-col"
                  onClick={() => {
                    setIsOpen(false);
                    router.push(item.route);
                  }}
                  data-testid="omnibox-result"
                >
                  <div className="flex items-center justify-between">
                    <span className="font-semibold text-gray-900 dark:text-gray-100">
                      {item.title}
                    </span>
                    <span className="text-xs px-2 py-1 bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded uppercase tracking-wider">
                      {item.entity_type}
                    </span>
                  </div>
                  <span className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                    {item.subtitle}
                  </span>
                </div>
              ))}
            </div>
          ) : query.trim() && !loading ? (
            <div className="p-8 text-center text-gray-500 dark:text-gray-400">
              No results found for "{query}".
            </div>
          ) : (
            <div className="p-8 text-center text-gray-500 dark:text-gray-400 flex flex-col items-center">
              <span className="mb-2">Type to search your workspace</span>
              <span className="text-xs opacity-70">Customers • Orders • Messages</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
