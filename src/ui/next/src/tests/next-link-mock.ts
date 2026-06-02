import React from 'react';
export default function Link({ children, href }: { children: React.ReactNode, href: string }) {
  return React.createElement('a', { href }, children);
}