'use client';
import { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function NewOfferingPage() {
  const router = useRouter();
  const [intent, setIntent] = useState('');
  const [loading, setLoading] = useState(false);
  const [offeringData, setOfferingData] = useState<{ title: string; description: string; type: string; price: string } | null>(null);
  const [published, setPublished] = useState(false);

  const handleIntentSubmit = () => {
    if (!intent.trim()) return;
    setLoading(true);
    // Simulate AI processing
    setTimeout(() => {
      setOfferingData({
        title: 'Beginner Guitar Lesson (1 Hour)',
        description: 'Learn the basics of guitar playing in a 1-hour personalized session. Covering chords, strumming, and basic theory.',
        type: 'Service',
        price: '50.00'
      });
      setLoading(false);
    }, 1500);
  };

  const handlePublish = () => {
    setPublished(true);
  };

  if (published) {
    return (
      <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col items-center justify-center">
        <div className="text-6xl mb-4">🎉</div>
        <h2 className="text-2xl font-bold text-gray-900 mb-2">Offering Published!</h2>
        <p className="text-gray-600 mb-6 text-center">Your new offering is now live on your storefront.</p>
        <button
          onClick={() => router.push('/dashboard')}
          className="w-full max-w-xs py-3 bg-gray-900 text-white rounded-xl font-bold shadow-md hover:bg-black text-center"
        >
          Return to Dashboard
        </button>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter">
      <div className="flex items-center mb-6 border-b border-gray-200 pb-4">
        <Link href="/dashboard" className="text-[#0066FF] font-semibold mr-4">&lt; Back</Link>
        <h1 className="text-xl font-bold font-outfit text-gray-900">Add Offering</h1>
      </div>

      {!loading && !offeringData && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
          <div className="w-full">
            <label className="block text-sm font-bold text-gray-700 mb-2 text-center">What do you want to offer?</label>
            <textarea
              value={intent}
              onChange={(e) => setIntent(e.target.value)}
              placeholder="e.g. Guitar lessons for beginners, 1 hour"
              className="w-full border-2 border-gray-300 rounded-xl p-4 text-gray-900 focus:outline-none focus:border-[#0066FF] min-h-[120px]"
            />
          </div>
          <button
            onClick={handleIntentSubmit}
            disabled={!intent.trim()}
            className="w-full py-3.5 bg-[#0071E3] disabled:bg-blue-300 text-white font-bold rounded-xl shadow-md hover:bg-blue-700 transition-colors"
          >
            Generate Details
          </button>
        </div>
      )}

      {loading && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
          <div className="w-16 h-16 border-4 border-[#0066FF] border-t-transparent rounded-full animate-spin"></div>
          <p className="text-sm font-semibold text-[#0071E3] animate-pulse text-center">AI is preparing your offering...</p>
        </div>
      )}

      {offeringData && !loading && (
        <div className="flex-1 flex flex-col gap-6 animate-fade-in-up">
           <div className="p-5 shadow-lg flex flex-col gap-4 relative overflow-hidden"
                style={{
                   background: 'rgba(255, 255, 255, 0.65)',
                   backdropFilter: 'blur(30px) saturate(210%)',
                   border: '1px solid rgba(255, 255, 255, 0.4)'
                }}>
              <div className="absolute top-2 right-2 px-2 py-1 bg-gradient-to-r from-blue-500 to-purple-500 text-white text-[10px] font-bold rounded-full uppercase tracking-wider shadow-sm flex items-center gap-1">
                 <span>✨</span> AI Generated
              </div>

              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Title</label>
                  <input
                    type="text"
                    value={offeringData.title}
                    onChange={(e) => setOfferingData({...offeringData, title: e.target.value})}
                    className="w-full bg-white border border-white/60 px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                  />
              </div>

              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    value={offeringData.description}
                    onChange={(e) => setOfferingData({...offeringData, description: e.target.value})}
                    rows={4}
                    className="w-full bg-white border border-white/60 px-3 py-2 text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                  />
              </div>

              <div className="flex gap-4">
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Type</label>
                      <input
                        type="text"
                        value={offeringData.type}
                        onChange={(e) => setOfferingData({...offeringData, type: e.target.value})}
                        className="w-full bg-white border border-white/60 px-3 py-2 text-gray-900 font-semibold text-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                      />
                  </div>
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price</label>
                      <div className="relative">
                          <span className="absolute left-3 top-2 text-gray-500 font-semibold">$</span>
                          <input
                            type="text"
                            value={offeringData.price}
                            onChange={(e) => setOfferingData({...offeringData, price: e.target.value})}
                            className="w-full bg-white border border-white/60 pl-7 pr-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                          />
                      </div>
                  </div>
              </div>
           </div>

           <button
             onClick={handlePublish}
             className="w-full py-3.5 bg-[#0066FF] text-white font-bold shadow-md hover:bg-[#0071E3] transition-colors text-lg"
           >
             Publish Offering
           </button>
        </div>
      )}
    </div>
  );
}
