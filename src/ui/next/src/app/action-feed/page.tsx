'use client';
import { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import Head from 'next/head';

function ActionFeedContent() {
  const searchParams = useSearchParams();
  const token = searchParams.get('token');
  const [loading, setLoading] = useState(true);
  const [tokenInfo, setTokenInfo] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!token) {
      setError("No action token provided.");
      setLoading(false);
      return;
    }

    async function fetchTokenInfo() {
      try {
        const res = await fetch(`/api/agents/action_tokens/${token}`);
        if (res.ok) {
          const data = await res.json();
          setTokenInfo(data);
        } else {
          setError("Action token is invalid, expired, or already consumed.");
        }
      } catch (err) {
        setError("Failed to fetch action token data.");
      } finally {
        setLoading(false);
      }
    }

    fetchTokenInfo();
  }, [token]);

  const handleDecision = async (approved: boolean) => {
    setSubmitting(true);
    try {
      const res = await fetch(`/api/agents/action_tokens/${token}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved })
      });
      if (res.ok) {
        setSuccess(true);
      } else {
        setError("Failed to process approval decision.");
      }
    } catch (e) {
      setError("Failed to communicate with server.");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter max-w-[375px] mx-auto overflow-hidden relative shadow-2xl">
      <header className="px-5 py-4 bg-white shadow-sm z-10 flex items-center gap-3">
        <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
          <span className="text-blue-600 text-lg">🤖</span>
        </div>
        <div>
          <h1 className="text-lg font-bold text-gray-900 font-outfit leading-tight">Agent Action Required</h1>
          <p className="text-xs text-gray-500">1-Tap Approval</p>
        </div>
      </header>

      <main className="flex-1 p-5 overflow-y-auto">
        {loading ? (
          <div className="flex flex-col items-center justify-center h-full gap-3 opacity-60">
            <div className="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
            <p className="text-sm font-medium text-gray-600">Loading action details...</p>
          </div>
        ) : error ? (
          <div className="bg-red-50 border border-red-100 rounded-xl p-5 text-center flex flex-col items-center gap-3">
            <span className="text-3xl">⚠️</span>
            <p className="text-sm font-medium text-red-800">{error}</p>
          </div>
        ) : success ? (
          <div className="bg-green-50 border border-green-100 rounded-xl p-6 text-center flex flex-col items-center gap-4 animate-in fade-in zoom-in duration-300">
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center">
              <svg className="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
            </div>
            <div>
              <h2 className="text-xl font-bold text-green-800 font-outfit mb-1">Action Processed!</h2>
              <p className="text-sm text-green-600">The AI agent is now executing this task.</p>
            </div>
          </div>
        ) : (
          <div className="flex flex-col gap-6">
            <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-5 relative overflow-hidden">
              <div className="absolute top-0 right-0 w-24 h-24 bg-blue-50 rounded-bl-full -z-10 opacity-50"></div>

              <div className="flex items-start justify-between mb-4">
                <span className="px-2.5 py-1 bg-yellow-100 text-yellow-800 rounded-md text-[10px] font-bold uppercase tracking-wide">
                  Pending Review
                </span>
                <span className="text-xs text-gray-400 font-medium">{new Date(tokenInfo.created_at).toLocaleDateString()}</span>
              </div>

              <h2 className="text-xl font-bold text-gray-900 mb-2 font-outfit">
                {tokenInfo.description ? "Approval Request" : "Approval Request"}
              </h2>

              <p className="text-sm text-gray-600 leading-relaxed mb-6">
                {tokenInfo.description || "The agent requires your approval to proceed with this drafted action."}
              </p>

              <div className="flex flex-col gap-3">
                <button
                  onClick={() => handleDecision(true)}
                  disabled={submitting}
                  className="w-full py-3.5 bg-blue-600 hover:bg-blue-700 text-white rounded-xl font-semibold text-sm transition-all shadow-md active:scale-[0.98] disabled:opacity-70 flex justify-center items-center gap-2"
                >
                  {submitting ? 'Processing...' : 'Approve & Execute'}
                </button>
                <button
                  onClick={() => handleDecision(false)}
                  disabled={submitting}
                  className="w-full py-3.5 bg-red-50 hover:bg-red-100 text-red-600 rounded-xl font-semibold text-sm transition-all active:scale-[0.98] disabled:opacity-70"
                >
                  Reject Action
                </button>
              </div>
            </div>
          </div>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}

export default function ActionFeedPage() {
  return (
    <Suspense fallback={<div className="flex min-h-screen items-center justify-center text-gray-500">Loading...</div>}>
      <ActionFeedContent />
    </Suspense>
  );
}
