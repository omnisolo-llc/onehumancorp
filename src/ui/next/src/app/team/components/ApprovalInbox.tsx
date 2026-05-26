"use client";

import React, { useState } from "react";
import { ApprovalRequest } from "../page";

type Props = {
  departmentId: string;
  departmentName: string;
  approvals: ApprovalRequest[];
  onBack: () => void;
  onApprove: (id: string) => void;
  onReject: (id: string) => void;
};

export default function ApprovalInbox({
  departmentId,
  departmentName,
  approvals,
  onBack,
  onApprove,
  onReject,
}: Props) {
  const [reviewAll, setReviewAll] = useState(true);
  const [selectedReview, setSelectedReview] = useState<ApprovalRequest | null>(null);

  const handleToggle = async () => {
    const newValue = !reviewAll;
    setReviewAll(newValue);
    try {
      await fetch(`/api/agents/settings/${departmentId}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tone_of_voice: "Friendly",
          auto_approve_limits: newValue ? 0.0 : 1000.0,
        }),
      });
    } catch (e) {
      console.error(e);
      setReviewAll(!newValue); // Revert on failure
    }
  };

  const extractPayload = (description: string) => {
    const parts = description.split(" | Payload: ");
    if (parts.length > 1) {
      try {
        return { desc: parts[0], payload: JSON.parse(parts[1]) };
      } catch (e) {
        return { desc: parts[0], payload: null };
      }
    }
    return { desc: description, payload: null };
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button
            onClick={onBack}
            className="w-10 h-10 flex items-center justify-center rounded-full bg-white shadow-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
          >
            <svg
              className="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">
              {departmentName}
            </h1>
            <p className="text-gray-500 text-xs font-medium uppercase tracking-wider mt-1">
              Approval Inbox
            </p>
          </div>
        </div>


        {/* Settings Toggle */}
        <div className="px-6 py-4 bg-white/40 border-b border-white/40 flex items-center justify-between">
          <span className="text-sm font-medium text-gray-700">Review all messages before sending</span>
          <button
            onClick={handleToggle}
            className={`w-12 h-6 rounded-full p-1 transition-colors flex ${reviewAll ? 'bg-blue-500 justify-end' : 'bg-gray-300 justify-start'}`}
          >
            <div className={`w-4 h-4 bg-white rounded-full transition-transform`} />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 space-y-4 hide-scrollbar">
          {approvals.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center px-8">
              <div className="w-16 h-16 bg-green-50 text-green-500 rounded-full flex items-center justify-center mb-4">
                <svg
                  className="w-8 h-8"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
              </div>
              <h3 className="font-outfit font-bold text-gray-900 text-lg mb-2">
                All Caught Up!
              </h3>
              <p className="text-sm text-gray-500">
                There are no pending actions requiring your review.
              </p>
            </div>
          ) : (
            approvals.map((req) => {
              const { desc, payload } = extractPayload(req.description);
              return (

              <div
                key={req.id}
                className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] transition-all duration-300"
              >
                <div className="flex items-center gap-2 mb-3">
                  <span
                    className={`px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider ${
                      req.action_risk.toLowerCase() === "high"
                        ? "bg-orange-100 text-orange-700"
                        : "bg-blue-100 text-blue-700"
                    }`}
                  >
                    {req.action_risk} Risk
                  </span>
                  <span className="text-xs text-gray-400 font-medium">
                    {req.status}
                  </span>
                </div>

                <p className="text-gray-800 text-sm leading-relaxed mb-6 font-medium">
                  {desc}
                </p>

                {req.payload?.feature_type === "ambassador_reply" && (
                  <div className="mb-6 p-4 rounded-xl bg-blue-50 border border-blue-100 flex flex-col gap-3">
                    <div className="flex items-center gap-2 text-blue-800 font-semibold text-sm">
                      <svg
                        className="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"
                        />
                      </svg>
                      Customer Inquiry
                    </div>

                    <div className="bg-white p-3 rounded-lg border border-blue-100 text-xs text-gray-700 italic">
                      "{req.payload.original_message}"
                    </div>

                    <div className="text-blue-800 font-semibold text-sm mt-2 flex items-center gap-2">
                       <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                           <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                       </svg>
                       AI Draft
                    </div>
                    <div className="bg-blue-600 p-3 rounded-lg text-xs text-white shadow-inner">
                      {req.payload.generated_response}
                    </div>
                  </div>
                )}

                {req.payload?.feature_type === "legal_compliance" && (
                  <div className="mb-6 p-4 rounded-xl bg-orange-50 border border-orange-100 flex flex-col gap-3">
                    <div className="flex items-center gap-2 text-orange-800 font-semibold text-sm">
                      <svg
                        className="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        />
                      </svg>
                      Compliance Warning
                    </div>
                    <div className="text-xs text-orange-700">
                      Sales are approaching €10,000. New tax rules require an
                      updated Privacy Policy.
                    </div>
                    <div className="bg-white p-3 rounded-lg border border-orange-100 text-xs text-gray-600">
                      Drafting updated European privacy policy...
                    </div>
                  </div>
                )}

                {req.payload?.feature_type === "global_localization" && (
                  <div className="mb-6 p-4 rounded-xl bg-indigo-50 border border-indigo-100 flex flex-col gap-3">
                    <div className="flex items-center justify-between text-indigo-800 font-semibold text-sm">
                      <div className="flex items-center gap-2">
                        <svg
                          className="w-5 h-5"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129"
                          />
                        </svg>
                        Global Reach Preview
                      </div>
                      <span className="text-[10px] bg-indigo-100 px-2 py-0.5 rounded">
                        Spanish
                      </span>
                    </div>
                    <div className="grid grid-cols-2 gap-2 text-xs">
                      <div className="bg-white p-2 rounded border border-indigo-50">
                        <span className="text-gray-400 block mb-1">
                          Original (EN)
                        </span>
                        <div>
                          Vegan Cake
                          <br />
                          $25.00
                        </div>
                      </div>
                      <div className="bg-white p-2 rounded border border-indigo-100 ring-1 ring-indigo-500/20">
                        <span className="text-indigo-400 block mb-1">
                          Preview (ES)
                        </span>
                        <div>
                          Pastel Vegano
                          <br />
                          €23.50
                        </div>
                      </div>
                    </div>
                  </div>
                )}

                {req.payload?.feature_type === "ai_geo" && (
                  <div className="mb-6 p-4 rounded-xl bg-emerald-50 border border-emerald-100 flex flex-col gap-3">
                    <div className="flex items-center gap-2 text-emerald-800 font-semibold text-sm">
                      <svg
                        className="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                        />
                      </svg>
                      Smart Search Setup
                    </div>
                    <div className="text-xs text-emerald-700">
                      Updating your store's information so it can be easily
                      found by AI search tools like ChatGPT.
                    </div>
                    <div className="flex gap-2 text-[10px] text-emerald-600 mt-1">
                      <span className="bg-emerald-100 px-2 py-1 rounded">
                        Smart Formatting
                      </span>
                      <span className="bg-emerald-100 px-2 py-1 rounded">
                        Search Engine Data
                      </span>
                      <span className="bg-emerald-100 px-2 py-1 rounded">
                        Answer Formatting
                      </span>
                    </div>
                  </div>
                )}

                {req.payload?.feature_type === "social_calendar" && (
                  <div className="mb-6 p-4 rounded-xl bg-purple-50 border border-purple-100 flex flex-col gap-3">
                    <div className="flex items-center gap-2 text-purple-800 font-semibold text-sm">
                      <svg
                        className="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                        />
                      </svg>
                      7-Day Social Calendar Generated
                    </div>
                    <div className="text-xs text-purple-700">
                      The Generative Promoter has created a week of content
                      based on your new product.
                    </div>

                    <div className="flex gap-2 overflow-x-auto pb-2 hide-scrollbar">
                      {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map(
                        (day, idx) => (
                          <div
                            key={day}
                            className="flex-shrink-0 w-24 bg-white rounded-lg border border-purple-100 p-2 shadow-sm"
                          >
                            <div className="text-[10px] font-bold text-gray-400 uppercase mb-1">
                              {day}
                            </div>
                            <div className="w-full h-16 bg-gray-100 rounded mb-1 flex items-center justify-center text-xl">
                              {["📸", "✨", "🎂", "🎉", "🌟", "🛍️", "🔥"][idx]}
                            </div>
                            <div className="text-[8px] text-gray-500 leading-tight line-clamp-2">
                              {
                                [
                                  "New flavor drop!",
                                  "Behind the scenes",
                                  "Customer favorite",
                                  "Special discount",
                                  "Weekend vibes",
                                  "Shop local",
                                  "Sunday showcase",
                                ][idx]
                              }
                            </div>
                          </div>
                        ),
                      )}
                    </div>
                  </div>
                )}

                {req.payload?.feature_type === "abandoned_cart" && (
                  <div className="mb-6 p-4 rounded-xl bg-rose-50 border border-rose-100 flex flex-col gap-3">
                    <div className="flex items-center gap-2 text-rose-800 font-semibold text-sm">
                      <svg
                        className="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"
                        />
                      </svg>
                      Abandoned Cart Detected
                    </div>
                    <div className="text-xs text-rose-700 font-medium">
                      Sarah left a $45 Vegan Chocolate Cake in her cart.
                    </div>

                    <div className="bg-white p-3 rounded-lg border border-rose-100 relative">
                      <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 absolute top-2 right-2">
                        AI Draft
                      </div>
                      <p className="text-xs text-gray-700 italic">
                        "Hi Sarah, noticed you left the Vegan Chocolate Cake in
                        your cart! Here's 10% off to sweeten the deal if you
                        finish your order today."
                      </p>
                    </div>

                    <div className="flex gap-2 mt-1">
                      <span className="text-[10px] bg-rose-100 text-rose-700 px-2 py-1 rounded font-medium">
                        Margin Safe: 10% Discount
                      </span>
                      <span className="text-[10px] bg-gray-100 text-gray-600 px-2 py-1 rounded font-medium">
                        SMS
                      </span>
                    </div>
                  </div>
                )}

                <div className="flex gap-3">
                  <button
                    onClick={() => {
                      if (payload && payload.original_message) {
                        setSelectedReview(req);
                      } else {
                        onReject(req.id);
                      }
                    }}
                    className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all min-h-[44px]"
                  >
                    {payload && payload.original_message ? 'Review' : 'Reject / Edit'}
                  </button>
                  <button
                    onClick={() => onApprove(req.id)}
                    className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all min-h-[44px]"
                  >
                    Approve
                  </button>
                </div>
              </div>
            );
          })
          )}
        </div>

        {/* Review Modal */}
        {selectedReview && (
          <div className="absolute inset-0 bg-black/40 z-50 flex flex-col justify-end">
            <div
              className="bg-white rounded-t-3xl p-6 shadow-2xl transition-transform duration-300"
              style={{ animation: 'slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}
            >
              <h2 className="text-xl font-bold mb-4 font-outfit text-gray-900">Review Draft</h2>

              <div className="mb-4">
                <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">Context</p>
                <div className="bg-gray-50 p-3 rounded-xl border border-gray-100 text-sm text-gray-700">
                   {extractPayload(selectedReview.description).payload?.original_message || "N/A"}
                </div>
              </div>

              <div className="mb-6">
                <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">Draft</p>
                <div className="bg-blue-50 p-3 rounded-xl border border-blue-100 text-sm text-gray-800 italic relative">
                  {extractPayload(selectedReview.description).payload?.generated_response || "N/A"}
                </div>
              </div>

              <div className="flex gap-3">
                <button
                  onClick={() => {
                    onReject(selectedReview.id);
                    setSelectedReview(null);
                  }}
                  className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 min-h-[44px]"
                >
                  Discard
                </button>
                <button
                  onClick={() => {
                    setSelectedReview(null);
                  }}
                  className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 min-h-[44px]"
                >
                  Edit
                </button>
                <button
                  onClick={() => {
                    onApprove(selectedReview.id);
                    setSelectedReview(null);
                  }}
                  className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 min-h-[44px]"
                >
                  Send Now
                </button>
              </div>
            </div>
          </div>
        )}
        <style dangerouslySetInnerHTML={{__html: `
          @keyframes slideUp {
            from { transform: translateY(100%); }
            to { transform: translateY(0); }
          }
        `}} />
      </div>
    </div>

  );
}
