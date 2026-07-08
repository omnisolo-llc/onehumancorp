'use client';
import { useState } from 'react';
import Link from 'next/link';

export function FloatingActionButton() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-6 right-6 z-50 flex flex-col items-end gap-3">
      {isOpen && (
        <div className="flex flex-col gap-2 mb-2 animate-in slide-in-from-bottom-5">
          <Link href="/offering/new" className="px-4 py-2 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📝 New Offering
          </Link>
          <Link href="/products/new" className="px-4 py-2 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📦 New Product
          </Link>
          <Link href="/services/new" className="px-4 py-2 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📅 New Service
          </Link>
          <Link href="/dashboard/receipt" className="px-4 py-2 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap" data-testid="snap-receipt-fab">
            📸 Snap Receipt
          </Link>
          <Link href="/pos/terminal" className="px-4 py-2 bg-[#0066FF] text-white rounded-full shadow-lg font-bold border border-blue-400 hover:bg-blue-700 whitespace-nowrap">
            📱 Quick Charge (POS)
          </Link>
        </div>
      )}

      <button
        onClick={() => setIsOpen(!isOpen)}
        className="rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm w-14 h-14 min-w-[44px] min-h-[44px] bg-[#0071E3] hover:bg-blue-700 text-white rounded-full shadow-xl flex items-center justify-center text-3xl transition-transform hover:scale-105"
        style={{ transform: isOpen ? 'rotate(45deg)' : 'none' }}
      >
        +
      </button>
    </div>
  );
}
