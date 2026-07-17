"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';
import { NetworkStatusIndicator } from '../../components/NetworkStatusIndicator';
import { LocalOrderCache, PreOrder } from '../../lib/offline/localOrderCache';
import { queueOrderAction } from '../../lib/offline/syncQueue';

export default function OperationsPage() {
  const [orders, setOrders] = useState<PreOrder[]>([]);
  const [loading, setLoading] = useState(true);
  const [spicyChickenSoldOut, setSpicyChickenSoldOut] = useState(false);

  useEffect(() => {
    async function fetchOrders() {
      try {
        let localOrders = await LocalOrderCache.getOrders();

        // Mocking some data if the cache is empty
        if (localOrders.length === 0) {
           const mockOrders: PreOrder[] = [
             {
               id: '1',
               customerName: 'Alice Smith',
               items: ['Spicy Chicken Wrap', 'Mango Lassi'],
               status: 'Pending',
               pickupTime: '12:30 PM',
               totalAmount: 14.50,
               paid: true
             },
             {
               id: '2',
               customerName: 'Sarah Johnson',
               items: ['Falafel Bowl'],
               status: 'Preparing',
               pickupTime: '12:45 PM',
               totalAmount: 10.00,
               paid: false
             }
           ];
           for (const order of mockOrders) {
               await LocalOrderCache.saveOrder(order);
           }
           localOrders = mockOrders;
        }

        setOrders(localOrders);
      } catch (err) {
         console.error("Error fetching orders from cache", err);
      } finally {
        setLoading(false);
      }
    }

    fetchOrders();
  }, []);

  const handleUpdateStatus = async (orderId: string, newStatus: PreOrder['status']) => {
    // Optimistic UI update
    setOrders(prevOrders => prevOrders.map(order =>
      order.id === orderId ? { ...order, status: newStatus } : order
    ));

    // Update local cache
    const orderToUpdate = orders.find(o => o.id === orderId);
    if (orderToUpdate) {
        await LocalOrderCache.saveOrder({ ...orderToUpdate, status: newStatus });
    }

    // Queue for sync
    await queueOrderAction(orderId, 'UPDATE_ORDER_STATUS', { newStatus });
  };

  const handleToggleSoldOut = async () => {
    const newState = !spicyChickenSoldOut;
    setSpicyChickenSoldOut(newState);
    await queueOrderAction('item_spicy_chicken', 'TOGGLE_SOLD_OUT', { soldOut: newState });
  };

  return (
    <AppShell title="Operations Copilot">
      <NetworkStatusIndicator />
      <div className="p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto space-y-8 animate-in fade-in duration-500">
        <header className="mb-8">
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Today's Pre-Orders</h1>
          <p className="text-[#86868B] dark:text-[#A1A1A6] text-lg mt-2 font-inter">Manage your active orders and menu availability.</p>
        </header>

        <section className="glassmorphism p-6 border border-white/40 dark:border-white/10 shadow-sm relative overflow-hidden mb-6">
           <div className="flex justify-between items-center">
             <div>
                <h2 className="text-lg font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Menu Quick Toggles</h2>
                <p className="text-sm text-[#86868B] dark:text-[#A1A1A6]">Update availability instantly.</p>
             </div>
             <div className="flex items-center gap-3">
                 <span className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">Spicy Chicken</span>
                 <button
                    onClick={handleToggleSoldOut}
                    className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${spicyChickenSoldOut ? 'bg-red-500' : 'bg-green-500'}`}
                 >
                     <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${spicyChickenSoldOut ? 'translate-x-6' : 'translate-x-1'}`} />
                 </button>
                 <span className="text-xs text-[#86868B] dark:text-[#A1A1A6] w-12">{spicyChickenSoldOut ? 'Sold Out' : 'Available'}</span>
             </div>
           </div>
        </section>

        {loading ? (
           <div className="text-center py-10 text-[#86868B]">Loading orders...</div>
        ) : (
          <section className="space-y-4">
            <div className="flex flex-col gap-4">
              {orders.length === 0 ? (
                 <div className="glassmorphism p-8 text-center text-[#86868B]">No active orders at the moment.</div>
              ) : (
                orders.map((order) => (
                  <div key={order.id} className={`glassmorphism p-4 border ${order.status === 'Ready' ? 'border-green-200 dark:border-green-800' : 'border-blue-200 dark:border-blue-800'} shadow-md flex flex-col sm:flex-row gap-4 items-start sm:items-center relative overflow-hidden`}>
                      <div className={`absolute left-0 top-0 bottom-0 w-1 ${order.status === 'Ready' ? 'bg-green-500' : 'bg-[#0066FF]'} rounded-l-[16px]`}></div>
                      <div className="w-20 flex flex-col items-center justify-center shrink-0">
                          <span className="text-sm font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">{order.pickupTime}</span>
                          <span className="text-xs text-[#86868B] dark:text-[#A1A1A6]">Pickup</span>
                      </div>
                      <div className="flex-1">
                          <div className="flex justify-between items-start mb-1">
                             <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg">{order.customerName}</h3>
                             <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${order.totalAmount.toFixed(2)}</span>
                          </div>
                          <p className="text-sm text-[#86868B] dark:text-[#A1A1A6] mb-2">{order.items.join(', ')}</p>

                          <div className="flex flex-col sm:flex-row gap-2 shrink-0 w-full sm:w-auto mt-3">
                              {order.status === 'Pending' && (
                                <button onClick={() => handleUpdateStatus(order.id, 'Preparing')} className="px-4 py-2 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 hover:bg-blue-200 text-sm font-medium rounded-lg transition-colors">Start Preparing</button>
                              )}
                              {order.status === 'Preparing' && (
                                <button onClick={() => handleUpdateStatus(order.id, 'Ready')} className="px-4 py-2 bg-[#0071E3] hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors shadow-sm">Mark as Ready</button>
                              )}
                              {order.status === 'Ready' && (
                                <button onClick={() => handleUpdateStatus(order.id, 'Completed')} className="px-4 py-2 bg-green-500 hover:bg-green-600 text-white text-sm font-medium rounded-lg transition-colors shadow-sm">Complete Order</button>
                              )}
                          </div>
                      </div>
                      <div className="flex flex-col gap-2 shrink-0 self-start sm:self-end">
                           <span className={`px-3 py-1 text-xs font-medium rounded-full text-center ${
                             order.status === 'Ready' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' :
                             order.status === 'Preparing' ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400' :
                             order.status === 'Completed' ? 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400' :
                             'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
                           }`}>
                             {order.status}
                           </span>
                           <span className={`px-3 py-1 text-xs font-medium rounded-full text-center ${order.paid ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' : 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'}`}>
                              {order.paid ? 'Paid' : 'Unpaid'}
                           </span>
                      </div>
                  </div>
                ))
              )}
            </div>
          </section>
        )}
      </div>
    </AppShell>
  );
}
