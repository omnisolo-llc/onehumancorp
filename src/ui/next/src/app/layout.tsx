import './globals.css';
import { Outfit, Inter } from 'next/font/google';

const outfit = Outfit({ subsets: ['latin'], variable: '--font-outfit' });
const inter = Inter({ subsets: ['latin'], variable: '--font-inter' });

import type { Metadata } from 'next';
import { WalkthroughTarget } from '../components/Walkthrough';
import { WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';

import { HelpChat } from "../components/HelpChat";

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
    <html lang="en" className={`${outfit.variable} ${inter.variable}`}>
      <head>
        <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
        <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
      </head>
      <body>
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
