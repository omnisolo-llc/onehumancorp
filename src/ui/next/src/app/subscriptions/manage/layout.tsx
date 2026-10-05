import type { Metadata } from 'next';
import type { ReactNode } from 'react';

export const metadata: Metadata = {
  title: 'Manage Subscriptions | OmniSolo OneHumanCorp',
};

export default function RouteLayout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
