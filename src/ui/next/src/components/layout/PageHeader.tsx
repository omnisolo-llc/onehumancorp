import React from 'react';

export const PageHeader = ({ title, description }: { title: string; description?: string }) => (
  <div className="mb-6 p-4 rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[2.1] bg-white/65 dark:bg-[#16161a]/70 shadow-[0_4px_24px_rgba(0,0,0,0.04)] border border-white/40 dark:border-white/10 sticky top-0 z-10 transition-colors duration-300">
    <h1 className="text-[32px] font-bold tracking-tight text-gray-900 dark:text-gray-100 drop-shadow-sm font-outfit">{title}</h1>
    {description && <p className="mt-2 text-sm text-gray-600 dark:text-gray-400 font-medium">{description}</p>}
  </div>
);
