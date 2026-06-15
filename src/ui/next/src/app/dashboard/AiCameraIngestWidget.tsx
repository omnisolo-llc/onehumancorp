'use client';
import { useState, useRef } from 'react';
import { useRouter } from 'next/navigation';

export function AiCameraIngestWidget() {
  const [isOpen, setIsOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [approvalData, setApprovalData] = useState<any>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  const handleCapture = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    setIsOpen(true);
    setIsLoading(true);
    setApprovalData(null);

    const formData = new FormData();
    formData.append('image', file);

    try {
      const res = await fetch('/api/v1/catalog/ai-ingest', {
        method: 'POST',
        headers: {
          'Authorization': 'Bearer test-token',
        },
        body: formData,
      });

      if (!res.ok) {
        throw new Error('Failed to ingest image');
      }

      const data = await res.json();

      // Simulate waiting for KAIROS Orchestrator or websocket,
      // For now we mock the returning data. In a real application,
      // we'd listen to the mesh event or poll the job.
      setTimeout(() => {
        setIsLoading(false);
        setApprovalData({
          title: 'Generated Item',
          description: 'A beautifully generated item from your image.',
          price: '45.00',
        });
      }, 3000);

    } catch (e) {
      console.error(e);
      setIsLoading(false);
      setIsOpen(false);
    }
  };

  const handleApprove = () => {
    // Usually this updates the database record from PENDING_APPROVAL to ACTIVE
    // We navigate to products list or dashboard
    setIsOpen(false);
    alert('Item Approved and Published!');
  };

  return (
    <>
      <input
        type="file"
        accept="image/*"
        capture="environment"
        className="hidden"
        ref={fileInputRef}
        onChange={handleCapture}
      />

      <button
        onClick={() => fileInputRef.current?.click()}
        className="fixed bottom-24 right-6 z-50 glassmorphism w-14 h-14 min-w-[44px] min-h-[44px] bg-purple-600 hover:bg-purple-700 text-white rounded-full shadow-xl flex items-center justify-center text-xl transition-transform hover:scale-105"
      >
        📷
      </button>

      {isOpen && (
        <div className="fixed inset-0 z-[100] bg-black/50 backdrop-blur-md flex items-center justify-center p-4">
          <div className="glassmorphism bg-white/10 dark:bg-black/20 p-6 rounded-2xl w-full max-w-[375px] shadow-2xl border border-white/20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center space-y-4 animate-pulse">
                <div className="w-16 h-16 bg-white/20 rounded-full"></div>
                <div className="h-4 bg-white/20 rounded w-3/4"></div>
                <div className="h-4 bg-white/20 rounded w-1/2"></div>
                <p className="text-sm text-gray-200 mt-4 text-center">AI Agent is analyzing and writing descriptions...</p>
              </div>
            ) : approvalData ? (
              <div className="flex flex-col space-y-4">
                <h3 className="text-xl font-semibold text-white">Pending Approval</h3>
                <div className="space-y-2">
                  <label className="text-xs text-gray-300">Title</label>
                  <input type="text" className="w-full bg-white/5 border border-white/10 rounded p-2 text-white" defaultValue={approvalData.title} />
                </div>
                <div className="space-y-2">
                  <label className="text-xs text-gray-300">Suggested Price</label>
                  <input type="text" className="w-full bg-white/5 border border-white/10 rounded p-2 text-white" defaultValue={`$${approvalData.price}`} />
                </div>
                <div className="space-y-2">
                  <label className="text-xs text-gray-300">Description</label>
                  <textarea className="w-full bg-white/5 border border-white/10 rounded p-2 text-white h-24" defaultValue={approvalData.description}></textarea>
                </div>
                <button
                  onClick={handleApprove}
                  className="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-3 rounded-xl mt-4 min-h-[44px]"
                >
                  Approve & Publish
                </button>
                <button
                  onClick={() => setIsOpen(false)}
                  className="w-full bg-transparent text-gray-300 hover:text-white py-2 min-h-[44px]"
                >
                  Cancel
                </button>
              </div>
            ) : null}
          </div>
        </div>
      )}
    </>
  );
}
