import React from 'react';
import { PageHeader } from '@/components/layout/PageHeader';
import { StoreManagerChat } from './StoreManagerChat';

export default function StoreManagerPage() {
  return (
    <div className="flex flex-col h-screen max-w-md mx-auto bg-gray-50 pb-20 md:pb-0">
<<<<<<< HEAD
      <PageHeader title="Store Manager AI"  />
=======
      <PageHeader title="Store Manager AI" backUrl="/dashboard" />
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)
      <div className="flex-grow overflow-hidden">
        <StoreManagerChat />
      </div>
    </div>
  );
}
