import React from 'react';

export const PageHeader = ({ title, description }: { title: string; description?: string }) => (
  <div className="mb-6">
    <h1 className="text-2xl font-bold">{title}</h1>
    {description && <p className="text-gray-500">{description}</p>}
  </div>
);
