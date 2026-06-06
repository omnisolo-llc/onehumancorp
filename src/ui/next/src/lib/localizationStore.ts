import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

interface FxRate {
  from: string;
  to: string;
  rate: number;
}

interface LocalizationState {
  locale: string;
  currency: string;
  translations: Record<string, string>;
  fxRates: FxRate[];
  setLocale: (locale: string) => void;
  setCurrency: (currency: string) => void;
  setTranslations: (translations: Record<string, string>) => void;
  setFxRates: (rates: FxRate[]) => void;
  syncLocalization: () => Promise<void>;
  t: (key: string) => string;
  convert: (amount: number, from: string, to: string) => { amount: number; rate: number; isOffline: boolean };
}

export const useLocalizationStore = create<LocalizationState>()(
  persist(
    (set, get) => ({
      locale: 'en',
      currency: 'USD',
      translations: {},
      fxRates: [],
      setLocale: (locale) => set({ locale }),
      setCurrency: (currency) => set({ currency }),
      setTranslations: (translations) => set({ translations }),
      setFxRates: (fxRates) => set({ fxRates }),
      syncLocalization: async () => {
        try {
          // If offline, skip sync to rely on cached state.
          if (typeof navigator !== 'undefined' && !navigator.onLine) return;

          const response = await fetch('/api/v1/localization/sync');
          if (response.ok) {
             const data = await response.json();
             set({
               locale: data.locale || get().locale,
               translations: data.translations || {},
               fxRates: data.fx_rates || [],
             });
          }
        } catch (error) {
          console.error("Failed to sync localization:", error);
        }
      },
      t: (key) => get().translations[key] || key,
      convert: (amount, from, to) => {
        if (from === to) return { amount, rate: 1.0, isOffline: typeof navigator !== 'undefined' && !navigator.onLine };
        const rateEntry = get().fxRates.find(r => r.from === from && r.to === to);
        const isOffline = typeof navigator !== 'undefined' && !navigator.onLine;

        if (rateEntry) {
          let convertedAmount = amount * rateEntry.rate;

          if (isOffline) {
              // Apply cosmetic rounding offline (e.g. 49.99 format, converting cents back and forth)
              // amount is typically in cents.
              const dollars = convertedAmount / 100;
              const roundedDollars = Math.ceil(dollars) - 0.01;
              convertedAmount = Math.round(roundedDollars * 100);
          } else {
              convertedAmount = Math.round(convertedAmount);
          }

          return {
            amount: convertedAmount,
            rate: rateEntry.rate,
            isOffline
          };
        }
        // Fallback
        return { amount, rate: 1.0, isOffline };
      }
    }),
    {
      name: 'ohc-localization-storage',
      storage: createJSONStorage(() => {
        if (typeof window !== 'undefined') return localStorage;
        return {
          getItem: () => null,
          setItem: () => {},
          removeItem: () => {},
        };
      }),
    }
  )
);

export const useTranslation = () => {
  const t = useLocalizationStore((state) => state.t);
  const locale = useLocalizationStore((state) => state.locale);
  return { t, locale };
};

export const useCurrency = () => {
  const currency = useLocalizationStore((state) => state.currency);
  const convert = useLocalizationStore((state) => state.convert);
  const setCurrency = useLocalizationStore((state) => state.setCurrency);
  return { currency, convert, setCurrency };
};
