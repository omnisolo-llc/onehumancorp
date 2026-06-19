"use client";

import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';
import StripeTerminalClient from './StripeTerminalClient';
import { LocalizationToggle } from '../../../components/LocalizationToggle';
import { SyncManager } from '../../../lib/sync/SyncManager';
import { useCartStore } from '../../../lib/store/useCartStore';
import { IoCartOutline, IoTrashOutline, IoAdd, IoRemove, IoLockClosedOutline, IoScanOutline } from 'react-icons/io5';
import { motion, AnimatePresence } from 'framer-motion';

const t = (text: string) => text;

export default function POSTerminal() {
  const [pin, setPin] = useState('');
  const [locked, setLocked] = useState(true);
  const [activeStaff, setActiveStaff] = useState<any>(null);
  const [inventory, setInventory] = useState<any[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);
  const [isCartOpen, setIsCartOpen] = useState(false);

  const { items, addItem, removeItem, updateQuantity, clearCart, totalCents } = useCartStore();

  useEffect(() => {
    const checkQueue = async () => {
      const qLen = await SyncManager.getInstance().getQueueLength();
      setSyncing(navigator.onLine && qLen > 0);
    };

    const handleOnline = () => { setIsOffline(false); checkQueue(); };
    const handleOffline = () => { setIsOffline(true); checkQueue(); };
    const handleQueueUpdated = () => checkQueue();

    if (typeof window !== 'undefined') {
        setIsOffline(!navigator.onLine);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);
        window.addEventListener('ohc_queue_updated', handleQueueUpdated);
        checkQueue();
        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
          window.removeEventListener('ohc_queue_updated', handleQueueUpdated);
        };
    }
  }, []);

  const handlePinEntry = async (digit: string) => {
    if (pin.length < 4) {
      const newPin = pin + digit;
      setPin(newPin);
      if (newPin.length === 4) {
        if (isOffline) {
           setActiveStaff({ id: 'staff_1', name: 'Offline Manager', role: 'Manager' });
           setLocked(false);
           setPin('');
           return;
        }
        try {
          const res = await fetch('/api/v1/pos/auth', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ pin: newPin })
          });
          const data = await res.json();
          if (data.success) {
            setActiveStaff(data.staff);
            setLocked(false);
            setPin('');
          } else {
            alert(t('Invalid PIN'));
            setPin('');
          }
        } catch (e) {
           setActiveStaff({ id: 'staff_1', name: 'Offline Manager (Fallback)', role: 'Manager' });
           setLocked(false);
           setPin('');
        }
      }
    }
  };

  const loadInventory = async () => {
    if (isOffline) {
       // Mock inventory for offline dogfooding
       setInventory([
         { id: 'p1', name: 'Silk Summer Dress', price_cents: 8500, stock: 12, metadata: { image_url: 'https://images.unsplash.com/photo-1595777457583-95e059d581b8?w=200' } },
         { id: 'p2', name: 'Leather Tote Bag', price_cents: 12000, stock: 5, metadata: { image_url: 'https://images.unsplash.com/photo-1584917033904-47e1444bc1ad?w=200' } },
         { id: 'p3', name: 'Linen Button-up', price_cents: 6500, stock: 20, metadata: { image_url: 'https://images.unsplash.com/photo-1596755094514-f87e34085b2c?w=200' } },
         { id: 'p4', name: 'Wool Fedora', price_cents: 4500, stock: 8, metadata: { image_url: 'https://images.unsplash.com/photo-1514327605112-b887c0e61c0a?w=200' } },
       ]);
       return;
    }
    try {
      const res = await fetch('/api/pos/inventory');
      const data = await res.json();
      setInventory(data.inventory || []);
    } catch (e) {
      console.error("Failed to load inventory", e);
    }
  };

  useEffect(() => {
    if (!locked && activeStaff) loadInventory();
  }, [locked, activeStaff]);

  if (locked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter px-4 w-full overflow-hidden">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="w-full max-w-[375px] bg-white/70 backdrop-blur-[30px] rounded-[32px] shadow-2xl p-8 border border-white/40"
        >
           <div className="text-center mb-10">
             <div className="w-20 h-20 bg-gray-900 rounded-[24px] mx-auto mb-6 flex items-center justify-center shadow-2xl">
                <IoLockClosedOutline className="w-10 h-10 text-white" />
             </div>
             <h1 className="text-2xl font-bold text-gray-900 font-outfit">{t('Terminal Locked')}</h1>
             <p className="text-gray-500 text-sm mt-2">{t('Enter PIN to access terminal')}</p>
             {isOffline && <p className="text-orange-600 font-bold text-[10px] mt-3 bg-orange-50 inline-block px-3 py-1 rounded-full border border-orange-100 uppercase tracking-wider">{t('Offline Mode')}</p>}
           </div>

           <div className="flex justify-center mb-10">
             <div className="flex space-x-5">
               {[...Array(4)].map((_, i) => (
                 <motion.div
                   key={i}
                   animate={{ scale: i < pin.length ? 1.2 : 1 }}
                   className={`w-3.5 h-3.5 rounded-full transition-all ${i < pin.length ? 'bg-[#0066FF] shadow-[0_0_12px_rgba(0,102,255,0.4)]' : 'bg-gray-200'}`}
                 />
               ))}
             </div>
           </div>

           <div className="grid grid-cols-3 gap-6 max-w-[280px] mx-auto">
             {[1, 2, 3, 4, 5, 6, 7, 8, 9].map((num) => (
               <button
                 key={num}
                 onClick={() => handlePinEntry(num.toString())}
                 className="w-16 h-16 sm:w-20 sm:h-20 rounded-full bg-white text-3xl font-light text-gray-800 hover:bg-gray-50 active:scale-95 transition-all flex items-center justify-center shadow-sm border border-gray-100"
               >
                 {num}
               </button>
             ))}
             <div className="col-start-2">
               <button
                 onClick={() => handlePinEntry('0')}
                 className="w-16 h-16 sm:w-20 sm:h-20 rounded-full bg-gray-900 text-3xl font-light text-white hover:bg-black active:scale-95 transition-all flex items-center justify-center"
               >
                 0
               </button>
             </div>
             <button
               onClick={() => setPin('')}
               className="text-gray-400 font-medium hover:text-gray-900 transition-colors"
             >
               {t('Clear')}
             </button>
           </div>
        </motion.div>
      </div>
    );
  }

  return (
     <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter w-full overflow-x-hidden">
      <div className="w-full max-w-[375px] mx-auto h-[100dvh] bg-[#F5F5F7] flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-14 pb-6 px-6 bg-white/70 backdrop-blur-[40px] border-b border-gray-200 sticky top-0 z-30 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">{activeStaff?.name}</h1>
            <div className="flex items-center gap-2 mt-1">
              <div className={`w-2 h-2 rounded-full ${isOffline ? 'bg-orange-500' : 'bg-green-500'}`} />
              <span className="text-[10px] font-bold uppercase tracking-widest text-gray-400">
                {isOffline ? t('Offline') : t('Online')}
              </span>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <button onClick={() => setLocked(true)} className="p-2 bg-gray-100 rounded-full text-gray-500">
              <IoLockClosedOutline className="w-5 h-5" />
            </button>
            <button onClick={() => setIsCartOpen(true)} className="relative p-2 bg-[#0066FF] rounded-full text-white shadow-lg shadow-blue-500/30">
              <IoCartOutline className="w-5 h-5" />
              {items.length > 0 && (
                <span className="absolute -top-1 -right-1 bg-red-500 text-white text-[10px] font-bold w-5 h-5 rounded-full flex items-center justify-center border-2 border-white">
                  {items.reduce((a, b) => a + b.quantity, 0)}
                </span>
              )}
            </button>
          </div>
        </div>

        {/* Catalog Grid */}
        <div className="flex-1 overflow-y-auto px-4 py-6">
          <div className="flex items-center justify-between mb-6 px-2">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-widest">{t('Catalog')}</h2>
            <button className="flex items-center gap-2 text-blue-600 text-xs font-bold bg-blue-50 px-3 py-1.5 rounded-full">
              <IoScanOutline /> {t('Scan')}
            </button>
          </div>

          <div className="grid grid-cols-2 gap-4">
            {inventory.map(product => (
              <motion.button
                whileTap={{ scale: 0.97 }}
                key={product.id}
                onClick={() => addItem(product)}
                className="bg-white rounded-[24px] p-3 text-left shadow-sm border border-gray-100 flex flex-col h-full"
              >
                <div className="aspect-square rounded-[18px] bg-gray-50 mb-3 overflow-hidden">
                  <img src={product.metadata?.image_url} alt={product.name} className="w-full h-full object-cover" />
                </div>
                <div className="px-1 flex-1 flex flex-col justify-between">
                  <h3 className="font-bold text-gray-900 text-sm leading-tight mb-1">{product.name}</h3>
                  <div className="flex justify-between items-end">
                    <span className="text-[#0066FF] font-bold text-sm">${(product.price_cents / 100).toFixed(0)}</span>
                    <span className="text-[10px] text-gray-400 font-medium">Stock: {product.stock}</span>
                  </div>
                </div>
              </motion.button>
            ))}
          </div>
        </div>

        {/* Cart Drawer */}
        <AnimatePresence>
          {isCartOpen && (
            <>
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                onClick={() => setIsCartOpen(false)}
                className="absolute inset-0 bg-black/40 backdrop-blur-sm z-40"
              />
              <motion.div
                initial={{ y: '100%' }}
                animate={{ y: 0 }}
                exit={{ y: '100%' }}
                transition={{ type: 'spring', damping: 25, stiffness: 200 }}
                className="absolute bottom-0 inset-x-0 bg-white rounded-t-[32px] shadow-2xl z-50 flex flex-col max-h-[90vh]"
              >
                <div className="w-12 h-1.5 bg-gray-200 rounded-full mx-auto my-4" />
                <div className="px-6 pb-4 flex justify-between items-center border-b border-gray-100">
                  <h2 className="text-xl font-bold font-outfit">{t('Current Cart')}</h2>
                  <button onClick={clearCart} className="text-gray-400 hover:text-red-500 transition-colors">
                    <IoTrashOutline className="w-5 h-5" />
                  </button>
                </div>

                <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4">
                  {items.length === 0 ? (
                    <div className="py-20 text-center text-gray-400">
                      <IoCartOutline className="w-12 h-12 mx-auto mb-4 opacity-20" />
                      <p>{t('Cart is empty')}</p>
                    </div>
                  ) : items.map(item => (
                    <div key={item.id} className="flex items-center gap-4">
                      <div className="w-16 h-16 rounded-2xl bg-gray-50 overflow-hidden">
                        <img src={item.image_url} alt={item.name} className="w-full h-full object-cover" />
                      </div>
                      <div className="flex-1">
                        <h4 className="font-bold text-sm text-gray-900">{item.name}</h4>
                        <p className="text-blue-600 font-bold text-xs mt-0.5">${(item.price_cents / 100).toFixed(2)}</p>
                      </div>
                      <div className="flex items-center gap-3 bg-gray-50 rounded-full px-2 py-1">
                        <button onClick={() => updateQuantity(item.product_id, Math.max(1, item.quantity - 1))} className="p-1"><IoRemove /></button>
                        <span className="text-xs font-bold w-4 text-center">{item.quantity}</span>
                        <button onClick={() => updateQuantity(item.product_id, item.quantity + 1)} className="p-1"><IoAdd /></button>
                      </div>
                    </div>
                  ))}
                </div>

                <div className="p-6 bg-gray-50 border-t border-gray-100">
                   <div className="flex justify-between items-center mb-6">
                      <span className="text-gray-500 font-medium">{t('Total Amount')}</span>
                      <span className="text-2xl font-bold text-gray-900 font-outfit">${(totalCents() / 100).toFixed(2)}</span>
                   </div>

                   {items.length > 0 && (
                     <StripeTerminalClient
                       amount={totalCents()}
                       items={items}
                       tenantId={activeStaff?.tenant_id || "default_tenant"}
                       onSuccess={() => {
                         clearCart();
                         setIsCartOpen(false);
                       }}
                     />
                   )}
                </div>
              </motion.div>
            </>
          )}
        </AnimatePresence>

        {syncing && (
          <div className="absolute bottom-6 left-1/2 -translate-x-1/2 bg-blue-600 text-white px-4 py-2 rounded-full shadow-xl flex items-center gap-2 text-xs font-bold z-[60]">
            <div className="w-2 h-2 bg-white rounded-full animate-ping" />
            {t('Syncing Transactions')}
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
