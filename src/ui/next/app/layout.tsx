import React from 'react';
import { ContextualTooltip } from '../docsSystem';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <nav style={{ padding: '20px', borderBottom: '1px solid #eee', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div style={{ fontWeight: 'bold', fontSize: '20px', fontFamily: 'Outfit' }}>OHC</div>
            <div style={{ display: 'flex', gap: '20px' }}>
                <ContextualTooltip text="View and manage your product catalog here.">
                    <a href="#" style={{ textDecoration: 'none', color: '#333' }}>Products</a>
                </ContextualTooltip>
                <ContextualTooltip text="Check your daily sales and revenue history.">
                    <a href="#" style={{ textDecoration: 'none', color: '#333' }}>Sales</a>
                </ContextualTooltip>
                <ContextualTooltip text="Update your store name, payment methods, and AI agent settings.">
                    <a href="#" style={{ textDecoration: 'none', color: '#333' }}>Settings</a>
                </ContextualTooltip>
            </div>
        </nav>
        {children}
      </body>
    </html>
  );
}
