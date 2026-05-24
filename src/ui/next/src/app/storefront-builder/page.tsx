'use client';

import React, { useState } from 'react';

export default function StorefrontBuilderPage() {
  const [showPublish, setShowPublish] = useState(false);
  const [showEmbed, setShowEmbed] = useState(false);

  return (
    <div className="p-8 font-inter">
      <h1 className="text-2xl font-bold font-outfit mb-4">Storefront Builder</h1>
      <button onClick={() => setShowPublish(true)} className="bg-blue-600 text-white px-4 py-2 rounded">Publish Changes</button>
      <button onClick={() => setShowEmbed(true)} className="bg-gray-800 text-white px-4 py-2 rounded ml-2">Embed</button>

      <div className="mt-8 border p-4 rounded bg-gray-50">
        <h2 className="text-xl font-bold mb-2">Storefront Preview</h2>
        {/* Placeholder for preview content */}
        <div className="h-64 flex items-center justify-center border-dashed border-2 border-gray-300">
           Content...
        </div>
        <div className="powered-by-footer mt-4 text-center py-2 text-sm text-gray-500">
          ⚡ Powered by OHC
          <br/>
          <a href="ohc://join?ref=storefront" className="text-blue-500">Get your own free store</a>
        </div>
      </div>

      {showPublish && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center">
          <div className="bg-white p-6 rounded-[8px] max-w-sm w-full">
            <h2 className="text-xl font-bold mb-4">Publish Site</h2>
            <button className="bg-gray-100 p-2 rounded w-full text-left font-semibold">Free OHC Subdomain</button>
            <input type="text" placeholder="mybusiness" className="mt-2 border p-2 w-full rounded" />
            <button className="mt-4 w-full bg-blue-600 text-white p-2 rounded" onClick={() => setShowPublish(false)}>Close</button>
          </div>
        </div>
      )}

      {showEmbed && (
        <div id="embed-setup-sheet" className="open fixed inset-0 bg-black/50 flex items-center justify-center">
          <div className="bg-white p-6 rounded-[8px] max-w-sm w-full">
            <h2 className="text-xl font-bold mb-4">Embed Storefront</h2>
            <textarea id="embed-code-textarea" readOnly className="w-full h-32 border p-2 text-xs font-mono rounded" value={'<iframe src="https://mybusiness.ohc.store/api/v1/growth/storefront/embed" width="100%" height="600"></iframe>'} />
            <button className="mt-4 w-full bg-blue-600 text-white p-2 rounded" onClick={() => setShowEmbed(false)}>Close</button>
          </div>
        </div>
      )}
    </div>
  );
}
