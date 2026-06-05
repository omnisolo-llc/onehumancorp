"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

interface ReputationData {
  average_rating: number;
  total_reviews: number;
}

interface Review {
  id: string;
  customer_id: string;
  order_id: string;
  rating: number;
  comment: string;
  reply_text: string;
}

export default function ReputationPage() {
  const [reputation, setReputation] = useState<ReputationData | null>(null);
  const [reviews, setReviews] = useState<Review[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const tenantId = typeof window !== "undefined" ? localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default" : "default";

  useEffect(() => {
    let mounted = true;

    async function fetchData() {
      try {
        const [repRes, revRes] = await Promise.all([
          fetch("/api/v1/growth/reputation", {
            headers: { "x-tenant-id": tenantId }
          }),
          fetch("/api/v1/growth/reputation/reviews", {
            headers: { "x-tenant-id": tenantId }
          })
        ]);

        if (repRes.ok && revRes.ok) {
          const repData = await repRes.json();
          const revData = await revRes.json();
          if (mounted) {
            setReputation(repData);
            setReviews(revData.reviews || []);
          }
        } else {
          throw new Error("Failed to load reputation data");
        }
      } catch (err: any) {
        if (mounted) setError(err.message);
      } finally {
        if (mounted) setLoading(false);
      }
    }

    fetchData();
    return () => { mounted = false; };
  }, [tenantId]);

  return (
    <AppShell title="Reputation Pulse" subtitle="Monitor and manage your business reviews.">
      <main className="max-w-5xl mx-auto py-4 px-4" aria-label="Reputation Management">

        {error && (
          <div className="w-full mb-6 p-4 mac-glass-container rounded-[16px] border border-red-500/50 bg-red-500/10 text-red-500 text-center">
            {error}
          </div>
        )}

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          {/* Reputation Pulse Card */}
          <div className="md:col-span-1 mac-glass-container p-6 rounded-[16px] border border-white/40 dark:border-white/10 flex flex-col justify-center items-center">
            <h2 className="text-lg font-semibold text-gray-700 dark:text-gray-300 mb-2">Overall Rating</h2>
            {loading ? (
              <div className="animate-pulse w-24 h-12 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
            ) : (
              <div className="text-5xl font-bold text-gray-900 dark:text-white mb-2 flex items-center gap-2">
                <span>{reputation?.average_rating.toFixed(1) || "0.0"}</span>
                <span className="text-yellow-400 text-4xl">★</span>
              </div>
            )}
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Based on {reputation?.total_reviews || 0} reviews
            </p>
          </div>

          <div className="md:col-span-2 mac-glass-container p-6 rounded-[16px] border border-white/40 dark:border-white/10 flex flex-col justify-center">
             <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Automation Settings</h2>
             <div className="flex items-center justify-between mb-3">
               <div>
                 <p className="font-medium text-gray-900 dark:text-white">Auto-Request Reviews</p>
                 <p className="text-sm text-gray-500">Send an SMS/Email request 24 hours after order completion.</p>
               </div>
               <div className="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                  <input type="checkbox" name="toggle" id="toggle1" checked readOnly className="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer border-green-500 translate-x-6"/>
                  <label htmlFor="toggle1" className="toggle-label block overflow-hidden h-6 rounded-full bg-green-500 cursor-pointer"></label>
              </div>
             </div>
             <div className="flex items-center justify-between">
               <div>
                 <p className="font-medium text-gray-900 dark:text-white">The Publicist Drafts</p>
                 <p className="text-sm text-gray-500">Automatically draft context-aware replies for new reviews.</p>
               </div>
               <div className="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                  <input type="checkbox" name="toggle" id="toggle2" checked readOnly className="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer border-green-500 translate-x-6"/>
                  <label htmlFor="toggle2" className="toggle-label block overflow-hidden h-6 rounded-full bg-green-500 cursor-pointer"></label>
              </div>
             </div>
          </div>
        </div>

        {/* Unified Review Inbox */}
        <div className="mac-glass-container p-6 rounded-[16px] border border-white/40 dark:border-white/10">
          <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-6">Review Inbox</h2>

          {loading ? (
            <div className="text-center py-8 text-gray-500">Loading inbox...</div>
          ) : reviews.length === 0 ? (
            <div className="text-center py-12">
              <div className="text-4xl mb-4">📬</div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">No reviews yet</h3>
              <p className="text-gray-500">When customers leave reviews, they will appear here.</p>
            </div>
          ) : (
            <div className="space-y-4">
              {reviews.map((review) => (
                <div key={review.id} className="p-4 bg-white/50 dark:bg-gray-800/50 rounded-xl border border-gray-200 dark:border-gray-700">
                  <div className="flex justify-between items-start mb-2">
                    <div>
                      <div className="flex items-center gap-1 mb-1">
                        {Array.from({ length: 5 }).map((_, i) => (
                          <span key={i} className={`text-lg ${i < review.rating ? 'text-yellow-400' : 'text-gray-300'}`}>★</span>
                        ))}
                      </div>
                      <p className="text-sm font-medium text-gray-900 dark:text-white">Customer: {review.customer_id}</p>
                    </div>
                  </div>
                  <p className="text-gray-700 dark:text-gray-300 italic">"{review.comment}"</p>

                  {review.reply_text ? (
                    <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg border-l-4 border-blue-500">
                      <p className="text-xs font-semibold text-blue-800 dark:text-blue-300 mb-1">Your Reply (Published)</p>
                      <p className="text-sm text-gray-800 dark:text-gray-200">{review.reply_text}</p>
                    </div>
                  ) : (
                     <div className="mt-4 flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                       <span className="text-sm text-gray-500">No reply published yet.</span>
                       <span className="text-xs px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded text-gray-600 dark:text-gray-300">
                          Check Agent Feed for drafts
                       </span>
                     </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </main>
    </AppShell>
  );
}
