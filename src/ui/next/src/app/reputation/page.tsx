"use client";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

export default function ReputationDashboard() {
  const [reputation, setReputation] = useState({ average_rating: 0, total_reviews: 0 });
  const [reviews, setReviews] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  const fetchReputation = async () => {
    try {
      const tenantId = localStorage.getItem("tenant_id") || "default";
      const res = await fetch("/api/v1/growth/reputation", {
        headers: { "x-tenant-id": tenantId },
      });
      if (res.ok) {
        const data = await res.json();
        setReputation(data);
      }
    } catch (err) {
      console.error(err);
    }
  };

  const fetchReviews = async () => {
    try {
      const tenantId = localStorage.getItem("tenant_id") || "default";
      const res = await fetch("/api/v1/growth/reputation/reviews", {
        headers: { "x-tenant-id": tenantId },
      });
      if (res.ok) {
        const data = await res.json();
        if (data.reviews) {
          setReviews(data.reviews);
        }
      }
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    Promise.all([fetchReputation(), fetchReviews()]).finally(() => setLoading(false));
  }, []);

  const handleApprove = async (replyId: string) => {
    // Optimistic UI update
    setReviews(prev => prev.map(r =>
        r.reply?.id === replyId ? { ...r, reply: { ...r.reply, status: 'Approved' } } : r
    ));

    try {
      const tenantId = localStorage.getItem("tenant_id") || "default";
      const res = await fetch(`/api/v1/growth/reputation/reviews/${replyId}/approve`, {
        method: "POST",
        headers: { "x-tenant-id": tenantId },
      });

      if (!res.ok) {
        throw new Error("Failed to approve");
      }
    } catch (err) {
      console.error(err);
      // Revert optimistic update
      fetchReviews();
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: "#F5F5F7" }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", borderBottom: "1px solid rgba(255, 255, 255, 0.4)", position: "sticky", top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: "#1D1D1F", letterSpacing: "-0.02em" }}>Reputation ⭐️</h1>
        <button onClick={() => router.push("/dashboard")} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-2xl mx-auto w-full flex flex-col gap-6">
        {/* Reputation Pulse Card */}
        <div className="bg-white/80 backdrop-blur-xl rounded-[20px] shadow-sm border border-gray-200 p-6 flex flex-col md:flex-row items-center justify-between">
          <div>
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">Your Reputation Pulse</h2>
              <p className="text-sm text-gray-500">
                {reviews.filter(r => r.reply?.status === 'Drafted').length} Replies Needed
              </p>
          </div>
          {loading ? (
             <p className="text-gray-400 text-sm mt-4 md:mt-0">Loading...</p>
          ) : (
             <div className="flex flex-col items-center mt-4 md:mt-0">
                <div className="text-4xl font-black text-yellow-500">⭐️ {reputation.average_rating.toFixed(1)}</div>
                <div className="text-gray-600 font-medium text-sm">{reputation.total_reviews} Reviews</div>
             </div>
          )}
        </div>

        {/* Reviews Inbox */}
        <div className="flex flex-col gap-4">
          <h3 className="text-lg font-bold font-outfit text-gray-800">Unified Review Inbox</h3>
          {loading && <p className="text-gray-500 text-center py-8">Loading reviews...</p>}
          {!loading && reviews.length === 0 && (
             <div className="text-center py-10 bg-white/50 backdrop-blur-md rounded-[20px] border border-gray-200">
                <p className="text-gray-500">No reviews yet.</p>
             </div>
          )}
          {reviews.map(review => (
             <div key={review.id} className="bg-white/80 backdrop-blur-xl rounded-[20px] shadow-sm border border-gray-200 p-5 flex flex-col gap-4">
                 <div className="flex justify-between items-start">
                     <div className="flex items-center gap-2">
                         <div className="w-10 h-10 rounded-full bg-gray-200 flex items-center justify-center font-bold text-gray-500 uppercase text-sm">
                             {review.customer_id.substring(0, 2)}
                         </div>
                         <div>
                             <p className="font-semibold text-gray-900 text-sm">Customer {review.customer_id}</p>
                             <div className="text-yellow-500 text-xs">{"⭐️".repeat(review.rating)}</div>
                         </div>
                     </div>
                     <span className="text-xs text-gray-400">
                        {new Date(review.created_at_unix * 1000).toLocaleDateString()}
                     </span>
                 </div>

                 <p className="text-gray-700 text-sm">{review.comment}</p>

                 {review.reply && review.reply.status === 'Drafted' && (
                     <div className="mt-2 bg-indigo-50/50 border border-indigo-100 rounded-[16px] p-4 flex flex-col gap-3">
                         <div className="flex items-center gap-2">
                             <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 bg-indigo-100 px-2 py-1 rounded-md">
                                 The Publicist
                             </span>
                             <span className="text-xs text-indigo-400 font-medium">Drafted a reply</span>
                         </div>
                         <p className="text-sm text-gray-800 italic">"{review.reply.content}"</p>
                         <button
                             onClick={() => handleApprove(review.reply.id)}
                             className="w-full mt-1 py-3 bg-[#0A84FF] hover:bg-[#007AFF] text-white font-semibold rounded-xl shadow-sm transition-all"
                         >
                             Approve & Post
                         </button>
                     </div>
                 )}

                 {review.reply && review.reply.status === 'Approved' && (
                     <div className="mt-2 bg-gray-50 border border-gray-100 rounded-[16px] p-4 flex flex-col gap-2">
                         <div className="flex items-center gap-2">
                             <span className="text-xs font-bold uppercase tracking-wider text-green-600 bg-green-100 px-2 py-1 rounded-md">
                                 Replied
                             </span>
                         </div>
                         <p className="text-sm text-gray-600">"{review.reply.content}"</p>
                     </div>
                 )}
             </div>
          ))}
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
