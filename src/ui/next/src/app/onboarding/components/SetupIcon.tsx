import React from 'react';

export type SetupIconName = 'dashboard' | 'eye' | 'launch' | 'next' | 'save' | 'sparkles';

export function SetupIcon({ name }: { name: SetupIconName }) {
  const paths: Record<SetupIconName, string[]> = {
    dashboard: ['M4 5h7v7H4z', 'M13 5h7v4h-7z', 'M13 11h7v8h-7z', 'M4 14h7v5H4z'],
    eye: ['M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z', 'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z'],
    launch: ['M13 10V3L4 14h7v7l9-11h-7z'],
    next: ['M5 12h14', 'M13 6l6 6-6 6'],
    save: ['M5 4h12l2 2v16H5z', 'M8 4v7h8V4', 'M8 18h8'],
    sparkles: ['M21 12l-3-1 1-3 1 3 3 1-3 1-1 3-1-3zM8 21l-3-4-4-3 4-3 3-4 3 4 4 3-4 3z'],
  };

  return (
    <svg className="h-4 w-4 flex-none" aria-hidden="true" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} viewBox="0 0 24 24">
      {paths[name].map((d) => <path key={d} d={d} />)}
    </svg>
  );
}
