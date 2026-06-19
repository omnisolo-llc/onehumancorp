import React from 'react';
import { SetupIcon, SetupIconName } from './SetupIcon';

export function IconLabel({ icon, children }: { icon: SetupIconName; children: React.ReactNode }) {
  return (
    <span className="inline-flex items-center justify-center gap-2 flex-none font-inter">
      <span className="flex-none inline-flex items-center justify-center w-4 h-4 transition-transform duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] group-hover:scale-110">
        <SetupIcon name={icon} />
      </span>
      <span className="whitespace-nowrap font-bold">{children}</span>
    </span>
  );
}
