"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function ReviewsPage() {
  const router = useRouter();
  const [copied, setCopied] = useState(false);
  const [incentiveEnabled, setIncentiveEnabled] = useState(true);
  const [tenantId, setTenantId] = useState('my-store');
  const [isSaving, setIsSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState('');

  // Growth Loop URL - normally generated dynamically per customer/tenant
  const shareLink = `https://ohc.store/review-share?ref=${tenantId}`;
  const [shareMessage, setShareMessage] = useState(`I just had a fantastic experience with this store on OHC! Check them out: ${shareLink}`);

  React.useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant');
      if (storedTenant) {
        setTenantId(storedTenant);
      }
    }
  }, []);

  React.useEffect(() => {
    // Fetch initial settings
    const fetchSettings = async () => {
      try {
        const res = await fetch(`/api/v1/growth/reviews/settings?tenantId=${tenantId}`);
        if (res.ok) {
          const json = await res.json();
          if (json.data) {
            setIncentiveEnabled(json.data.incentiveEnabled);
            setShareMessage(json.data.shareMessage);
          }
        }
      } catch (err) {
        console.error("Failed to load settings", err);
      }
    };
    fetchSettings();
  }, [tenantId]);

  const handleSave = async () => {
    setIsSaving(true);
    setSaveMessage('');
    try {
      const res = await fetch('/api/v1/growth/reviews/settings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          tenantId,
          incentiveEnabled,
          shareMessage
        })
      });

      if (res.ok) {
        setSaveMessage('Configurations saved successfully!');
        setTimeout(() => setSaveMessage(''), 3000);
      } else {
        setSaveMessage('Failed to save configurations.');
      }
    } catch (err) {
      console.error(err);
      setSaveMessage('Error saving configurations.');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Automated Review Requests ⭐️</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Settings Panel */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 shadow-md h-full" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Growth Loop Settings</h2>

                <p className="text-sm text-gray-600 mb-6 leading-relaxed">
                  Turn positive reviews into new customers! Automatically email customers after a purchase, asking for a review. If they leave 4 or 5 stars, instantly prompt them to share your store on social media for a reward.
                </p>

                <div className="flex flex-col gap-4">
                    <div className="flex items-center justify-between p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
                        <div>
                            <h3 className="font-semibold text-gray-900">Enable Social Incentive</h3>
                            <p className="text-xs text-gray-500 mt-1">Offer 10% off their next order if they share.</p>
                        </div>
                        <button
                            onClick={() => setIncentiveEnabled(!incentiveEnabled)}
                            className={`w-12 h-6 rounded-full transition-colors duration-300 relative ${incentiveEnabled ? 'bg-green-500' : 'bg-gray-300'}`}
                        >
                            <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${incentiveEnabled ? 'translate-x-6' : 'translate-x-0'}`}></span>
                        </button>
                    </div>

                    <div className="mt-4">
                        <label className="block text-sm font-medium text-gray-700 mb-1">Pre-filled Share Message</label>
                        <textarea
                            value={shareMessage}
                            onChange={(e) => setShareMessage(e.target.value)}
                            rows={3}
                            className="w-full px-4 py-2 text-sm border border-gray-200 bg-gray-50 rounded-lg focus:outline-none"
                        />
                    </div>

                    <button
                        onClick={handleSave}
                        disabled={isSaving}
                        className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-md transition-colors ${isSaving ? 'bg-gray-500 cursor-not-allowed' : 'bg-gray-900 hover:bg-black'}`}
                    >
                        {isSaving ? 'Saving...' : 'Save Configurations'}
                    </button>
                    {saveMessage && (
                        <p className={`text-sm text-center mt-2 ${saveMessage.includes('Failed') || saveMessage.includes('Error') ? 'text-red-600' : 'text-green-600'}`}>
                            {saveMessage}
                        </p>
                    )}
                </div>
            </div>
        </section>

        {/* Customer Experience Preview */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 shadow-md h-full flex flex-col" style={{ background: 'rgba(255, 255, 255, 0.95)', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
                <div className="flex items-center justify-between mb-6 border-b pb-4">
                    <h2 className="text-lg font-bold font-outfit text-gray-900">Customer Preview</h2>
                    <span className="px-2 py-1 bg-indigo-50 text-indigo-700 text-xs font-semibold rounded">Post-Review Step</span>
                </div>

                <div className="flex-1 flex flex-col items-center justify-center text-center p-4">
                    <div className="text-5xl mb-4">🌟🌟🌟🌟🌟</div>
                    <h3 className="text-xl font-bold font-outfit text-gray-900 mb-2">Thanks for the 5-star review!</h3>

                    {incentiveEnabled ? (
                      <>
                        <p className="text-sm text-gray-600 mb-6">Want <strong>10% off</strong> your next order? Share your experience with friends!</p>
                        <div className="w-full max-w-sm flex flex-col gap-3">
                            <a
                                href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareMessage)}`}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="w-full flex items-center justify-center gap-2 bg-[#1DA1F2] text-white py-3 rounded-xl font-semibold text-sm shadow-sm hover:opacity-90 transition-all"
                            >
                                Share on Twitter (X)
                            </a>
                            <a
                                href={`https://wa.me/?text=${encodeURIComponent(shareMessage)}`}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="w-full flex items-center justify-center gap-2 bg-[#25D366] text-white py-3 rounded-xl font-semibold text-sm shadow-sm hover:opacity-90 transition-all"
                            >
                                Share on WhatsApp
                            </a>
                            <button
                                onClick={() => {
                                    navigator.clipboard.writeText(shareMessage);
                                    setCopied(true);
                                    setTimeout(() => setCopied(false), 2000);
                                }}
                                className="w-full flex items-center justify-center gap-2 bg-gray-100 text-gray-800 py-3 rounded-xl font-semibold text-sm hover:bg-gray-200 transition-all"
                            >
                                {copied ? 'Link Copied!' : 'Copy Link'}
                            </button>
                        </div>
                      </>
                    ) : (
                      <p className="text-sm text-gray-600 mb-6">We appreciate your feedback and hope to see you again soon.</p>
                    )}
                </div>
            </div>
        </section>

      </main>
    </div>
  );
}