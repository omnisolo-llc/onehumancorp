'use client';

import React, { useState } from 'react';

export default function AbandonedCartsPage() {
  const [enabled, setEnabled] = useState(false);
  const [showToast, setShowToast] = useState(false);

  const handleToggle = () => {
    setEnabled(!enabled);
    setShowToast(true);
    setTimeout(() => setShowToast(false), 3000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <title>AI Abandoned Cart Recovery</title>

      <main className="flex-1 max-w-4xl w-full mx-auto p-4 md:p-8 mt-16 md:mt-0">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8">AI Abandoned Cart Recovery</h1>

        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8 mb-8 relative overflow-hidden">
          <div className="max-w-2xl">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-4">Turn Lost Sales into Revenue</h2>
            <p className="text-gray-600 mb-8">
              Enable our AI-driven recovery sequence. We automatically detect when a customer leaves items in their cart and send them a personalized, high-converting email to bring them back.
            </p>

            <div className="flex items-center justify-between bg-gray-50 p-4 rounded-xl border border-gray-200">
              <div>
                <h3 className="font-bold text-gray-900">Enable AI Recovery</h3>
                <p className="text-sm text-gray-500">Automatically email customers after 1 hour of inactivity.</p>
              </div>
              <button aria-label="Enable AI Recovery"
                onClick={handleToggle}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none ${enabled ? 'bg-indigo-600' : 'bg-gray-300'}`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${enabled ? 'translate-x-6' : 'translate-x-1'}`} />
              </button>
            </div>

            {showToast && (
              <div className="mt-4 p-3 rounded-lg bg-green-50 border border-green-200 text-green-700 text-sm font-semibold">
                {enabled ? 'Success! AI Recovery Sequence is now active.' : 'AI Recovery Sequence disabled.'}
              </div>
            )}
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mb-8">
            <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
                <h3 className="text-xl font-bold font-outfit text-gray-900 mb-4">Email Preview</h3>
                <div className="bg-gray-50 p-4 rounded-xl border border-gray-200 text-sm text-gray-700 font-mono">
                  <p><strong>Subject:</strong> You left something behind! 🛒</p>
                  <p className="mt-2">Hi [Name],</p>
                  <p className="mt-2">We noticed you left some great items in your cart. Our AI has generated a special 10% discount just for you if you complete your purchase today!</p>
                  <button aria-label="Enable AI Recovery" className="mt-4 w-full bg-indigo-600 text-white font-bold py-2 rounded-lg" disabled>Return to Cart</button>
                </div>
            </div>

            <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
               <h3 className="text-xl font-bold font-outfit text-gray-900 mb-4">Recent Recoveries</h3>
               <div className="space-y-4">
                 <div className="flex justify-between items-center border-b border-gray-100 pb-2">
                   <div>
                     <p className="font-semibold text-gray-900">Sarah J.</p>
                     <p className="text-xs text-gray-500">2 hours ago</p>
                   </div>
                   <p className="font-bold text-green-600">+$124.50</p>
                 </div>
                 <div className="flex justify-between items-center border-b border-gray-100 pb-2">
                   <div>
                     <p className="font-semibold text-gray-900">Mike T.</p>
                     <p className="text-xs text-gray-500">Yesterday</p>
                   </div>
                   <p className="font-bold text-green-600">+$89.99</p>
                 </div>
                 <div className="flex justify-between items-center">
                   <div>
                     <p className="font-semibold text-gray-900">Emma W.</p>
                     <p className="text-xs text-gray-500">2 days ago</p>
                   </div>
                   <p className="font-bold text-green-600">+$210.00</p>
                 </div>
               </div>
            </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
