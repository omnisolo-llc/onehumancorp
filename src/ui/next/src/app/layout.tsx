import React from 'react';
import { TooltipProvider } from '../components/TooltipRegistry';
import { AiHelpChat } from '../components/AiHelpChat';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Outfit:wght@400;500;600;700&display=swap" rel="stylesheet" />
      </head>
      <body style={{ margin: 0, padding: 0, background: '#fdfdfd' }}>
        <TooltipProvider>
          {children}
          <AiHelpChat />
        </TooltipProvider>
      </body>
    </html>
  );
}
