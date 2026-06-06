"use client";

import { useState } from "react";
import { AppShell } from "../../components/AppShell";

export default function InventoryScanner() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any>(null);

  const handleScan = async () => {
    setLoading(true);
    // Simulate picking an image and uploading it to our endpoint
    const formData = new FormData();
    formData.append('image', new Blob(['fake image data']), 'image.jpg');

    try {
      const res = await fetch('/api/pos/inventory/scanner', {
        method: 'POST',
        body: formData,
      });
      const data = await res.json();
      setResult(data);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppShell title="Autonomous Inventory Scanner" subtitle="Point camera at packing slips or items to auto-draft product listings.">
      <div className="app-panel max-w-2xl mx-auto mt-6">
        <div className="app-panel-header">
          <div className="app-panel-title">Scanner</div>
        </div>
        <div className="app-panel-body flex flex-col items-center gap-6 p-8">
          <div className="w-full aspect-video bg-gray-100 rounded-lg flex items-center justify-center border-2 border-dashed border-gray-300">
            {loading ? (
              <span className="animate-pulse text-gray-500">Processing with Vision AI...</span>
            ) : (
              <span className="text-gray-400">Camera Viewfinder</span>
            )}
          </div>

          <button
            onClick={handleScan}
            disabled={loading}
            className="app-btn-primary w-full max-w-sm"
          >
            {loading ? "Scanning..." : "📷 Capture Image"}
          </button>

          {result && result.success && (
            <div className="w-full mt-6 bg-white border border-gray-200 rounded-lg p-6 text-left">
              <h3 className="font-bold text-lg mb-4 text-green-700">✓ Extracted Data</h3>
              {result.items.map((item: any, i: number) => (
                <div key={i} className="mb-4">
                  <p className="font-semibold">{item.name}</p>
                  <ul className="list-disc pl-5 mt-2 text-sm text-gray-700">
                    {item.variants.map((v: any, j: number) => (
                      <li key={j}>Size {v.size}: {v.quantity} units</li>
                    ))}
                  </ul>
                </div>
              ))}
              <button className="app-btn-secondary w-full mt-4">Confirm & Save to Catalog</button>
            </div>
          )}
        </div>
      </div>
    </AppShell>
  );
}
