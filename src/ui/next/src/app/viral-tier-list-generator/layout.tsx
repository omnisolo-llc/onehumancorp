import type { Metadata } from 'next';
import type { ReactNode } from 'react';

export const metadata: Metadata = {
  title: 'Viral Tier List Generator',
};

export default function RouteLayout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
