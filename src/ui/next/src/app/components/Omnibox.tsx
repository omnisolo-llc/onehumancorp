"use client";

import React, { useState, useEffect, useRef } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";

interface SearchResultItem {
  id: string;
  entity_type: string;
  title: string;
  subtitle: string;
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
      }
      if (e.key === "Escape") {
        setIsOpen(false);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 50);
    } else {
      setQuery("");
      setResults([]);
    }
  }, [isOpen]);

  useEffect(() => {
    const fetchResults = async () => {
      if (!query.trim()) {
        setResults([]);
        return;
      }
      setLoading(true);
      try {
        const res = await fetch(`/api/v1/search/global?q=${encodeURIComponent(query)}&tenant_id=default`);
        if (res.ok) {
          const data = await res.json();
          setResults(data.results || []);
        }
      } catch (e) {
        console.error("Search error", e);
      } finally {
        setLoading(false);
      }
    };

    const debounceTimer = setTimeout(fetchResults, 300);
    return () => clearTimeout(debounceTimer);
  }, [query]);

  if (!isOpen) return null;

  const getHref = (item: SearchResultItem) => {
    switch (item.entity_type) {
      case "customer":
        return `/customers/${item.id}`;
      case "order":
        return `/orders/${item.id}`;
      case "message":
        return `/inbox/${item.id}`;
      default:
        return "#";
    }
  };

  return (
    <div className="fixed inset-0 z-[100] flex items-start justify-center pt-[10vh] px-4">
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm"
        onClick={() => setIsOpen(false)}
      />
      <div className="relative w-full max-w-2xl bg-white dark:bg-[#1C1C1E]/90 dark:backdrop-blur-xl rounded-[16px] shadow-2xl border border-black/10 dark:border-white/10 overflow-hidden flex flex-col max-h-[80vh]">
        <div className="flex items-center p-4 border-b border-black/10 dark:border-white/10">
          <svg className="w-5 h-5 text-gray-400 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            ref={inputRef}
            type="text"
            className="flex-1 bg-transparent border-none outline-none text-lg text-gray-900 dark:text-white placeholder-gray-500"
            placeholder="Search customers, orders, or messages..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <div className="text-xs text-gray-400 border border-gray-200 dark:border-white/20 rounded px-2 py-1 ml-3">ESC</div>
        </div>

        <div className="overflow-y-auto p-2 flex-1">
          {loading && (
            <div className="p-4 text-center text-sm text-gray-500">Searching...</div>
          )}
          {!loading && query && results.length === 0 && (
            <div className="p-4 text-center text-sm text-gray-500">No results found for "{query}"</div>
          )}
          {!loading && results.map((item) => (
            <Link
              key={`${item.entity_type}-${item.id}`}
              href={getHref(item)}
              onClick={() => setIsOpen(false)}
              className="flex items-center p-3 rounded-lg hover:bg-gray-100 dark:hover:bg-white/5 transition-colors group"
            >
              <div className="flex-1 min-w-0">
                <div className="text-sm font-medium text-gray-900 dark:text-white truncate">
                  {item.title}
                </div>
                <div className="text-xs text-gray-500 dark:text-gray-400 truncate">
                  {item.subtitle} &middot; {item.entity_type}
                </div>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
