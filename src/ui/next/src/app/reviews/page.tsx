"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import Link from "next/link";

type LocalReview = {
  review_id: string;
  reviewer_name: string;
  star_rating: number;
  comment?: string;
  ai_draft_reply?: string;
  reply_status: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function ReviewsPage() {
  const [reviews, setReviews] = useState<LocalReview[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [approving, setApproving] = useState<string | null>(null);

  useEffect(() => {
    async function loadReviews() {
      setLoading(true);
      try {
        const token = localStorage.getItem("token") || "";
        const res = await fetch(`/api/v1/local_seo/reviews/pending?tenant_id=${encodeURIComponent(tenantId())}`, {
          headers: { "Authorization": `Bearer ${token}` }
        });
        if (!res.ok) throw new Error("Failed to load pending reviews");
        const data = await res.json();
        setReviews(data);
      } catch (e: any) {
        setError(e?.message || "Failed to load reviews");
      } finally {
        setLoading(false);
      }
    }
    loadReviews();
  }, []);

  async function handleApprove(reviewId: string, replyContent: string) {
    try {
      setApproving(reviewId);
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/v1/local_seo/reviews/${reviewId}/approve`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({ reply_content: replyContent })
      });
      if (res.ok) {
        setReviews(reviews.filter((r) => r.review_id !== reviewId));
      } else {
        alert("Failed to approve reply.");
      }
    } catch (e) {
      console.error(e);
      alert("Error approving reply.");
    } finally {
      setApproving(null);
    }
  }

  const averageRating = reviews.length > 0
    ? (reviews.reduce((acc, curr) => acc + curr.star_rating, 0) / reviews.length).toFixed(1)
    : "N/A";

  return (
    <AppShell
      title="Reputation Inbox"
      subtitle="Manage your public reviews and let The Publicist draft replies."
      statusItems={[
        { label: "Pending Replies", value: String(reviews.length), tone: reviews.length > 0 ? "warn" : "good" },
        { label: "Avg Rating", value: averageRating, tone: "good" }
      ]}
    >
      <div className="flex flex-col gap-6">
        {loading && <div className="p-8 text-center text-gray-500">Loading reviews...</div>}
        {error && <div className="p-8 text-center text-red-500">{error}</div>}

        {!loading && !error && reviews.length === 0 && (
          <div className="mac-glass-container p-12 text-center rounded-[16px] border border-white/40 dark:border-white/10">
            <div className="text-4xl mb-4">🌟</div>
            <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white">All Caught Up!</h3>
            <p className="text-gray-500 mt-2">There are no pending reviews needing your attention.</p>
          </div>
        )}

        {!loading && reviews.map((review) => (
          <div key={review.review_id} className="mac-glass-container p-6 rounded-[16px] border border-white/40 dark:border-white/10 flex flex-col md:flex-row gap-6 shadow-sm">
            <div className="flex-1">
              <div className="flex items-center justify-between mb-2">
                <span className="font-bold text-gray-900 dark:text-white">{review.reviewer_name}</span>
                <span className="bg-yellow-100 text-yellow-800 text-xs font-semibold px-2.5 py-0.5 rounded dark:bg-yellow-900 dark:text-yellow-300">
                  {review.star_rating} Stars
                </span>
              </div>
              <p className="text-gray-700 dark:text-gray-300 italic mb-4">"{review.comment || "No comment provided."}"</p>

              <div className="bg-gray-50 dark:bg-gray-800/50 p-4 rounded-xl border border-gray-100 dark:border-gray-700">
                <div className="flex items-center gap-2 mb-2 text-sm text-indigo-600 dark:text-indigo-400 font-semibold">
                  <span>✨</span> The Publicist drafted:
                </div>
                <p className="text-gray-800 dark:text-gray-200">
                  {review.ai_draft_reply || "Working on a draft..."}
                </p>
              </div>
            </div>

            <div className="flex flex-col justify-end min-w-[140px]">
              <button
                disabled={approving === review.review_id || !review.ai_draft_reply}
                onClick={() => handleApprove(review.review_id, review.ai_draft_reply || "")}
                className="w-full min-h-[44px] px-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-medium rounded-xl transition-colors disabled:opacity-50"
              >
                {approving === review.review_id ? "Publishing..." : "Approve & Post"}
              </button>
            </div>
          </div>
        ))}
      </div>
    </AppShell>
  );
}
