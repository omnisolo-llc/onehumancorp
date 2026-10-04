'use client';

import { Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import { CustomerQuoteView } from '@/components/quotes/CustomerQuoteView';

function CustomerProposalViewContent() {
  const id = (useSearchParams().get('id') ?? '').toLowerCase();
  return <CustomerQuoteView key={id} id={id} />;
}

export default function CustomerProposalView() {
  return <Suspense fallback={<p role="status" aria-busy="true">Loading quote...</p>}>
    <CustomerProposalViewContent />
  </Suspense>;
}
