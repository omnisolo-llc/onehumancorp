"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";

import { WithTooltip } from "../../components/TooltipRegistry";

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
    <div className="min-h-screen bg-[#F5F5F7]/80 p-8 backdrop-blur-[30px] saturate-[210%] font-inter">
      <div className="bg-yellow-50/80 backdrop-blur-[30px] saturate-[210%] border-l-4 border-yellow-400 p-4 mb-8 rounded-r-xl shadow-sm font-inter">
        <div className="text-yellow-700 text-sm">
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
        <div className="app-card backdrop-blur-[30px] saturate-[210%] p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/40 bg-white/65">
          <SwaggerUI spec={spec} />
        </div>
      )}
    </div>
  );
}
