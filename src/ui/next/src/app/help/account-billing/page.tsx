"use client";

import React from "react";

export default function AccountBillingHelp() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-2xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <a href="/dashboard" className="text-blue-600 text-sm font-bold mb-6 inline-block hover:underline">← Back to Dashboard</a>
        <h1 className="text-3xl font-extrabold text-gray-900 font-outfit mb-6">Account & Billing</h1>
        <div className="prose prose-blue text-gray-600">
          <p>Manage your subscription and account settings here.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Upgrading to Premium</h2>
          <p>The Premium plan removes "Powered by OHC" branding from your store, lets you use your own custom web address (like www.my-awesome-store.com), and gives you priority AI processing. You can upgrade from the builder screen or the settings menu.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Viewing Your Receipts</h2>
          <p>To see how much you have paid for your OHC subscription, go to your Account Settings and click on "Billing History". You can download PDF receipts for your taxes.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Canceling Your Plan</h2>
          <p>We'd hate to see you go, but you can cancel your subscription at any time from the Account Settings page. Your store will remain active until the end of your current billing cycle.</p>
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
