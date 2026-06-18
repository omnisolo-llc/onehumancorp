import React from 'react';
import { SetupIcon, SetupIconName } from './SetupIcon';

export function IconLabel({ icon, children }: { icon: SetupIconName; children: React.ReactNode }) {
  return (
    <span className="inline-flex items-center justify-center gap-2 flex-none">
      <span className="flex-none inline-flex items-center justify-center w-4 h-4">
        <SetupIcon name={icon} />
      </span>
      <span className="whitespace-nowrap">{children}</span>
    </span>
  );
}
