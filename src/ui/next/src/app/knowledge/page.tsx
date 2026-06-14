"use client";

import React, { useState, useEffect } from "react";

export default function KnowledgePage() {
  const t = (k: string) => k;
  const [documents, setDocuments] = useState<any[]>([]);
  const [isSyncing, setIsSyncing] = useState(false);
  const [isReady, setIsReady] = useState(true);

  const handleUpload = () => {
    setIsSyncing(true);
    setIsReady(false);

    // Simulate API call
    setTimeout(() => {
      setIsSyncing(false);
      setIsReady(true);
      setDocuments([...documents, { id: Date.now(), name: "New Policy Document.pdf", status: "Active" }]);
    }, 2000);
  };

  return (
    <div className="app-page">
      <div className="mb-6"><h1 className="text-2xl font-bold text-gray-900">{t("Knowledge & Documents")}</h1></div>

      <div className="p-4">
        <div className="glassmorphism p-6 rounded-2xl mb-6">
          <h2 className="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Document Library</h2>
          <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
            Upload policies, FAQs, and business documents. The Knowledge Assistant will use these to answer customer questions and draft accurate responses.
          </p>

          <button
            onClick={handleUpload}
            disabled={isSyncing}
            className="w-full md:w-auto px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-xl transition-all shadow-sm disabled:opacity-50 min-h-[44px]"
          >
            {isSyncing ? "Syncing..." : "Upload New Document"}
          </button>
        </div>

        <div className="glassmorphism rounded-2xl overflow-hidden">
          {documents.length === 0 ? (
            <div className="p-12 text-center text-gray-500">
              No documents uploaded yet.
            </div>
          ) : (
            <div className="divide-y divide-gray-100 dark:divide-gray-800">
              {documents.map((doc, idx) => (
                <div key={idx} className="p-4 flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-lg bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-blue-600 dark:text-blue-400">
                      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                    </div>
                    <div>
                      <h3 className="font-medium text-gray-900 dark:text-white">{doc.name}</h3>
                      <p className="text-xs text-gray-500">Updated just now</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400">
                      Active
                    </span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
