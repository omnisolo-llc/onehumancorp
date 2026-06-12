"use client";

import React, { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

export default function InstantQuotePage() {
  return (
    <Suspense fallback={<div className="flex h-screen items-center justify-center">Loading...</div>}>
      <InstantQuoteContent />
    </Suspense>
  );
}

interface Modifier {
  id: string;
  label: string;
  type: 'fixed' | 'percentage';
  value: number;
}

interface PricingRule {
  id: string;
  name: string;
  base_price_cents: number;
  rules_json: {
    modifiers: Modifier[];
  };
}

function InstantQuoteContent() {
  const searchParams = useSearchParams();
  const tenantId = searchParams.get('tenant') || 'default';

  const [rule, setRule] = useState<PricingRule | null>(null);
  const [selectedModifiers, setSelectedModifiers] = useState<Set<string>>(new Set());

  useEffect(() => {
    async function loadRules() {
      try {
        const res = await fetch('/api/pricing-rules', {
          headers: { 'x-tenant-id': tenantId }
        });
        if (res.ok) {
          const rules = await res.json();
          if (rules.length > 0) {
            setRule(rules[0]);
          }
        }
      } catch (err) {
        console.error(err);
      }
    }
    loadRules();
  }, [tenantId]);

  if (!rule) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-50 text-gray-500">
        <p>Loading rules...</p>
      </div>
    );
  }

  const toggleModifier = (id: string) => {
    setSelectedModifiers(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const calculatePrice = () => {
    let price = rule.base_price_cents;
    const activeModifiers = rule.rules_json.modifiers.filter(m => selectedModifiers.has(m.id));

    // Apply percentages first (or whatever business logic makes sense)
    const pctModifiers = activeModifiers.filter(m => m.type === 'percentage');
    let pctMultiplier = 1.0;
    pctModifiers.forEach(m => {
      pctMultiplier += (m.value / 100);
    });
    price = price * pctMultiplier;

    // Apply fixed additions
    const fixedModifiers = activeModifiers.filter(m => m.type === 'fixed');
    fixedModifiers.forEach(m => {
      price += m.value;
    });

    return (price / 100).toFixed(2);
  };

  return (
    <div className="min-h-screen bg-gray-50 p-6 font-sans">
      <div className="max-w-md mx-auto glassmorphism rounded-2xl p-6 border border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold mb-4">{rule.name || 'Instant Quote'}</h1>

        <div className="space-y-4 mb-8">
          <p className="font-medium text-gray-700">Options</p>
          {rule.rules_json.modifiers.map(mod => (
            <label key={mod.id} className="flex items-center space-x-3 cursor-pointer p-3 rounded-xl border hover:bg-gray-50 transition-colors">
              <input
                type="checkbox"
                checked={selectedModifiers.has(mod.id)}
                onChange={() => toggleModifier(mod.id)}
                className="w-5 h-5 text-blue-600 rounded"
              />
              <div className="flex-1">
                <span className="font-medium">{mod.label}</span>
                <span className="text-gray-500 text-sm ml-2">
                  {mod.type === 'fixed' ? `+$${(mod.value / 100).toFixed(2)}` : `+${mod.value}%`}
                </span>
              </div>
            </label>
          ))}
        </div>

        <div className="sticky bottom-0 bg-white border-t p-4 rounded-xl shadow-md">
          <div className="flex justify-between items-center">
            <span className="text-gray-600 font-medium">Estimated Price</span>
            <span className="text-3xl font-bold text-gray-900" data-testid="instant-price">${calculatePrice()}</span>
          </div>
          <button className="w-full mt-4 py-3 bg-blue-600 text-white rounded-xl font-semibold hover:bg-blue-700 transition-colors">
            Request Final Quote
          </button>
        </div>
      </div>
    </div>
  );
}
