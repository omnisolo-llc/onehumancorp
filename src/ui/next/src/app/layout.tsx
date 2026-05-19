import './globals.css';
import type { Metadata } from 'next';
import { TooltipRegistryProvider, WalkthroughProvider, HelpWidget } from '../components/help';

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
        <TooltipRegistryProvider>
          <WalkthroughProvider>
            {children}
            <HelpWidget />
          </WalkthroughProvider>
        </TooltipRegistryProvider>
      </body>
    </html>
  );
}
