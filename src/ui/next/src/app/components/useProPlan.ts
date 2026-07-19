"use client";

import { useCallback, useEffect, useState } from 'react';

export function useProPlan() {
  const [hasPro, setHasPro] = useState(false);
  const [planError, setPlanError] = useState<string | null>(null);
  const [claimError, setClaimError] = useState<string | null>(null);

  const refreshPlan = useCallback(async () => {
    try {
      const response = await fetch('/api/v1/billing/my-plan');
      if (!response.ok) throw new Error('Plan data is unavailable.');
      const data = await response.json();
      if (typeof data.current_plan !== 'string') throw new Error('Plan data is unavailable.');
      setHasPro(['pro', 'business'].includes(data.current_plan.toLowerCase()));
      setPlanError(null);
    } catch {
      setHasPro(false);
      setPlanError('Plan data is unavailable.');
    }
  }, []);

  useEffect(() => {
    void refreshPlan();
  }, [refreshPlan]);

  const claimTrial = useCallback(async () => {
    setClaimError(null);
    try {
      const response = await fetch('/api/v1/growth/trial-extension/claim', { method: 'POST' });
      if (!response.ok) throw new Error('Pro activation is unavailable.');
      setHasPro(true);
      return true;
    } catch {
      setClaimError('Pro activation is unavailable.');
      return false;
    }
  }, []);

  return {
    hasPro,
    planError,
    claimError,
    claimTrial,
    refreshPlan,
    confirmPro: () => setHasPro(true),
  };
}
