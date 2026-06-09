'use client';
import { useState, useEffect, useRef } from 'react';
import { useRouter } from 'next/navigation';

export function Omnibox() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<{ id: string; entity_type: string; title: string; subtitle: string | null }[]>([]);
  const router = useRouter();
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
    if (query.trim().length > 0) {
      const delayDebounceFn = setTimeout(() => {
        fetch(`/api/v1/search?q=${encodeURIComponent(query)}`)
          .then((res) => res.json())
          .then((data) => setResults(data.results || []))
          .catch((err) => console.error("Search failed:", err));
      }, 300);

      return () => clearTimeout(delayDebounceFn);
    } else {
      setResults([]);
    }
  }, [query]);

  const handleResultClick = (result: any) => {
    setIsOpen(false);
    if (result.entity_type === 'customer') {
      router.push(`/customers/${result.id}`);
    } else if (result.entity_type === 'order') {
      router.push(`/orders/${result.id}`);
    } else if (result.entity_type === 'message') {
      router.push(`/inbox`);
    }
  };

  return (
    <>
      <div className="fixed top-0 left-0 right-0 z-40 md:hidden bg-white/80 backdrop-blur-md border-b border-gray-200 px-4 py-3 flex items-center justify-between">
        <div className="text-lg font-semibold text-gray-800 tracking-tight">OHC</div>
        <button
          onClick={() => setIsOpen(true)}
          className="text-gray-500 hover:text-gray-700 focus:outline-none"
          aria-label="Open search"
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
             <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </button>
      </div>

      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-start justify-center pt-16 bg-black/40 backdrop-blur-sm sm:pt-24" onClick={() => setIsOpen(false)}>
          <div
            className="w-full max-w-2xl bg-white/90 backdrop-blur-md rounded-xl shadow-2xl overflow-hidden border border-gray-200 mx-4 md:mx-auto"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="relative border-b border-gray-200">
              <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                 <svg className="h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fillRule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clipRule="evenodd" />
                 </svg>
              </div>
              <input
                ref={inputRef}
                type="text"
                className="w-full pl-11 pr-6 py-4 text-lg md:text-xl bg-transparent outline-none text-gray-800 placeholder-gray-400"
                placeholder="Search customers, orders, messages... (Cmd+K)"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
              />
            </div>

            {results.length > 0 && (
              <div className="max-h-[60vh] md:max-h-96 overflow-y-auto p-2">
                {results.map((result) => (
                  <div
                    key={`${result.entity_type}-${result.id}`}
                    className="px-4 py-3 cursor-pointer hover:bg-gray-100 rounded-lg flex flex-col transition-colors"
                    onClick={() => handleResultClick(result)}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium text-gray-800 truncate pr-4">{result.title}</span>
                      <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider bg-gray-200 px-2 py-1 rounded flex-shrink-0">{result.entity_type}</span>
                    </div>
                    {result.subtitle && (
                      <span className="text-sm text-gray-500 mt-1 truncate">{result.subtitle}</span>
                    )}
                  </div>
                ))}
              </div>
            )}

            {query.trim().length > 0 && results.length === 0 && (
              <div className="px-6 py-8 text-center text-gray-500">
                No results found for "{query}".
              </div>
            )}
          </div>
        </div>
      )}
    </>
  );
}
