import React from 'react';

export const ErrorState = ({ title, message }: { title?: string; message: string }) => (
  <div className="p-4 rounded-xl text-red-700 dark:text-red-400 backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_4px_24px_rgba(0,0,0,0.04)]">
    {title && <h3 className="font-semibold mb-1">{title}</h3>}
    <p className="text-sm font-medium">{message}</p>
  </div>
);
