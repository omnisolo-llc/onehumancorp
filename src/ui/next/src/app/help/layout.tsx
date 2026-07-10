import type { Metadata } from 'next';
import React from 'react';

export const metadata: Metadata = {
  title: 'Help Center | OneHumanCorp',
  description: 'In-App Help Center for OneHumanCorp work assistant.',
};

export default function HelpLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="help-layout">
      {children}
    </div>
  );
}
