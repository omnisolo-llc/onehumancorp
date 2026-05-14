import React from 'react';
import './globals.css';

export const metadata = {
  title: 'One Human Corp',
  description: 'Your business, live in minutes.',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
