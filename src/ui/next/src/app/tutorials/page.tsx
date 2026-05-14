"use client";

import React from 'react';

export default function TutorialsPage() {
  return (
    <div className="container mx-auto p-8 max-w-4xl">
      <h1 className="text-3xl font-bold mb-6">Video Tutorials</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="border rounded-lg overflow-hidden">
          <div className="bg-gray-200 h-48 flex items-center justify-center">
            <span className="text-gray-500">Video Player (45s)</span>
          </div>
          <div className="p-4">
            <h3 className="font-bold">How to add a product</h3>
          </div>
        </div>
      </div>
    </div>
  );
}
