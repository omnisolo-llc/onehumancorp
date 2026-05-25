import './globals.css';
import type { Metadata } from 'next';
import {  WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';
import { HelpChat } from '../components/HelpChat';

export const metadata: Metadata = {
  title: 'OHC Builder',
  description: 'Automated storefront builder',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <TooltipProvider>

          <WalkthroughProvider>
            {children}
            <HelpWidget />
            <HelpChat />
          </WalkthroughProvider>

        </TooltipProvider>
      </body>
    </html>
  );
}
