"use client";

import { useCallback, useEffect, useState } from "react";

const AVAILABLE_CURRENCIES = ["USD", "EUR", "GBP", "CAD", "AUD", "JPY"] as const;
type Currency = (typeof AVAILABLE_CURRENCIES)[number];

type CurrencySettings = Readonly<{
  base_currency: Currency;
  enabled_currencies: Currency[];
}>;

function isCurrency(value: unknown): value is Currency {
  return typeof value === "string" && AVAILABLE_CURRENCIES.includes(value as Currency);
}

function parseSettings(value: unknown): CurrencySettings | null {
  if (value === null || typeof value !== "object" || !("tenant" in value)) return null;
  const tenant = (value as { tenant?: unknown }).tenant;
  if (tenant === null || typeof tenant !== "object") return null;
  const candidate = tenant as Record<string, unknown>;
  if (!isCurrency(candidate.base_currency) || !Array.isArray(candidate.enabled_currencies)) return null;
  const enabled = [...new Set(candidate.enabled_currencies.filter(isCurrency))];
  if (!enabled.includes(candidate.base_currency)) return null;
  return { base_currency: candidate.base_currency, enabled_currencies: enabled };
}

export default function GlobalCommerceSettingsPage() {
  const [baseCurrency, setBaseCurrency] = useState<Currency>("USD");
  const [enabledCurrencies, setEnabledCurrencies] = useState<Currency[]>(["USD"]);
  const [isLoading, setIsLoading] = useState(true);
  const [hasLoaded, setHasLoaded] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);

  const loadSettings = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await fetch("/api/v1/settings/global-commerce", { cache: "no-store" });
      const settings = parseSettings(await response.json());
      if (!response.ok || settings === null) throw new Error("settings unavailable");
      setBaseCurrency(settings.base_currency);
      setEnabledCurrencies(settings.enabled_currencies);
      setHasLoaded(true);
    } catch {
      setError("Currency settings are unavailable.");
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadSettings();
  }, [loadSettings]);

  function changeBaseCurrency(currency: Currency) {
    setBaseCurrency(currency);
    setEnabledCurrencies((current) => current.includes(currency) ? current : [...current, currency]);
    setStatus(null);
  }

  function toggleCurrency(currency: Currency) {
    if (currency === baseCurrency) return;
    setEnabledCurrencies((current) => current.includes(currency)
      ? current.filter((item) => item !== currency)
      : [...current, currency]);
    setStatus(null);
  }

  async function saveSettings() {
    if (isSaving) return;
    setIsSaving(true);
    setError(null);
    setStatus(null);
    try {
      const response = await fetch("/api/v1/settings/global-commerce", {
        method: "PUT",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          base_currency: baseCurrency,
          enabled_currencies: enabledCurrencies,
        }),
      });
      if (!response.ok) throw new Error("save rejected");
      setStatus("Currency settings saved.");
    } catch {
      setError("Currency settings could not be saved.");
    } finally {
      setIsSaving(false);
    }
  }

  if (isLoading) {
    return <p className="py-10 text-sm text-gray-600" role="status">Loading currency settings...</p>;
  }

  if (error !== null && !hasLoaded) {
    return (
      <div className="app-panel max-w-xl rounded-lg p-5">
        <p className="text-sm text-red-700 dark:text-red-300" role="alert">{error}</p>
        <button className="app-button mt-4" onClick={() => void loadSettings()} type="button">Retry</button>
      </div>
    );
  }

  return (
    <div className="max-w-2xl space-y-5">
      <section className="app-panel rounded-lg p-5" aria-labelledby="default-currency-title">
        <h2 className="text-base font-semibold" id="default-currency-title">Default currency</h2>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-300">
          Used for store prices and business reporting.
        </p>
        <label className="mt-4 block text-sm font-medium" htmlFor="base-currency">
          Base currency
        </label>
        <select
          className="mt-2 min-h-11 w-full border border-gray-300 bg-white px-3 dark:border-gray-600 dark:bg-gray-800"
          id="base-currency"
          onChange={(event) => changeBaseCurrency(event.target.value as Currency)}
          value={baseCurrency}
        >
          {AVAILABLE_CURRENCIES.map((currency) => <option key={currency}>{currency}</option>)}
        </select>
      </section>

      <fieldset className="app-panel rounded-lg p-5">
        <legend className="px-1 text-base font-semibold">Enabled currencies</legend>
        <p className="mb-4 text-sm text-gray-600 dark:text-gray-300">
          Choose the currencies customers can use at checkout.
        </p>
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
          {AVAILABLE_CURRENCIES.map((currency) => (
            <label className="flex min-h-11 items-center gap-3 text-sm font-medium" key={currency}>
              <input
                checked={enabledCurrencies.includes(currency)}
                disabled={currency === baseCurrency}
                onChange={() => toggleCurrency(currency)}
                type="checkbox"
              />
              {currency}
            </label>
          ))}
        </div>
      </fieldset>

      {error && <p className="text-sm text-red-700 dark:text-red-300" role="alert">{error}</p>}
      {status && <p className="text-sm text-green-700 dark:text-green-300" role="status">{status}</p>}
      <button
        className="app-button min-h-11 bg-[#0066FF] px-5 py-2.5 font-semibold text-white disabled:opacity-60"
        disabled={isSaving}
        onClick={() => void saveSettings()}
        type="button"
      >
        {isSaving ? "Saving..." : "Save changes"}
      </button>
    </div>
  );
}
