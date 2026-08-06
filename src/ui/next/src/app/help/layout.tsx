import type { Metadata } from 'next';
import React from 'react';

export const metadata: Metadata = {
  title: 'Help Center | OneHumanCorp',
  description: 'In-App Help Center for OneHumanCorp work assistant.',
};

export default function HelpLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="help-layout bg-gradient-to-b from-[#F5F5F7] to-[#E8E8ED] dark:from-[#16161a] dark:to-[#0f0f13] min-h-screen">
      {children}
    </div>
  );
}
