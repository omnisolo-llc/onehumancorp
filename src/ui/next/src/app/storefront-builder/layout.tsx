import React from 'react';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Storefront Builder',
  openGraph: {
    images: [
      {
        url: 'https://ohc.store/api/v1/growth/storefront/og-card?tenant=mybusiness&product_name=NovaPremium',
      },
    ],
  },
};

export default function Layout({ children }: { children: React.ReactNode }) {
  return <>{children}</>;
}
