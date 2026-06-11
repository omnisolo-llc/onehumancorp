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

  const [splitEnabled, setSplitEnabled] = useState(false);
  const [splitPartner, setSplitPartner] = useState('');
  const [splitPercentage, setSplitPercentage] = useState<number>(0);


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

  const handlePublish = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/product', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: offeringData?.title,
          description: offeringData?.description,
          price: offeringData?.price,
          item_type: offeringData?.type,
          is_subscription: false,
          split_partner_id: splitEnabled && splitPartner.trim() ? splitPartner.trim() : undefined,
          split_percentage: splitEnabled && splitPercentage > 0 ? splitPercentage : undefined
        })
      });
      if (response.ok) {
        setPublished(true);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
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
        <Link href="/dashboard" className="text-blue-500 font-semibold mr-4">&lt; Back</Link>
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
              className="w-full border-2 border-gray-300 rounded-xl p-4 text-gray-900 focus:outline-none focus:border-blue-500 min-h-[120px]"
            />
          </div>
          <button
            onClick={handleIntentSubmit}
            disabled={!intent.trim()}
            className="w-full py-3.5 bg-blue-600 disabled:bg-blue-300 text-white font-bold rounded-xl shadow-md hover:bg-blue-700 transition-colors"
          >
            Generate Details
          </button>
        </div>
      )}

      {loading && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
          <div className="w-16 h-16 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
          <p className="text-sm font-semibold text-blue-600 animate-pulse text-center">AI is preparing your offering...</p>
        </div>
      )}

      {offeringData && !loading && (
        <div className="flex-1 flex flex-col gap-6 animate-fade-in-up">
           <div className="p-5 rounded-[16px] shadow-lg flex flex-col gap-4 relative overflow-hidden"
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
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                  />
              </div>

              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    value={offeringData.description}
                    onChange={(e) => setOfferingData({...offeringData, description: e.target.value})}
                    rows={4}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                  />
              </div>

              <div className="flex gap-4">
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Type</label>
                      <input
                        type="text"
                        value={offeringData.type}
                        onChange={(e) => setOfferingData({...offeringData, type: e.target.value})}
                        className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
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
                            className="w-full bg-white/50 border border-white/60 rounded-[8px] pl-7 pr-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                          />
                      </div>
                  </div>
              </div>
           </div>


           {/* Split Configurator */}
           <div className="p-5 rounded-[16px] shadow-sm flex flex-col gap-4 relative overflow-hidden bg-white/50 border border-white/60">
              <div className="flex items-center justify-between">
                 <label className="text-sm font-bold text-gray-900">Split this payment</label>
                 <label className="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" className="sr-only peer" checked={splitEnabled} onChange={(e) => setSplitEnabled(e.target.checked)} />
                    <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#0066FF]"></div>
                 </label>
              </div>

              {splitEnabled && (
                  <div className="mt-4 pt-4 border-t border-gray-200 flex flex-col gap-4 animate-in slide-in-from-top-2">
                      <div>
                          <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Who gets a cut?</label>
                          <input
                            type="text"
                            placeholder="Partner name, phone, or email"
                            value={splitPartner}
                            onChange={(e) => setSplitPartner(e.target.value)}
                            className="w-full bg-white/80 border border-gray-300 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                          />
                      </div>

                      <div>
                          <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Percentage: {splitPercentage}%</label>
                          <input
                             type="range"
                             min="0"
                             max="100"
                             value={splitPercentage}
                             onChange={(e) => setSplitPercentage(parseInt(e.target.value))}
                             className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-[#0066FF]"
                          />
                      </div>

                      {splitPartner.trim() && splitPercentage > 0 && offeringData && (
                          <div className="p-3 bg-blue-50 border border-blue-100 rounded-lg text-sm text-blue-800 font-medium">
                              If this sells for ${offeringData.price}, {splitPartner} gets ${(parseFloat(offeringData.price) * (splitPercentage / 100)).toFixed(2)}, you get ${(parseFloat(offeringData.price) * ((100 - splitPercentage) / 100)).toFixed(2)}.
                          </div>
                      )}
                  </div>
              )}
           </div>

           <button
             onClick={handlePublish}
             className="w-full py-3.5 bg-[#0066FF] text-white font-bold rounded-[8px] shadow-md hover:bg-blue-600 transition-colors text-lg"
           >
             Publish Offering
           </button>
        </div>
      )}
    </div>
  );
}
