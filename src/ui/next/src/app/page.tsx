"use client";

import React, { useState, useEffect } from 'react';
import { AgentActionNotification } from '../components/AgentActionNotification';

export default function Home() {
  const [inquiry, setInquiry] = useState<any>(null);

  useEffect(() => {
    // Simulate an incoming inquiry after 1 second for the test
    const timer = setTimeout(() => {
      setInquiry({
        id: "evt-123",
        summary: "New plumbing inquiry from Carlos",
        draftResponse: "Hi Carlos, I can help with that. Estimated price is $150.",
        actionSummary: "Send quote for $150 and propose Tuesday",
      });
    }, 1000);
    return () => clearTimeout(timer);
  }, []);

  return (
    <main className="p-8">
      <h1 className="text-2xl font-bold">Dashboard</h1>
      {inquiry && (
        <AgentActionNotification
          id={inquiry.id}
          summary={inquiry.summary}
          draftResponse={inquiry.draftResponse}
          actionSummary={inquiry.actionSummary}
          onApprove={() => setInquiry(null)}
          onEdit={() => setInquiry(null)}
          onDecline={() => setInquiry(null)}
        />
      )}
    </main>
  );
}
