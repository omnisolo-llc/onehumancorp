export interface FxRate {
  base_currency: string;
  target_currency: string;
  rate: number;
}

export interface FxMargin {
  base_currency: string;
  target_currency: string;
  safe_margin: number;
}

export interface I18nString {
  key: string;
  language: string;
  value: string;
}

export class OfflineLocalizationSync {
  static async syncFxRates(tenantId: string): Promise<FxRate[]> {
    try {
      const response = await fetch(`/api/localization/fx-rates/${tenantId}`);
      const data = await response.json();
      localStorage.setItem(`fx_rates_${tenantId}`, JSON.stringify(data));
      return data;
    } catch (e) {
      console.warn("Failed to fetch fx rates, using cache");
      const cached = localStorage.getItem(`fx_rates_${tenantId}`);
      return cached ? JSON.parse(cached) : [];
    }
  }

  static async syncFxMargins(tenantId: string): Promise<FxMargin[]> {
    try {
      const response = await fetch(`/api/localization/fx-margins/${tenantId}`);
      const data = await response.json();
      localStorage.setItem(`fx_margins_${tenantId}`, JSON.stringify(data));
      return data;
    } catch (e) {
      console.warn("Failed to fetch fx margins, using cache");
      const cached = localStorage.getItem(`fx_margins_${tenantId}`);
      return cached ? JSON.parse(cached) : [];
    }
  }

  static async syncI18nStrings(tenantId: string, language: string): Promise<Record<string, string>> {
    try {
      const response = await fetch(`/api/localization/i18n/${tenantId}/${language}`);
      const data: I18nString[] = await response.json();
      const stringsRecord = data.reduce((acc, curr) => {
        acc[curr.key] = curr.value;
        return acc;
      }, {} as Record<string, string>);
      localStorage.setItem(`i18n_${tenantId}_${language}`, JSON.stringify(stringsRecord));
      return stringsRecord;
    } catch (e) {
      console.warn("Failed to fetch i18n strings, using cache");
      const cached = localStorage.getItem(`i18n_${tenantId}_${language}`);
      return cached ? JSON.parse(cached) : {};
    }
  }

  static getCachedI18nString(tenantId: string, language: string, key: string, fallback: string = ""): string {
    const cached = localStorage.getItem(`i18n_${tenantId}_${language}`);
    if (cached) {
      const parsed = JSON.parse(cached);
      return parsed[key] || fallback;
    }
    return fallback;
  }
}
