"use client";

import React, { useState } from "react";
import Link from "next/link";

export default function HybridLandingPage() {
  const [downloading, setDownloading] = useState(false);

  const handleDownload = () => {
    setDownloading(true);
    setTimeout(() => {
      setDownloading(false);
      alert("Desktop App Download Started! (Simulation)");
    }, 1500);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50 font-inter flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8">
      {/* Header */}
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 mb-4 px-4 py-2 rounded-full bg-indigo-100 text-indigo-800 text-sm font-semibold tracking-wide shadow-sm">
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 10V3L4 14h7v7l9-11h-7z"
            />
          </svg>
          OHC Hybrid OS
        </div>
        <h1 className="text-4xl sm:text-5xl md:text-6xl font-extrabold font-outfit text-gray-900 tracking-tight leading-tight mb-6">
          Your Business.
          <br />
          <span className="text-transparent bg-clip-text bg-gradient-to-r from-indigo-600 to-purple-600">
            Your AI. Your Rules.
          </span>
        </h1>
        <p className="text-xl text-gray-600 max-w-2xl mx-auto font-medium">
          Choose how you deploy your AI assistant. Maintain absolute data
          sovereignty on your own machine, or collaborate seamlessly in the
          cloud.
        </p>
      </div>

      {/* Main Cards Container */}
      <div className="w-full max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8 lg:gap-12">
        {/* Card 1: Standalone */}
        <div className="ohc-growth-card bg-white p-8 md:p-10 rounded-[24px] border border-white/50 shadow-xl bg-white/40 backdrop-blur-xl relative overflow-hidden group hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-1">
          <div className="absolute top-0 right-0 p-6 opacity-10 group-hover:opacity-20 transition-opacity">
            <svg
              className="w-32 h-32 text-indigo-900"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z" />
            </svg>
          </div>

          <div className="relative z-10">
            <div className="w-16 h-16 rounded-2xl bg-indigo-100 flex items-center justify-center text-indigo-600 mb-6 shadow-inner">
              <svg
                className="w-8 h-8"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                />
              </svg>
            </div>

            <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-4">
              Local Sovereignty
            </h2>
            <p className="text-gray-600 mb-8 font-medium leading-relaxed">
              Run the full OHC OS directly on your hardware. Unparalleled
              privacy for prosumers and enterprise operators.
            </p>

            <ul className="space-y-4 mb-10">
              <li className="flex items-start">
                <svg
                  className="w-6 h-6 text-green-500 mr-3 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700 font-medium">
                  <strong>Zero Data Leakage:</strong> 100% local SQLite.
                </span>
              </li>
              <li className="flex items-start">
                <svg
                  className="w-6 h-6 text-green-500 mr-3 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700 font-medium">
                  <strong>Air-Gapped Autonomy:</strong> Works offline.
                </span>
              </li>
              <li className="flex items-start">
                <svg
                  className="w-6 h-6 text-green-500 mr-3 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700 font-medium">
                  <strong>Self-Hosted LLMs:</strong> Bring your own models.
                </span>
              </li>
            </ul>

            <button
              onClick={handleDownload}
              disabled={downloading}
              className="w-full py-4 px-6 bg-gray-900 hover:bg-black text-white rounded-xl font-bold text-lg shadow-lg hover:shadow-xl transition-all flex items-center justify-center gap-3 disabled:opacity-70 disabled:cursor-not-allowed"
            >
              {downloading ? (
                <>
                  <svg
                    className="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                  >
                    <circle
                      className="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      strokeWidth="4"
                    ></circle>
                    <path
                      className="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    ></path>
                  </svg>
                  Downloading...
                </>
              ) : (
                <>
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                    />
                  </svg>
                  Download Desktop
                </>
              )}
            </button>
            <p className="text-center text-sm text-gray-500 mt-4 font-medium">
              macOS, Windows, Linux
            </p>
          </div>
        </div>

        {/* Card 2: Cloud */}
        <div className="ohc-growth-card bg-white p-8 md:p-10 rounded-[24px] border border-white/50 shadow-xl bg-white/40 backdrop-blur-xl relative overflow-hidden group hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-1">
          <div className="absolute top-0 right-0 p-6 opacity-10 group-hover:opacity-20 transition-opacity">
            <svg
              className="w-32 h-32 text-purple-900"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z" />
            </svg>
          </div>

          <div className="relative z-10">
            <div className="w-16 h-16 rounded-2xl bg-purple-100 flex items-center justify-center text-purple-600 mb-6 shadow-inner">
              <svg
                className="w-8 h-8"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"
                />
              </svg>
            </div>

            <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-4">
              Cloud Convenience
            </h2>
            <p className="text-gray-600 mb-8 font-medium leading-relaxed">
              Instant access from any device. Perfect for teams, rapid
              deployment, and zero-maintenance operations.
            </p>

            <ul className="space-y-4 mb-10">
              <li className="flex items-start">
                <svg
                  className="w-6 h-6 text-green-500 mr-3 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700 font-medium">
                  <strong>Team Collaboration:</strong> Seamless Multi-tenant.
                </span>
              </li>
              <li className="flex items-start">
                <svg
                  className="w-6 h-6 text-green-500 mr-3 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700 font-medium">
                  <strong>Anywhere Access:</strong> Mobile & Desktop web.
                </span>
              </li>
              <li className="flex items-start">
                <svg
                  className="w-6 h-6 text-green-500 mr-3 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700 font-medium">
                  <strong>Fully Managed:</strong> We handle the AI &
                  infrastructure.
                </span>
              </li>
            </ul>

            <Link
              href="/dashboard"
              className="w-full py-4 px-6 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-700 hover:to-indigo-700 text-white rounded-xl font-bold text-lg shadow-lg hover:shadow-xl transition-all flex items-center justify-center gap-3"
            >
              Start Web Trial
              <svg
                className="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M14 5l7 7m0 0l-7 7m7-7H3"
                />
              </svg>
            </Link>
            <p className="text-center text-sm text-gray-500 mt-4 font-medium">
              No credit card required
            </p>
          </div>
        </div>
      </div>

      <style
        dangerouslySetInnerHTML={{
          __html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .bg-white {
          backdrop-filter: blur(30px) saturate(210%);
        }
      `,
        }}
      />
    </div>
  );
}
