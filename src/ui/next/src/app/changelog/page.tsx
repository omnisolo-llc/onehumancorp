"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

type ChangelogSection = {
  version: string;
  date?: string;
  contentLines: string[];
  imageUrl?: string;
};

export default function ChangelogPage() {
  const [sections, setSections] = useState<ChangelogSection[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/changelog')
      .then(res => res.json())
      .then(data => {
        setSections(data);
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 text-center font-inter">
        <p className="text-gray-500">Loading updates...</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <h1 className="text-3xl sm:text-4xl font-extrabold text-[#1D1D1F] mb-8 font-outfit text-center tracking-tight">Release Notes & Changelog</h1>
        <div className="space-y-8">
          {sections.map((section, idx) => (
            <div key={idx} className="bg-white/60 backdrop-blur-[20px] saturate-200 p-6 sm:p-8 rounded-3xl shadow-xl border border-white/40 transition-all">
              <div className="flex justify-between items-center mb-6">
                <h2 className="text-xl sm:text-2xl font-bold text-blue-600 font-outfit">{section.version}</h2>
                {section.date && <span className="text-sm text-gray-500 font-medium">{section.date}</span>}
              </div>
              <div className="space-y-3">
                {section.contentLines.map((line, lidx) => {
                  if (line.startsWith('### ')) {
                    return <h3 key={lidx} className="text-lg font-bold text-[#1D1D1F] mt-6 mb-2 font-outfit">{line.replace('### ', '')}</h3>;
                  }
                  if (line.startsWith('- ')) {
                    return (
                      <div key={lidx} className="flex gap-3 text-gray-700 leading-relaxed">
                        <span className="text-blue-500 font-bold">•</span>
                        <span>{line.replace('- ', '')}</span>
                      </div>
                    );
                  }
                  return <p key={lidx} className="text-gray-700 leading-relaxed">{line}</p>;
                })}
              </div>
              {section.imageUrl && (
                <div className="mt-8 rounded-2xl overflow-hidden border border-white/40 shadow-lg">
                  <img src={section.imageUrl} alt={section.version} className="w-full h-auto" />
                </div>
              )}
            </div>
          ))}

          <div className="mt-12 text-center">
            <a
              href="https://onehumancorp.com/changelog"
              target="_blank"
              rel="noopener noreferrer"
              className="text-blue-600 font-bold hover:text-blue-800 transition-colors bg-blue-50/80 backdrop-blur-md px-8 py-4 rounded-2xl border border-blue-100 inline-flex items-center gap-2 shadow-sm min-h-[44px]"
            >
              Read the full technical changelog
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7-7 7" /></svg>
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
