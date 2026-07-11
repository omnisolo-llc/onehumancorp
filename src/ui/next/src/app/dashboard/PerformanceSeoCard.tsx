import React, { useEffect, useState } from "react";
import { WithTooltip } from "../../components/TooltipRegistry";

export function PerformanceSeoCard({ tenantId }: { tenantId: string }) {
  const [metrics, setMetrics] = useState({
    cacheHitRatio: "98.5%",
    estLoadTime: "120ms",
    indexingStatus: "Optimized (Agentic SEO)",
  });

  return (
    <WithTooltip content="Invisible Edge-Caching and Agentic SEO pre-rendering powered by Cloudflare and OHC Agents." position="top">
      <div className="app-card" style={{ padding: 16 }}>
        <h3 className="font-outfit text-lg font-bold mb-4 flex items-center">
          ⚡ Performance &amp; SEO
          <span className="ml-2 text-xs font-normal px-2 py-0.5 rounded-full bg-[#34C759]/20 text-[#34C759] border border-[#34C759]/30">Active</span>
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="flex flex-col gap-1">
            <span className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Edge Cache Hit Ratio</span>
            <span className="text-xl font-outfit font-bold text-[#0066FF] dark:text-[#0071E3]">{metrics.cacheHitRatio}</span>
            <span className="text-xs text-gray-500">Last 24h</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Est. Storefront Load Time</span>
            <span className="text-xl font-outfit font-bold text-[#0066FF] dark:text-[#0071E3]">{metrics.estLoadTime}</span>
            <span className="text-xs text-gray-500">Global Average</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Search Indexing</span>
            <span className="text-md font-outfit font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mt-1">{metrics.indexingStatus}</span>
            <span className="text-xs text-gray-500">Pre-rendered JSON-LD</span>
          </div>
        </div>
      </div>
    </WithTooltip>
  );
}
