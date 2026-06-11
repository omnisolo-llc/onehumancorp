"use client";

import { useEffect, useState } from "react";
import { PowerSyncContext, db, setupPowerSync } from "../lib/powersync";

export function PowerSyncProvider({ children }: { children: React.ReactNode }) {
  const [powerSyncInitialized, setPowerSyncInitialized] = useState(false);

  useEffect(() => {
    async function init() {
      await setupPowerSync();
      setPowerSyncInitialized(true);
    }
    init();
  }, []);

  return (
    <PowerSyncContext.Provider value={db}>
      {children}
    </PowerSyncContext.Provider>
  );
}