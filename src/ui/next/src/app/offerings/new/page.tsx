'use client';

import { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function NewOfferingPage() {
  const router = useRouter();
  const [intent, setIntent] = useState('');
  const [loading, setLoading] = useState(false);
  const [offeringData, setOfferingData] = useState<any>(null);
  const [published, setPublished] = useState(false);
  const [error, setError] = useState('');

  const handleGenerate = async () => {
    if (!intent.trim()) {
      setError('Please describe what you want to offer.');
      return;
    }

    setError('');
    setLoading(true);

    try {
      const response = await fetch('/api/offerings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ intent }),
      });

      if (!response.ok) {
        throw new Error('Failed to generate offering details.');
      }

      const data = await response.json();
      setOfferingData(data);
    } catch (err) {
      setError('An error occurred while communicating with the AI agent.');
    } finally {
      setLoading(false);
    }
  };

  const handlePublish = () => {
    // Optimistic update
    setPublished(true);
    setTimeout(() => {
      router.push('/dashboard');
    }, 2000);
  };

  if (published) {
    return (
      <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col items-center justify-center font-inter text-center">
         <div className="w-20 h-20 bg-green-100 rounded-full flex items-center justify-center mb-6">
            <span className="text-4xl text-green-500">✓</span>
         </div>
         <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Offering Published!</h2>
         <p className="text-sm text-gray-600 mb-6">Your new offering is now live on your storefront.</p>
         <Link href="/dashboard" className="w-full max-w-xs py-3 bg-gray-900 text-white rounded-xl font-bold shadow-md hover:bg-black transition-colors">
            Return to Dashboard
         </Link>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter relative pb-20">
      <div className="flex items-center mb-6 border-b border-gray-200 pb-4">
        <Link href="/dashboard" className="text-blue-500 font-semibold mr-4">&lt; Back</Link>
        <h1 className="text-xl font-bold font-outfit text-gray-900">New Offering</h1>
      </div>

      {error && (
        <div className="mb-4 rounded-[8px] border border-red-200 bg-red-50 px-4 py-3 text-sm font-semibold text-red-700">
          {error}
        </div>
      )}

      {!loading && !offeringData && (
        <div className="flex-1 flex flex-col items-center justify-center pt-8">
          <div className="w-full mb-6">
             <label className="block text-lg font-bold text-gray-900 mb-2">What do you want to offer?</label>
             <p className="text-sm text-gray-500 mb-4">Type a simple sentence. Our AI will set up the details, calendar, and payment links.</p>
             <textarea
               value={intent}
               onChange={(e) => setIntent(e.target.value)}
               placeholder="e.g. Guitar lessons for beginners, 1 hour"
               className="w-full h-32 p-4 border border-gray-300 rounded-2xl shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-800 resize-none text-lg"
             ></textarea>
          </div>

          <button
            onClick={handleGenerate}
            className="w-full py-4 bg-[#0066FF] text-white rounded-xl font-bold shadow-md hover:bg-blue-600 transition-colors text-lg flex items-center justify-center gap-2"
          >
            <span>✨</span> Generate Details
          </button>
        </div>
      )}

      {loading && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6 pt-12">
           <div className="w-24 h-24 bg-white rounded-full shadow-lg flex items-center justify-center"
                style={{
                   background: 'rgba(255, 255, 255, 0.65)',
                   backdropFilter: 'blur(30px) saturate(210%)',
                   border: '1px solid rgba(255, 255, 255, 0.4)'
                }}>
              <div className="text-5xl animate-bounce">✨</div>
           </div>
           <div className="w-full space-y-4 px-4">
              <div className="h-4 bg-gray-200 rounded-md animate-pulse w-3/4 mx-auto"></div>
              <div className="h-4 bg-gray-200 rounded-md animate-pulse w-1/2 mx-auto"></div>
           </div>
           <p className="text-sm font-semibold text-blue-600 animate-pulse text-center mt-4">
             AI Agents are provisioning your offering...
           </p>
        </div>
      )}

      {offeringData && !loading && (
        <div className="flex-1 flex flex-col gap-5 animate-fade-in-up">

           <div className="p-5 rounded-[16px] shadow-lg flex flex-col gap-4 relative overflow-hidden"
                style={{
                   background: 'rgba(255, 255, 255, 0.65)',
                   backdropFilter: 'blur(30px) saturate(210%)',
                   border: '1px solid rgba(255, 255, 255, 0.4)'
                }}>
              <div className="absolute top-2 right-2 px-2 py-1 bg-gradient-to-r from-blue-500 to-purple-500 text-white text-[10px] font-bold rounded-full uppercase tracking-wider shadow-sm flex items-center gap-1">
                 <span>✨</span> AI Drafted
              </div>

              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Title</label>
                  <input
                    type="text"
                    value={offeringData.title}
                    onChange={(e) => setOfferingData({...offeringData, title: e.target.value})}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-bold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all text-lg"
                  />
              </div>

              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    value={offeringData.description}
                    onChange={(e) => setOfferingData({...offeringData, description: e.target.value})}
                    rows={3}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all resize-none"
                  />
              </div>

              <div className="flex gap-3">
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Type</label>
                      <input
                        type="text"
                        value={offeringData.type}
                        readOnly
                        className="w-full bg-gray-100/50 border border-gray-200/60 rounded-[8px] px-3 py-2 text-gray-600 font-semibold focus:outline-none text-sm cursor-not-allowed"
                      />
                  </div>
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price</label>
                      <div className="relative">
                          <span className="absolute left-3 top-2 text-gray-500 font-semibold">$</span>
                          <input
                            type="number"
                            value={offeringData.price}
                            onChange={(e) => setOfferingData({...offeringData, price: e.target.value})}
                            className="w-full bg-white/50 border border-white/60 rounded-[8px] pl-7 pr-3 py-2 text-gray-900 font-bold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                          />
                      </div>
                  </div>
              </div>

              {offeringData.type === 'Service' && (
                <div className="mt-2 p-3 bg-blue-50/50 border border-blue-100 rounded-xl flex items-center gap-3">
                  <div className="text-blue-500 text-xl">📅</div>
                  <div className="text-xs text-gray-700">
                    <span className="font-semibold block">Calendar connected</span>
                    Customers will be able to book available slots.
                  </div>
                </div>
              )}
           </div>

           <button
             onClick={handlePublish}
             className="w-full py-4 bg-[#0066FF] text-white font-bold rounded-xl shadow-md hover:bg-blue-600 transition-colors text-lg mt-4"
           >
             Publish to Storefront
           </button>
        </div>
      )}
    </div>
  );
}
