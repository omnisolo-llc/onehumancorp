"use client";

import React from 'react';
import LiveWebWidget from '../../components/chat/LiveWebWidget';
import { AppShell } from '../../components/AppShell';

export default function ChatWidgetDemoPage() {
  return (
    <AppShell>
      <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center font-inter p-6">
         <div className="max-w-2xl text-center space-y-6">
            <h1 className="text-4xl font-bold font-outfit text-gray-900 tracking-tight">Your Custom Website</h1>
            <p className="text-lg text-gray-600">
               This is a simulated external website. You should see the OHC Live Web Chat widget floating in the bottom right corner.
            </p>
            <div className="p-8 bg-white rounded-2xl shadow-sm border border-gray-100">
               <h2 className="text-xl font-semibold mb-4 text-gray-800">Products</h2>
               <div className="grid grid-cols-2 gap-4">
                  <div className="h-32 bg-gray-100 rounded-xl flex items-center justify-center text-gray-400">Product 1</div>
                  <div className="h-32 bg-gray-100 rounded-xl flex items-center justify-center text-gray-400">Product 2</div>
               </div>
            </div>
         </div>

         <LiveWebWidget tenantId="demo-tenant-123" />
      </div>
    </AppShell>
  );
}
