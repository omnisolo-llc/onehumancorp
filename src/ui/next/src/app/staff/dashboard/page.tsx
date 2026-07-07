import React from 'react';
import { StaffDashboard } from '@/components/StaffDashboard';

export default function StaffDashboardPage() {
  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center">
      <header className="w-full max-w-md bg-white border-b border-gray-200 px-4 py-3 sticky top-0 z-10 backdrop-blur-md bg-white/70">
        <h1 className="text-xl font-semibold text-gray-900">Shift Dashboard</h1>
      </header>
      <main className="flex-1 w-full max-w-md p-4">
        <StaffDashboard />
      </main>
    </div>
  );
}
