"use client";

import React, { useState, useEffect } from "react";
import { PageHeader } from "@/components/layout/PageHeader";

type MemoryDocument = {
  id: string;
  name: string;
  content: string;
  reliabilityScore?: number;
  lastReferencedAt?: string;
};

function parseDocuments(value: unknown): MemoryDocument[] | null {
  const rows = Array.isArray(value)
    ? value
    : value && typeof value === "object" && Array.isArray((value as { memories?: unknown }).memories)
      ? (value as { memories: unknown[] }).memories
      : null;
  if (!rows) return null;
  const documents: MemoryDocument[] = [];
  for (const row of rows) {
    if (!row || typeof row !== "object") return null;
    const record = row as Record<string, unknown>;
    if (typeof record.id !== "string" || !record.id.trim() || typeof record.content !== "string") return null;
    const source = typeof record.source_type === "string" && record.source_type.trim()
      ? record.source_type.trim()
      : typeof record.source === "string" && record.source.trim()
        ? record.source.trim()
        : "Document";
    const reliability = typeof record.reliability_score === "number" && Number.isInteger(record.reliability_score)
      && record.reliability_score >= 0 && record.reliability_score <= 100
      ? record.reliability_score
      : undefined;
    const timestamp = typeof record.last_referenced_at === "string" && Number.isFinite(Date.parse(record.last_referenced_at))
      ? record.last_referenced_at
      : typeof record.created_at === "string" && Number.isFinite(Date.parse(record.created_at))
        ? record.created_at
        : undefined;
    documents.push({ id: record.id, name: source, content: record.content, reliabilityScore: reliability, lastReferencedAt: timestamp });
  }
  return documents;
}

export default function KnowledgePage() {
  const t = (s: string) => s;
  const [documents, setDocuments] = useState<MemoryDocument[]>([]);
  const [isSyncing, setIsSyncing] = useState(false);
  const [loadState, setLoadState] = useState<"loading" | "ready" | "error">("loading");
  const fileInputRef = React.useRef<HTMLInputElement>(null);

  const fetchDocuments = async () => {
    try {
      const res = await fetch("/api/v1/memory");
      if (!res.ok) throw new Error("Document request failed");
      const parsed = parseDocuments(await res.json());
      if (!parsed) throw new Error("Invalid document response");
      setDocuments(parsed);
      setLoadState("ready");
    } catch {
      setDocuments([]);
      setLoadState("error");
    }
  };

  useEffect(() => {
    fetchDocuments();
  }, []);

  const handleDelete = async (id: string) => {
    try {
      const response = await fetch(`/api/v1/memory/${id}`, {
          method: "DELETE",
      });
      if (!response.ok) throw new Error("Failed to delete document");
      fetchDocuments();
    } catch (e) {
      console.error(e);
    }
  };

  const handleUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setIsSyncing(true);

    try {
      if (file.size > 750_000) throw new Error("Document exceeds the 750 KB limit");
      const content = await file.text();

      const response = await fetch("/api/v1/memory/upload", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ content, source_type: file.name }),
      });
      if (!response.ok) throw new Error("Failed to upload document");
      await fetchDocuments();
    } catch (error) {
      console.error(error);
    } finally {
      setIsSyncing(false);
      if (fileInputRef.current) fileInputRef.current.value = "";
    }
  };

  return (
    <div className="app-page">
      <PageHeader title={t("Knowledge & Documents")} />

      <div className="p-4">
        <div className="glassmorphism p-6 rounded-2xl mb-6">
          <h2 className="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Document Library</h2>
          <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
            Upload policies, FAQs, and business documents. The Knowledge Assistant will use these to answer customer questions and draft accurate responses.
          </p>

          <input
            type="file"
            ref={fileInputRef}
            onChange={handleUpload}
            className="hidden"
            accept=".txt,.md,.csv"
          />
          <button
            onClick={() => fileInputRef.current?.click()}
            disabled={isSyncing}
            className="w-full md:w-auto px-6 py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-xl transition-all shadow-sm disabled:opacity-50 min-h-[44px]"
          >
            {isSyncing ? "Syncing..." : "Upload New Document"}
          </button>
        </div>

        <div className="glassmorphism rounded-2xl overflow-hidden">
          {loadState === "loading" ? (
            <div className="p-12 text-center text-gray-500">Loading documents…</div>
          ) : loadState === "error" ? (
            <div className="p-12 text-center text-red-600" role="alert">Document data is unavailable.</div>
          ) : documents.length === 0 ? (
            <div className="p-12 text-center text-gray-500">
              No documents uploaded yet.
            </div>
          ) : (
            <div className="divide-y divide-gray-100 dark:divide-gray-800">
              {documents.map((doc) => (
                <div key={doc.id} className="p-4 flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-lg bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-[#0071E3] dark:text-blue-400">
                      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                    </div>
                    <div>
                      <h3 className="font-medium text-gray-900 dark:text-white">{doc.name}</h3>
                      {doc.lastReferencedAt && <p className="text-xs text-gray-500">Last referenced {new Date(doc.lastReferencedAt).toLocaleString()}</p>}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    {typeof doc.reliabilityScore === "number" && <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300">
                      Reliability {doc.reliabilityScore}%
                    </span>}
                    <button onClick={() => handleDelete(doc.id)} className="text-[#FF3B30] hover:text-red-700 text-sm font-medium ml-2">
                      Delete
                    </button>
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
