"use client";
import React, { useState, useEffect } from 'react';
import { QuoteReviewCard } from '../../components/QuoteReviewCard';

export default function QuoteFeed() {
  const [quotes, setQuotes] = useState([]);

  useEffect(() => {
    // In a real app this would fetch the quotes that need review.
    // For demo/testing we can just show empty state or fetch from an endpoint.
  }, []);

  const handleApprove = async (id: string) => {
    await fetch(`/api/quotes/${id}/approve`, { method: 'POST' });
    // refresh list...
  };

  return (
    <div className="flex flex-col items-center p-4">
      <h1 className="text-2xl mb-4">Active Quotes</h1>
      {quotes.length === 0 ? (
        <p>No quotes to review.</p>
      ) : (
        quotes.map(q => (
          <QuoteReviewCard
            key={q.id}
            quote={q}
            onApprove={handleApprove}
            onReject={() => {}}
            onEdit={() => {}}
          />
        ))
      )}
    </div>
  );
}
