"use client";

import React, { useState, useEffect } from 'react';

type FeedItem = {
  id: string;
  department: string;
  description: string;
  timestamp: string;
};

export default function DailyBrief() {
  const [feed, setFeed] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchFeed = async () => {
      try {
        const response = await fetch('/api/agents/feed');
        if (response.ok) {
          const data = await response.json();
          setFeed(data.feed || []);
        }
      } catch (error) {
        console.error("Failed to fetch feed", error);
      } finally {
        setLoading(false);
      }
    };
    fetchFeed();
  }, []);

  return (
    <div className="mt-8">
      <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4 px-1">Daily Brief</h2>

      {loading ? (
        <div className="flex justify-center py-4">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900"></div>
        </div>
      ) : feed.length === 0 ? (
        <p className="text-sm text-gray-500 text-center py-4">No recent activity.</p>
      ) : (
        <div className="space-y-3">
          {feed.map(item => (
            <div key={item.id} className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-xl p-4 shadow-sm">
              <div className="flex justify-between items-start mb-1">
                <span className="text-xs font-bold text-blue-600 font-outfit uppercase tracking-wider">{item.department}</span>
                <span className="text-xs text-gray-400">{item.timestamp}</span>
              </div>
              <p className="text-sm text-gray-800">{item.description}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
