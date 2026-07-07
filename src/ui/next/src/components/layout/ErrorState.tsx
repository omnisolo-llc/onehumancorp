import React from 'react';

export const ErrorState = ({ title, message }: { title?: string; message: string }) => (
  <div className="p-4 rounded-xl text-red-700 dark:text-red-400 backdrop-blur-[30px] backdrop-saturate-[210%] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-[0_4px_24px_rgba(0,0,0,0.04)]">
    {title && <h3 className="font-semibold mb-1">{title}</h3>}
    <p className="text-sm font-medium">{message}</p>
  </div>
);
