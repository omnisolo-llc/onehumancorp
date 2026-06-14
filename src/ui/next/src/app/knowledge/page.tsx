"use client";

import React, { useState } from 'react';

export default function KnowledgeDocuments() {
  const [documents, setDocuments] = useState<any[]>([]);
  const [isSyncing, setIsSyncing] = useState(false);

  const handleUpload = async () => {
    setIsSyncing(true);
    try {
      const response = await fetch('/api/knowledge/sync', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ documents: [{ id: Date.now().toString(), name: `Policy_${documents.length + 1}.pdf` }] }),
      });

      if (response.ok) {
        const data = await response.json();
        setDocuments([...documents, ...data.synced_documents]);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setIsSyncing(false);
    }
  };

  return (
    <div className="p-4 max-w-lg mx-auto">
      <h1 className="text-xl font-semibold mb-4">Knowledge & Documents</h1>

      <div className="flex justify-between items-center mb-6">
        <button
          onClick={handleUpload}
          disabled={isSyncing}
          className="px-4 py-2 bg-blue-600 text-white rounded disabled:bg-blue-300"
          data-testid="upload-button"
        >
          Upload Policies
        </button>

        {isSyncing && (
          <div className="flex items-center text-sm font-medium text-amber-600 bg-amber-50 px-3 py-1 rounded-full border border-amber-200 shadow-sm" data-testid="syncing-indicator">
            <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-amber-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Syncing...
          </div>
        )}
      </div>

      <div className="space-y-3">
        {documents.map((doc) => (
          <div key={doc.id} className="p-3 border rounded flex justify-between items-center bg-white shadow-sm" data-testid="document-item">
            <span>{doc.name}</span>
            <span className="text-xs font-medium bg-green-100 text-green-800 px-2 py-1 rounded-full">Ready</span>
          </div>
        ))}
        {documents.length === 0 && !isSyncing && (
          <div className="text-gray-500 text-center py-8 text-sm">
            No documents uploaded yet.
          </div>
        )}
      </div>
    </div>
  );
}
