'use client';

import React from 'react';

export default function IntegrationsPage() {
  return (
    <div className="p-8 max-w-4xl mx-auto space-y-8">
      <h1 className="text-3xl font-semibold mb-6">Integrations</h1>

      <section className="bg-white rounded-lg shadow-sm p-6 border border-gray-100">
        <h2 className="text-xl font-medium mb-4">Connect Custom Software</h2>
        <p className="text-gray-600 mb-6">Connect your existing tools to OHC.</p>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="border border-gray-200 rounded p-4 flex items-center justify-between">
            <span className="font-medium">Stripe</span>
            <button className="text-sm bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded">Connect</button>
          </div>
          <div className="border border-gray-200 rounded p-4 flex items-center justify-between">
            <span className="font-medium">QuickBooks</span>
            <button className="text-sm bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded">Connect</button>
          </div>
        </div>
      </section>

      <section className="bg-white rounded-lg shadow-sm p-6 border border-gray-100 mt-8">
        <h2 className="text-xl font-medium mb-4">Social Media Accounts</h2>
        <p className="text-gray-600 mb-6">Link your social media to enable the Promoter AI.</p>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="border border-gray-200 rounded p-4 flex items-center justify-between">
            <span className="font-medium">Instagram</span>
            <button className="text-sm bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded">Connect</button>
          </div>
          <div className="border border-gray-200 rounded p-4 flex items-center justify-between">
            <span className="font-medium">Facebook</span>
            <button className="text-sm bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded">Connect</button>
          </div>
        </div>
      </section>
    </div>
  );
}
