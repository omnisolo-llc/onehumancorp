"use client";

import React, { useState, useEffect } from 'react';

export default function ServicesPage() {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setTimeout(() => setLoading(false), 500);
  }, []);

  if (loading) {
    return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <main id="services-screen" className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">
        <h1 className="text-2xl font-bold font-outfit">Service Manager</h1>
        <div className="space-y-4">
            <div className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
              <span className="font-medium">Status: running</span>
            </div>
            <div className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
              <span className="font-medium">Resource usage: CPU 5%, memory 128MB</span>
            </div>
        </div>
        <button className="bg-blue-600 text-white px-4 py-2 rounded font-bold hover:bg-blue-700 transition">Restart</button>
        <label className="flex items-center gap-2 mt-4 cursor-pointer">
           <input type="checkbox" defaultChecked />
           Auto restart
        </label>
      </main>
    </div>
  );
}
