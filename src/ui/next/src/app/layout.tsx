import './globals.css';
import type { Metadata } from 'next';
import { WalkthroughTarget } from '../components/Walkthrough';
import { WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';

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
            {children}
            <WalkthroughTarget id="help-widget-container"><HelpWidget /></WalkthroughTarget>
          </WalkthroughProvider>
        </TooltipProvider>
      </body>
    </html>
  );
}
