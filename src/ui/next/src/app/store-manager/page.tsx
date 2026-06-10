import React from 'react';
import { PageHeader } from '@/components/layout/PageHeader';
import { StoreManagerChat } from './StoreManagerChat';

export default function StoreManagerPage() {
  return (
    <div className="flex flex-col h-screen max-w-md mx-auto bg-gray-50 pb-20 md:pb-0">
      <PageHeader title="Store Manager AI"  />
      <div className="flex-grow overflow-hidden">
        <StoreManagerChat />
      </div>
    </div>
  );
}
