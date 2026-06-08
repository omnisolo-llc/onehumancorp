"use client";

import React, { useState, useEffect } from 'react';
import { useTranslation, useCurrency } from '../../../lib/localizationStore';
import { LocalizationToggle } from '../../../components/LocalizationToggle';
import StripeTerminalClient from './StripeTerminalClient';

// Offline storage helper
const OfflineStore = {
  getStaff: () => JSON.parse(localStorage.getItem('ohc_offline_staff') || '[]'),
  setStaff: (staff: any[]) => localStorage.setItem('ohc_offline_staff', JSON.stringify(staff)),

  getProducts: () => JSON.parse(localStorage.getItem('ohc_offline_products') || '[]'),
  setProducts: (products: any[]) => localStorage.setItem('ohc_offline_products', JSON.stringify(products)),

  getEvents: () => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'),
  addEvent: (event: any) => {
    const events = OfflineStore.getEvents();
    events.push(event);
    localStorage.setItem('ohc_offline_events', JSON.stringify(events));
  },
  clearEvents: () => localStorage.setItem('ohc_offline_events', '[]'),

  getPosTransactions: () => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'),
  setPosTransactions: (transactions: any[]) => localStorage.setItem('ohc_offline_pos_tx', JSON.stringify(transactions)),
  addPosTransaction: (tx: any) => {
    const transactions = OfflineStore.getPosTransactions();
    transactions.push(tx);
    localStorage.setItem('ohc_offline_pos_tx', JSON.stringify(transactions));
  },
  clearPosTransactions: () => localStorage.setItem('ohc_offline_pos_tx', '[]'),

  getAgentApprovals: () => JSON.parse(localStorage.getItem('ohc_offline_agent_approvals') || '[]'),
  setAgentApprovals: (approvals: any[]) => localStorage.setItem('ohc_offline_agent_approvals', JSON.stringify(approvals)),
};

interface Product {
  id: string;
  title: string;
  price_cents: number;
  inventory_count: number;
  image_url?: string;
}

export default function TerminalPage() {
  const { t } = useTranslation();
  const { currency, convert } = useCurrency();
  const [pin, setPin] = useState('');
  const [activeStaff, setActiveStaff] = useState<any | null>(null);
  const [clockedIn, setClockedIn] = useState(false);
  const [error, setError] = useState('');
  const [syncing, setSyncing] = useState(false);
  const [syncCount, setSyncCount] = useState(0);
  const [orderStatus, setOrderStatus] = useState('');
  const [isOffline, setIsOffline] = useState(false);
  const [products, setProducts] = useState<Product[]>([]);
  const [cart, setCart] = useState<{product: Product, quantity: number}[]>([]);
  const [showCheckout, setShowCheckout] = useState(false);
  const [agentNotification, setAgentNotification] = useState<any | null>(null);

  // Network listener
  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    setIsOffline(!navigator.onLine);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  // Initial Data Fetch
  useEffect(() => {
    if (navigator.onLine) {
      // Fetch Staff
      fetch('/api/staff')
        .then(res => res.json())
        .then(data => {
          const staff = Array.isArray(data) ? data : data.staff || [];
          OfflineStore.setStaff(staff);
        }).catch(console.error);

      // Fetch Products
      fetch('/api/v1/catalog/products')
        .then(res => res.json())
        .then(data => {
          const prods = Array.isArray(data) ? data : data.products || [];
          setProducts(prods);
          OfflineStore.setProducts(prods);
        }).catch(console.error);
    } else {
      setProducts(OfflineStore.getProducts());
    }
  }, []);

  // Background Sync & Agent Notification Polling
  useEffect(() => {
    const syncInterval = setInterval(async () => {
      if (navigator.onLine) {
        const events = OfflineStore.getEvents();
        const posTransactions = OfflineStore.getPosTransactions();

        if (events.length > 0 || posTransactions.length > 0) {
          setSyncCount(events.length + posTransactions.length);
          setSyncing(true);
          try {
            if (events.length > 0) {
              const res = await fetch('/api/staff/timecard', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(events)
              });
              if (res.ok) OfflineStore.clearEvents();
            }

            if (posTransactions.length > 0) {
              const sessionId = localStorage.getItem('ohc_active_terminal_session_id');
              const res = await fetch('/api/v1/payments/terminal/sync_offline', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ session_id: sessionId, transactions: posTransactions })
              });
              if (res.ok) {
                const data = await res.json();
                if (data.failed_transaction_ids && data.failed_transaction_ids.length > 0) {
                  const failedTxs = posTransactions.filter((tx: any) => data.failed_transaction_ids.includes(tx.client_id || tx.id));
                  OfflineStore.setPosTransactions(failedTxs);
                } else {
                  OfflineStore.clearPosTransactions();
                }
              }
            }
          } catch (e) {
            console.error("Sync failed", e);
          } finally {
            setSyncing(false);
            setSyncCount(0);
          }
        }

        // Poll for Agent Approvals (Restock Suggestions)
        try {
          const res = await fetch('/api/agents/approvals');
          if (res.ok) {
            const data = await res.json();
            const approvals = data.pending_approvals || [];
            const operationsApprovals = approvals.filter((a: any) => a.department === 'operations' && a.status === 'DRAFT');
            if (operationsApprovals.length > 0 && !agentNotification) {
               setAgentNotification(operationsApprovals[0]);
            }
          }
        } catch (e) {}
      }
    }, 10000);

    return () => clearInterval(syncInterval);
  }, [agentNotification]);

  const handlePinEntry = (digit: string) => {
    if (pin.length < 4) {
      const newPin = pin + digit;
      setPin(newPin);
      if (newPin.length === 4) {
        const staff = OfflineStore.getStaff();
        const found = staff.find((s: any) => s.pin_hash === newPin);
        if (found) {
          setActiveStaff(found);
          const events = OfflineStore.getEvents().filter((e: any) => e.staff_id === found.id);
          if (events.length > 0) {
             setClockedIn(events[events.length - 1].event_type === 'CLOCK_IN');
          }
        } else {
          setError(t('Invalid PIN'));
          setPin('');
        }
      }
    }
  };

  const addToCart = (product: Product) => {
    setCart(prev => {
      const existing = prev.find(item => item.product.id === product.id);
      if (existing) {
        return prev.map(item => item.product.id === product.id ? { ...item, quantity: item.quantity + 1 } : item);
      }
      return [...prev, { product, quantity: 1 }];
    });
  };

  const cartTotal = cart.reduce((sum, item) => sum + (item.product.price_cents * item.quantity), 0);

  const handleCheckout = async () => {
    if (cart.length === 0) return;
    setOrderStatus(t('Processing...'));

    const tx = {
      id: `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      amount_cents: cartTotal,
      currency: currency,
      payload: JSON.stringify(cart.map(item => ({ product_id: item.product.id, quantity: item.quantity }))),
      client_id: 'terminal_mobile_1',
      timestamp: new Date().toISOString()
    };

    if (isOffline) {
      OfflineStore.addPosTransaction(tx);
      setOrderStatus(t('Sale saved offline.'));
      setTimeout(() => {
        setCart([]);
        setShowCheckout(false);
        setOrderStatus('');
      }, 2000);
    } else {
      try {
        const res = await fetch('/api/v1/payments/terminal/sync_offline', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ transactions: [tx] })
        });
        if (res.ok) {
          setOrderStatus(t('Payment Completed'));
          setTimeout(() => {
            setCart([]);
            setShowCheckout(false);
            setOrderStatus('');
          }, 2000);
        }
      } catch (e) {
        OfflineStore.addPosTransaction(tx);
        setOrderStatus(t('Error: Sale saved offline.'));
      }
    }
  };

  const handleApproveAgent = async (id: string) => {
    if (!navigator.onLine) return;
    await fetch(`/api/agents/approvals/${id}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ approved: true })
    });
    setAgentNotification(null);
  };

  if (!activeStaff) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter">
        <div className="w-[375px] h-[812px] bg-black text-white p-8 flex flex-col items-center relative overflow-hidden">
           <div className="absolute top-8 right-8 flex items-center gap-4">
              {isOffline && <span className="text-red-500 font-bold text-xs bg-red-100/10 px-2 py-1 rounded">{t('Offline Mode')}</span>}
              <LocalizationToggle />
           </div>
           <div className="mt-20 mb-12 text-center">
             <h1 className="text-2xl font-bold font-outfit mb-2">{t('Terminal Locked')}</h1>
             <p className="text-gray-400">{t('Enter PIN to unlock')}</p>
           </div>
           <div className="flex gap-4 mb-12">
              {[...Array(4)].map((_, i) => (
                <div key={i} className={`w-4 h-4 rounded-full border-2 ${pin.length > i ? 'bg-white border-white' : 'border-gray-600'}`}></div>
              ))}
           </div>
           <div className="grid grid-cols-3 gap-6 w-full max-w-[280px]">
             {[1, 2, 3, 4, 5, 6, 7, 8, 9, '', 0, 'C'].map((val, i) => (
               <button
                 key={i}
                 onClick={() => val === 'C' ? setPin('') : val !== '' && handlePinEntry(val.toString())}
                 className={`w-20 h-20 rounded-full flex items-center justify-center text-3xl font-light transition-colors ${val === '' ? 'invisible' : 'bg-gray-800 hover:bg-gray-700 active:bg-gray-600'}`}
               >
                 {val}
               </button>
             ))}
           </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 font-inter py-10">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/80 backdrop-blur-md border-b sticky top-0 z-20 flex justify-between items-center">
          <div>
            <h1 className="text-lg font-bold font-outfit text-gray-900">{activeStaff.name}</h1>
            <div className="flex items-center gap-2">
               <div className={`w-2 h-2 rounded-full ${isOffline ? 'bg-red-500' : 'bg-green-500'}`}></div>
               <span className="text-xs font-medium text-gray-500 uppercase tracking-wider">{isOffline ? t('Offline') : t('Online')}</span>
            </div>
          </div>
          <button onClick={() => setActiveStaff(null)} className="text-xs font-bold text-blue-600 uppercase tracking-widest">{t('Lock')}</button>
        </div>

        {/* Product Grid */}
        <div className="flex-1 overflow-y-auto p-4 bg-gray-50">
          <div className="grid grid-cols-2 gap-3">
            {products.map(product => (
              <button
                key={product.id}
                onClick={() => addToCart(product)}
                className="bg-white p-3 rounded-2xl border border-gray-100 shadow-sm text-left active:scale-95 transition-transform min-h-[120px] flex flex-col justify-between"
              >
                <div className="text-sm font-bold text-gray-900 line-clamp-2">{product.title}</div>
                <div className="flex justify-between items-end mt-2">
                  <span className="text-blue-600 font-bold">${(product.price_cents / 100).toFixed(2)}</span>
                  <span className="text-[10px] text-gray-400">Qty: {product.inventory_count}</span>
                </div>
              </button>
            ))}
          </div>
        </div>

        {/* Cart Summary Bar */}
        {cart.length > 0 && !showCheckout && (
          <button
            onClick={() => setShowCheckout(true)}
            className="absolute bottom-6 left-4 right-4 bg-blue-600 text-white p-4 rounded-2xl shadow-xl flex justify-between items-center animate-in slide-in-from-bottom"
          >
            <div className="flex items-center gap-3">
               <span className="bg-white/20 w-8 h-8 rounded-full flex items-center justify-center font-bold">{cart.reduce((a,b) => a + b.quantity, 0)}</span>
               <span className="font-bold">{t('View Cart')}</span>
            </div>
            <span className="font-outfit text-xl font-bold">${(cartTotal / 100).toFixed(2)}</span>
          </button>
        )}

        {/* Checkout Modal */}
        {showCheckout && (
          <div className="absolute inset-0 bg-white z-30 animate-in slide-in-from-bottom duration-300 flex flex-col">
            <div className="p-6 border-b flex justify-between items-center">
               <h2 className="text-xl font-bold font-outfit">{t('Checkout')}</h2>
               <button onClick={() => setShowCheckout(false)} className="text-gray-400"><svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M6 18L18 6M6 6l12 12" strokeWidth="2" strokeLinecap="round"/></svg></button>
            </div>
            <div className="flex-1 overflow-y-auto p-6 space-y-4">
               {cart.map(item => (
                 <div key={item.product.id} className="flex justify-between items-center">
                    <div>
                      <div className="font-bold text-gray-900">{item.product.title}</div>
                      <div className="text-sm text-gray-500">x{item.quantity}</div>
                    </div>
                    <div className="font-bold text-gray-900">${(item.product.price_cents * item.quantity / 100).toFixed(2)}</div>
                 </div>
               ))}
               <div className="border-t pt-4 flex justify-between items-center">
                  <span className="text-gray-500 font-medium">{t('Total')}</span>
                  <span className="text-2xl font-bold font-outfit">${(cartTotal / 100).toFixed(2)}</span>
               </div>
            </div>
            <div className="p-6 space-y-3">
               <button
                 onClick={handleCheckout}
                 className="w-full bg-green-600 text-white py-4 rounded-2xl font-bold text-lg shadow-lg active:scale-95 transition-transform"
               >
                 {t('Complete Sale')}
               </button>
               {orderStatus && <div className="text-center text-sm font-bold text-blue-600 animate-pulse">{orderStatus}</div>}
            </div>
          </div>
        )}

        {/* Sync Snackbar */}
        {syncing && (
          <div className="absolute top-24 left-1/2 -translate-x-1/2 bg-blue-600 text-white px-6 py-2 rounded-full shadow-lg text-xs font-bold flex items-center gap-2 z-50">
             <div className="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
             {t('Syncing')} {syncCount} {t('sales...')}
          </div>
        )}

        {/* Agent Restock Notification (Glass Card) */}
        {agentNotification && (
          <div className="absolute inset-x-4 bottom-24 bg-white/30 backdrop-blur-xl border border-white/40 p-5 rounded-[2rem] shadow-2xl z-40 animate-in zoom-in-95 slide-in-from-bottom-10 duration-500">
             <div className="flex items-start gap-4">
                <div className="bg-blue-500 w-12 h-12 rounded-2xl flex items-center justify-center text-white shrink-0 shadow-lg shadow-blue-500/30">
                   <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M13 10V3L4 14h7v7l9-11h-7z" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/></svg>
                </div>
                <div>
                   <h4 className="font-bold text-gray-900 text-lg leading-tight">{t('Restock Suggestion')}</h4>
                   <p className="text-sm text-gray-700 mt-1 font-medium">{agentNotification.description}</p>
                </div>
             </div>
             <div className="flex gap-2 mt-6">
                <button
                   onClick={() => handleApproveAgent(agentNotification.id)}
                   className="flex-1 bg-gray-900 text-white py-3 rounded-xl font-bold text-sm shadow-xl active:scale-95 transition-transform"
                >
                   {t('Approve')}
                </button>
                <button
                   onClick={() => setAgentNotification(null)}
                   className="flex-1 bg-white/50 text-gray-900 py-3 rounded-xl font-bold text-sm border border-black/5 hover:bg-white/80 active:scale-95 transition-transform"
                >
                   {t('Dismiss')}
                </button>
             </div>
          </div>
        )}
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .animate-in { animation: animate-in 0.3s ease-out; }
        @keyframes animate-in {
          from { opacity: 0; transform: translateY(20px); }
          to { opacity: 1; transform: translateY(0); }
        }
      `}} />
    </div>
  );
}
