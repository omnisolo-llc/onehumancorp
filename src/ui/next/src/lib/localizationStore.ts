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
  t: (key: string, options?: { count?: number }) => string;
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
      t: (key, options) => {
        let translated = get().translations[key] || key;
        if (options?.count !== undefined) {
          translated = translated.replace('{{count}}', options.count.toString());
        }
        return translated;
      },
      convert: (amount, from, to) => {
        if (from === to) return { amount, rate: 1.0, isOffline: false };
        const rateEntry = get().fxRates.find(r => r.from === from && r.to === to);
        if (rateEntry) {
          return {
            amount: Math.round(amount * rateEntry.rate),
            rate: rateEntry.rate,
            isOffline: !navigator.onLine
          };
        }
        // Fallback or error handling
        return { amount, rate: 1.0, isOffline: false };
      }
    }),
    {
      name: 'ohc-localization-storage',
      storage: createJSONStorage(() => localStorage),
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
