"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function WaitlistPage() {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');
  const [referralLink, setReferralLink] = useState('');
  const [position, setPosition] = useState(0);
  const [isCopied, setIsCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(referralLink);
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy', err);
    }
  };

  const shareText = encodeURIComponent(`I just joined the waitlist for OneHumanCorp. Join me to get early access!`);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setErrorMessage('');

    try {
      const response = await fetch('/api/v1/growth/waitlist', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email }),
      });

      if (!response.ok) {
        throw new Error('Failed to join waitlist. Please try again.');
      }

      const data = await response.json();
      setReferralLink(data.referral_link || `https://ohc.app/waitlist?ref=user`);
      setPosition(data.position || 42);

      setIsSuccess(true);
    } catch (error: any) {
      setErrorMessage(error.message || 'An error occurred.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-xl font-bold font-outfit text-[#1D1D1F] tracking-tight">OneHumanCorp</h1>
        <button
          onClick={() => router.push('/')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back Home
        </button>
      </header>

      <main className="flex-1 flex flex-col items-center justify-center p-6 md:p-12 text-center w-full max-w-2xl mx-auto">
        {isSuccess ? (
          <div className="w-full max-w-lg bg-white/65 backdrop-blur-md rounded-2xl shadow-sm border border-white/40 p-8 flex flex-col items-center">
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mb-4 text-green-600">
              ✓
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You're on the list!</h2>
            <p className="text-gray-600 font-medium mb-2">
              You are #{position} in line.
            </p>
            <p className="text-gray-600 mb-8 text-sm">
              Invite friends to move up the list.
            </p>

            <div className="w-full bg-white rounded-xl border border-gray-200 p-2 flex items-center mb-6 shadow-inner">
              <input
                type="text"
                readOnly
                value={referralLink}
                className="flex-1 bg-transparent text-sm text-gray-600 px-3 outline-none"
              />
              <button
                onClick={handleCopy}
                className="px-4 py-2 bg-[#0066FF] hover:bg-blue-700 active:scale-95 text-white text-sm font-semibold rounded-lg transition-all"
              >
                {isCopied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>

            <div className="flex gap-4 mb-8 w-full">
              <a
                href={`https://twitter.com/intent/tweet?text=${shareText}&url=${encodeURIComponent(referralLink)}`}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl border border-gray-200 hover:bg-gray-50 transition-colors font-medium text-gray-700 text-sm"
              >
                Share on X
              </a>
              <a
                href={`https://wa.me/?text=${shareText} ${encodeURIComponent(referralLink)}`}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl border border-green-200 bg-green-50 hover:bg-green-100 transition-colors font-medium text-green-700 text-sm"
              >
                Share to WhatsApp
              </a>
            </div>

            <button
              onClick={() => setIsSuccess(false)}
              className="text-sm text-gray-500 hover:text-gray-900 transition-colors mb-6"
            >
              Sign up another email
            </button>
          </div>
        ) : (
          <>
            <h1 className="text-4xl md:text-5xl font-bold font-outfit text-gray-900 mb-4">
              The AI platform for <br /> small business.
            </h1>
            <p className="text-lg text-gray-600 mb-8 max-w-xl mx-auto">
              Join the waitlist to be among the first to experience radical simplicity and invisible AI agents that run your business.
            </p>

            <form onSubmit={handleSubmit} className="w-full max-w-md bg-white/65 backdrop-blur-md p-6 rounded-2xl shadow-sm border border-white/40">
              <div className="flex flex-col gap-4">
                <div>
                  <label htmlFor="email" className="sr-only">Email address</label>
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
                      ? 'bg-indigo-400 cursor-not-allowed'
                      : 'bg-indigo-600 hover:bg-indigo-700 hover:-translate-y-0.5 active:translate-y-0'
                  }`}
                >
                  {isSubmitting ? 'Joining...' : 'Join the Waitlist'}
                </button>
                {errorMessage && (
                  <p className="text-red-500 text-sm mt-2">{errorMessage}</p>
                )}
              </div>
            </form>
          </>
        )}
      </main>

      <footer className="w-full p-6 text-center">
        <a
          href="/onboarding?ref=waitlist_footer"
          target="_blank"
          rel="noopener noreferrer"
          className="text-gray-500 hover:text-gray-900 text-sm font-semibold transition-colors flex items-center justify-center gap-2"
        >
          ⚡ Powered by OHC
        </a>
      </footer>
    </div>
  );
}
