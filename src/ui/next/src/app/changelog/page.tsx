"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

type Section = {
  version: string;
  contentLines: string[];
};

export default function ChangelogPage() {
  const [sections, setSections] = useState<Section[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/changelog')
      .then((res) => res.json())
      .then((data) => {
        setSections(data);
        setLoading(false);
      })
      .catch((error) => {
        console.error("Failed to fetch changelog", error);
        setLoading(false);
      });
  }, []);

  return (
    <div className="min-h-screen bg-[#F5F5F7]/80 py-12 px-4 sm:px-6 lg:px-8 font-inter backdrop-blur-[20px] saturate-200">
      <div className="max-w-3xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 font-outfit text-center tracking-tight">Release Notes & Changelog</h1>

        {loading ? (
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mt-20"></div>
        ) : (
          <div className="space-y-8">
            {sections.map((section, idx) => (
              <div key={idx} className="bg-white/70 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/60 transition-all">
                <h2 className="text-xl font-bold text-blue-600 mb-4 font-outfit">{section.version}</h2>
                <div className="space-y-2">
                  {section.contentLines.map((line, lidx) => {
                    if (line.startsWith('### ')) {
                      return <h3 key={lidx} className="text-lg font-semibold text-gray-800 mt-4 mb-2">{line.replace('### ', '')}</h3>;
                    }
                    if (line.startsWith('- ')) {
                      return <li key={lidx} className="text-gray-600 ml-4 list-disc">{line.replace('- ', '')}</li>;
                    }
                    return <p key={lidx} className="text-gray-600">{line}</p>;
                  })}
                </div>
                {idx === 0 && (
                  <img src="/dashboard_with_charts.png" alt="Screenshot" className="rounded-xl mt-4 max-w-full shadow-lg border border-gray-200/50" />
                )}
              </div>
            ))}

            <div className="mt-8 text-center">
              <a href="https://onehumancorp.com/changelog" target="_blank" rel="noopener noreferrer" className="text-blue-600 font-bold hover:underline bg-blue-50/80 backdrop-blur-md px-6 py-3 rounded-full border border-blue-100 inline-block shadow-sm">
                Read the full technical changelog on our website →
              </a>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
