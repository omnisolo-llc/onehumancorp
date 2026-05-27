import React from 'react';
import Link from 'next/link';

export default function HelpLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 sticky top-0 z-10 shadow-sm">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Link href="/" className="text-gray-500 hover:text-gray-900 transition-colors">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <h1 className="text-xl font-bold text-gray-900 font-outfit">One Human Corp <span className="text-blue-600">Help Center</span></h1>
          </div>
          <div>
            <Link href="/api-docs" className="text-sm font-medium text-gray-500 hover:text-gray-900 hidden sm:inline-block">Advanced API Docs</Link>
          </div>
        </div>
      </header>
      <main className="flex-1 w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12">
        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 sm:p-10">
          {children}
        </div>
      </main>
      <footer className="bg-white border-t border-gray-200 py-8 text-center text-sm text-gray-500">
        <p>One Human Corp - Empowering Small Businesses</p>
      </footer>
    </div>
  );
}
