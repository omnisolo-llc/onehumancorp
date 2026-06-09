"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function WaitlistPage() {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');

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

      setIsSuccess(true);
    } catch (error: any) {
      setErrorMessage(error.message || 'An error occurred.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 glassmorphism/65 backdrop-blur-md border-white/40">
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
          <div className="w-full glassmorphism/65 backdrop-blur-md rounded-2xl shadow-sm border border-white/40 p-8 flex flex-col items-center">
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mb-4 text-green-600">
              ✓
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You're on the list!</h2>
            <p className="text-gray-600 mb-6">
              Thanks for joining. We'll let you know as soon as OneHumanCorp is ready for you.
            </p>
            <button
              onClick={() => setIsSuccess(false)}
              className="px-6 py-2.5 bg-gray-900 text-white font-medium rounded-xl hover:bg-black transition-colors"
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

            <form onSubmit={handleSubmit} className="w-full max-w-md glassmorphism/65 backdrop-blur-md p-6 rounded-2xl shadow-sm border border-white/40">
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
    </div>
  );
}
