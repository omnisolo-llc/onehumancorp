import './globals.css';
import type { Metadata } from 'next';
import { WalkthroughTarget } from '../components/Walkthrough';
import { WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';

import { HelpChat } from "../components/HelpChat";
import { OfflineSyncHandler } from "./OfflineSyncHandler";

export const metadata: Metadata = {
  title: 'OHC Builder',
  description: 'Automated storefront builder',
  manifest: "/manifest.json",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <head>
        <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
        <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
      </head>
      <body>
        <OfflineSyncHandler />
        <TooltipProvider>
                  <WalkthroughProvider>
            {children}
            <WalkthroughTarget id="help-widget-container"><HelpWidget /></WalkthroughTarget>
            <HelpChat />
          </WalkthroughProvider>
                </TooltipProvider>
      </body>
    </html>
  );
}
