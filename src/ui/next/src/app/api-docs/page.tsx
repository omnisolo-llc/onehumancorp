"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function ApiDocsPage() {
  const [mounted, setMounted] = useState(false);
  const [spec, setSpec] = useState<Record<string, unknown> | null>(null);

  useEffect(() => {
    setMounted(true);
    fetch("/api/docs/spec")
      .then(res => res.json())
      .then(data => {
        // Optionally inject the current origin into the servers list
        if (data && Array.isArray(data.servers) && data.servers.length > 0) {
          data.servers[0].url = window.location.origin || "http://localhost:8080";
        }
        setSpec(data);
      })
      .catch(console.error);
  }, []);

  return (
    <div className="min-h-screen bg-[#F5F5F7]/80 p-8 backdrop-blur-[20px] saturate-200 font-inter">
      <div className="bg-yellow-50/80 backdrop-blur-[20px] saturate-200 border-l-4 border-yellow-400 p-4 mb-8 rounded-r-xl shadow-sm font-inter">
        <div className="text-yellow-700 text-sm">
          <WithTooltip id="api-docs-tooltip" defaultText="Direct API access is only for custom integrations.">
            <span className="font-outfit cursor-help font-bold">Advanced:</span>
          </WithTooltip>{" "}This section is for developers directly integrating with our APIs. Not required for normal use.
        </div>
      </div>
      {mounted && spec ? (
        <div className="bg-white/60 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/40">
          <SwaggerUI spec={spec} />
        </div>
      ) : (
        <div className="flex justify-center items-center py-12">
          <svg className="w-8 h-8 animate-spin text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
        </div>
      )}
    </div>
  );
}
