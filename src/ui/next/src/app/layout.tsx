import './globals.css';
import type { Metadata } from 'next';
import { WalkthroughTarget } from '../components/Walkthrough';
import { WalkthroughProvider, HelpWidget } from '../components/help';
import { TooltipProvider } from '../components/TooltipRegistry';
import { HelpChat } from '../components/HelpChat';

import { NetworkStatusIndicator } from "../components/NetworkStatusIndicator";
import { SyncManagerInitializer } from "../components/SyncManagerInitializer";
import { NotificationManager } from "../components/NotificationManager";
import { RateLimitWarningProvider } from '../components/RateLimitWarning';
import { ProductShellGuard } from './components/ProductShellGuard';
import { OmnichannelChatWidget } from '../components/ui/OmnichannelChatWidget';

export const viewport = {
  width: 'device-width',
  initialScale: 1,
  maximumScale: 1,
};

export const metadata: Metadata = {
  title: 'In-App Help Center',
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
        <RateLimitWarningProvider>
          <TooltipProvider>
                    <WalkthroughProvider>
              <ProductShellGuard>{children}</ProductShellGuard>
              <WalkthroughTarget id="ohc-floating-help-widget"><HelpWidget /></WalkthroughTarget>
              <HelpChat />
              <NetworkStatusIndicator />
              <SyncManagerInitializer />
              <NotificationManager />
              <OmnichannelChatWidget />
            </WalkthroughProvider>
                  </TooltipProvider>
        </RateLimitWarningProvider>
      </body>
    </html>
  );
}
