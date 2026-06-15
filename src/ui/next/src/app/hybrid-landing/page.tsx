import React, { useState } from 'react';
import { CollectivePulse, NeighborhoodDiscoveryWidget } from '../../components/collective';

export default function HybridLandingPage() {
  const [downloading, setDownloading] = useState(false);

  const handleDownload = () => {
    setDownloading(true);
    alert("Desktop App Download Started! (Simulation)");
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-4">
      <h1 className="text-4xl font-bold mb-8">OHC Hybrid OS</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl">
        <div className="bg-white p-6 rounded-xl shadow-md flex flex-col gap-4">
          <h2 className="text-2xl font-semibold">Local Sovereignty</h2>
          <ul className="list-disc pl-5">
            <li>Zero Data Leakage</li>
            <li>Air-Gapped Autonomy</li>
            <li>Self-Hosted LLMs</li>
          </ul>
          <button onClick={handleDownload} className="mt-auto bg-black text-white px-4 py-2 rounded-lg">
            {downloading ? 'Downloading...' : 'Download Desktop'}
          </button>
        </div>
        <div className="bg-white p-6 rounded-xl shadow-md flex flex-col gap-4">
          <h2 className="text-2xl font-semibold">Cloud Convenience</h2>
          <ul className="list-disc pl-5">
            <li>Team Collaboration</li>
            <li>Anywhere Access</li>
            <li>Fully Managed</li>
          </ul>
          <a href="/dashboard" className="mt-auto text-center bg-blue-600 text-white px-4 py-2 rounded-lg">Start Web Trial</a>
        </div>
      </div>
      <div className="mt-12 flex flex-col items-center gap-6">
        <CollectivePulse />
      </div>
      <NeighborhoodDiscoveryWidget />
    </div>
  );
}
