'use client';
import { useState } from 'react';
import Link from 'next/link';

export function FloatingActionButton() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-6 right-6 z-50 flex flex-col items-end gap-3">
      {isOpen && (
        <div className="flex flex-col gap-2 mb-2 animate-in slide-in-from-bottom-5">
          <Link href="/offering/new" className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📝 New Offering
          </Link>
          <Link href="/products/new" className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📦 New Product
          </Link>
          <Link href="/services/new" className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap">
            📅 New Service
          </Link>
          <label className="px-4 py-2 glassmorphism text-gray-900 dark:text-gray-100 rounded-full shadow-lg font-semibold whitespace-nowrap cursor-pointer">
            📸 Snap Receipt
            <input
              type="file"
              accept="image/*"
              className="hidden"
              onChange={async (e) => {
                const file = e.target.files?.[0];
                if (!file) return;

                const formData = new FormData();
                formData.append('receipt', file);

                try {
                  const res = await fetch('/api/ledger/receipt', {
                    method: 'POST',
                    body: formData
                  });
                  if (res.ok) {
                    alert('Receipt uploaded successfully. AI is processing it.');
                  } else {
                    alert('Failed to upload receipt');
                  }
                } catch (error) {
                  alert('Error uploading receipt');
                }
              }}
            />
          </label>
          <Link href="/pos/terminal" className="px-4 py-2 bg-[#0066FF] text-white rounded-full shadow-lg font-bold border border-blue-400 hover:bg-blue-700 whitespace-nowrap">
            📱 Quick Charge (POS)
          </Link>
        </div>
      )}

      <button
        onClick={() => setIsOpen(!isOpen)}
        className="glassmorphism w-14 h-14 min-w-[44px] min-h-[44px] bg-[#0071E3] hover:bg-blue-700 text-white rounded-full shadow-xl flex items-center justify-center text-3xl transition-transform hover:scale-105"
        style={{ transform: isOpen ? 'rotate(45deg)' : 'none' }}
      >
        +
      </button>
    </div>
  );
}
