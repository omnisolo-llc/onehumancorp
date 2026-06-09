import React, { useState, useEffect, useRef } from 'react';
import { useRouter } from 'next/navigation';

type SearchResultItem = {
  entity_type: string;
  id: string;
  title: string;
  subtitle: string;
  url_path: string;
};

export const Omnibox = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResultItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  // Handle global keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Cmd+K or Ctrl+K
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsOpen((prev) => !prev);
      }
      // Pressing "/" focuses the omnibox if not open
      if (e.key === '/' && !isOpen && document.activeElement?.tagName !== 'INPUT' && document.activeElement?.tagName !== 'TEXTAREA') {
        e.preventDefault();
        setIsOpen(true);
      }
      // Escape closes it
      if (e.key === 'Escape' && isOpen) {
        setIsOpen(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen]);

  // Focus input when modal opens
  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 100);
      setQuery('');
      setResults([]);
      setSelectedIndex(0);
    }
  }, [isOpen]);

  // Debounced search
  useEffect(() => {
    if (!query.trim()) {
      setResults([]);
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    const delayDebounceFn = setTimeout(async () => {
      try {
        const res = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
        if (res.ok) {
          const data = await res.json();
          setResults(data.results || []);
          setSelectedIndex(0);
        }
      } catch (error) {
        console.error('Search error:', error);
      } finally {
        setIsLoading(false);
      }
    }, 300);

    return () => clearTimeout(delayDebounceFn);
  }, [query]);

  // Handle keyboard navigation within results
  const handleInputKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev < results.length - 1 ? prev + 1 : prev));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (results.length > 0 && results[selectedIndex]) {
        handleSelect(results[selectedIndex]);
      }
    }
  };

  const handleSelect = (item: SearchResultItem) => {
    setIsOpen(false);
    router.push(item.url_path);
  };

  const getIconForType = (type: string) => {
    switch (type) {
      case 'customer':
        return <span className="text-blue-500">👤</span>;
      case 'order':
        return <span className="text-green-500">📦</span>;
      case 'message':
        return <span className="text-purple-500">💬</span>;
      default:
        return <span className="text-gray-500">🔍</span>;
    }
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center pt-[10vh] sm:pt-[20vh]"
      data-testid="omnibox-overlay"
    >
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/40 backdrop-blur-sm"
        onClick={() => setIsOpen(false)}
      ></div>

      {/* Modal */}
      <div className="relative z-10 w-full max-w-2xl bg-white/90 dark:bg-gray-900/90 backdrop-blur-md rounded-2xl shadow-2xl border border-gray-200 dark:border-gray-800 overflow-hidden m-4 sm:m-0 flex flex-col max-h-[80vh]">
        {/* Input header */}
        <div className="flex items-center px-4 py-4 border-b border-gray-200 dark:border-gray-800">
          <span className="text-gray-400 w-5 h-5 mr-3">🔍</span>
          <input
            ref={inputRef}
            type="text"
            className="flex-1 bg-transparent border-none outline-none text-lg text-gray-900 dark:text-white placeholder-gray-500"
            placeholder="Search customers, orders, messages, or type a command..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleInputKeyDown}
            data-testid="omnibox-input"
          />
          <div className="flex items-center space-x-1 text-xs text-gray-400 border border-gray-200 dark:border-gray-700 rounded px-2 py-1 ml-3 hidden sm:flex">
            <span className="w-3 h-3">⌘</span>
            <span>K</span>
          </div>
        </div>

        {/* Results area */}
        <div className="overflow-y-auto overflow-x-hidden p-2 flex-1">
          {isLoading && query && (
            <div className="p-4 text-center text-sm text-gray-500">Searching...</div>
          )}

          {!isLoading && query && results.length === 0 && (
            <div className="p-4 text-center text-sm text-gray-500">
              No results found for "{query}"
            </div>
          )}

          {!isLoading && results.length > 0 && (
            <ul className="space-y-1 pb-2">
              {results.map((item, index) => (
                <li key={`${item.entity_type}-${item.id}`}>
                  <button
                    className={`w-full flex items-center justify-between px-4 py-3 rounded-xl transition-colors ${
                      index === selectedIndex
                        ? 'bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400'
                        : 'hover:bg-gray-100 dark:hover:bg-gray-800/50 text-gray-700 dark:text-gray-300'
                    }`}
                    onClick={() => handleSelect(item)}
                    onMouseEnter={() => setSelectedIndex(index)}
                    data-testid={`omnibox-result-${index}`}
                  >
                    <div className="flex items-center min-w-0">
                      <div className="flex-shrink-0 w-8 h-8 flex items-center justify-center rounded-lg bg-white dark:bg-gray-800 shadow-sm border border-gray-200 dark:border-gray-700 mr-3">
                        {getIconForType(item.entity_type)}
                      </div>
                      <div className="flex flex-col items-start truncate">
                        <span className="font-medium text-sm truncate">{item.title}</span>
                        {item.subtitle && (
                          <span className={`text-xs truncate ${index === selectedIndex ? 'text-blue-500 dark:text-blue-300' : 'text-gray-500'}`}>
                            {item.subtitle}
                          </span>
                        )}
                      </div>
                    </div>
                    <div className="flex-shrink-0 ml-3 text-xs uppercase tracking-wider font-medium opacity-50">
                      {item.entity_type}
                    </div>
                  </button>
                </li>
              ))}
            </ul>
          )}

          {/* Quick actions/suggestions when empty */}
          {!query && (
            <div className="p-4">
              <div className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3 px-2">Suggestions</div>
              <ul className="space-y-1">
                {[
                  { icon: <span>👤</span>, label: 'Find a customer', action: () => setQuery('customer') },
                  { icon: <span>📦</span>, label: 'Look up an order', action: () => setQuery('order') },
                  { icon: <span>💬</span>, label: 'Draft an invoice', action: () => setQuery('invoice') },
                ].map((suggestion, i) => (
                  <li key={i}>
                    <button
                      className="w-full flex items-center px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
                      onClick={suggestion.action}
                    >
                      <span className="mr-3 text-gray-400">{suggestion.icon}</span>
                      {suggestion.label}
                    </button>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
