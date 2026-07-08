"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";
import { motion } from "framer-motion";

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
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-black py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <h1 data-testid="changelog-title" className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 dark:text-gray-100 mb-8 text-center tracking-tight">
          Release Notes & Changelog
        </h1>
        <div className="space-y-8">
          {loading ? (
            <div className="flex justify-center py-12">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-[#0071E3]"></div>
            </div>
          ) : sections.length === 0 ? (
            <p className="text-center text-gray-500 font-medium py-8 backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] rounded-3xl">
              No changelog available.
            </p>
          ) : (
            sections.map((section, idx) => (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.1, duration: 0.4 }}
                className="backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] p-6 sm:p-8 rounded-3xl transition-all hover:-translate-y-1 hover:shadow-[0_12px_40px_rgba(0,0,0,0.12)] hover:border-blue-300 dark:hover:border-blue-700"
              >
                <h2 className="text-xl sm:text-2xl font-bold text-[#0071E3] dark:text-blue-400 mb-4 font-outfit">
                  {section.version}
                </h2>
                <div className="space-y-3">
                  {section.contentLines.map((line, lidx) => {
                    if (line.startsWith("### ")) {
                      return (
                        <h3
                          key={lidx}
                          className="text-lg font-semibold text-gray-800 dark:text-gray-200 mt-6 mb-2 font-outfit tracking-tight"
                        >
                          {line.replace("### ", "")}
                        </h3>
                      );
                    }
                    if (line.startsWith("- ")) {
                      return (
                        <li key={lidx} className="text-gray-600 dark:text-gray-300 ml-5 list-disc pl-1 marker:text-[#0066FF]">
                          {line.replace("- ", "")}
                        </li>
                      );
                    }
                    return (
                      <p key={lidx} className="text-gray-600 dark:text-gray-300 leading-relaxed">
                        {line}
                      </p>
                    );
                  })}
                </div>
                {section.screenshot_url && (
                  <img
                    src={section.screenshot_url}
                    alt={`${section.version} Screenshot`}
                    loading="lazy"
                    className="rounded-2xl mt-6 w-full shadow-lg border border-gray-200/50 dark:border-gray-700/50 object-cover"
                  />
                )}
              </motion.div>
            ))
          )}

          <div className="mt-10 text-center">
            <a
              href="https://onehumancorp.com/changelog"
              target="_blank"
              rel="noopener noreferrer"
              className="text-[#0071E3] dark:text-blue-400 font-bold hover:text-blue-700 dark:hover:text-blue-300 bg-blue-50/80 dark:bg-blue-900/20 px-10 py-5 text-lg rounded-full border border-blue-100 dark:border-blue-800/50 inline-block shadow-sm backdrop-blur-xl saturate-[210%] transition-all hover:shadow-md hover:-translate-y-0.5"
            >
              Read the full technical changelog on our website →
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
