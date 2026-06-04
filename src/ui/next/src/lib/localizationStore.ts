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
  syncFromBackend: () => Promise<void>;
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
      setLocale: (locale) => {
        set({ locale });
        get().syncFromBackend();
      },
      setCurrency: (currency) => set({ currency }),
      setTranslations: (translations) => set({ translations }),
      setFxRates: (fxRates) => set({ fxRates }),
      syncFromBackend: async () => {
        if (!navigator.onLine) return;
        try {
          const locale = get().locale;
          const [transRes, fxRes] = await Promise.all([
            fetch(`/api/v1/localization/translations/${locale}`),
            fetch('/api/v1/localization/fx_rates')
          ]);
          if (transRes.ok) {
            const data: { key: string; value: string }[] = await transRes.json();
            const map: Record<string, string> = {};
            data.forEach(item => map[item.key] = item.value);
            set({ translations: map });
          }
          if (fxRes.ok) {
            const fxData: FxRate[] = await fxRes.json();
            set({ fxRates: fxData });
          }
        } catch (e) {
          console.error("Localization sync failed", e);
        }
      },
      t: (key) => get().translations[key] || key,
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
