"use client";

import { useState, useEffect } from "react";

interface KnowledgeDoc {
  id: string;
  title: string;
  status: string;
}

export default function KnowledgeHub() {
  const [docs, setDocs] = useState<KnowledgeDoc[]>([]);
  const [uploading, setUploading] = useState(false);

  useEffect(() => {
    // Mock fetch documents
    setDocs([
      { id: "1", title: "Store Policy.pdf", status: "LEARNED" },
      { id: "2", title: "Employee Handbook.docx", status: "PENDING" },
    ]);
  }, []);

  const handleUpload = () => {
    setUploading(true);
    setTimeout(() => {
      setDocs([
        { id: Math.random().toString(), title: "New Document.txt", status: "PENDING" },
        ...docs,
      ]);
      setUploading(false);
    }, 1000);
  };

  return (
    <div className="min-h-screen bg-gray-50 p-4 sm:p-6 lg:p-8">
      <div className="mx-auto max-w-3xl">
        <h1 className="text-2xl font-bold text-gray-900 mb-6">Knowledge Hub</h1>

        <div className="bg-white/80 backdrop-blur-md shadow-sm rounded-xl p-6 mb-8 border border-gray-100">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-lg font-semibold text-gray-800">Learned Documents</h2>
            <button
              onClick={handleUpload}
              disabled={uploading}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 text-sm font-medium transition-colors"
            >
              {uploading ? "Uploading..." : "+ Add Document"}
            </button>
          </div>

          <ul className="divide-y divide-gray-100">
            {docs.map((doc) => (
              <li key={doc.id} className="py-4 flex justify-between items-center">
                <span className="text-gray-700 font-medium">{doc.title}</span>
                <span className={`px-2.5 py-1 text-xs font-semibold rounded-full ${
                  doc.status === "LEARNED" ? "bg-green-100 text-green-800" :
                  doc.status === "PENDING" ? "bg-yellow-100 text-yellow-800" :
                  "bg-gray-100 text-gray-800"
                }`}>
                  {doc.status === "PENDING" ? "Learning..." : doc.status}
                </span>
              </li>
            ))}
            {docs.length === 0 && (
              <li className="py-8 text-center text-gray-500">
                No documents uploaded yet.
              </li>
            )}
          </ul>
        </div>
      </div>
    </div>
  );
}
