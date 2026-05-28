"use client";

import React, { useEffect } from 'react';
import MissionBlockedPage from './mission-blocked/page';

export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error(error);
  }, [error]);

  // If the error digest or message indicates a DB issue, show the Mission Blocked view.
  // Note: in Next.js production, error.message is often stripped for server errors,
  // but digest can be used, or we can check the error context if passed down.
  // We'll broaden the condition to catch 'failed to fetch' or digest issues as well.
  const isDbError =
    error.message.toLowerCase().includes('postgresql') ||
    error.message.toLowerCase().includes('agent_missions') ||
    error.message.toLowerCase().includes('database') ||
    error.message.toLowerCase().includes('failed to fetch') ||
    error.message.toLowerCase().includes('connection refused') ||
    (error.digest && error.digest.toLowerCase().includes('database'));

  if (isDbError) {
    return <MissionBlockedPage />;
  }

  // Otherwise, fallback to the generic Next.js error fallback but cleanly styled.
  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter items-center p-4">
      <div className="w-full max-w-[375px] bg-[#F5F5F7] shadow-xl rounded-3xl p-6 text-center">
        <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Something went wrong!</h2>
        <button
          onClick={() => reset()}
          className="w-full py-3 bg-gray-900 text-white rounded-xl font-bold hover:bg-black transition-all"
        >
          Try again
        </button>
      </div>
    </div>
  );
}
