"use client";

import React from 'react';
import Link from 'next/link';
import { motion } from "framer-motion";

export default function OutcomeFeedAuthorityHelpPage() {
  return (
    <div className="min-h-screen bg-[#F5F5F7] py-6 sm:py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <Link href="/help" className="inline-flex items-center text-[#0071E3] hover:text-[#0077ED] font-medium mb-8 transition-colors">
          <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Help Center
        </Link>

        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }} className="backdrop-blur-[40px] saturate-[210%] bg-white/70 border border-white/40 shadow-lg p-8 sm:p-10 rounded-3xl">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 mb-6">Managing Standing Authority Limits</h1>
          <div className="prose prose-lg prose-blue text-gray-600 max-w-none">
            <p>Your AI team executes routine work within the "standing authority" limits you set during onboarding. This ensures they can accomplish daily tasks like sending emails and proposals without requiring repeated approval dialogs for every step.</p>

            <h3 className="text-xl font-bold text-gray-800 mt-8 mb-4">When Exceptions Occur</h3>
            <p>High-impact changes—such as exceeding a budget cap, modifying a core offering, or completing tasks that mandate your legal/professional review—will automatically pause execution. The AI team will expose the exact dependencies in your Outcome Feed, allowing you to quickly review and approve the required exception.</p>

            <h3 className="text-xl font-bold text-gray-800 mt-8 mb-4">Adjusting Your Limits</h3>
            <p>If you find that the AI team is pausing too frequently on routine decisions, you can adjust your standing authority limits in your settings to grant them more autonomy, while maintaining hard boundaries where necessary.</p>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
