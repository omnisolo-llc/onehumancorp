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
    <div className="min-h-screen bg-[#F5F5F7]/80 p-4 sm:p-8 backdrop-blur-[30px] saturate-[210%] font-inter flex flex-col items-center overflow-x-hidden w-full max-w-[100vw]">
      <style dangerouslySetInnerHTML={{__html: `
        .swagger-ui { background: transparent; border-radius: 12px; padding: 12px; width: 100%; box-sizing: border-box; max-width: 100vw; overflow-x: hidden; }
        @media (min-width: 640px) { .swagger-ui { padding: 24px; } }
        .swagger-ui .wrapper { width: 100%; max-width: 100vw; overflow-x: hidden; padding: 0 10px; box-sizing: border-box; }
        .swagger-ui .opblock-body pre { white-space: pre-wrap; word-wrap: break-word; overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui table { display: block; overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .markdown p { word-break: break-word; box-sizing: border-box; }
        .swagger-ui .info { margin: 20px 0; box-sizing: border-box; }
        .swagger-ui .scheme-container { background: transparent; padding: 10px 0; margin-bottom: 20px; border-radius: 12px; box-shadow: none; border: 1px solid rgba(0,0,0,0.1); box-sizing: border-box; width: 100%; }
        .swagger-ui .responses-inner { overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .model-box { overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .opblock-tag { font-size: 20px; padding: 10px; box-sizing: border-box; }
        .swagger-ui .opblock .opblock-summary { padding: 5px; box-sizing: border-box; }
        .swagger-ui .opblock .opblock-summary-method { min-width: 60px; font-size: 12px; }
        .swagger-ui .opblock .opblock-summary-path { font-size: 14px; max-width: calc(100vw - 120px); overflow-wrap: break-word; word-break: break-all; }
        .swagger-ui .info h1, .swagger-ui .info h2, .swagger-ui .info h3, .swagger-ui .info p, .swagger-ui .opblock-summary-method, .swagger-ui .opblock-summary-path, .swagger-ui .opblock-summary-description { color: inherit !important; }
      `}} />
      <div data-testid="api-docs-title" className="w-full max-w-6xl bg-yellow-50/80 backdrop-blur-[30px] saturate-[210%] border-l-4 border-yellow-400 p-4 mb-8 rounded-r-xl shadow-sm font-inter">
        <div className="text-yellow-700 text-sm">
          <WithTooltip id="api-docs-tooltip" defaultText="Direct API access is only for custom integrations.">
            <span className="font-outfit cursor-help font-bold">Advanced:</span>
          </WithTooltip>{" "}This section is for developers directly integrating with our APIs. Not required for normal use.
        </div>
      </div>
      {mounted && loading && (
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-[#0071E3]"></div>
        </div>
      )}
      {mounted && !loading && spec && (
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
