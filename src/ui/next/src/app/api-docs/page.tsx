"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";

import { WithTooltip } from "../../components/TooltipRegistry";

export default function ApiDocsPage() {
  const [mounted, setMounted] = useState(false);
  const [spec, setSpec] = useState<Record<string, unknown> | null>(null);
  const [loading, setLoading] = useState(true);
  const [isDarkMode, setIsDarkMode] = useState(false);

  useEffect(() => {
    setIsDarkMode(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = () => setIsDarkMode(mediaQuery.matches);
    mediaQuery.addEventListener('change', handler);
    return () => mediaQuery.removeEventListener('change', handler);
  }, []);

  const glassStyle = isDarkMode ? {
    background: 'rgba(22, 22, 26, 0.7)',
    backdropFilter: 'blur(30px) saturate(210%)',
    border: '1px solid rgba(255, 255, 255, 0.1)',
  } : {
    background: 'rgba(255, 255, 255, 0.65)',
    backdropFilter: 'blur(30px) saturate(210%)',
    border: '1px solid rgba(255, 255, 255, 0.4)',
  };

  useEffect(() => {
    setMounted(true);
    fetch('/api/api-docs-spec')
      .then(res => res.json())
      .then(data => {
        setSpec(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Failed to load api-docs spec", err);
        setLoading(false);
      });
  }, []);

  return (
    <div className={`min-h-screen ${isDarkMode ? 'bg-gray-900 text-white' : 'bg-[#F5F5F7] text-[#1D1D1F]'} p-8 transition-colors duration-300 font-inter`}>
      <div className="bg-yellow-50/80 dark:bg-yellow-900/30 backdrop-blur-[30px] saturate-[210%] border-l-4 border-yellow-400 p-4 mb-8 rounded-r-xl shadow-sm font-inter">
        <div className="text-yellow-700 dark:text-yellow-400 text-sm">
          <WithTooltip id="api-docs-tooltip" defaultText="Direct API access is only for custom integrations.">
            <span className="font-outfit cursor-help font-bold">Advanced:</span>
          </WithTooltip>{" "}This section is for developers directly integrating with our APIs. Not required for normal use.
        </div>
      </div>
      {mounted && loading && (
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
      )}
      {mounted && !loading && spec && (
        <div className="flex flex-col h-full rounded-xl p-2 sm:p-6 overflow-x-hidden shadow-sm" style={glassStyle}>
          <div className="overflow-x-auto w-full max-w-[calc(100vw-32px)]">
            <SwaggerUI spec={spec} />
          </div>
        </div>
      )}
    </div>
  );
}
