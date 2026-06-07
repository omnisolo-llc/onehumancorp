"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";

import { WithTooltip } from "../../components/TooltipRegistry";

export default function ApiDocsPage() {
  const [mounted, setMounted] = useState(false);
  const [spec, setSpec] = useState<Record<string, unknown> | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setMounted(true);
    fetch("/api/docs/openapi")
      .then((res) => {
        if (!res.ok) {
          throw new Error("Failed to load API spec");
        }
        return res.json();
      })
      .then((data) => {
        // dynamically adjust the server URL
        if (data && typeof data === 'object' && Array.isArray(data.servers) && data.servers.length > 0) {
            data.servers[0].url = window.location.origin;
        }
        setSpec(data);
      })
      .catch((err) => {
        setError(err.message);
      });
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
      {mounted && !spec && !error && (
         <div className="bg-white/60 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/40 flex justify-center py-20">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
         </div>
      )}
      {mounted && error && (
         <div className="bg-red-50/80 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-red-200 text-red-600">
            {error}
         </div>
      )}
      {mounted && spec && <div className="bg-white/60 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/40"><SwaggerUI spec={spec} /></div>}
    </div>
  );
}
