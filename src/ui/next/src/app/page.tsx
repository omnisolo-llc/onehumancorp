"use client";
import React, { useState } from 'react';
import { HelpCenter } from '../components/HelpCenter';
import { ApiDocs } from '../components/ApiDocs';
import { ReleaseNotes } from '../components/ReleaseNotes';
import { Walkthrough } from '../components/Walkthrough';
import { TooltipTarget } from '../components/TooltipRegistry';

export default function Page() {
  const [tab, setTab] = useState('help');

  const navStyle = { padding: '10px 20px', cursor: 'pointer', border: 'none', background: 'none', fontSize: '16px', fontFamily: 'Outfit, sans-serif' };

  return (
    <div>
      <nav style={{ padding: '20px', borderBottom: '1px solid #eaeaea', display: 'flex', gap: '20px', background: '#fff' }}>
        <TooltipTarget id="nav-help" content="Find answers and video guides here.">
          <button onClick={() => setTab('help')} style={{...navStyle, fontWeight: tab === 'help' ? 'bold' : 'normal', color: tab === 'help' ? '#0070f3' : '#333'}}>Help Center</button>
        </TooltipTarget>
        <TooltipTarget id="nav-updates" content="See what is new in the latest version.">
          <button onClick={() => setTab('updates')} style={{...navStyle, fontWeight: tab === 'updates' ? 'bold' : 'normal', color: tab === 'updates' ? '#0070f3' : '#333'}}>What's New</button>
        </TooltipTarget>
        <TooltipTarget id="nav-api" content="Advanced tools for custom integrations.">
          <button onClick={() => setTab('api')} style={{...navStyle, fontWeight: tab === 'api' ? 'bold' : 'normal', color: tab === 'api' ? '#0070f3' : '#333'}}>Advanced (API)</button>
        </TooltipTarget>
      </nav>

      <main>
        {tab === 'help' && (
          <>
            <HelpCenter />
            <Walkthrough />
          </>
        )}
        {tab === 'updates' && <ReleaseNotes />}
        {tab === 'api' && <ApiDocs />}
      </main>
    </div>
  );
}
