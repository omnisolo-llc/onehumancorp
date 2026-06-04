import React from 'react';

export default function BuilderLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-gray-100 flex justify-center items-center">
      {children}
    </div>
  );
}
