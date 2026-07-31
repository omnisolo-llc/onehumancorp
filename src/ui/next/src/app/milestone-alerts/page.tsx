"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';

export default function MilestoneAlertsPage() {
  const [tenant, setTenant] = useState('my-business');
  const [milestones, setMilestones] = useState([
    { id: 1, title: '10th Order!', message: 'Congratulations on reaching 10 orders! Share the good news with your network.', icon: '🎉', achieved: true },
    { id: 2, title: '100th Customer', message: 'You just welcomed your 100th customer. Keep growing!', icon: '💯', achieved: false },
    { id: 3, title: '$10k Revenue', message: 'You have reached $10,000 in revenue. Amazing milestone!', icon: '💸', achieved: false }
  ]);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name') || 'my-business';
      setTenant(storedTenant);
    }
  }, []);

  const shareMilestone = (title: string) => {
    const shareText = `We just reached a new milestone: ${title}! Powered by OHC. https://ohc.app?ref=${encodeURIComponent(tenant)}`;
    if (navigator.share) {
      navigator.share({
        title: title,
        text: shareText,
      }).catch(console.error);
    } else {
      navigator.clipboard.writeText(shareText);
      alert('Milestone share text copied to clipboard!');
    }
  };

  if (!isClient) return <div className="min-h-screen bg-indigo-50" />;

  return (
    <AppShell title="Milestone Alerts">
      <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 py-10 px-4">
        <div className="max-w-4xl mx-auto w-full">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Success Milestone Alerts</h1>
          <p className="text-gray-600 mb-8 text-sm">Celebrate your achievements and grow your business by sharing your milestones.</p>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {milestones.map((milestone) => (
              <div key={milestone.id} className={`bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border ${milestone.achieved ? 'border-indigo-200' : 'border-gray-100 opacity-60'} p-6 flex flex-col items-center text-center transition-all hover:shadow-md`}>
                <div className="text-5xl mb-4">{milestone.icon}</div>
                <h3 className="text-xl font-bold text-gray-900 mb-2">{milestone.title}</h3>
                <p className="text-gray-600 text-sm mb-6 flex-1">{milestone.message}</p>
                <button
                  onClick={() => shareMilestone(milestone.title)}
                  disabled={!milestone.achieved}
                  className={`w-full py-3 rounded-xl font-semibold transition-all ${milestone.achieved ? 'bg-indigo-600 text-white hover:bg-indigo-700 shadow-md hover:shadow-lg' : 'bg-gray-200 text-gray-500 cursor-not-allowed'}`}
                >
                  {milestone.achieved ? 'Share Milestone' : 'Keep Working'}
                </button>
              </div>
            ))}
          </div>
        </div>
      </div>
    </AppShell>
  );
}
