"use client";
import React, { useEffect, useState } from "react";
import { format } from "date-fns";

interface DiscoveryReport {
  id: string;
  month: string;
  plain_language_summary: string;
  metrics: Record<string, number>;
}

export default function DiscoveryReportPage() {
  const [reports, setReports] = useState<DiscoveryReport[] | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchReports() {
      try {
        const res = await fetch("/api/v1/seo/discovery_report");
        if (res.ok) {
          const data = await res.json();
          setReports(data);
        } else {
          setReports([]);
        }
      } catch (err) {
        console.error("Failed to fetch discovery reports:", err);
        setReports([]);
      } finally {
        setLoading(false);
      }
    }
    fetchReports();
  }, []);

  return (
    <div className="min-h-screen bg-[#f5f5f7] dark:bg-black text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-inter">
      <div className="max-w-[375px] mx-auto w-full">
        <header className="mb-6 pt-4">
          <h1 className="text-2xl font-bold font-outfit">AI Discovery Report</h1>
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
            See how often AI search engines recommend your business.
          </p>
        </header>

        {loading ? (
          <div className="bg-white dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-sm flex items-center justify-center min-h-[150px]">
            <p className="text-sm text-gray-500">Loading your report...</p>
          </div>
        ) : !reports || reports.length === 0 ? (
          <div className="bg-white dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-sm">
            <div className="flex items-center justify-center w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 mb-4">
              <svg className="w-6 h-6 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <h2 className="font-semibold mb-2">No Reports Yet</h2>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Your first AI Discovery Report will be generated soon. The Promoter Agent is silently optimizing your storefront.
            </p>
          </div>
        ) : (
          <div className="space-y-4">
            {reports.map((report) => (
              <div key={report.id} className="bg-white dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-sm">
                <div className="flex justify-between items-center mb-4">
                  <h3 className="font-semibold text-lg">{report.month}</h3>
                  <span className="text-xs font-medium px-2.5 py-1 bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400 rounded-full">
                    Optimized
                  </span>
                </div>

                <p className="text-base text-gray-800 dark:text-gray-200 leading-relaxed">
                  {report.plain_language_summary}
                </p>

                {report.metrics && Object.keys(report.metrics).length > 0 && (
                  <div className="mt-4 pt-4 border-t border-gray-200/50 dark:border-gray-800/50">
                    <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Breakdown</h4>
                    <div className="space-y-2">
                      {Object.entries(report.metrics).map(([key, value]) => (
                        <div key={key} className="flex justify-between text-sm">
                          <span className="text-gray-600 dark:text-gray-400 capitalize">{key.replace(/_/g, ' ')}</span>
                          <span className="font-medium">{value as React.ReactNode}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
