"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";

export default function ApiDocsPage() {
  const [mounted, setMounted] = useState(false);
  const [spec, setSpec] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setMounted(true);
    fetch("/api/docs/spec")
      .then((res) => {
        if (!res.ok) {
          throw new Error("Failed to load API spec");
        }
        return res.json();
      })
      .then((data) => setSpec(data))
      .catch((err) => setError(err.message));
  }, []);

  return (
    <div className="min-h-screen bg-gray-50/50 p-8">
      <div className="bg-yellow-50/80 backdrop-blur-[20px] saturate-200 border-l-4 border-yellow-400 p-4 mb-8 rounded-r-xl shadow-sm">
        <p className="text-yellow-700 text-sm">
          <strong>Advanced:</strong> This section is for developers directly integrating with our APIs. Not required for normal use.
        </p>
      </div>
      {error && (
        <div className="bg-red-50/80 backdrop-blur-[20px] saturate-200 border-l-4 border-red-400 p-4 mb-8 rounded-r-xl shadow-sm">
          <p className="text-red-700 text-sm">{error}</p>
        </div>
      )}
      {mounted && spec && (
        <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-xl border border-gray-100/50">
          <SwaggerUI spec={spec} />
        </div>
      )}
      {mounted && !spec && !error && (
        <div className="flex justify-center items-center py-12">
          <p className="text-gray-500">Loading API Spec...</p>
        </div>
      )}
    </div>
  );
}
