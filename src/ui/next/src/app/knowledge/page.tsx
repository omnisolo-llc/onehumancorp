'use client';

import React, { useState, useEffect } from 'react';

type Document = {
  id: string;
  name: string;
  sync_status: 'pending' | 'synced' | 'error';
};

export default function KnowledgePage() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [isUploading, setIsUploading] = useState(false);

  useEffect(() => {
    const pendingDocs = documents.filter(doc => doc.sync_status === 'pending');
    if (pendingDocs.length === 0) return;

    const interval = setInterval(async () => {
      try {
        const ids = pendingDocs.map(d => d.id).join(',');
        const res = await fetch(`/api/v1/knowledge/status?ids=${ids}`);
        const data = await res.json();

        setDocuments(prevDocs =>
          prevDocs.map(doc => {
            if (data.statuses[doc.id]) {
              return { ...doc, sync_status: data.statuses[doc.id] };
            }
            return doc;
          })
        );
      } catch (err) {
        console.error('Error polling status', err);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [documents]);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files || e.target.files.length === 0) return;

    setIsUploading(true);
    const formData = new FormData();
    Array.from(e.target.files).forEach(file => {
      formData.append('files', file);
    });

    try {
      const res = await fetch('/api/v1/knowledge/upload', {
        method: 'POST',
        body: formData,
      });
      const data = await res.json();
      setDocuments(prev => [...prev, ...data.documents]);
    } catch (err) {
      console.error('Failed to upload', err);
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="min-h-screen p-4 md:p-8 bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 font-sans">
      <div className="max-w-4xl mx-auto space-y-6">
        <header className="mb-8">
          <h1 className="text-3xl font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]">
            Knowledge & Documents
          </h1>
          <p className="text-sm text-gray-500 mt-2">Manage your policies, menus, and context for your Assistant.</p>
        </header>

        <section className="p-6 rounded-2xl bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 shadow-sm">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-medium">Upload Documents</h2>
          </div>
          <div className="relative border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl p-8 text-center hover:bg-gray-50/50 dark:hover:bg-gray-800/50 transition-colors">
            <input
              type="file"
              multiple
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              onChange={handleFileUpload}
              disabled={isUploading}
              data-testid="document-upload-input"
            />
            <p className="text-sm font-medium">
              {isUploading ? 'Uploading...' : 'Drag and drop or click to upload PDF policies'}
            </p>
          </div>
        </section>

        <section className="space-y-4">
          <h2 className="text-xl font-medium px-1">Your Documents</h2>
          {documents.length === 0 ? (
            <p className="text-sm text-gray-500 px-1">No documents uploaded yet.</p>
          ) : (
            <div className="grid gap-3">
              {documents.map((doc) => (
                <div
                  key={doc.id}
                  className="flex items-center justify-between p-4 rounded-xl bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 shadow-sm"
                  data-testid={`document-item-${doc.id}`}
                >
                  <span className="font-medium truncate mr-4">{doc.name}</span>
                  {doc.sync_status === 'pending' && (
                    <span className="text-xs font-semibold px-2.5 py-1 rounded-full bg-[#FF9500]/10 text-[#FF9500] dark:bg-[#FF9F1A]/10 dark:text-[#FF9F1A]" data-testid="status-syncing">
                      Syncing...
                    </span>
                  )}
                  {doc.sync_status === 'synced' && (
                    <span className="text-xs font-semibold px-2.5 py-1 rounded-full bg-[#34C759]/10 text-[#34C759] dark:bg-[#00C24B]/10 dark:text-[#00C24B]" data-testid="status-synced">
                      Active
                    </span>
                  )}
                  {doc.sync_status === 'error' && (
                    <span className="text-xs font-semibold px-2.5 py-1 rounded-full bg-[#FF3B30]/10 text-[#FF3B30] dark:bg-[#DE1B1B]/10 dark:text-[#DE1B1B]" data-testid="status-error">
                      Error
                    </span>
                  )}
                </div>
              ))}
            </div>
          )}
        </section>
      </div>
    </div>
  );
}
