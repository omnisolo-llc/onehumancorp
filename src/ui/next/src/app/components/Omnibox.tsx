"use client";

import React, { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";
import { QUEUE_IDENTITY_EPOCH_KEY } from "@/lib/sync/queueIdentity";

interface SearchResult {
  id: string;
  entity_type: string;
  title: string;
  subtitle: string;
  route: string;
}

function localSearchRoute(route: unknown): route is string {
  if (typeof route !== 'string' || !route.startsWith('/') || route.startsWith('//')
    || Array.from(route).some(character => character.charCodeAt(0) <= 32 || character.charCodeAt(0) === 127 || character === '\\')) return false;
  try { return new URL(route, window.location.origin).origin === window.location.origin; }
  catch { return false; }
}

export function Omnibox() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [ready, setReady] = useState(false);
  const [completed, setCompleted] = useState(false);
  const [error, setError] = useState("");
  const generation = useRef(0);
  const request = useRef<AbortController | null>(null);
  const router = useRouter();
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const clear = () => {
      generation.current += 1;
      request.current?.abort();
      setIsOpen(false); setQuery(""); setResults([]); setError(""); setCompleted(false); setLoading(false);
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
        event.preventDefault(); setIsOpen(previous => !previous);
      } else if (event.key === "Escape") clear();
    };
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) clear(); };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("omnisolo_auth_changed", clear);
    window.addEventListener("storage", storage);
    setReady(true);
    return () => {
      generation.current += 1; request.current?.abort();
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("omnisolo_auth_changed", clear);
      window.removeEventListener("storage", storage);
    };
  }, []);

  useEffect(() => {
    if (isOpen) inputRef.current?.focus();
    else { setQuery(""); setResults([]); }
  }, [isOpen]);

  useEffect(() => {
    const current = ++generation.current;
    const controller = new AbortController();
    request.current = controller;
    const active = () => generation.current === current && !controller.signal.aborted;
    setError(""); setCompleted(false); setResults([]);
    if (!isOpen || Array.from(query.trim()).length < 2) { setLoading(false); return () => controller.abort(); }
    setLoading(true);
    const timer = setTimeout(async () => {
      try {
        const response = await fetch(`/api/v1/search?q=${encodeURIComponent(query.trim())}`, {
          signal: controller.signal, credentials: 'same-origin', redirect: 'error', cache: 'no-store',
        });
        if (!response.ok) throw new Error('Search unavailable');
        const data = await response.json();
        if (data?.success !== true || !Array.isArray(data.results) || data.error != null
          || !data.results.every((item: SearchResult) => item && typeof item.id === 'string' && item.id
            && ['customer', 'order', 'message'].includes(item.entity_type) && typeof item.title === 'string'
            && typeof item.subtitle === 'string' && localSearchRoute(item.route))) throw new Error('Invalid search response');
        if (active()) { setResults(data.results); setCompleted(true); }
      } catch {
        if (active()) setError('Search is unavailable. Please try again.');
      } finally {
        if (active()) setLoading(false);
      }
    }, 300);
    return () => { clearTimeout(timer); controller.abort(); };
  }, [query, isOpen]);

  const readiness = <span hidden data-testid="omnibox-readiness" data-ready={String(ready)} />;
  if (!isOpen) return readiness;

  return (
    <>
    {readiness}
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
            onChange={(e) => setQuery(Array.from(e.target.value).slice(0, 200).join(''))}
          />
          {loading && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-gray-900 dark:border-white ml-2"></div>
          )}
        </div>

        <div className="max-h-[60vh] overflow-y-auto">
          {error ? <p role="alert" className="p-8 text-center">{error}</p> : loading ? <p role="status" className="p-8 text-center">Searching…</p> : results.length > 0 ? (
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
          ) : query.trim() && Array.from(query.trim()).length < 2 ? (
            <p className="p-8 text-center">Enter at least 2 characters to search.</p>
          ) : query.trim() && completed ? (
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
    </>
  );
}
