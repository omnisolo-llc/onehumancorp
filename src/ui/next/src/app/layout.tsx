"use client";

import './globals.css';
import { Inter } from 'next/font/google';
import HelpChatFloatingButton from '../components/help/HelpChatFloatingButton';
import WalkthroughOverlay from '../components/walkthrough/WalkthroughOverlay';
import TooltipRegistry from '../components/help/TooltipRegistry';

const inter = Inter({ subsets: ['latin'] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        {children}
        <HelpChatFloatingButton />
        <WalkthroughOverlay />
        <TooltipRegistry />
      </body>
    </html>
  );
}
