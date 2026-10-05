import type { Metadata } from 'next';
import type { ReactNode } from 'react';

export const metadata: Metadata = {
  title: 'Exit-Intent Pop-up Builder | OmniSolo OneHumanCorp',
};

export default function RouteLayout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
