import React from 'react';
import TooltipRegistry from '../components/help/TooltipRegistry';
import Walkthrough from '../components/help/Walkthrough';
import HelpChat from '../components/help/HelpChat';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        {children}
        <TooltipRegistry />
        <Walkthrough />
        <HelpChat />
      </body>
    </html>
  );
}
