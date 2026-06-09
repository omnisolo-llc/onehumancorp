import React from 'react';

export const ErrorState = ({ title, message }: { title?: string; message: string }) => (
  <div className="p-4 bg-red-50 text-red-700 border border-red-200 rounded-md">
    {title && <h3 className="font-bold mb-1">{title}</h3>}
    {message}
  </div>
);
