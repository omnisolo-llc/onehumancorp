"use client";

import React, { useEffect } from 'react';
import { useLocalizationStore } from '../lib/localizationStore';

export const LocalizationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { locale, setFxRates, setTranslations } = useLocalizationStore();

  useEffect(() => {
    // Only fetch if online
    if (typeof window !== 'undefined' && navigator.onLine) {
      Promise.all([
        fetch('/api/v1/localization/fx_rates').then(r => r.ok ? r.json() : []),
        fetch(`/api/v1/localization/translations/${locale}`).then(r => r.ok ? r.json() : [])
      ]).then(([rates, transArray]) => {
        if (rates && Array.isArray(rates)) {
          setFxRates(rates);
        }
        if (transArray && Array.isArray(transArray)) {
          const transMap = transArray.reduce((acc: any, t: any) => {
            acc[t.key] = t.value;
            return acc;
          }, {});
          setTranslations(transMap);
        }
      }).catch(err => {
        console.error("Failed to fetch localization data", err);
      });
    }
  }, [locale, setFxRates, setTranslations]);

  return <>{children}</>;
};
