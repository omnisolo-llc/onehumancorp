import React from 'react';
import { HelpCenter } from '../../components/help/HelpCenter';
import { VideoTutorials } from '../../components/help/VideoTutorials';
import { ReleaseNotes } from '../../components/help/ReleaseNotes';
import { FloatingHelpChat } from '../../components/help/FloatingHelpChat';
import { TooltipProvider, ContextualTooltip } from '../../components/help/TooltipRegistry';

export default function HelpPage() {
  return (
    <TooltipProvider>
      <div style={{ background: '#f5f7fa', minHeight: '100vh', paddingBottom: '100px' }}>
        <header style={{ background: 'white', padding: '16px 20px', borderBottom: '1px solid #eaeaea', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <h1 style={{ margin: 0, fontSize: '20px', fontFamily: 'Outfit, sans-serif' }}>OHC Support</h1>

          <ContextualTooltip id="api-docs-link" defaultText="For developers only: API documentation">
            <a href="/help/api" style={{ color: '#0056b3', textDecoration: 'none', fontSize: '14px', fontFamily: 'Inter, sans-serif' }}>Advanced: API Docs</a>
          </ContextualTooltip>
        </header>

        <main style={{ padding: '40px 20px' }}>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '40px', maxWidth: '1000px', margin: '0 auto' }}>
            <div style={{ background: 'white', borderRadius: '12px', boxShadow: '0 2px 12px rgba(0,0,0,0.05)', overflow: 'hidden' }}>
              <HelpCenter />
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px' }}>
              <div style={{ background: 'white', borderRadius: '12px', boxShadow: '0 2px 12px rgba(0,0,0,0.05)', overflow: 'hidden' }}>
                <VideoTutorials />
              </div>

              <div style={{ background: 'white', borderRadius: '12px', boxShadow: '0 2px 12px rgba(0,0,0,0.05)', overflow: 'hidden' }}>
                <ReleaseNotes />
              </div>
            </div>
          </div>
        </main>

        <FloatingHelpChat />
      </div>
    </TooltipProvider>
  );
}
