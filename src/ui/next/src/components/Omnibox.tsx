"use client";

import React, { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";
import "./Omnibox.css";

type SearchResultItem = {
  id: string;
  name?: string;
  email?: string;
  status?: string;
  total_amount?: number;
  content?: string;
};

type SearchResponse = {
  customers: SearchResultItem[];
  orders: SearchResultItem[];
  messages: SearchResultItem[];
};

export const Omnibox = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResponse>({ customers: [], orders: [], messages: [] });
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();
  const inputRef = useRef<HTMLInputElement>(null);

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
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    } else if (!isOpen) {
      setQuery("");
      setResults({ customers: [], orders: [], messages: [] });
    }
  }, [isOpen]);

  useEffect(() => {
    if (!query) {
      setResults({ customers: [], orders: [], messages: [] });
      return;
    }

    const timer = setTimeout(async () => {
      setIsLoading(true);
      try {
        const res = await fetch(`/api/v1/omnibox/search?q=${encodeURIComponent(query)}`);
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
    <div className="omnibox-overlay" onClick={() => setIsOpen(false)}>
      <div className="omnibox-container" onClick={(e) => e.stopPropagation()}>
        <input
          ref={inputRef}
          type="text"
          className="omnibox-input"
          placeholder="Search customers, orders, messages... (Cmd+K)"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        {isLoading && <div className="omnibox-loading">Searching...</div>}
        <div className="omnibox-results">
          {results.customers.length > 0 && (
            <div className="omnibox-section">
              <h3>Customers</h3>
              {results.customers.map((c) => (
                <div
                  key={c.id}
                  className="omnibox-item"
                  onClick={() => {
                    setIsOpen(false);
                    router.push(`/dashboard/customers/${c.id}`);
                  }}
                >
                  <span className="omnibox-item-title">{c.name}</span>
                  {c.email && <span className="omnibox-item-subtitle">{c.email}</span>}
                </div>
              ))}
            </div>
          )}
          {results.orders.length > 0 && (
            <div className="omnibox-section">
              <h3>Orders</h3>
              {results.orders.map((o) => (
                <div
                  key={o.id}
                  className="omnibox-item"
                  onClick={() => {
                    setIsOpen(false);
                    router.push(`/dashboard/orders/${o.id}`);
                  }}
                >
                  <span className="omnibox-item-title">Order #{o.id.split('-')[0]}</span>
                  <span className="omnibox-item-subtitle">
                    {o.status} - {o.total_amount ? `$${o.total_amount}` : ""}
                  </span>
                </div>
              ))}
            </div>
          )}
          {results.messages.length > 0 && (
            <div className="omnibox-section">
              <h3>Messages</h3>
              {results.messages.map((m) => (
                <div
                  key={m.id}
                  className="omnibox-item"
                  onClick={() => {
                    setIsOpen(false);
                    router.push(`/dashboard/inbox/${m.id}`);
                  }}
                >
                  <span className="omnibox-item-title">{m.content?.substring(0, 50)}...</span>
                </div>
              ))}
            </div>
          )}
          {!isLoading && query && results.customers.length === 0 && results.orders.length === 0 && results.messages.length === 0 && (
            <div className="omnibox-empty">No results found for "{query}"</div>
          )}
        </div>
      </div>
    </div>
  );
};
