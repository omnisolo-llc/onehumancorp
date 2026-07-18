"use client";

import React from 'react';
import { AppShell } from '../components/AppShell';

export default function ServicesPage() {
  return (
    <AppShell
      title="Service Manager"
      subtitle="Monitor runtime health and restart policy for local services."
      statusItems={[{ label: 'Telemetry', value: 'Unavailable', tone: 'warn' }]}
    >
      <section id="services-screen" className="app-panel glassmorphism">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Runtime Controls</div>
            <div className="app-list-subtitle font-inter text-[#86868B] dark:text-[#A1A1A6]">
              Live resource metrics and restart controls are not exposed by the service API.
            </div>
          </div>
          <span className="app-badge warn bg-[#FF9500] text-white dark:bg-[#FF9F1A]">unavailable</span>
        </div>
        <div className="app-panel-body">
          <p className="text-sm text-gray-600 dark:text-gray-400 font-inter" role="status">
            No runtime status is being reported. Configure a real health and service-control API before managing services here.
          </p>
        </div>
      </section>
    </AppShell>
  );
}
