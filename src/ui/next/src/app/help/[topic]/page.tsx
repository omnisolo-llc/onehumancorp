import React from 'react';
import Link from 'next/link';

export default function HelpTopicPage({ params }: { params: { topic: string } }) {
  // Simple humanization of the URL slug
  const title = params.topic.split('-').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white p-8 rounded-xl shadow-sm border border-gray-100">
        <Link href="/dashboard" className="text-blue-600 hover:underline mb-6 inline-block font-medium text-sm">
          &larr; Back to Dashboard
        </Link>
        <h1 className="text-3xl font-bold text-gray-900 mb-6 font-outfit">{title}</h1>

        <div className="prose prose-blue max-w-none text-gray-600">
          <p className="text-lg mb-4">
            Welcome to the <strong>{title}</strong> guide. Here you will find all the information you need to get started and succeed.
          </p>
          <p className="mb-4">
            As a small business owner, your time is valuable. This guide is designed to be quick and easy to understand, with no technical jargon.
          </p>
          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Getting Started Steps</h2>
          <ul className="list-disc pl-5 mb-6 space-y-2">
            <li>Review your current setup and ensure all details are correct.</li>
            <li>Follow the step-by-step walkthroughs available in the help widget.</li>
            <li>If you get stuck, simply click the "Ask anything" button to get instant answers from our AI Help Agent.</li>
          </ul>
          <p>
            Still need help? You can always reach out to our support team or use the interactive tours built directly into your dashboard.
          </p>
        </div>
      </div>
    </div>
  );
}
