"use client";

import React from 'react';

export default function ReleaseNotesPage() {
  return (
    <div className="container mx-auto p-8 max-w-4xl">
      <h1 className="text-3xl font-bold mb-6">What's New</h1>
      <div className="border-l-4 border-blue-500 pl-4 mb-8">
        <h2 className="text-xl font-bold">New Help Center & AI Chat</h2>
        <p className="text-gray-500 text-sm mb-2">Released Today</p>
        <p>We've added a comprehensive Help Center and a new floating AI Chat button to answer all your questions instantly.</p>
      </div>
    </div>
  );
}
