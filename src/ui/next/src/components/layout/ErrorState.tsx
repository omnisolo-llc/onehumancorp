import React from 'react';

export const ErrorState = ({ title, message }: { title?: string; message: string }) => (
  <div className="p-4 bg-red-50/65 dark:bg-red-900/40 text-red-800 dark:text-red-200 border border-red-200/40 dark:border-red-800/40 rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[210%] shadow-sm glassmorphism">
    {title && <h3 className="font-bold mb-1">{title}</h3>}
    {message}
  </div>
);
