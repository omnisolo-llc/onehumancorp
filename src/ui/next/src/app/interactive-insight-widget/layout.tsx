import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Insight Widget | OmniSolo',
  description: 'Interactive insight widget builder',
};

export default function InteractiveInsightWidgetLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <>{children}</>;
}
