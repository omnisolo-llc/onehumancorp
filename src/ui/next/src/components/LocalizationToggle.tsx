"use client";

import React, { useState } from 'react';
import { useLocalizationStore } from '../lib/localizationStore';

const locales = [
  { code: 'en', name: 'English', flag: '🇺🇸' },
  { code: 'ar', name: 'العربية', flag: '🇸🇦' },
  { code: 'es', name: 'Español', flag: '🇪🇸' },
];

const currencies = [
  { code: 'USD', symbol: '$' },
  { code: 'EUR', symbol: '€' },
  { code: 'GBP', symbol: '£' },
];

export const LocalizationToggle: React.FC = () => {
  const { locale, currency, setLocale, setCurrency } = useLocalizationStore();
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 px-3 py-1.5 rounded-lg glassmorphism/60 backdrop-blur-xl border border-white/40 shadow-sm hover:glassmorphism/80 transition-all text-sm font-medium"
      >
        <span>{locales.find(l => l.code === locale)?.flag}</span>
        <span className="text-gray-900">{currency}</span>
      </button>

      {isOpen && (
        <>
          <div
            className="fixed inset-0 z-40"
            onClick={() => setIsOpen(false)}
          />
          <div className="absolute top-full mt-2 right-0 w-48 rounded-2xl glassmorphism/70 backdrop-blur-2xl border border-white/50 shadow-2xl z-50 p-2 animate-in fade-in zoom-in duration-200">
            <div className="mb-2">
              <p className="text-[10px] font-bold text-gray-400 uppercase tracking-widest px-2 mb-1">Language</p>
              {locales.map((l) => (
                <button
                  key={l.code}
                  onClick={() => {
                    setLocale(l.code);
                    setIsOpen(false);
                  }}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-xl text-sm transition-colors ${
                    locale === l.code ? 'bg-blue-500 text-white' : 'text-gray-700 hover:glassmorphism/50'
                  }`}
                >
                  <span>{l.flag}</span>
                  <span className="font-medium">{l.name}</span>
                </button>
              ))}
            </div>

            <div>
              <p className="text-[10px] font-bold text-gray-400 uppercase tracking-widest px-2 mb-1">Currency</p>
              <div className="grid grid-cols-3 gap-1">
                {currencies.map((c) => (
                  <button
                    key={c.code}
                    onClick={() => {
                      setCurrency(c.code);
                      setIsOpen(false);
                    }}
                    className={`flex flex-col items-center justify-center p-2 rounded-xl text-xs transition-colors ${
                      currency === c.code ? 'bg-blue-500 text-white' : 'text-gray-700 hover:glassmorphism/50'
                    }`}
                  >
                    <span className="text-lg font-bold">{c.symbol}</span>
                    <span className="font-medium opacity-70">{c.code}</span>
                  </button>
                ))}
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
};
