"use client";

import React, { useState, useEffect } from "react";

type FeedItem = {
  id: string;
  department: string;
  description: string;
  timestamp: string;
};

export default function ActionFeed() {
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

  if (loading) {
    return <div className="p-4 text-center text-sm text-gray-500 animate-pulse">Loading activity...</div>;
  }

  if (feed.length === 0) {
    return (
      <div className="p-6 text-center">
        <p className="text-sm text-gray-500 font-medium">No recent activity</p>
      </div>
    );
  }

  return (
    <div className="px-4 py-2 flex flex-col gap-3">
      <h2 className="text-xs font-bold uppercase tracking-wider text-gray-400 ml-2 mb-1">Recent Activity</h2>
      {feed.map(item => (
        <div key={item.id} className="bg-white/60 backdrop-blur-[20px] saturate-[200%] border border-white/40 p-4 rounded-2xl shadow-sm flex flex-col gap-2 relative overflow-hidden group hover:bg-white/80 transition-all">
          <div className="flex justify-between items-start">
             <div className="flex items-center gap-2">
                 <div className="w-6 h-6 rounded-full bg-blue-100 flex items-center justify-center border border-blue-200">
                    <span className="text-[10px] font-bold text-blue-600">{item.department.charAt(4)}</span>
                 </div>
                 <span className="text-xs font-semibold text-gray-700">{item.department}</span>
             </div>
             <span className="text-[10px] text-gray-400 font-medium">{item.timestamp}</span>
          </div>
          <p className="text-sm text-gray-800 font-medium pl-8">{item.description}</p>
        </div>
      ))}
    </div>
  );
}
