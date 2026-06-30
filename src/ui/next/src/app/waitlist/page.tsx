"use client";

import React, { useState } from "react";
import { useRouter } from "next/navigation";
import { PoweredByOHC } from "../components/PoweredByOHC";

export default function WaitlistPage() {
  const router = useRouter();
  const [email, setEmail] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");
  const [position, setPosition] = useState<number | null>(null);
  const [referralLink, setReferralLink] = useState("");
  const [copied, setCopied] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setErrorMessage("");

    try {
      const response = await fetch("/api/v1/growth/waitlist", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
      });

      if (!response.ok) {
        throw new Error("Failed to join waitlist. Please try again.");
      }

      const data = await response.json();
      if (data.position) {
        setPosition(data.position);
      }
      if (data.referral_link) {
        setReferralLink(data.referral_link);
      }

      setIsSuccess(true);
    } catch (error: any) {
      setErrorMessage(error.message || "An error occurred.");
    } finally {
      setIsSubmitting(false);
    }
  };

  const copyLink = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40">
        <h1 className="text-xl font-bold font-outfit text-[#1D1D1F] tracking-tight">
          OneHumanCorp
        </h1>
        <button
          onClick={() => router.push("/")}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back Home
        </button>
      </header>

      <main className="flex-1 flex flex-col items-center justify-center p-6 md:p-12 text-center w-full max-w-2xl mx-auto">
        {isSuccess ? (
          <div className="w-full bg-white/65 backdrop-blur-[30px] saturate-[210%] rounded-2xl shadow-sm border border-white/40 p-8 flex flex-col items-center">
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mb-4 text-green-600">
              ✓
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
              {position
                ? `You're #${position} on the list!`
                : "You're on the list!"}
            </h2>
            <p className="text-gray-600 mb-6">
              Thanks for joining. We'll let you know as soon as OneHumanCorp is
              ready for you.
            </p>

            {referralLink && (
              <div className="w-full mt-4 mb-6 p-6 bg-gradient-to-br from-indigo-50/50 to-purple-50/50 border border-indigo-100/50">
                <h3 className="font-semibold text-gray-900 mb-2">
                  Move up the list!
                </h3>
                <p className="text-sm text-gray-600 mb-4">
                  Invite friends with your unique link. Each friend who joins
                  moves you up 5 spots.
                </p>
                <div className="flex gap-2 mb-4">
                  <input
                    type="text"
                    readOnly
                    value={referralLink}
                    className="flex-1 px-4 py-2 bg-white rounded-lg border border-gray-200 text-sm text-gray-600 outline-none"
                  />
                  <button
                    onClick={copyLink}
                    className={`px-4 py-2 rounded-lg font-medium text-sm transition-colors ${
                      copied
                        ? "bg-green-100 text-green-700"
                        : "bg-gray-900 text-white hover:bg-gray-800"
                    }`}
                  >
                    {copied ? "Copied!" : "Copy"}
                  </button>
                </div>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`I just joined the waitlist for OneHumanCorp! The AI platform for small business. Join me here: ${referralLink} \n\n⚡ Powered by OHC`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="w-full flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-bold text-sm shadow-md hover:bg-gray-800 transition-all hover:-translate-y-0.5"
                >
                  <svg
                    className="w-4 h-4"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z" />
                  </svg>
                  Share on X
                </a>
              </div>
            )}

            <button
              onClick={() => setIsSuccess(false)}
              className="mt-2 text-sm text-gray-500 hover:text-gray-700 transition-colors"
            >
              Sign up another email
            </button>
            <div className="mt-8 pt-6 border-t border-gray-200/60 w-full flex justify-center">
              <PoweredByOHC tenantId="waitlist" />
            </div>
          </div>
        ) : (
          <>
            <h1 className="text-4xl md:text-5xl font-bold font-outfit text-gray-900 mb-4">
              The AI platform for <br /> small business.
            </h1>
            <p className="text-lg text-gray-600 mb-8 max-w-xl mx-auto">
              Join the waitlist to be among the first to experience radical
              simplicity and invisible AI agents that run your business.
            </p>

            <form
              onSubmit={handleSubmit}
              className="w-full max-w-md bg-white/65 backdrop-blur-[30px] saturate-[210%] p-6 rounded-2xl shadow-sm border border-white/40"
            >
              <div className="flex flex-col gap-4">
                <div>
                  <label htmlFor="email" className="sr-only">
                    Email address
                  </label>
                  <input
                    type="email"
                    id="email"
                    required
                    placeholder="Enter your email address"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all shadow-inner"
                  />
                </div>
                <button
                  type="submit"
                  disabled={isSubmitting || !email}
                  className={`w-full py-3 px-4 font-semibold text-white rounded-xl shadow-md transition-all ${
                    isSubmitting || !email
                      ? "bg-indigo-400 cursor-not-allowed"
                      : "bg-indigo-600 hover:bg-indigo-700 hover:-translate-y-0.5 active:translate-y-0"
                  }`}
                >
                  {isSubmitting ? "Joining..." : "Join the Waitlist"}
                </button>
                {errorMessage && (
                  <p className="text-[#FF3B30] text-sm mt-2">{errorMessage}</p>
                )}
              </div>
            </form>
          </>
        )}
      </main>
    </div>
  );
}
