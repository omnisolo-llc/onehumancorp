"use client";

import React from 'react';
import Link from 'next/link';
import { VideoTutorialList } from '../../../components/VideoTutorialList';

export default function VideoTutorialsPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="mb-6">
          <Link href="/help" className="text-blue-600 hover:text-blue-800 flex items-center gap-2 font-medium">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
            Back to Help Center
          </Link>
        </div>

        <div className="text-center mb-8">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] tracking-tight mb-4">Video Guides</h1>
          <p className="text-lg text-gray-600 max-w-2xl mx-auto">Watch quick, simple tutorials to learn how to manage your store like a pro.</p>
        </div>

        <VideoTutorialList />
      </div>
    </div>
  );
}
