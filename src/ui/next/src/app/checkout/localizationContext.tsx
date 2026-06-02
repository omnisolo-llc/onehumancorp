"use client";

import React, { createContext, useContext, useState, useEffect } from 'react';

type LocalizationContextType = {
  locale: string;
  currency: string;
  setLocale: (l: string) => void;
  setCurrency: (c: string) => void;
  t: (key: string, defaultText: string) => string;
  formatCurrency: (amount: number) => string;
  isOffline: boolean;
};

const LocalizationContext = createContext<LocalizationContextType | undefined>(undefined);

export function LocalizationProvider({ children }: { children: React.ReactNode }) {
  const [locale, setLocale] = useState('en');
  const [currency, setCurrency] = useState('USD');
  const [isOffline, setIsOffline] = useState(false);
  const [translations, setTranslations] = useState<Record<string, string>>({});
  const [fxRate, setFxRate] = useState<number>(1.0);

  useEffect(() => {
    const checkOnline = () => setIsOffline(!navigator.onLine);
    window.addEventListener('online', checkOnline);
    window.addEventListener('offline', checkOnline);
    checkOnline();
    return () => {
      window.removeEventListener('online', checkOnline);
      window.removeEventListener('offline', checkOnline);
    };
  }, []);

  useEffect(() => {
    const fetchLocalizationData = async () => {
      try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';

        // Use cached data if offline
        if (isOffline) {
           const cachedTranslations = localStorage.getItem(`translations_${locale}`);
           if (cachedTranslations) {
               setTranslations(JSON.parse(cachedTranslations));
           }
           const cachedFxRate = localStorage.getItem(`fx_rate_${currency}`);
           if (cachedFxRate) {
               setFxRate(parseFloat(cachedFxRate));
           }
           return;
        }

        const transRes = await fetch(`/api/v1/localization/${tenantId}/translations/${locale}/bulk`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ keys: ['checkout.title', 'checkout.description', 'checkout.pay_now', 'checkout.cancel'] })
        });
        if (transRes.ok) {
           const data = await transRes.json();
           setTranslations(data.translations || {});
           localStorage.setItem(`translations_${locale}`, JSON.stringify(data.translations || {}));
        }

        const fxRes = await fetch(`/api/v1/localization/${tenantId}/fx/${currency}`);
        if (fxRes.ok) {
           const data = await fxRes.json();
           setFxRate(data.rate);
           localStorage.setItem(`fx_rate_${currency}`, data.rate.toString());
        } else {
           setFxRate(1.0);
        }
      } catch (e) {
        console.error("Localization fetch failed", e);
      }
    };
    fetchLocalizationData();
  }, [locale, currency, isOffline]);

  const t = (key: string, defaultText: string) => translations[key] || defaultText;

  const formatCurrency = (amount: number) => {
     const converted = amount * fxRate;
     return new Intl.NumberFormat(locale, { style: 'currency', currency }).format(converted);
  };

  return (
    <LocalizationContext.Provider value={{ locale, currency, setLocale, setCurrency, t, formatCurrency, isOffline }}>
      {children}
    </LocalizationContext.Provider>
  );
}

export function useLocalization() {
  const context = useContext(LocalizationContext);
  if (!context) throw new Error('useLocalization must be used within LocalizationProvider');
  return context;
}
