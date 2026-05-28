"use client";

import { useState } from "react";
import Link from "next/link";

type Review = {
  id: string;
  author: string;
  rating: number;
  content: string;
  date: string;
  source: string;
  icon: string;
  status: "pending" | "replied";
  draft?: string;
  publishedReply?: string;
};

export default function ReputationPage() {
  const [reviews, setReviews] = useState<Review[]>([
    {
      id: "rev-1",
      author: "Sarah Jenkins",
      rating: 5,
      content: "Absolutely wonderful service! Carlos was very professional and fixed my car in no time.",
      date: "2 days ago",
      source: "Google Business Profile",
      icon: "🌐",
      status: "pending",
      draft: "Hi Sarah! Thank you so much for the 5-star review. We're thrilled to hear that Carlos was able to get your car fixed quickly and professionally. We appreciate your business and hope to see you again for any future auto repair needs!"
    },
    {
      id: "rev-2",
      author: "Mike Thompson",
      rating: 4,
      content: "Good work on the repairs, but the waiting room was a bit small.",
      date: "1 week ago",
      source: "Google Business Profile",
      icon: "🌐",
      status: "replied",
      publishedReply: "Hi Mike, thanks for your feedback and the 4-star rating! We're glad you were satisfied with the repair work. We understand the waiting area is cozy, and we're looking into ways to make it more comfortable for our customers. We hope to serve you again!"
    }
  ]);

  const [editingId, setEditingId] = useState<string | null>(null);
  const [replyInput, setReplyInput] = useState("");

  const handleSendReply = (reviewId: string) => {
    let contentToSend = replyInput;
    const review = reviews.find(r => r.id === reviewId);

    if (!contentToSend && review && review.draft) {
      contentToSend = review.draft;
    }

    if (!contentToSend) return;

    setReviews(prev => prev.map(r =>
      r.id === reviewId ? { ...r, status: "replied", publishedReply: contentToSend, draft: undefined } : r
    ));
    setEditingId(null);
    setReplyInput("");
  };

  const startEditing = (reviewId: string, initialText: string) => {
    setEditingId(reviewId);
    setReplyInput(initialText);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>

      {/* Premium Dashboard Header */}
      <div className="bg-gradient-to-r from-gray-900 to-black text-white px-6 py-8 shadow-md">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div>
            <Link href="/dashboard" className="text-gray-400 hover:text-white text-sm font-semibold mb-4 inline-block transition-colors">
              &larr; Back to Dashboard
            </Link>
            <div className="flex items-center gap-2 mb-2">
              <span className="bg-gradient-to-r from-yellow-300 to-yellow-500 text-black text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wide">Premium</span>
            </div>
            <h1 className="text-3xl font-bold font-outfit mb-1">Reputation Management</h1>
            <p className="text-gray-400 text-sm">Manage public reviews and respond with AI-drafted messages.</p>
          </div>
          <div className="hidden md:flex w-16 h-16 bg-white/10 rounded-2xl border border-white/20 items-center justify-center text-3xl">
            ⭐
          </div>
        </div>
      </div>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full">

        {/* Reviews List */}
        <div className="space-y-6">
          {reviews.map(review => (
            <div key={review.id} className="rounded-[16px] p-6 shadow-sm flex flex-col transition-shadow hover:shadow-md"
                 style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>

              {/* Review Header */}
              <div className="flex justify-between items-start mb-4">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center text-lg shadow-inner">
                    👤
                  </div>
                  <div>
                    <h3 className="font-bold text-gray-900 font-outfit">{review.author}</h3>
                    <div className="flex items-center gap-2 text-sm text-gray-500">
                      <span className="flex text-yellow-400">
                        {Array(review.rating).fill('★').join('')}{Array(5 - review.rating).fill('☆').join('')}
                      </span>
                      <span>•</span>
                      <span>{review.date}</span>
                      <span>•</span>
                      <span className="flex items-center gap-1">{review.icon} {review.source}</span>
                    </div>
                  </div>
                </div>
                <span className={`text-xs font-bold px-2 py-1 rounded-md uppercase tracking-wide ${
                  review.status === 'replied' ? 'bg-green-100 text-green-700' : 'bg-orange-100 text-orange-700'
                }`}>
                  {review.status}
                </span>
              </div>

              {/* Review Content */}
              <p className="text-gray-800 text-sm leading-relaxed mb-6 bg-white p-4 rounded-xl border border-gray-100 shadow-sm">
                "{review.content}"
              </p>

              {/* Reply Section */}
              {review.status === 'replied' && review.publishedReply ? (
                <div className="bg-gray-50 border border-gray-200 rounded-xl p-4 ml-8 relative shadow-sm">
                  <div className="absolute -left-3 top-4 text-gray-300 text-2xl">↳</div>
                  <div className="flex items-center gap-2 mb-2">
                     <span className="text-sm font-bold text-gray-900">Your Reply</span>
                     <span className="text-xs text-gray-500">Published</span>
                  </div>
                  <p className="text-sm text-gray-700">{review.publishedReply}</p>
                </div>
              ) : review.draft ? (
                <div className="bg-[#f9f5ff] border border-[#e9d8fd] rounded-xl p-5 ml-8 relative shadow-sm">
                   <div className="absolute -left-3 top-4 text-[#d6bcfa] text-2xl">↳</div>
                   <div className="absolute -top-3 left-4 bg-[#e9d8fd] text-[#553c9a] text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wide flex items-center gap-1">
                      <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                      AI Suggested Reply
                   </div>

                   {editingId === review.id ? (
                      <div className="mt-2">
                        <textarea
                          value={replyInput}
                          onChange={e => setReplyInput(e.target.value)}
                          className="w-full border border-[#d6bcfa] rounded p-3 text-sm text-gray-900 bg-white focus:outline-none focus:ring-2 focus:ring-[#9f7aea] shadow-sm"
                          rows={4}
                        />
                        <div className="flex justify-end mt-3 gap-2">
                           <button onClick={() => setEditingId(null)} className="text-sm font-semibold text-gray-500 hover:text-gray-700 px-4 py-2">Cancel</button>
                           <button onClick={() => handleSendReply(review.id)} className="bg-[#805ad5] text-white text-sm font-bold px-6 py-2 rounded-lg shadow-sm hover:bg-[#6b46c1] transition-colors">Publish Reply</button>
                        </div>
                      </div>
                   ) : (
                      <>
                        <p className="text-sm text-gray-800 mt-2 italic font-medium">"{review.draft}"</p>
                        <div className="flex gap-3 mt-4 pt-4 border-t border-[#e9d8fd]/50">
                           <button onClick={() => handleSendReply(review.id)} className="flex-1 bg-[#805ad5] text-white font-bold py-2.5 rounded-lg text-sm shadow-sm hover:bg-[#6b46c1] transition-colors flex items-center justify-center gap-2">
                               <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                               Approve & Publish
                           </button>
                           <button onClick={() => startEditing(review.id, review.draft || "")} className="flex-1 bg-white text-[#805ad5] border border-[#d6bcfa] font-bold py-2.5 rounded-lg text-sm shadow-sm hover:bg-gray-50 transition-colors flex items-center justify-center gap-2">
                               <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" /></svg>
                               Edit Reply
                           </button>
                        </div>
                      </>
                   )}
                </div>
              ) : null}

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
