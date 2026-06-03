"use client";

import React, { useState } from "react";

type ActionRisk = "LOW" | "HIGH";
type ApprovalStatus = "PendingApproval" | "Approved" | "Rejected";

export interface ApprovalRequest {
  id: string;
  department: string;
  description: string;
  status: ApprovalStatus;
  action_risk: ActionRisk;
  payload?: any;
}

type Props = {
  departmentId: string;
  departmentName: string;
  approvals: ApprovalRequest[];
  onApprove: (id: string) => void;
  onReject: (id: string) => void;
  onBack: () => void;
};

function extractPayload(description: string): {
  type: string;
  payload: any;
} {
  try {
    const parts = description.split("PAYLOAD:");
    if (parts.length > 1) {
      const jsonStr = parts[1].trim();
      const payload = JSON.parse(jsonStr);
      return { type: payload.feature_type || "unknown", payload };
    }
  } catch (e) {
    console.error("Failed to parse action payload", e);
  }
  return { type: "unknown", payload: null };
}

export default function ApprovalInbox({
  departmentId,
  departmentName,
  approvals,
  onApprove,
  onReject,
  onBack,
}: Props) {
  const [selectedReview, setSelectedReview] = useState<ApprovalRequest | null>(
    null,
  );

  return (
    <div className="absolute inset-0 z-20 flex flex-col h-full bg-[#f8fafc]">
      <div className="flex-1 overflow-y-auto">
        {/* Header */}
        <div className="sticky top-0 z-30 bg-[#f8fafc]/90 backdrop-blur-xl border-b border-black/5 px-4 py-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button
              onClick={onBack}
              className="w-10 h-10 flex items-center justify-center rounded-full bg-white border border-gray-200 shadow-sm active:scale-95 transition-all text-gray-600 hover:text-gray-900"
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
                  strokeWidth={2.5}
                  d="M15 19l-7-7 7-7"
                />
              </svg>
            </button>
            <div>
              <h1 className="text-xl font-bold font-outfit text-gray-900 leading-tight">
                {departmentName}
              </h1>
              <p className="text-xs font-medium text-gray-500 uppercase tracking-wider">
                Action Inbox
              </p>
            </div>
          </div>
          <div className="bg-orange-100 text-orange-700 px-3 py-1 rounded-full text-sm font-bold shadow-sm border border-orange-200">
            {approvals.length} Action{approvals.length !== 1 ? "s" : ""}
          </div>
        </div>

        {/* Content */}
        <div className="p-4 space-y-4 pb-32">
          {approvals.length === 0 ? (
            <div className="text-center py-20 px-6">
              <div className="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6 shadow-inner border border-green-200">
                <svg
                  className="w-10 h-10"
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
              <h3 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                All caught up!
              </h3>
              <p className="text-gray-500 text-sm leading-relaxed">
                Your AI team is running smoothly in the background. No manual
                approvals needed.
              </p>
              <button
                onClick={onBack}
                className="mt-8 py-3 px-6 bg-white border border-gray-200 rounded-xl shadow-sm text-sm font-semibold text-gray-700 active:scale-95 transition-all"
              >
                Back to Team
              </button>
            </div>
          ) : (
            approvals.map((req) => {
              const { type, payload } = extractPayload(req.description);

              return (
                <div
                  key={req.id}
                  className="bg-white rounded-3xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-gray-100 relative overflow-hidden"
                >
                  {/* Decorative background for specific features */}
                  {req.payload?.feature_type === "ambassador_reply" && (
                    <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-bl from-blue-50 to-transparent rounded-bl-full opacity-50 pointer-events-none" />
                  )}

                  <div className="flex justify-between items-start mb-4 relative z-10">
                    <div>
                      <span className={`text-[10px] font-bold tracking-widest uppercase px-2 py-1 rounded-md ${req.payload?.feature_type === "ambassador_reply" ? "text-blue-500 bg-blue-50 border border-blue-100" : "text-orange-500 bg-orange-50"}`}>
                        {req.action_risk === "HIGH"
                          ? "Review Required"
                          : "Auto-Action"}
                      </span>
                      <h3 className="font-outfit font-bold text-gray-900 text-lg mt-2 leading-tight">
                        {req.payload?.feature_type === "ambassador_reply" ? "Customer Message" : req.description.split("PAYLOAD:")[0].trim()}
                      </h3>
                    </div>
                    {req.payload?.feature_type === "ambassador_reply" && (
                      <span className="text-xs font-semibold text-gray-400 bg-gray-50 px-2 py-1 rounded-md border border-gray-100">
                        {req.payload.platform}
                      </span>
                    )}
                  </div>

                  {/* Ambassador Reply Render Block */}
                  {req.payload?.feature_type === "ambassador_reply" && (
                    <div className="mb-6 p-4 rounded-xl bg-blue-50 border border-blue-100 flex flex-col gap-3 relative z-10">
                      <div className="text-xs text-blue-800 font-medium italic border-l-2 border-blue-300 pl-3">
                        "{req.payload.original_message}"
                      </div>

                      <div className="bg-white p-3 rounded-lg border border-blue-100 relative shadow-sm">
                        <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 absolute top-2 right-2 flex gap-1">
                          AI Draft {req.payload.confidence_score && <span className={req.payload.confidence_score >= 90 ? "text-green-500" : "text-orange-400"}>({req.payload.confidence_score}%)</span>}
                        </div>
                        <p className="text-sm text-gray-800 font-medium pr-20 pt-1 pb-1">
                          {req.payload.generated_response}
                        </p>
                      </div>

                      <div className="mt-1 pt-2 border-t border-blue-100/50">
                        <span className="text-[10px] uppercase font-bold text-blue-400">Context Used</span>
                        <p className="text-[10px] text-gray-500 line-clamp-2 leading-relaxed mt-0.5">{req.payload.context_used}</p>
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "social_calendar" && (
                    <div className="mb-6 overflow-hidden rounded-xl border border-gray-200">
                      <div className="aspect-[4/3] bg-gray-100 relative">
                        <img
                          src={req.payload.image_url}
                          alt="Social Post"
                          className="w-full h-full object-cover"
                        />
                        <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
                        <div className="absolute bottom-3 left-3 right-3 text-white">
                          <p className="text-xs font-medium opacity-90 mb-1">
                            Suggested Caption
                          </p>
                          <p className="text-sm font-semibold line-clamp-2">
                            {req.payload.caption}
                          </p>
                        </div>
                      </div>
                      <div className="bg-gray-50 p-3 flex justify-between items-center text-xs font-semibold text-gray-600 border-t border-gray-200">
                        <span className="flex items-center gap-1.5">
                          <svg
                            className="w-4 h-4"
                            fill="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zM12 0C8.741 0 8.333.014 7.053.072 2.695.272.273 2.69.073 7.052.014 8.333 0 8.741 0 12c0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98C8.333 23.986 8.741 24 12 24c3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98C15.668.014 15.259 0 12 0zm0 5.838a6.162 6.162 0 100 12.324 6.162 6.162 0 000-12.324zM12 16a4 4 0 110-8 4 4 0 010 8zm6.406-11.845a1.44 1.44 0 100 2.881 1.44 1.44 0 000-2.881z" />
                          </svg>
                          Instagram
                        </span>
                        <span>Today, 2:00 PM</span>
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "case_study" && (
                    <div className="mb-6 p-4 rounded-xl bg-gray-50 border border-gray-200">
                      <div className="flex items-center gap-3 mb-3">
                        <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 font-bold">
                          {req.payload.customer_name?.charAt(0) || "C"}
                        </div>
                        <div>
                          <p className="text-sm font-bold text-gray-900">
                            {req.payload.customer_name}
                          </p>
                          <div className="flex text-yellow-400 text-[10px]">
                            ★★★★★
                          </div>
                        </div>
                      </div>
                      <p className="text-xs text-gray-600 italic mb-3">
                        "{req.payload.testimonial}"
                      </p>
                      <div className="flex flex-wrap gap-2">
                        {req.payload.tags?.map((tag: string, i: number) => (
                          <span
                            key={i}
                            className="text-[10px] font-semibold bg-white border border-gray-200 text-gray-500 px-2 py-1 rounded"
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "legal_compliance" && (
                    <div className="mb-6 p-4 rounded-xl bg-amber-50 border border-amber-200">
                      <div className="flex items-start gap-3">
                        <div className="mt-0.5 text-amber-600">
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
                        </div>
                        <div>
                          <p className="text-sm font-bold text-amber-900 mb-1">
                            {req.payload.document_type ||
                              "Compliance Requirement"}
                          </p>
                          <p className="text-xs text-amber-800 leading-relaxed">
                            {req.payload.reason}
                          </p>
                          <div className="mt-3 p-3 bg-white/60 rounded border border-amber-200/50">
                            <p className="text-xs font-medium text-amber-900">
                              Proposed Update:
                            </p>
                            <p className="text-xs text-amber-700 italic mt-1 line-clamp-2">
                              "...{req.payload.proposed_text}..."
                            </p>
                          </div>
                        </div>
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "global_localization" && (
                    <div className="mb-6">
                      <div className="grid grid-cols-2 gap-3 mb-3">
                        <div className="p-3 bg-gray-50 rounded-xl border border-gray-100">
                          <p className="text-[10px] font-bold text-gray-400 uppercase mb-1">
                            Original (EN)
                          </p>
                          <p className="text-xs text-gray-700 font-medium">
                            {req.payload.original_text}
                          </p>
                        </div>
                        <div className="p-3 bg-blue-50 rounded-xl border border-blue-100">
                          <p className="text-[10px] font-bold text-blue-400 uppercase mb-1 flex items-center justify-between">
                            Translated ({req.payload.target_language})
                            <span className="bg-blue-100 text-blue-600 px-1.5 py-0.5 rounded text-[8px]">
                              AI DRAFT
                            </span>
                          </p>
                          <p className="text-xs text-blue-900 font-bold font-noto-sans-jp">
                            {req.payload.translated_text}
                          </p>
                        </div>
                      </div>
                      <div className="flex gap-2">
                        {["Pricing Updated", "SEO Tags Generated"].map(
                          (tag, i) => (
                            <span
                              key={i}
                              className="text-[10px] font-semibold bg-green-50 text-green-700 px-2 py-1 rounded border border-green-100 flex items-center gap-1"
                            >
                              <svg
                                className="w-3 h-3"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path
                                  strokeLinecap="round"
                                  strokeLinejoin="round"
                                  strokeWidth={3}
                                  d="M5 13l4 4L19 7"
                                />
                              </svg>
                              {tag}
                            </span>
                          ),
                        )}
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "business_advisory" && (
                    <div className="mb-6 p-4 rounded-xl bg-gradient-to-br from-indigo-50 to-purple-50 border border-indigo-100">
                      <div className="flex items-center gap-2 text-indigo-800 font-bold text-sm mb-2">
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
                        Strategic Insight
                      </div>
                      <p className="text-xs text-indigo-900 leading-relaxed mb-4">
                        "{req.payload.insight}"
                      </p>

                      <p className="text-[10px] uppercase font-bold text-indigo-400 mb-2">
                        Recommended Actions
                      </p>
                      <div className="space-y-2">
                        {req.payload.recommended_actions?.map(
                          (action: string, idx: number) => (
                            <div
                              key={idx}
                              className="bg-white p-3 rounded-lg border border-indigo-50 shadow-sm flex items-start gap-3"
                            >
                              <div className="w-5 h-5 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center text-[10px] font-bold flex-shrink-0 mt-0.5">
                                {idx + 1}
                              </div>
                              <div className="text-xs font-semibold text-gray-700">
                                {action}
                                {
                                  [
                                    " (AI can draft this)",
                                    " (Takes 2 mins)",
                                    " (Requires review)",
                                  ][idx]
                                }
                              </div>
                            </div>
                          ),
                        )}
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "social_promoter_campaign" && (
                    <div className="mb-6 p-4 rounded-xl bg-gradient-to-br from-pink-50 to-rose-50 border border-pink-100">
                      <div className="flex items-center justify-between mb-3">
                        <div className="flex items-center gap-2 text-pink-800 font-bold text-sm">
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
                              d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"
                            />
                          </svg>
                          Campaign Drafted
                        </div>
                        <span className="text-[10px] font-bold bg-white text-rose-600 px-2 py-1 rounded shadow-sm border border-rose-100 flex items-center gap-1">
                          <svg
                            className="w-3 h-3"
                            fill="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zM12 0C8.741 0 8.333.014 7.053.072 2.695.272.273 2.69.073 7.052.014 8.333 0 8.741 0 12c0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98C8.333 23.986 8.741 24 12 24c3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98C15.668.014 15.259 0 12 0zm0 5.838a6.162 6.162 0 100 12.324 6.162 6.162 0 000-12.324zM12 16a4 4 0 110-8 4 4 0 010 8zm6.406-11.845a1.44 1.44 0 100 2.881 1.44 1.44 0 000-2.881z" />
                          </svg>
                          {req.payload.platform || "Instagram"}
                        </span>
                      </div>

                      <div className="bg-white p-3 rounded-lg shadow-sm border border-pink-50 mb-3">
                        <p className="text-[10px] uppercase font-bold text-gray-400 mb-1">
                          Generated Caption
                        </p>
                        <p className="text-sm font-medium text-gray-700 leading-relaxed whitespace-pre-wrap">
                          {req.payload.caption}
                        </p>
                        {req.payload.hashtags && (
                          <p className="text-xs text-pink-600 font-semibold mt-2">
                            {req.payload.hashtags.join(" ")}
                          </p>
                        )}
                      </div>

                      <div className="flex gap-2 mb-2 overflow-x-auto pb-1 snap-x">
                        {req.payload.media_assets?.map(
                          (asset: string, idx: number) => (
                            <div
                              key={idx}
                              className="snap-start flex-shrink-0 w-24 h-24 bg-gray-100 rounded-lg overflow-hidden relative shadow-sm border border-black/5"
                            >
                              <img
                                src={asset}
                                alt={`Asset ${idx + 1}`}
                                className="w-full h-full object-cover"
                              />
                              <div className="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent" />
                              <div className="absolute bottom-1 right-1 bg-black/60 backdrop-blur-md px-1.5 py-0.5 rounded text-[8px] font-bold text-white uppercase tracking-wider border border-white/20">
                                {
                                  [
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

                  {req.payload?.feature_type === "zero_touch_portfolio" && (
                    <div className="mb-6 p-4 rounded-xl bg-gradient-to-br from-emerald-50 to-teal-50 border border-emerald-100">
                      <div className="flex items-center justify-between mb-3">
                        <div className="flex items-center gap-2 text-emerald-800 font-bold text-sm">
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
                              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                            />
                          </svg>
                          Portfolio Drafted
                        </div>
                        <span className="text-[10px] font-bold bg-white text-teal-600 px-2 py-1 rounded shadow-sm border border-teal-100">
                          {req.payload.industry || "Creative"}
                        </span>
                      </div>

                      <div className="bg-white p-3 rounded-lg shadow-sm border border-emerald-50 mb-3">
                        <p className="text-[10px] uppercase font-bold text-gray-400 mb-1">
                          Generated Bio
                        </p>
                        <p className="text-sm font-medium text-gray-700 leading-relaxed italic">
                          "{req.payload.bio}"
                        </p>
                      </div>

                      <p className="text-[10px] uppercase font-bold text-emerald-400 mb-2 mt-4">
                        Curated Categories
                      </p>
                      <div className="grid grid-cols-2 gap-2 mb-2">
                        {req.payload.categories?.map(
                          (cat: string, idx: number) => (
                            <div
                              key={idx}
                              className="bg-white p-2 rounded border border-emerald-50 shadow-sm flex items-center justify-between"
                            >
                              <span className="text-xs font-semibold text-gray-700 truncate pr-2">
                                {cat}
                              </span>
                              <div className="flex gap-0.5">
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-400"></div>
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-300"></div>
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-200"></div>
                              </div>
                            </div>
                          ),
                        )}
                        {!req.payload.categories &&
                          [
                            "Recent Work",
                            "Case Studies",
                            "Services",
                            "About",
                          ].map((cat, idx) => (
                            <div
                              key={idx}
                              className="bg-white p-2 rounded border border-emerald-50 shadow-sm flex items-center justify-between"
                            >
                              <span className="text-xs font-semibold text-gray-700 truncate pr-2">
                                {cat}
                              </span>
                              <div className="flex gap-0.5">
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-400"></div>
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-300"></div>
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-200"></div>
                              </div>
                            </div>
                          ))}
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
                          "Hi Sarah, noticed you left the Vegan Chocolate Cake
                          in your cart! Here's 10% off to sweeten the deal if
                          you finish your order today."
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

                  <div className="flex gap-3 relative z-10">
                    <button
                      onClick={() => {
                        if (payload && payload.original_message || req.payload?.feature_type === "ambassador_reply") {
                          setSelectedReview(req);
                        } else {
                          onReject(req.id);
                        }
                      }}
                      className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all min-h-[44px]"
                    >
                      {req.payload?.feature_type === "ambassador_reply" ? "Edit Draft" :
                        (payload && payload.original_message
                        ? "Review"
                        : "Reject / Edit")}
                    </button>
                    <button
                      onClick={() => onApprove(req.id)}
                      className="flex-[2] py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all min-h-[44px]"
                    >
                      {req.payload?.feature_type === "ambassador_reply" ? "Approve & Send" :
                        (req.payload?.feature_type === "case_study"
                        ? "Publish to Website"
                        : "Approve")}
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
              style={{
                animation: "slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1)",
              }}
            >
              <h2 className="text-xl font-bold mb-4 font-outfit text-gray-900">
                Review Draft
              </h2>

              <div className="mb-4">
                <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">
                  Context
                </p>
                <div className="bg-gray-50 p-3 rounded-xl border border-gray-100 text-sm text-gray-700">
                  {extractPayload(selectedReview.description).payload
                    ?.original_message || "N/A"}
                </div>
              </div>

              <div className="mb-6">
                <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">
                  Draft
                </p>
                <div className="bg-blue-50 p-3 rounded-xl border border-blue-100 text-sm text-gray-800 italic relative">
                  {extractPayload(selectedReview.description).payload
                    ?.generated_response || "N/A"}
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
        <style
          dangerouslySetInnerHTML={{
            __html: `
          @keyframes slideUp {
            from { transform: translateY(100%); }
            to { transform: translateY(0); }
          }
        `,
          }}
        />
      </div>
    </div>
  );
}
