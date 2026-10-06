import type { ReactNode } from 'react';
import LegacyPaymentNotice from './LegacyPaymentNotice';

export default function POSLayout({ children }: { children: ReactNode }) {
  return <><LegacyPaymentNotice />{children}</>;
}
