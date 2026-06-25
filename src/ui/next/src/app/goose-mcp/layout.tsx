import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Goose MCP',
  description: 'Goose MCP Extensions',
};

export default function Layout({ children }: { children: React.ReactNode }) {
  return <div>{children}</div>;
}
