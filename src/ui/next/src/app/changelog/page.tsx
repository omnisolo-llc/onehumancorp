"use client";

import React, { useEffect, useState } from "react";
// Next.js static rendering may fail with standard DOMPurify import in page components
// Better to dynamically import it or only run sanitize on client side, or rely on our static trusted input
import DOMPurify from 'dompurify';

export default function ChangelogPage() {
  const [changelogHtml, setChangelogHtml] = useState<string>("Loading...");
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    fetch("/api/changelog")
      .then(res => res.json())
      .then(data => setChangelogHtml(data.html))
      .catch(() => setChangelogHtml("<p>Could not load changelog.</p>"));
  }, []);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>What's New</h1>
         <div className="flex items-center gap-3">
             <a href="/dashboard" className="text-sm font-medium text-blue-600 hover:text-blue-800">Back to Dashboard</a>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-8">
        <div className="bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
          {mounted ? (
            <div className="prose" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(changelogHtml) }} />
          ) : (
             <div className="prose">Loading...</div>
          )}

          <div className="mt-12 pt-6 border-t border-gray-100 text-center">
            <a href="https://example.com/changelog" target="_blank" rel="noopener noreferrer" className="text-blue-600 hover:underline font-medium text-sm">
              Read the full technical changelog on our website
            </a>
          </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
