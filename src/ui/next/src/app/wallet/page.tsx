'use client';

import React, { useEffect, useState } from 'react';

interface Wallet {
  id: string;
  tenant_id: string;
  available_balance_cents: number;
  currency: string;
}

interface VirtualCard {
  id: string;
  status: string;
  last_four: string;
  expiry_month: number;
  expiry_year: number;
  cardholder_name: string;
}

interface RevealResponse {
  pan: string;
  cvc: string;
  expiry_month: number;
  expiry_year: number;
}

export default function WalletPage() {
  const [wallet, setWallet] = useState<Wallet | null>(null);
  const [card, setCard] = useState<VirtualCard | null>(null);
  const [revealedData, setRevealedData] = useState<RevealResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [revealing, setRevealing] = useState(false);

  useEffect(() => {
    async function fetchData() {
      try {
        const [walletRes, cardRes] = await Promise.all([
          fetch('/api/v1/wallet'),
          fetch('/api/v1/wallet/virtual-card')
        ]);

        if (walletRes.ok) {
          const wData = await walletRes.json();
          setWallet(wData.wallet);
        }

        if (cardRes.ok) {
          const cData = await cardRes.json();
          setCard(cData.card);
        }
      } catch (e) {
        console.error("Failed to fetch wallet data", e);
      } finally {
        setLoading(false);
      }
    }

    fetchData();
  }, []);

  const handleReveal = async () => {
    setRevealing(true);
    try {
      const res = await fetch('/api/v1/wallet/virtual-card/reveal', { method: 'POST' });
      if (res.ok) {
        const data = await res.json();
        setRevealedData(data);

        // Auto-hide after 30 seconds for security
        setTimeout(() => {
          setRevealedData(null);
        }, 30000);
      }
    } catch (e) {
      console.error("Failed to reveal card", e);
    } finally {
      setRevealing(false);
    }
  };

  if (loading) {
    return (
      <div className="flex h-screen w-full items-center justify-center p-4">
        <div className="h-8 w-8 animate-spin rounded-full border-b-2 border-t-2 border-blue-500"></div>
      </div>
    );
  }

  const formatCurrency = (cents: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(cents / 100);
  };

  const formatExpiry = (month: number, year: number) => {
    return `${month.toString().padStart(2, '0')}/${year.toString().slice(-2)}`;
  };

  return (
    <div className="mx-auto max-w-lg p-4 md:p-8" data-testid="wallet-dashboard">
      <h1 className="mb-6 text-2xl font-semibold text-gray-900 dark:text-gray-100">Capital</h1>

      {/* Balance Card */}
      <div
        className="mb-8 rounded-2xl border border-white/40 bg-white/65 p-6 shadow-lg backdrop-blur-[30px] backdrop-saturate-[210%] dark:border-white/10 dark:bg-[#16161A]/70"
        data-testid="wallet-balance-card"
      >
        <p className="mb-1 text-sm font-medium text-gray-500 dark:text-gray-400">Available Balance</p>
        <h2 className="text-4xl font-bold tracking-tight text-gray-900 dark:text-white">
          {wallet ? formatCurrency(wallet.available_balance_cents) : '$0.00'}
        </h2>
        <div className="mt-4 flex gap-3">
          <button className="flex-1 rounded-xl bg-blue-600 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-blue-700 active:bg-blue-800">
            Add Funds
          </button>
          <button className="flex-1 rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 dark:border-white/10 dark:bg-white/5 dark:text-gray-200 dark:hover:bg-white/10">
            Withdraw
          </button>
        </div>
      </div>

      {/* Virtual Card */}
      <div>
        <div className="mb-4 flex items-center justify-between">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">Virtual Card</h3>
          <span className="inline-flex items-center gap-1.5 rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800 dark:bg-green-900/30 dark:text-green-400">
            <span className="h-1.5 w-1.5 rounded-full bg-green-500"></span>
            Active
          </span>
        </div>

        {card ? (
          <div
            className="relative overflow-hidden rounded-2xl border border-white/20 bg-gradient-to-br from-gray-900 to-black p-6 shadow-xl dark:border-white/10"
            data-testid="virtual-card-container"
          >
            {/* Card UI elements */}
            <div className="absolute right-0 top-0 h-32 w-32 -translate-y-8 translate-x-8 rounded-full bg-blue-500/20 blur-2xl"></div>
            <div className="absolute bottom-0 left-0 h-32 w-32 -translate-x-8 translate-y-8 rounded-full bg-purple-500/20 blur-2xl"></div>

            <div className="relative z-10 flex h-full flex-col justify-between">
              <div className="flex items-center justify-between">
                <div className="text-xl font-bold tracking-widest text-white">OHC</div>
                <svg className="h-8 w-12 text-gray-300" viewBox="0 0 48 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M24 16C24 20.4183 20.4183 24 16 24C11.5817 24 8 20.4183 8 16C8 11.5817 11.5817 8 16 8C20.4183 8 24 11.5817 24 16Z" fill="currentColor" fillOpacity="0.8"/>
                  <path d="M40 16C40 20.4183 36.4183 24 32 24C27.5817 24 24 20.4183 24 16C24 11.5817 27.5817 8 32 8C36.4183 8 40 11.5817 40 16Z" fill="currentColor" fillOpacity="0.8"/>
                </svg>
              </div>

              <div className="mt-8 space-y-4">
                {revealedData ? (
                  <div className="font-mono text-xl tracking-[0.2em] text-white" data-testid="revealed-pan">
                    {revealedData.pan}
                  </div>
                ) : (
                  <div className="flex items-center gap-3 font-mono text-xl tracking-[0.2em] text-gray-300">
                    <span>••••</span>
                    <span>••••</span>
                    <span>••••</span>
                    <span>{card.last_four}</span>
                  </div>
                )}

                <div className="flex items-end justify-between">
                  <div>
                    <div className="text-[10px] font-medium uppercase tracking-wider text-gray-400">Cardholder</div>
                    <div className="font-medium tracking-wide text-white">{card.cardholder_name}</div>
                  </div>

                  <div className="flex gap-6">
                    <div>
                      <div className="text-[10px] font-medium uppercase tracking-wider text-gray-400">Valid Thru</div>
                      <div className="font-mono text-sm tracking-widest text-white" data-testid="card-expiry">
                        {revealedData ? formatExpiry(revealedData.expiry_month, revealedData.expiry_year) : formatExpiry(card.expiry_month, card.expiry_year)}
                      </div>
                    </div>
                    {revealedData && (
                      <div>
                        <div className="text-[10px] font-medium uppercase tracking-wider text-gray-400">CVC</div>
                        <div className="font-mono text-sm tracking-widest text-white" data-testid="revealed-cvc">{revealedData.cvc}</div>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </div>

            {/* Reveal Action Overlay */}
            {!revealedData && (
              <div className="absolute inset-0 z-20 flex items-center justify-center rounded-2xl bg-black/40 opacity-0 backdrop-blur-sm transition-opacity hover:opacity-100">
                <button
                  onClick={handleReveal}
                  disabled={revealing}
                  className="rounded-full bg-white/20 px-6 py-2.5 font-medium text-white shadow-lg backdrop-blur-md transition-colors hover:bg-white/30"
                  data-testid="reveal-card-btn"
                >
                  {revealing ? 'Authenticating...' : 'Reveal Details'}
                </button>
              </div>
            )}
          </div>
        ) : (
          <div className="flex h-48 flex-col items-center justify-center rounded-2xl border border-dashed border-gray-300 bg-gray-50 dark:border-white/10 dark:bg-white/5">
            <p className="text-sm text-gray-500 dark:text-gray-400">No virtual card issued yet</p>
            <button className="mt-4 rounded-xl bg-gray-900 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800 dark:bg-white dark:text-black dark:hover:bg-gray-100">
              Issue Card
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
