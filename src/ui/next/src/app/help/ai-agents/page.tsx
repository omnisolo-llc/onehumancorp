"use client";

import React from "react";

export default function AiAgentsHelp() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-2xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <a href="/dashboard" className="text-blue-600 text-sm font-bold mb-6 inline-block hover:underline">← Back to Dashboard</a>
        <h1 className="text-3xl font-extrabold text-gray-900 font-outfit mb-6">Using AI Agents</h1>
        <div className="prose prose-blue text-gray-600">
          <p>Think of AI Agents as your digital employees. They can work 24/7 to help run your business.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">What can AI Agents do?</h2>
          <p>Agents can answer customer questions, write product descriptions, suggest marketing ideas, and even design parts of your website.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">How to hire an agent</h2>
          <p>Go to your dashboard and look for the "Team" or "AI Agents" section. You can activate different agents based on what you need help with. For example, turn on the "Support Agent" to automatically answer common questions from your buyers.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Are they safe?</h2>
          <p>Yes. You are always in control. You can see everything your agents are doing in the Team Activity feed, and you can turn them off at any time.</p>
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
