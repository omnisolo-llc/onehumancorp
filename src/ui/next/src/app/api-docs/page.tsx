"use client";

import React, { useEffect, useState, memo } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";

import { WithTooltip } from "../../components/TooltipRegistry";
import { motion } from "framer-motion";

const MemoizedSwaggerUI = memo(SwaggerUI);

export default function ApiDocsPage() {
  const [mounted, setMounted] = useState(false);
  const [spec, setSpec] = useState<Record<string, unknown> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setMounted(true);
    fetch('/api/v1/api-docs-spec')
      .then(async (res) => {
        if (!res.ok) {
           throw new Error("Failed to load API Documentation.");
        }
        return res.json();
      })
      .then(data => {
        setSpec(data);
        setLoading(false);
      })
      .catch(() => {
        setError("Failed to load API Documentation.");
        setLoading(false);
      });
  }, []);

  return (
    <div className="min-h-screen bg-gradient-to-b from-[#F5F5F7] to-[#E8E8ED] dark:from-[#16161a] dark:to-[#0f0f13] p-2 sm:p-8 backdrop-blur-[30px] saturate-[210%] font-inter flex flex-col items-center overflow-x-hidden w-full max-w-[100vw]">
      <style dangerouslySetInnerHTML={{__html: `
        .swagger-ui { font-family: 'Inter', sans-serif; background: transparent; border-radius: 12px; padding: 12px; width: 100%; box-sizing: border-box; max-width: 100vw; overflow-x: hidden; }
        @media (min-width: 640px) { .swagger-ui { padding: 24px; } }
        .swagger-ui .wrapper { width: 100%; max-width: 100vw; overflow-x: hidden; padding: 0 10px; box-sizing: border-box; }
        .swagger-ui .opblock-body pre { white-space: pre-wrap; word-wrap: break-word; overflow-x: auto; max-width: 100%; box-sizing: border-box; border-radius: 12px; background: rgba(0,0,0,0.03); border: 1px solid rgba(0,0,0,0.05); }
        .swagger-ui table { display: block; overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .markdown p { word-break: break-word; box-sizing: border-box; }
        .swagger-ui .info { margin: 20px 0; box-sizing: border-box; font-family: 'Outfit', sans-serif; }
        .swagger-ui .info h1, .swagger-ui .info h2, .swagger-ui .info h3 { font-family: 'Outfit', sans-serif; font-weight: 800; color: #1D1D1F; }
        .swagger-ui .scheme-container { background: transparent; padding: 10px 0; margin-bottom: 20px; border-radius: 16px; box-shadow: none; background: rgba(255, 255, 255, 0.4); backdrop-filter: blur(20px); box-shadow: 0 4px 12px rgba(0,0,0,0.05); border: 1px solid rgba(255,255,255,0.3); box-sizing: border-box; width: 100%; }
        .swagger-ui .responses-inner { overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .model-box { overflow-x: auto; max-width: 100%; box-sizing: border-box; background: rgba(255,255,255,0.5); border-radius: 12px; padding: 10px; }
        .swagger-ui .opblock-tag { font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 24px; padding: 10px 10px 10px 0; box-sizing: border-box; border-bottom: 1px solid rgba(0,0,0,0.1); }
        .swagger-ui .opblock { border-radius: 16px; border: 1px solid rgba(0,0,0,0.1); background: rgba(255,255,255,0.5); box-shadow: 0 4px 12px rgba(0,0,0,0.02); overflow: hidden; margin-bottom: 15px; }
        .swagger-ui .opblock .opblock-summary { padding: 10px; box-sizing: border-box; transition: background 0.2s; }
        .swagger-ui .opblock .opblock-summary:hover { background: rgba(0,0,0,0.02); }
        .swagger-ui .opblock .opblock-summary-method { min-width: 60px; font-size: 13px; border-radius: 8px; font-weight: 700; font-family: 'Inter', sans-serif; }
        .swagger-ui .opblock .opblock-summary-path { font-size: 15px; max-width: calc(100vw - 120px); overflow-wrap: break-word; word-break: break-all; font-family: monospace; font-weight: 600; color: #1D1D1F; }
        .swagger-ui .opblock .opblock-summary-description { font-family: 'Inter', sans-serif; color: #555; }
        @media (prefers-color-scheme: dark) {
           .swagger-ui .info h1, .swagger-ui .info h2, .swagger-ui .info h3, .swagger-ui .opblock-tag, .swagger-ui .opblock .opblock-summary-path, .swagger-ui .markdown p { color: #E8E8ED !important; }
           .swagger-ui .opblock .opblock-summary-description, .swagger-ui .markdown li { color: #A1A1A6 !important; }
           .swagger-ui .opblock { background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.1); box-shadow: 0 4px 12px rgba(0,0,0,0.2); }
           .swagger-ui .opblock .opblock-summary:hover { background: rgba(255,255,255,0.05); }
           .swagger-ui .scheme-container { background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.1); }
           .swagger-ui .model-box { background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.05); }
           .swagger-ui .opblock-body pre { background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.05); }
           .swagger-ui .opblock-tag { border-bottom: 1px solid rgba(255,255,255,0.1); }
           .swagger-ui .response-col_status, .swagger-ui .response-col_description__inner div.markdown, .swagger-ui .tab li { color: #E8E8ED !important; }
           .swagger-ui section.models h4 { color: #E8E8ED !important; }
           .swagger-ui .model { color: #E8E8ED !important; }
           .swagger-ui .model-title { color: #E8E8ED !important; }
           .swagger-ui .prop-type { color: #A1A1A6 !important; }
           .swagger-ui table thead tr td, .swagger-ui table thead tr th { color: #E8E8ED !important; border-bottom: 1px solid rgba(255,255,255,0.1) !important; }
           .swagger-ui .parameters-col_name, .swagger-ui .parameters-col_description { color: #E8E8ED !important; }
           .swagger-ui select { background: rgba(0,0,0,0.3) !important; color: #E8E8ED !important; border: 1px solid rgba(255,255,255,0.1) !important; }
           .swagger-ui .btn { color: #E8E8ED !important; border: 1px solid rgba(255,255,255,0.2) !important; box-shadow: none !important; }
        }
      `}} />
      <div data-testid="api-docs-title" className="w-full max-w-6xl bg-[#FFCC00]/10 border border-[#FFCC00]/30 backdrop-blur-[30px] saturate-[210%] border-l-4 border-l-[#FFCC00] p-4 mb-8 rounded-r-xl shadow-sm font-inter">
        <div className="text-yellow-800 dark:text-yellow-400 text-sm font-medium">
          <WithTooltip id="api-docs-tooltip" defaultText="Direct API access is only for custom integrations.">
            <span className="font-outfit cursor-help font-bold">Advanced:</span>
          </WithTooltip>{" "}This section is for developers directly integrating with our APIs. Not required for normal use.
        </div>
      </div>
      {mounted && error && (
        <div className="w-full max-w-6xl flex flex-col items-center justify-center py-24 px-8 backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.08)] rounded-3xl min-h-[400px]">
           <svg className="w-16 h-16 text-red-500 mb-4 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
           <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Failed to load API documentation</h2>
           <p className="text-gray-600 dark:text-gray-400 text-center">{error}</p>
        </div>
      )}
      {mounted && loading && (
        <div className="w-full max-w-6xl flex flex-col h-full backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-sm min-h-[600px] animate-pulse">
            <div className="h-8 bg-gray-200 dark:bg-gray-800 rounded w-1/4 mb-10"></div>
            <div className="space-y-6">
                <div className="h-24 bg-gray-200 dark:bg-gray-800 rounded-xl w-full"></div>
                <div className="h-24 bg-gray-200 dark:bg-gray-800 rounded-xl w-full"></div>
                <div className="h-24 bg-gray-200 dark:bg-gray-800 rounded-xl w-full"></div>
                <div className="h-24 bg-gray-200 dark:bg-gray-800 rounded-xl w-full"></div>
            </div>
        </div>
      )}
      {mounted && !loading && !error && spec && (
        <div className="w-full max-w-6xl flex flex-col h-full backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 rounded-2xl p-0 sm:p-6 shadow-[0_12px_40px_rgba(0,0,0,0.15)] transition-all overflow-x-hidden box-border max-w-full">
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.4 }}
            className="w-full max-w-full overflow-x-hidden box-border"
          >
            <MemoizedSwaggerUI spec={spec} />
          </motion.div>
        </div>
      )}
    </div>
  );
}
