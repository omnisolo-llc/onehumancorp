import React from 'react';

export function ErrorState({ title, message }: { title?: string; message: string }) {
  return (
    <div className="bg-red-50 p-4 rounded-md border border-red-200">
      {title && <h3 className="text-red-800 font-medium mb-1">{title}</h3>}
      <p className="text-red-600 text-sm">{message}</p>
    </div>
  );
}
