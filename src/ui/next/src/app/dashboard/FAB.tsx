'use client';
import { useState } from 'react';
import Link from 'next/link';

export function FloatingActionButton() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-6 right-6 z-50 flex flex-col items-end gap-3">
      {isOpen && (
        <div className="flex flex-col gap-2 mb-2 animate-in slide-in-from-bottom-5">
          <Link href="/offering/new" className="px-4 py-2 glassmorphism text-gray-900 rounded-full shadow-lg font-semibold border border-gray-200 hover:bg-gray-50 whitespace-nowrap">
            📝 New Offering
          </Link>
          <Link href="/products/new" className="px-4 py-2 glassmorphism text-gray-900 rounded-full shadow-lg font-semibold border border-gray-200 hover:bg-gray-50 whitespace-nowrap">
            📦 New Product
          </Link>
          <Link href="/services/new" className="px-4 py-2 glassmorphism text-gray-900 rounded-full shadow-lg font-semibold border border-gray-200 hover:bg-gray-50 whitespace-nowrap">
            📅 New Service
          </Link>
        </div>
      )}

      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-14 h-14 bg-blue-600 hover:bg-blue-700 text-white rounded-full shadow-xl flex items-center justify-center text-3xl transition-transform hover:scale-105"
        style={{ transform: isOpen ? 'rotate(45deg)' : 'none' }}
      >
        +
      </button>
    </div>
  );
}
