"use client";

import React, { useState } from 'react';

export default function HelpCenterPage() {
  const [search, setSearch] = useState('');

  return (
    <div className="container mx-auto p-8 max-w-4xl">
      <h1 className="text-3xl font-bold mb-6">Help Center</h1>
      <input
        type="text"
        placeholder="Search for answers..."
        className="w-full border rounded p-4 mb-8 text-lg"
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="border p-6 rounded-lg">
          <h2 className="text-xl font-bold mb-2">Getting Started</h2>
          <ul className="list-disc pl-5">
            <li><a href="#" className="text-blue-600 hover:underline">Set up your store</a></li>
            <li><a href="#" className="text-blue-600 hover:underline">Add your first product</a></li>
          </ul>
        </div>
        <div className="border p-6 rounded-lg">
          <h2 className="text-xl font-bold mb-2">Payments</h2>
          <ul className="list-disc pl-5">
            <li><a href="#" className="text-blue-600 hover:underline">Connect your bank account</a></li>
            <li><a href="#" className="text-blue-600 hover:underline">Understanding fees</a></li>
          </ul>
        </div>
        <div className="border p-6 rounded-lg">
          <h2 className="text-xl font-bold mb-2">AI Agents</h2>
          <ul className="list-disc pl-5">
            <li><a href="#" className="text-blue-600 hover:underline">Hire an AI Support Agent</a></li>
          </ul>
        </div>
      </div>
    </div>
  );
}
