import React from 'react';

export const PageHeader = ({ title, description, backUrl }: { title: string; description?: string; backUrl?: string }) => (
  <div className="mb-6 p-4 rounded-xl backdrop-blur-xl bg-white/60 shadow-[0_4px_24px_rgba(0,0,0,0.04)] border border-white/40 sticky top-0 z-10">
    <h1 className="text-3xl font-semibold tracking-tight text-gray-900 drop-shadow-sm">{title}</h1>
    {description && <p className="mt-1 text-sm text-gray-600/90 font-medium">{description}</p>}
  </div>
);
