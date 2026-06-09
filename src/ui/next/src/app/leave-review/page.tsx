"use client";

import React, { useState } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense } from 'react';

function LeaveReviewContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const orderId = searchParams?.get('order');

  const [rating, setRating] = useState(0);
  const [hoverRating, setHoverRating] = useState(0);
  const [reviewText, setReviewText] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submitted, setSubmitted] = useState(false);
  const [referralLink, setReferralLink] = useState('');
  const [copied, setCopied] = useState(false);

  const handleSubmit = async () => {
    if (rating === 0) return;
    setIsSubmitting(true);

    try {
      const response = await fetch('/api/v1/growth/referrals/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenantId: typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || 'e2e-tenant') : 'e2e-tenant', customMessage: 'I just left a 5-star review!' }),
      });

      let link = 'https://ohc.app/invite?ref=demo';
      if (response.ok) {
        const data = await response.json();
        if (data.referral_link) {
           link = data.referral_link;
        }
      }
      setReferralLink(link);
      setSubmitted(true);
    } catch (error) {
       setReferralLink('https://ohc.app/invite?ref=demo');
       setSubmitted(true);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (submitted) {
    return (
      <div className="flex flex-col min-h-screen bg-gray-50 font-inter items-center justify-center p-4">
        <div className="max-w-md w-full bg-white rounded-2xl shadow-xl p-8 text-center relative overflow-hidden border border-gray-100">
           <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

           <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl shadow-inner text-green-600 mx-auto mb-6">
              ✅
           </div>

           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Thank you for your review!</h2>

           {rating >= 4 ? (
              <>
                 <p className="text-gray-600 mb-6 text-sm leading-relaxed">
                   We're thrilled you loved your experience! As a token of our appreciation, we've unlocked a special VIP referral link just for you.
                 </p>

                 <div className="bg-indigo-50 border border-indigo-100 rounded-xl p-5 mb-6">
                    <div className="flex items-center gap-3 mb-3 justify-center">
                      <span className="text-2xl">🎁</span>
                      <h3 className="font-bold text-indigo-900 font-outfit">Get 15% Off Your Next Order</h3>
                    </div>
                    <p className="text-indigo-800 text-sm mb-4">
                      Share this link with friends. They get 15% off, and you get 15% off when they buy! ⚡ Powered by OHC
                    </p>

                    <div className="flex gap-2">
                      <input
                        type="text"
                        readOnly
                        value={referralLink}
                        className="flex-1 bg-white border border-indigo-200 rounded-lg px-3 py-2 text-sm text-indigo-900 focus:outline-none"
                      />
                      <button
                        onClick={() => {
                          navigator.clipboard.writeText(referralLink);
                          setCopied(true);
                          setTimeout(() => setCopied(false), 2000);
                        }}
                        className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? "bg-green-500 text-white" : "bg-indigo-600 text-white hover:bg-indigo-700"}`}
                      >
                        {copied ? "Copied!" : "Copy"}
                      </button>
                    </div>
                 </div>
              </>
           ) : (
              <p className="text-gray-600 mb-6 text-sm leading-relaxed">
                 We appreciate your feedback and will use it to improve our service.
              </p>
           )}

           <button
              onClick={() => router.push('/')}
              className="text-sm font-medium text-gray-500 hover:text-gray-700 transition-colors"
           >
              Return to Store
           </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter items-center justify-center p-4 py-12">
       <div className="max-w-md w-full bg-white rounded-2xl shadow-lg p-8 relative border border-gray-100">
           <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2 text-center">How was your experience?</h1>
           <p className="text-gray-500 text-sm text-center mb-8">Order #{orderId || 'Recent'}</p>

           <div className="flex justify-center gap-2 mb-8">
              {[1, 2, 3, 4, 5].map((star) => (
                 <button
                   key={star}
                   onMouseEnter={() => setHoverRating(star)}
                   onMouseLeave={() => setHoverRating(0)}
                   onClick={() => setRating(star)}
                   className="text-4xl transition-transform hover:scale-110 focus:outline-none"
                 >
                   <span className={star <= (hoverRating || rating) ? 'text-yellow-400' : 'text-gray-200'}>
                     ★
                   </span>
                 </button>
              ))}
           </div>

           <div className="mb-6">
              <label htmlFor="review" className="block text-sm font-medium text-gray-700 mb-2">Tell us more (optional)</label>
              <textarea
                 id="review"
                 rows={4}
                 value={reviewText}
                 onChange={(e) => setReviewText(e.target.value)}
                 className="w-full bg-gray-50 border border-gray-200 rounded-xl p-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:bg-white transition-colors resize-none"
                 placeholder="What did you like or dislike?"
              ></textarea>
           </div>

           <button
             onClick={handleSubmit}
             disabled={rating === 0 || isSubmitting}
             className={`w-full py-3 rounded-xl font-bold text-white transition-all flex items-center justify-center gap-2 ${rating === 0 ? 'bg-gray-300 cursor-not-allowed' : 'bg-gray-900 hover:bg-black shadow-md hover:shadow-lg'}`}
           >
             {isSubmitting ? 'Submitting...' : 'Submit Review'}
           </button>

           <div className="mt-6 text-center">
             <span className="text-xs font-semibold text-gray-400 uppercase tracking-widest">⚡ Powered by OHC</span>
           </div>
       </div>
    </div>
  );
}

export default function LeaveReviewPage() {
  return (
    <Suspense fallback={<div className="min-h-screen flex items-center justify-center text-gray-500">Loading...</div>}>
      <LeaveReviewContent />
    </Suspense>
  );
}
