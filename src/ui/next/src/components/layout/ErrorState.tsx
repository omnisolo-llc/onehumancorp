import React from 'react';

export function ErrorState({ title, message }: { title: string, message: string }) {
  return (
    <div className="bg-red-50 border border-red-200 p-4 rounded-md">
      <h3 className="text-sm font-medium text-red-800">{title}</h3>
      <p className="mt-2 text-sm text-red-700">{message}</p>
    </div>
  );
}
