"use client";

import React from 'react';

export default function ApiDocsPage() {
  return (
    <div className="container mx-auto p-8 max-w-4xl">
      <h1 className="text-3xl font-bold mb-6">Advanced API Reference</h1>
      <p className="mb-4">This section is for developers who want to integrate directly with the OHC API.</p>
      <div className="bg-gray-100 p-4 rounded">
        <code>POST /api/v1/checkout</code>
        <p className="mt-2 text-sm">Creates a custom checkout session.</p>
      </div>
    </div>
  );
}
