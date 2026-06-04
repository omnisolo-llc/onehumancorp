"use client";

import { useState, useEffect } from "react";
import AppShell from "../components/AppShell";

export default function LocalSEO() {
  const [status, setStatus] = useState<any>(null);
  const [reviews, setReviews] = useState<any[]>([]);

  useEffect(() => {
    fetch('/api/local-seo/status')
      .then(res => res.json())
      .then(data => setStatus(data));

    fetch('/api/local-seo/reviews/pending')
      .then(res => res.json())
      .then(data => setReviews(data));
  }, []);

  const handleConnect = async () => {
    const res = await fetch('/api/local-seo/connect', { method: 'POST' });
    const data = await res.json();
    if (data.redirect_url) {
      window.location.href = data.redirect_url;
    }
  };

  const approveReply = async (reviewId: string, replyContent: string) => {
    await fetch(`/api/local-seo/reviews/${reviewId}/approve`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ reply_content: replyContent })
    });
    setReviews(reviews.filter(r => r.review_id !== reviewId));
  };

  return (
    <AppShell
      title="Google Business Profile"
      subtitle="Connect your Google Business Profile to manage hours, photos, and reviews directly from OneHumanCorp."
    >
      <div className="max-w-4xl mx-auto pb-24">

        <div className="app-panel mb-8">
          <div className="flex flex-col sm:flex-row items-center gap-6">
            <div className="flex-grow text-center sm:text-left">
              <h2 className="app-panel-title">Google Business Profile</h2>
              <p className="app-list-subtitle">Manage your local presence effectively.</p>
            </div>
            {!status?.connected ? (
              <button
                onClick={handleConnect}
                className="app-button"
              >
                Connect Profile
              </button>
            ) : (
              <div className="app-badge good">
                Connected
              </div>
            )}
          </div>
        </div>

        {status?.connected && (
          <div className="space-y-6">
            <h3 className="app-panel-title">Pending Reviews</h3>
            {reviews.length === 0 ? (
              <div className="app-panel p-8 text-center">
                <p className="app-empty">All caught up! No pending reviews.</p>
              </div>
            ) : (
              <div className="grid gap-4">
                {reviews.map(review => (
                  <div key={review.review_id} className="app-panel">
                    <div className="flex justify-between items-start mb-4">
                      <div>
                        <h4 className="font-bold text-gray-900">{review.reviewer_name}</h4>
                        <div className="flex text-yellow-400 mt-1">
                          {'★'.repeat(review.star_rating) + '☆'.repeat(5 - review.star_rating)}
                        </div>
                      </div>
                    </div>
                    <p className="text-gray-700 mb-6">{review.comment}</p>

                    {review.ai_draft_reply && (
                      <div className="bg-blue-50/50 border border-blue-100 rounded-xl p-4 mt-4">
                        <div className="flex items-center gap-2 mb-2">
                          <span className="text-xs font-bold text-blue-600 uppercase tracking-wider">AI Draft Reply</span>
                        </div>
                        <p className="text-sm text-gray-700 mb-4">{review.ai_draft_reply}</p>
                        <div className="flex gap-2">
                          <button
                            onClick={() => approveReply(review.review_id, review.ai_draft_reply)}
                            className="app-button"
                          >
                            Approve & Post
                          </button>
                        </div>
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </AppShell>
  );
}
