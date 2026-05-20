"use client";

import React from "react";

export default function PaymentsHelp() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-2xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <a href="/dashboard" className="text-blue-600 text-sm font-bold mb-6 inline-block hover:underline">← Back to Dashboard</a>
        <h1 className="text-3xl font-extrabold text-gray-900 font-outfit mb-6">Handling Payments</h1>
        <div className="prose prose-blue text-gray-600">
          <p>Getting paid is the most important part of your business. Here is how to make sure money goes from your customers to your bank account safely.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Connecting Your Bank</h2>
          <p>To receive money, you must link your business checking or savings account. Go to the Payments section, click "Connect Bank", and follow the secure instructions. We use bank-level security to keep your information safe.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">When Do I Get Paid?</h2>
          <p>When a customer buys something, the money takes about 2-3 business days to show up in your bank account. This is normal processing time.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Refunds</h2>
          <p>If a customer needs a refund, find their order in your dashboard and click "Issue Refund". The money will be sent back to their card within 5-10 days.</p>
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
