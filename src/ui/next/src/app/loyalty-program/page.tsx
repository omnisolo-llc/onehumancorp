"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LoyaltyProgramPage() {
  const router = useRouter();

  const [tenant, setTenant] = useState('my-store');
  const [storeName, setStoreName] = useState('My Store');
  const [enabled, setEnabled] = useState(false);
  const [programType, setProgramType] = useState<'punch_card' | 'points'>('punch_card');
  const [rewardThreshold, setRewardThreshold] = useState(10);
  const [rewardDescription, setRewardDescription] = useState('Get 1 Free Coffee');
  const [pointsPerDollar, setPointsPerDollar] = useState(1);
  const [isSaved, setIsSaved] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
        const storedTenant = localStorage.getItem('tenant') || localStorage.getItem('tenant_id') || 'my-store';
        setTenant(storedTenant);
        const storedName = localStorage.getItem('business_name');
        if (storedName) setStoreName(storedName);

        fetch(`/api/v1/loyalty/get?tenant=${storedTenant}`)
            .then(res => res.json())
            .then(data => {
                if (data.success && data.data) {
                    const parsed = data.data;
                    setEnabled(parsed.enabled ?? false);
                    setProgramType(parsed.programType || 'punch_card');
                    setRewardThreshold(parsed.rewardThreshold || 10);
                    setRewardDescription(parsed.rewardDescription || 'Get 1 Free Coffee');
                    setPointsPerDollar(parsed.pointsPerDollar || 1);
                } else {
                    const savedData = localStorage.getItem(`ohc_loyalty_${storedTenant}`);
                    if (savedData) {
                        try {
                            const parsed = JSON.parse(savedData);
                            setEnabled(parsed.enabled ?? false);
                            setProgramType(parsed.programType || 'punch_card');
                            setRewardThreshold(parsed.rewardThreshold || 10);
                            setRewardDescription(parsed.rewardDescription || 'Get 1 Free Coffee');
                            setPointsPerDollar(parsed.pointsPerDollar || 1);
                        } catch (e) {
                            console.error("Failed to parse loyalty data", e);
                        }
                    }
                }
            })
            .catch(e => console.error("Failed to fetch loyalty data", e));
    }
  }, []);

  const handleSave = async () => {
    const data = {
        enabled,
        programType,
        rewardThreshold,
        rewardDescription,
        pointsPerDollar
    };
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem(`ohc_loyalty_${tenant}`, JSON.stringify(data));
    }
    try {
        await fetch('/api/v1/loyalty/save', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });
    } catch(e) {
        console.error("Failed to save to backend", e);
    }
    setIsSaved(true);
    setTimeout(() => setIsSaved(false), 3000);
  };

  const handleShare = () => {
    const shareMessage = `Join our new ${programType === 'punch_card' ? 'Punch Card' : 'Rewards'} program at ${storeName}! Earn rewards with every purchase. Sign up here: https://ohc.store/${tenant}/loyalty`;
    window.open(`https://wa.me/?text=${encodeURIComponent(shareMessage)}`, '_blank');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Customer Loyalty 🎁</h1>
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
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <div className="flex justify-between items-center mb-6">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Program Settings</h2>
                    <label className="flex items-center cursor-pointer">
                        <div className="relative">
                            <input type="checkbox" className="sr-only" checked={enabled} onChange={() => setEnabled(!enabled)} />
                            <div className={`block w-14 h-8 rounded-full transition-colors ${enabled ? 'bg-green-500' : 'bg-gray-300'}`}></div>
                            <div className={`dot absolute left-1 top-1 bg-white w-6 h-6 rounded-full transition-transform ${enabled ? 'transform translate-x-6' : ''}`}></div>
                        </div>
                    </label>
                </div>

                <div className={`flex flex-col gap-4 transition-opacity ${enabled ? 'opacity-100' : 'opacity-50 pointer-events-none'}`}>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Program Type</label>
                        <div className="flex gap-2">
                            <button
                                onClick={() => setProgramType('punch_card')}
                                className={`flex-1 py-2 px-4 rounded-lg font-semibold text-sm border-2 transition-colors ${programType === 'punch_card' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Punch Card
                            </button>
                            <button
                                onClick={() => setProgramType('points')}
                                className={`flex-1 py-2 px-4 rounded-lg font-semibold text-sm border-2 transition-colors ${programType === 'points' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Points System
                            </button>
                        </div>
                    </div>

                    <div className="mt-2">
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                            {programType === 'punch_card' ? 'Number of purchases required (e.g., 10)' : 'Points needed for reward (e.g., 100)'}
                        </label>
                        <input
                            type="number"
                            value={rewardThreshold}
                            onChange={(e) => setRewardThreshold(Number(e.target.value))}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>

                    {programType === 'points' && (
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Points earned per $1 spent</label>
                            <input
                                type="number"
                                value={pointsPerDollar}
                                onChange={(e) => setPointsPerDollar(Number(e.target.value))}
                                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            />
                        </div>
                    )}

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Reward Description</label>
                        <input
                            type="text"
                            value={rewardDescription}
                            onChange={(e) => setRewardDescription(e.target.value)}
                            placeholder="e.g., 50% off your next item"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                </div>

                <div className="mt-8 flex flex-col gap-3">
                    <button
                        onClick={handleSave}
                        className={`w-full py-3 rounded-lg text-sm font-semibold transition-all text-white ${isSaved ? 'bg-green-600 hover:bg-green-700' : 'bg-gray-900 hover:bg-black'}`}
                    >
                        {isSaved ? 'Saved successfully!' : 'Save Program Settings'}
                    </button>
                    {enabled && (
                        <button
                            onClick={handleShare}
                            className="w-full py-3 rounded-lg text-sm font-semibold transition-all bg-[#25D366] text-white hover:bg-[#20bd5a] flex items-center justify-center gap-2"
                        >
                            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                            Share Program to Customers
                        </button>
                    )}
                </div>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex justify-center items-start">
             <div className="w-[375px] h-[700px] bg-white rounded-[40px] shadow-2xl overflow-hidden relative border-[8px] border-gray-900 flex flex-col bg-gray-50">
                 {/* Notch */}
                 <div className="absolute top-0 w-40 h-6 bg-gray-900 rounded-b-2xl z-50 self-center"></div>

                 <div className="w-full h-full flex flex-col overflow-y-auto pt-16 pb-8 px-6">
                    {!enabled ? (
                        <div className="flex flex-col items-center justify-center h-full text-center text-gray-400">
                            <span className="text-5xl mb-4">🚫</span>
                            <p className="font-semibold">Loyalty Program is Disabled</p>
                        </div>
                    ) : (
                        <div className="flex flex-col h-full">
                            <div className="mb-6 text-center">
                                <h2 className="text-xl font-bold font-outfit text-gray-900">{storeName}</h2>
                                <p className="text-sm text-gray-500">Loyalty Rewards</p>
                            </div>

                            <div className="w-full bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl p-6 shadow-lg text-white mb-6 relative overflow-hidden">
                                <div className="absolute top-0 right-0 w-32 h-32 bg-white/10 rounded-full blur-xl translate-x-1/4 -translate-y-1/4 pointer-events-none"></div>

                                {programType === 'punch_card' ? (
                                    <>
                                        <p className="text-sm font-semibold opacity-90 mb-1">Buy {rewardThreshold}, get a reward!</p>
                                        <h3 className="text-lg font-bold mb-6">{rewardDescription}</h3>

                                        <div className="grid grid-cols-5 gap-2">
                                            {Array.from({ length: rewardThreshold }).map((_, i) => (
                                                <div key={i} className={`w-10 h-10 rounded-full flex items-center justify-center font-bold text-sm shadow-inner ${i < 3 ? 'bg-white text-indigo-600' : 'bg-white/20 text-white'}`}>
                                                    {i < 3 ? '✓' : i + 1}
                                                </div>
                                            ))}
                                        </div>
                                        <p className="text-xs font-medium opacity-80 mt-4 text-center">3 punches earned</p>
                                    </>
                                ) : (
                                    <div className="text-center py-4">
                                        <p className="text-sm font-semibold opacity-90 mb-1">Your Points Balance</p>
                                        <h3 className="text-5xl font-bold font-outfit mb-2 drop-shadow-md">350</h3>
                                        <div className="w-full bg-white/20 rounded-full h-2 mb-2">
                                            <div className="bg-white h-2 rounded-full" style={{ width: `${Math.min(100, (350 / rewardThreshold) * 100)}%` }}></div>
                                        </div>
                                        <p className="text-xs font-medium opacity-90">
                                            {rewardThreshold - 350 > 0 ? `${rewardThreshold - 350} more points to unlock:` : 'Reward unlocked!'}
                                        </p>
                                        <p className="text-sm font-bold mt-1">{rewardDescription}</p>
                                    </div>
                                )}
                            </div>

                            <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex items-center justify-between mb-4">
                                <div className="flex items-center gap-3">
                                    <div className="w-10 h-10 rounded-full bg-gray-100 flex items-center justify-center text-xl">📱</div>
                                    <div>
                                        <p className="text-sm font-bold text-gray-900">Show to Cashier</p>
                                        <p className="text-xs text-gray-500">To earn or redeem</p>
                                    </div>
                                </div>
                                <div className="px-3 py-1 bg-gray-900 text-white text-xs font-bold rounded-lg">View QR</div>
                            </div>

                            <div className="mt-auto pt-4 border-t border-gray-200">
                                <p className="text-center text-xs text-gray-400 font-medium">Powered by OHC Loyalty</p>
                            </div>
                        </div>
                    )}
                 </div>
             </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
