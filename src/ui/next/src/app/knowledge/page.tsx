"use client";

import React, { useState } from "react";
import Head from "next/head";

export default function KnowledgeAndDocuments() {
  const [syncStatus, setSyncStatus] = useState<"idle" | "syncing" | "ready">("idle");
  const [documents, setDocuments] = useState<File[]>([]);

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setDocuments(Array.from(e.target.files));
      setSyncStatus("syncing");

      // Simulate the backend RAG sync delay
      setTimeout(() => {
        setSyncStatus("ready");
      }, 3000);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col items-center justify-center p-4">
      <Head>
        <title>Knowledge & Documents - OHC</title>
      </Head>

      <main className="w-full max-w-[375px] md:max-w-3xl flex flex-col gap-6">
        <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-100">
          Knowledge & Documents
        </h1>

        <div className="bg-white/65 dark:bg-gray-800/70 backdrop-blur-3xl border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-xl flex flex-col gap-4">
          <p className="text-gray-600 dark:text-gray-300 text-sm">
            Upload policies, guidelines, or documents to train your AI Assistant.
          </p>

          <input
            type="file"
            multiple
            accept=".pdf,.doc,.docx,.txt"
            onChange={handleFileUpload}
            className="block w-full text-sm text-gray-500 dark:text-gray-400 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100 cursor-pointer"
            data-testid="document-upload-input"
          />

          {documents.length > 0 && (
            <div className="flex flex-col gap-2 mt-4">
              <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200">
                Uploaded Documents
              </h3>
              <ul className="text-sm text-gray-600 dark:text-gray-400 flex flex-col gap-1">
                {documents.map((doc, i) => (
                  <li key={i}>• {doc.name}</li>
                ))}
              </ul>
            </div>
          )}

          {syncStatus !== "idle" && (
            <div className="mt-4 flex items-center gap-3">
              {syncStatus === "syncing" ? (
                <>
                  <div className="w-4 h-4 rounded-full border-2 border-t-transparent border-blue-500 animate-spin" />
                  <span className="text-sm font-medium text-blue-600 dark:text-blue-400">
                    Syncing...
                  </span>
                </>
              ) : (
                <>
                  <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7" />
                  </svg>
                  <span className="text-sm font-medium text-green-600 dark:text-green-400">
                    Ready
                  </span>
                </>
              )}
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
