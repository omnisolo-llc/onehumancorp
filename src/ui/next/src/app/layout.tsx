import './globals.css';
import type { Metadata } from 'next';
import { WalkthroughTarget } from '../components/Walkthrough';
import { WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';
import { HelpChat } from '../components/HelpChat';
import { FetchInterceptor } from '../components/FetchInterceptor';

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
            <FetchInterceptor>
              {children}
            </FetchInterceptor>
            <WalkthroughTarget id="help-widget-container"><HelpWidget /></WalkthroughTarget>
            <HelpChat />
          </WalkthroughProvider>
        </TooltipProvider>
      </body>
    </html>
  );
}
