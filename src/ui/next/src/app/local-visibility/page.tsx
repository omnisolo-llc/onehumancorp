'use client';

import React, { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function LocalVisibilityContent() {
  const searchParams = useSearchParams();
  const tenant = searchParams.get('tenant') || 'test-tenant';
  const [isConnected, setIsConnected] = useState(false);
  const [showApproval, setShowApproval] = useState(false);

  useEffect(() => {
    const connected = localStorage.getItem(`google_connected_${tenant}`);
    if (connected === 'true') {
      setIsConnected(true);
    }
  }, [tenant]);

  const handleConnect = async () => {
    localStorage.setItem(`google_connected_${tenant}`, 'true');
    window.location.href = `/oauth/google/connect?tenant_id=${tenant}`;
  };

  const simulateReview = async () => {
    await fetch('/api/v1/syndication/webhook', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        tenant_id: tenant,
        event_type: 'google.review.created',
        data: {
          review: 'Carlos did a great job fixing my sink.',
          rating: 5,
          author: 'John Doe',
          platform: 'Google'
        }
      })
    });
    setShowApproval(true);
  };

  const approveReply = async () => {
    await fetch('/api/agents/approvals/approve', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        approval_id: "mock_approval_id",
        tenant_id: tenant,
        payload_overrides: { generated_response: "Thank you for the 5-star review! We appreciate your business." }
      })
    });
    setShowApproval(false);
  };

  return (
    <div className="min-h-screen bg-gray-50 pb-20">
      <header className="bg-white/70 backdrop-blur-md sticky top-0 z-10 border-b border-gray-100 p-4">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Local Visibility</h1>
      </header>

      <main className="p-4 space-y-6 max-w-md mx-auto">
        <div className="bg-white/80 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-sm border border-gray-200">
          <h2 className="text-lg font-bold text-gray-900 mb-2 font-outfit">Google Business Profile</h2>
          {isConnected ? (
            <div>
              <p className="text-sm text-green-700 font-medium mb-4 flex items-center">
                <span className="mr-2">🟢</span> Synced with Google Maps
              </p>
              <button
                onClick={simulateReview}
                className="w-full py-3 bg-gray-100 text-gray-800 rounded-xl font-medium"
              >
                Simulate New Review
              </button>
            </div>
          ) : (
            <div>
              <p className="text-sm text-gray-600 mb-4 leading-relaxed">
                Connect your Google Business profile to automatically sync your hours, catalog, and reply to reviews with AI.
              </p>
              <button
                id="connect-google-btn"
                onClick={handleConnect}
                className="w-full min-h-[44px] bg-blue-600 text-white rounded-xl font-medium shadow-sm active:scale-95 transition-transform"
              >
                Connect Google Business
              </button>
            </div>
          )}
        </div>

        {showApproval && (
          <div id="review-approval-card" className="bg-white/80 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl shadow-sm border border-gray-200">
            <div className="flex justify-between items-start mb-4">
              <div>
                <h3 className="text-md font-bold text-gray-900 font-outfit">John Doe</h3>
                <p className="text-yellow-500 text-sm">★★★★★</p>
              </div>
              <span className="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded-full font-medium">Google</span>
            </div>
            <p className="text-sm text-gray-700 italic mb-4">"Carlos did a great job fixing my sink."</p>

            <div className="bg-gray-50 rounded-xl p-4 border border-gray-100">
              <p className="text-xs font-bold text-gray-500 uppercase tracking-wider mb-2">AI Drafted Reply</p>
              <p className="text-sm text-gray-800">"Thank you for the 5-star review! We appreciate your business."</p>
            </div>

            <div className="mt-4 flex gap-3">
              <button
                onClick={approveReply}
                className="flex-1 min-h-[44px] bg-green-600 text-white rounded-xl font-medium shadow-sm active:scale-95 transition-transform"
              >
                Approve & Reply
              </button>
              <button className="px-4 min-h-[44px] bg-gray-100 text-gray-700 rounded-xl font-medium">
                Edit
              </button>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default function LocalVisibilityPage() {
  return (
    <Suspense fallback={<div className="p-4">Loading Local Visibility...</div>}>
      <LocalVisibilityContent />
    </Suspense>
  );
}
