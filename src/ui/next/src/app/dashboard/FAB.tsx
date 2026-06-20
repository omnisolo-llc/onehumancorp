'use client';
import { useState } from 'react';


export function FloatingActionButton() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-6 right-6 z-50 flex flex-col items-end gap-3">
      {isOpen && (
        <div className="flex flex-col gap-2 mb-2 animate-in slide-in-from-bottom-5">
          <a href="/offering/new" className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📝 New Offering
          </a>
          <a href="/products/new" className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📦 New Product
          </a>
          <a href="/services/new" className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📅 New Service
          </a>
          <a href="/pos/terminal" className="px-4 py-2 bg-[#0066FF] text-white rounded-full shadow-lg font-bold border border-blue-400 hover:bg-blue-700 whitespace-nowrap">
            📱 Quick Charge (POS)
          </a>
        </div>
      )}

      <button
        onClick={() => setIsOpen(!isOpen)}
        className="glassmorphism w-14 h-14 min-w-[44px] min-h-[44px] bg-blue-600 hover:bg-blue-700 text-white rounded-full shadow-xl flex items-center justify-center text-3xl transition-transform hover:scale-105"
        style={{ transform: isOpen ? 'rotate(45deg)' : 'none' }}
      >
        +
      </button>
    </div>
  );
}
