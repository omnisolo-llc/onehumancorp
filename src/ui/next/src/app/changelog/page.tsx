"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

type ChangelogSection = {
  version: string;
  contentLines: string[];
  screenshot_url?: string;
};

export default function ChangelogPage() {
  const [sections, setSections] = useState<ChangelogSection[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("/api/changelog")
      .then((res) => res.json())
      .then((data) => {
        setSections(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Failed to load changelog", err);
        setLoading(false);
      });
  }, []);

  return (
    <div className="min-h-screen bg-[#F5F5F7]/80 py-12 px-4 sm:px-6 lg:px-8 font-inter backdrop-blur-[20px] saturate-200">
      <div className="max-w-3xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 font-outfit text-center tracking-tight">
          Release Notes & Changelog
        </h1>
        <div className="space-y-8">
          {loading ? (
            <div className="flex justify-center py-12">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
            </div>
          ) : sections.length === 0 ? (
            <p className="text-center text-gray-500 font-medium py-8 glassmorphism/70 backdrop-blur-[20px] rounded-2xl border border-white/60">
              No changelog available.
            </p>
          ) : (
            sections.map((section, idx) => (
              <div
                key={idx}
                className="glassmorphism/70 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/60 transition-all"
              >
                <h2 className="text-xl font-bold text-blue-600 mb-4 font-outfit">
                  {section.version}
                </h2>
                <div className="space-y-2">
                  {section.contentLines.map((line, lidx) => {
                    if (line.startsWith("### ")) {
                      return (
                        <h3
                          key={lidx}
                          className="text-lg font-semibold text-gray-800 mt-4 mb-2"
                        >
                          {line.replace("### ", "")}
                        </h3>
                      );
                    }
                    if (line.startsWith("- ")) {
                      return (
                        <li key={lidx} className="text-gray-600 ml-4 list-disc">
                          {line.replace("- ", "")}
                        </li>
                      );
                    }
                    return (
                      <p key={lidx} className="text-gray-600">
                        {line}
                      </p>
                    );
                  })}
                </div>
                {section.screenshot_url && (
                  <img
                    src={section.screenshot_url}
                    alt={`${section.version} Screenshot`}
                    className="rounded-xl mt-4 max-w-full shadow-lg border border-gray-200/50"
                  />
                )}
              </div>
            ))
          )}

          <div className="mt-8 text-center">
            <a
              href="https://onehumancorp.com/changelog"
              target="_blank"
              rel="noopener noreferrer"
              className="text-blue-600 font-bold hover:underline bg-blue-50/80 backdrop-blur-md px-6 py-3 rounded-full border border-blue-100 inline-block shadow-sm"
            >
              Read the full technical changelog on our website →
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
