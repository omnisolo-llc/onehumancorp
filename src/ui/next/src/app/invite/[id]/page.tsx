"use client";

import React, { useState, useEffect, use } from 'react';
import { useRouter } from 'next/navigation';

export default function InvitePage({ params }: { params: Promise<{ id: string }> }) {
  const router = useRouter();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [inviteId, setInviteId] = useState<string | null>(null);

  useEffect(() => {
    params.then((p) => {
      setInviteId(p.id);
      setLoading(false);
    });
  }, [params]);

  const handleAccept = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/v1/growth/team-invites/accept', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ id: inviteId }),
      });

      if (!res.ok) {
        throw new Error('Failed to accept invite');
      }

      setSuccess(true);
      setTimeout(() => {
        router.push('/dashboard');
      }, 2000);
    } catch (e: any) {
      setError(e.message || 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  if (loading && !inviteId) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7]">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7] font-inter p-4">
      <div className="w-full max-w-md bg-white rounded-[24px] shadow-2xl overflow-hidden relative border border-gray-100 p-8 text-center" style={{ backdropFilter: 'blur(20px) saturate(200%)' }}>
        {success ? (
          <>
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl mb-4 text-green-600 mx-auto">
              ✓
            </div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Invite Accepted!</h1>
            <p className="text-gray-600 mb-6">You've been added to the team. Redirecting to dashboard...</p>
          </>
        ) : (
          <>
            <div className="w-16 h-16 bg-blue-100 rounded-2xl shadow-inner flex items-center justify-center text-3xl mb-6 mx-auto">
              🤝
            </div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You've been invited!</h1>
            <p className="text-gray-600 mb-8">Join the Cloud Bridge to collaborate in a shared workspace.</p>

            <button
              onClick={handleAccept}
              disabled={loading}
              className={`w-full py-3 px-4 font-semibold text-white rounded-xl shadow-md transition-all ${
                loading ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:-translate-y-0.5 active:translate-y-0'
              }`}
            >
              {loading ? 'Processing...' : 'Accept Invitation'}
            </button>

            {error && (
              <p className="text-red-500 text-sm mt-4 font-medium">{error}</p>
            )}
          </>
        )}
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
