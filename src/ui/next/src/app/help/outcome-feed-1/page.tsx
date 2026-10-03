"use client";

import React from 'react';
import Link from 'next/link';
import { motion } from "framer-motion";

export default function OutcomeFeedHelpPage() {
  return (
    <div className="min-h-screen bg-[#F5F5F7] py-6 sm:py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <Link href="/help" className="inline-flex items-center text-[#0071E3] hover:text-[#0077ED] font-medium mb-8 transition-colors">
          <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Help Center
        </Link>

        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }} className="backdrop-blur-[40px] saturate-[210%] bg-white/70 border border-white/40 shadow-lg p-8 sm:p-10 rounded-3xl">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 mb-6">Understanding the Owner Outcome Feed</h1>
          <div className="prose prose-lg prose-blue text-gray-600 max-w-none">
            <p>The Owner Outcome Feed is your primary view into what your AI team has accomplished. It presents completed results, evidence, costs, and any precise remaining dependencies that require your attention.</p>

            <h3 className="text-xl font-bold text-gray-800 mt-8 mb-4">How to Identify Pending Exceptions</h3>
            <p>Tasks that require your manual intervention—such as high-impact changes, external dependencies, or actions outside of your standing authority limits—will appear as "Pending Exceptions" in the feed. They are clearly marked, and clicking on them will guide you to resolve the dependency without needing to click repeatedly for routine, autonomous decisions.</p>

            <h3 className="text-xl font-bold text-gray-800 mt-8 mb-4">Understanding Cost and Evidence</h3>
            <p>Every completed task attached to a client or outcome contains an itemized cost and the required evidence of completion (such as payment receipts or email logs). You can view the full context directly in the feed to verify exactly what the AI team delivered and how resources were spent.</p>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
