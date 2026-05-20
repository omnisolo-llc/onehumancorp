"use client";

import React, { useState, useEffect } from "react";

export default function ChangelogPage() {
  return (
    <div className="min-h-screen bg-gray-50 flex justify-center py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl w-full space-y-8 bg-white p-10 rounded-2xl shadow-sm border border-gray-100">
        <div className="text-center">
          <h1 className="text-4xl font-extrabold text-gray-900 font-outfit">What's New in OHC</h1>
          <p className="mt-2 text-lg text-gray-600">The latest updates and improvements to help your business grow.</p>
        </div>

        <div className="mt-10 space-y-12">
          {/* Release 1 */}
          <div className="relative pl-8 border-l-2 border-blue-100 pb-12">
            <div className="absolute w-4 h-4 bg-blue-600 rounded-full -left-[9px] top-1"></div>
            <span className="text-sm font-bold text-blue-600 mb-2 block">v0.4.43 • Today</span>
            <h2 className="text-2xl font-bold text-gray-900 mb-4 font-outfit">Cloud Scaling & Privacy Improvements</h2>

            <div className="prose prose-blue text-gray-600">
              <p>We've made significant updates to how our platform handles growing businesses and keeps your data secure.</p>

              <h3 className="text-lg font-semibold text-gray-800 mt-6 mb-2">🚀 Better Performance for Growing Stores</h3>
              <p>We've optimized our cloud database connections, meaning your store will load faster and remain stable even during high traffic periods like flash sales or holidays.</p>

              <h3 className="text-lg font-semibold text-gray-800 mt-6 mb-2">🔒 Enhanced Privacy Rules</h3>
              <p>We've updated our offline telemetry to give you more control over your data. Your local offline usage and privacy are now better protected.</p>
            </div>
          </div>

          {/* Release 2 */}
          <div className="relative pl-8 border-l-2 border-blue-100 pb-12">
            <div className="absolute w-4 h-4 bg-gray-300 rounded-full -left-[9px] top-1"></div>
            <span className="text-sm font-bold text-gray-500 mb-2 block">v0.4.42 • Last Week</span>
            <h2 className="text-2xl font-bold text-gray-900 mb-4 font-outfit">Desktop App Beta Enhancements</h2>

            <div className="prose prose-blue text-gray-600">
              <p>For users of our standalone desktop app, we've rolled out new features to improve your offline experience.</p>

              <ul className="list-disc pl-5 mt-4 space-y-2">
                <li>Enforced improved local offline usage for beta builds.</li>
                <li>Optimized multi-tenant scaling across our cloud staging environments.</li>
              </ul>
            </div>
          </div>

           {/* Release 3 */}
          <div className="relative pl-8 border-l-2 border-blue-100">
            <div className="absolute w-4 h-4 bg-gray-300 rounded-full -left-[9px] top-1"></div>
            <span className="text-sm font-bold text-gray-500 mb-2 block">v0.4.37 • Last Month</span>
            <h2 className="text-2xl font-bold text-gray-900 mb-4 font-outfit">Smarter AI Agents</h2>

            <div className="prose prose-blue text-gray-600">
              <p>Your AI workforce just got an upgrade!</p>

              <ul className="list-disc pl-5 mt-4 space-y-2">
                <li><strong>AutoDream Vector Data Pipelines:</strong> Your AI agents now have better memory and can remember context about your store more efficiently.</li>
                <li><strong>Local Agent Fallback:</strong> If you lose internet connection, your local agents will now gracefully fall back to local-only mode so you can keep working without interruption.</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
