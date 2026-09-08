import type { Metadata } from 'next';
import React from 'react';

export const metadata: Metadata = {
  title: 'Help Center | OmniSolo OneHumanCorp',
  description: 'In-App Help Center for OmniSolo work assistant.',
};

export default function HelpLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="help-layout">
      {children}
    </div>
  );
}
