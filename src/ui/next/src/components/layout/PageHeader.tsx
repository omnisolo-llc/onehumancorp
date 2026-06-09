import React from 'react';

export function PageHeader({ title, description, subtitle }: { title: string; description?: string; subtitle?: string }) {
  const subtext = subtitle || description;
  return (
    <div className="mb-6">
      <h1 className="text-2xl font-semibold text-gray-900 dark:text-white">{title}</h1>
      {subtext && <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">{subtext}</p>}
    </div>
  );
}
