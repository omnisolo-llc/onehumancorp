'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardHeader, CardContent, CardFooter } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

interface WorkItem {
  id: string;
  source: string;
  payload: any;
  status: string;
}

interface AgentDraft {
  id: string;
  response: string;
  status: string;
}

interface FeedItem {
  workItem: WorkItem;
  draft?: AgentDraft;
}

export default function UnifiedFeed() {
  const [feedItems, setFeedItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // In a real app, this would fetch from the backend via REST/gRPC
    // and subscribe to Redis Pub/Sub (e.g., via Server-Sent Events or WebSockets)
    setLoading(false);
  }, []);

  const handleApprove = (itemId: string) => {
    // Send approval to backend
  };

  const handleEdit = (itemId: string) => {
    // Open edit modal
  };

  if (loading) return <div className="p-4 text-center">Loading feed...</div>;

  return (
    <div className="w-full max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col">
      <header className="bg-white border-b border-gray-200 p-4 sticky top-0 z-10">
        <h1 className="text-xl font-bold text-gray-900">Unified Feed</h1>
      </header>

      <main className="flex-1 overflow-y-auto p-4 space-y-4">
        {feedItems.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            <p>No new work items.</p>
          </div>
        ) : (
          feedItems.map((item) => (
            <Card key={item.workItem.id} className="w-full bg-white shadow-sm border border-gray-100 rounded-xl overflow-hidden transition-all hover:shadow-md">
              <CardHeader className="p-4 pb-2 border-b border-gray-50">
                <div className="flex justify-between items-center">
                  <span className="text-xs font-semibold uppercase tracking-wider text-blue-600 bg-blue-50 px-2 py-1 rounded-full">
                    {item.workItem.source}
                  </span>
                  <span className="text-xs text-gray-400">Just now</span>
                </div>
              </CardHeader>
              <CardContent className="p-4">
                 <p className="text-sm text-gray-800 line-clamp-3">
                   {/* Extract meaningful text from payload */}
                   {item.workItem.payload?.msg || JSON.stringify(item.workItem.payload)}
                 </p>
              </CardContent>
              {item.draft && (
                <CardFooter className="p-4 pt-2 bg-gray-50 border-t border-gray-100 flex flex-col gap-3">
                   <div className="w-full text-sm text-gray-600 italic border-l-2 border-blue-300 pl-3 py-1">
                     "{item.draft.response}"
                   </div>
                   <div className="flex gap-2 w-full">
                     <Button
                       variant="outline"
                       className="flex-1 text-sm bg-white border-gray-200 text-gray-700 hover:bg-gray-100"
                       onClick={() => handleEdit(item.workItem.id)}
                     >
                       Edit
                     </Button>
                     <Button
                       className="flex-1 text-sm bg-blue-600 hover:bg-blue-700 text-white"
                       onClick={() => handleApprove(item.workItem.id)}
                     >
                       Approve & Send
                     </Button>
                   </div>
                </CardFooter>
              )}
            </Card>
          ))
        )}
      </main>
    </div>
  );
}
