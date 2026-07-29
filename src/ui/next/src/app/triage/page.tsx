'use client';

import React from 'react';
import { WorkTriageFeed } from './components/WorkTriageFeed';
import { AppShell } from '../components/AppShell';

export default function TriagePage() {
  return (
    <AppShell>
      <WorkTriageFeed />
    </AppShell>
  );
}
