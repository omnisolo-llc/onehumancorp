'use client';

import React, { useState } from 'react';

export default function StorefrontBuilderPage() {
  const [showPublish, setShowPublish] = useState(false);
  const [showEmbed, setShowEmbed] = useState(false);
  const [subdomain, setSubdomain] = useState('');
  const [showEditSheet, setShowEditSheet] = useState(false);
  const [heroTitle, setHeroTitle] = useState('My Awesome Store');
  const [blocks, setBlocks] = useState([
    { type: 'Hero', content: 'Hero' },
    { type: 'Product Grid', content: 'Product Grid' }
  ]);

  const handlePublish = async () => {
    const payload = {
      domain: subdomain,
      draft: {
        domain: subdomain,
        pages: [{
          path: '/',
          title: 'Home',
          blocks: blocks.map((b, i) => ({
            block_type: b.type === 'Hero' ? 'HeroBlock' : 'ProductGridBlock',
            content: { text: b.content },
            sort_order: i
          })),
          seo_metadata: {
            "@context": "https://schema.org",
            "@type": "LocalBusiness",
            "name": subdomain
          }
        }]
      }
    };

    try {
      const response = await fetch('/api/v1/builder/publish_draft', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      if (response.ok) {
        setShowPublish(false);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleRearrange = () => {
    setBlocks([...blocks].reverse());
  };

  return (
    <div className="p-8 font-inter">
      <h1 className="text-2xl font-bold font-outfit mb-4">Edit Website</h1>
      <div className="flex gap-2 mb-4">
        <button onClick={() => setShowPublish(true)} className="bg-blue-600 text-white px-4 py-2 rounded">Publish Changes</button>
        <button onClick={() => setShowEmbed(true)} className="bg-gray-800 text-white px-4 py-2 rounded">Embed</button>
        <button onClick={handleRearrange} className="bg-gray-200 text-gray-800 px-4 py-2 rounded">Rearrange</button>
      </div>

      <div className="mt-8 border p-4 rounded bg-gray-50 max-w-[375px] mx-auto glass-container">
        <h2 className="text-xl font-bold mb-2">Storefront Preview</h2>
        <div id="builder-preview-container" className="min-h-64 flex flex-col items-center justify-center border-dashed border-2 border-gray-300 gap-4 p-4">
           {blocks.map((b, i) => (
             <div key={i} className="w-full text-center p-4 bg-white shadow rounded glassmorphism" onClick={b.type === 'Hero' ? () => setShowEditSheet(true) : undefined}>
                {b.type === 'Hero' ? heroTitle : b.content}
             </div>
           ))}
        </div>
        <div className="powered-by-footer mt-4 text-center py-2 text-sm text-gray-500">
          ⚡ Powered by OHC
          <br/>
          <a href="ohc://join?ref=storefront" className="text-blue-500">Get your own free store</a>
        </div>
      </div>

      {showEditSheet && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-sm w-full">
            <h2 id="sheet-title" className="text-xl font-bold mb-4">Edit Hero</h2>
            <input id="edit-title" type="text" value={heroTitle} onChange={(e) => setHeroTitle(e.target.value)} className="w-full border p-2 rounded mb-4" />
            <button className="w-full bg-blue-600 text-white p-2 rounded" onClick={() => setShowEditSheet(false)}>Save</button>
          </div>
        </div>
      )}

      {showPublish && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-sm w-full">
            <h2 className="text-xl font-bold mb-4">Publish Site</h2>
            <button className="bg-gray-100 p-2 rounded w-full text-left font-semibold">Free OHC Subdomain</button>
            <input type="text" placeholder="mybusiness" value={subdomain} onChange={e => setSubdomain(e.target.value)} className="mt-2 border p-2 w-full rounded" />
            <div className="flex gap-2 mt-4">
              <button className="w-full bg-gray-200 text-gray-800 p-2 rounded" onClick={() => setShowPublish(false)}>Close</button>
              <button className="w-full bg-blue-600 text-white p-2 rounded" onClick={handlePublish}>Publish</button>
            </div>
          </div>
        </div>
      )}

      {showEmbed && (
        <div id="embed-setup-sheet" className="open fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-sm w-full">
            <h2 className="text-xl font-bold mb-4">Embed Storefront</h2>
            <textarea id="embed-code-textarea" readOnly className="w-full h-32 border p-2 text-xs font-mono rounded" value={'<iframe src="https://mybusiness.ohc.store/api/v1/growth/storefront/embed" width="100%" height="600"></iframe>'} />
            <button className="mt-4 w-full bg-blue-600 text-white p-2 rounded" onClick={() => setShowEmbed(false)}>Close</button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); }
      `}} />
    </div>
  );
}
