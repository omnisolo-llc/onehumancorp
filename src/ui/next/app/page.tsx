import React from 'react';
import { HelpCenter, VideoTutorials, ApiDocumentation, ReleaseNotes, InteractiveWalkthrough, walkthroughData } from '../docsSystem';

export default function Home() {
  return (
    <main style={{ padding: '40px', fontFamily: 'Inter, sans-serif' }}>
      <h1 style={{ fontFamily: 'Outfit' }}>OneHumanCorp Dashboard</h1>
      <p>Welcome to your control center.</p>

      {/* Integrating documentation features directly into the main view for non-technical users */}
      <section style={{ marginTop: '40px' }}>
        <HelpCenter />
      </section>

      <section style={{ marginTop: '40px' }}>
        <VideoTutorials />
      </section>

      <section style={{ marginTop: '40px' }}>
        <ReleaseNotes />
      </section>

      <section style={{ marginTop: '40px' }}>
        <ApiDocumentation />
      </section>

      {/* Example Walkthrough overlay trigger (hidden by default in real app, shown for demonstration) */}
      <InteractiveWalkthrough steps={walkthroughData.setupStore} onComplete={() => console.log('Walkthrough done')} />
    </main>
  );
}
