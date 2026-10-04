'use client';

import { useParams } from 'next/navigation';
import { CustomerQuoteView } from '@/components/quotes/CustomerQuoteView';

export default function InteractiveQuotePage() {
  const { id: rawId } = useParams<{ id: string }>();
  const id = typeof rawId === 'string' ? rawId.toLowerCase() : '';
  return <CustomerQuoteView key={id} id={id} />;
}
