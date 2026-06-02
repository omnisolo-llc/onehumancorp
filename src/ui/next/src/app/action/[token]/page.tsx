'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ActionApprovalPage({ params }: { params: { token: string } }) {
  const [isLoading, setIsLoading] = useState(true);
  const [actionData, setActionData] = useState<any>(null);
  const [error, setError] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [resultMessage, setResultMessage] = useState('');
  const router = useRouter();

  useEffect(() => {
    // In a real implementation we would fetch the action details from the backend using the token
    // For this simulation we mock the action data.
    setTimeout(() => {
      if (params.token) {
        setActionData({
          type: "Quote Approval",
          description: "Leaking pipe repair",
          amount: "$150.00",
          customer: "Alex Johnson"
        });
        setIsLoading(false);
      } else {
        setError('Invalid or expired action token.');
        setIsLoading(false);
      }
    }, 1000);
  }, [params.token]);

  const handleAction = async (action: 'approve' | 'reject') => {
    setIsProcessing(true);
    try {
      const res = await fetch('/api/agents/action', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token: params.token, action })
      });

      if (!res.ok) throw new Error('Action failed');

      const data = await res.json();
      setResultMessage(data.message || `Action ${action} successful.`);
    } catch (e: any) {
      setError(e.message || 'An error occurred.');
    } finally {
      setIsProcessing(false);
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-black flex items-center justify-center p-4">
        <div className="w-8 h-8 rounded-full border-4 border-blue-500 border-t-transparent animate-spin"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-black flex items-center justify-center p-4">
        <div className="bg-red-900/50 p-6 rounded-2xl max-w-sm w-full text-center border border-red-500/50">
          <svg className="w-12 h-12 text-red-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
          <h2 className="text-white font-bold text-xl mb-2">Error</h2>
          <p className="text-red-200">{error}</p>
        </div>
      </div>
    );
  }

  if (resultMessage) {
    return (
      <div className="min-h-screen bg-black flex items-center justify-center p-4">
        <div className="bg-[#1C1C1E] p-6 rounded-3xl max-w-sm w-full text-center shadow-2xl border border-white/10 animate-fade-in">
          <div className="w-16 h-16 bg-green-500/20 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h2 className="text-white font-bold text-xl mb-2">Done</h2>
          <p className="text-gray-400 mb-6">{resultMessage}</p>
          <button
            onClick={() => router.push('/dashboard')}
            className="w-full bg-white/10 hover:bg-white/20 text-white font-semibold py-3 rounded-xl transition-colors"
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-black/40 backdrop-blur-xl flex items-center justify-center p-4 font-outfit">
      <div className="bg-[#1C1C1E] rounded-3xl max-w-sm w-full shadow-2xl overflow-hidden border border-white/10">
        <div className="p-6">
          <div className="flex items-center gap-3 mb-6">
            <div className="w-10 h-10 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center shadow-lg">
              <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            </div>
            <div>
              <h1 className="text-white font-bold text-lg">Sales Agent</h1>
              <p className="text-gray-400 text-xs">Action Required</p>
            </div>
          </div>

          <h2 className="text-2xl font-bold text-white mb-2">{actionData?.type}</h2>
          <p className="text-gray-300 text-sm mb-6">Your agent has drafted a quote and needs your approval to send it to the customer.</p>

          <div className="bg-black/30 rounded-2xl p-4 mb-6 border border-white/5">
            <div className="flex justify-between items-center mb-3">
              <span className="text-gray-400 text-sm">Customer</span>
              <span className="text-white font-medium">{actionData?.customer}</span>
            </div>
            <div className="flex justify-between items-center mb-3">
              <span className="text-gray-400 text-sm">Service</span>
              <span className="text-white font-medium">{actionData?.description}</span>
            </div>
            <div className="flex justify-between items-center pt-3 border-t border-white/10">
              <span className="text-gray-400 text-sm">Quote Amount</span>
              <span className="text-green-400 font-bold text-lg">{actionData?.amount}</span>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => handleAction('reject')}
              disabled={isProcessing}
              className="bg-[#2C2C2E] hover:bg-[#3C3C3E] text-red-500 font-semibold py-4 rounded-2xl transition-colors disabled:opacity-50"
            >
              Reject
            </button>
            <button
              onClick={() => handleAction('approve')}
              disabled={isProcessing}
              className="bg-[#0A84FF] hover:bg-[#0070E0] text-white font-semibold py-4 rounded-2xl shadow-[0_0_15px_rgba(10,132,255,0.3)] transition-all disabled:opacity-50"
            >
              {isProcessing ? 'Processing...' : 'Approve & Send'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
