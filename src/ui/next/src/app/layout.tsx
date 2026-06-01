import './globals.css';
import type { Metadata } from 'next';
import { WalkthroughTarget } from '../components/Walkthrough';
import { WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';
import Link from 'next/link';

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
    <html lang="en">
      <head>
        <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
        <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
      </head>
      <body>
        <TooltipProvider>
          <WalkthroughProvider>
            {/* Top nav to ensure testability */}
            <div style={{ display: 'flex', padding: '16px', background: '#f8f9fa', borderBottom: '1px solid #e2e8f0' }} aria-label="Global Audit Navigation">
                <Link href="/agent-audit-dashboard" data-testid="agent-audit-link" style={{ fontWeight: 'bold', color: '#0066FF' }}>
                  Agent Audit Dashboard
                </Link>
            </div>
            {children}
            <WalkthroughTarget id="help-widget-container"><HelpWidget /></WalkthroughTarget>
            <HelpChat />
          </WalkthroughProvider>
        </TooltipProvider>
      </body>
    </html>
  );
}
