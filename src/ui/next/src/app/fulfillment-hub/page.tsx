'use client';

import React, { useState, useEffect } from 'react';
import { MutationService } from '../../lib/sync/MutationService';
import { NetworkStatusIndicator } from '../../components/NetworkStatusIndicator';


type Order = {
  id: string;
  fulfillment_mode: string;
  status: string;
  customer_name: string;
  items: string[];
  organization_id: string;
  driver_status?: string;
  driver_id?: string;
  driver_lat?: number;
  driver_lng?: number;
  provider_delivery_id?: string;
};

export default function FulfillmentHub() {
  const [toPack, setToPack] = useState<Order[]>([]);
  const [awaitingPickup, setAwaitingPickup] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchOrders = async () => {
    setLoading(true);
    try {
      const res = await fetch('/api/fulfillment');
      if (res.ok) {
        const data = await res.json();
        setToPack(data.to_pack || []);
        setAwaitingPickup(data.awaiting_pickup || []);
      }
    } catch (e) {
      console.error('Failed to fetch orders', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchOrders();
  }, []);

  const handleAction = async (id: string, action: string) => {
    const previousToPack = [...toPack];
    const previousAwaitingPickup = [...awaitingPickup];

    const optimisticUpdate = () => {
      if (action === 'mark_ready' || action === 'print_label') {
        const orderIndex = toPack.findIndex(o => o.id === id);
        if (orderIndex > -1) {
          const order = toPack[orderIndex];
          setToPack(prev => prev.filter(o => o.id !== id));
          setAwaitingPickup(prev => [...prev, { ...order, status: 'ReadyForPickup' }]);
        }
      } else if (action === 'hand_off') {
        setAwaitingPickup(prev => prev.filter(o => o.id !== id));
      } else if (action === 'request_driver') {
        const orderIndex = awaitingPickup.findIndex(o => o.id === id);
        if (orderIndex > -1) {
          const order = awaitingPickup[orderIndex];
          setAwaitingPickup(prev => prev.map(o => o.id === id ? { ...o, status: 'DriverRequested' } : o));
        }
      }
    };

    const rollback = () => {
      setToPack(previousToPack);
      setAwaitingPickup(previousAwaitingPickup);
    };

    await MutationService.getInstance().executeMutation(
      'fulfillment_action',
      { id, action },
      optimisticUpdate,
      rollback
    );
  };

  const driverBadge = (order: Order) => {
    if (typeof order.driver_lat === 'number' && typeof order.driver_lng === 'number') {
      return `${order.driver_status || 'Driver'} ${order.driver_lat.toFixed(4)}, ${order.driver_lng.toFixed(4)}`;
    }
    return order.driver_status || order.status;
  };

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center">
      <NetworkStatusIndicator />
      <div className="w-full max-w-[375px] bg-white relative pb-20 shadow-xl overflow-hidden">
        {/* App Bar (Translucent Glass) */}
        <div className="sticky top-0 z-50 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] backdrop-saturate-[210%] border-b border-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)] px-4 py-3 flex items-center justify-between">
          <h1 className="text-xl font-bold text-gray-900 tracking-tight">Fulfillment Hub</h1>
          <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center text-[#0071E3] font-bold text-sm">
            OHC
          </div>
        </div>

        <div className="p-4 space-y-8 h-[calc(100vh-60px)] overflow-y-auto pb-24">
          {loading ? (
            <div className="text-center text-gray-500 py-10">Syncing with AI Logistics...</div>
          ) : (
            <>
              {/* To Pack Section */}
              <section>
                <h2 className="text-lg font-semibold text-gray-800 mb-3 flex items-center">
                  <span className="w-2 h-2 rounded-full bg-[#FF9500] mr-2"></span> To Pack
                </h2>
                {toPack.length === 0 ? (
                  <p className="text-sm text-gray-500 italic">All caught up!</p>
                ) : (
                  <div className="space-y-3">
                    {toPack.map((order) => (
                      <div key={order.id} className="app-card rounded-xl shadow-sm border border-gray-100 p-4 relative overflow-hidden group">
                        <div className="absolute top-0 left-0 w-1 h-full bg-orange-400"></div>
                        <div className="flex justify-between items-start mb-2">
                          <div>
                            <p className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">
                              {order.fulfillment_mode === 'Shipping' ? '📦 Shipping' : '🚚 Local'}
                            </p>
                            <h3 className="font-medium text-gray-900">{order.customer_name}</h3>
                            <p className="text-sm text-gray-600">{order.items.join(', ')}</p>
                          </div>
                        </div>
                        <div className="mt-4">
                          {order.fulfillment_mode === 'Shipping' ? (
                            <button
                              onClick={() => handleAction(order.id, 'print_label')}
                              className="w-full py-2.5 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-lg text-sm transition-colors shadow-sm"
                            >
                              Print Label
                            </button>
                          ) : (
                            <button
                              onClick={() => handleAction(order.id, 'mark_ready')}
                              className="w-full py-2.5 bg-gray-900 hover:bg-gray-800 text-white font-medium rounded-lg text-sm transition-colors shadow-sm"
                            >
                              Mark Ready
                            </button>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </section>

              {/* Awaiting Pickup Section */}
              <section>
                <h2 className="text-lg font-semibold text-gray-800 mb-3 flex items-center">
                  <span className="w-2 h-2 rounded-full bg-[#34C759] mr-2"></span> Awaiting Pickup
                </h2>
                {awaitingPickup.length === 0 ? (
                  <p className="text-sm text-gray-500 italic">No pending pickups.</p>
                ) : (
                  <div className="space-y-3">
                    {awaitingPickup.map((order) => (
                      <div key={order.id} className="app-card rounded-xl shadow-sm border border-gray-100 p-4 relative overflow-hidden">
                         <div className="absolute top-0 left-0 w-1 h-full bg-green-400"></div>
                        <div className="flex justify-between items-start mb-2">
                          <div>
                            <p className="text-xs font-semibold text-green-600 bg-green-50 px-2 py-0.5 rounded-full inline-block mb-1">
                              Ready for {order.fulfillment_mode}
                            </p>
                            <h3 className="font-medium text-gray-900">{order.customer_name}</h3>
                            <p className="text-sm text-gray-600">{order.items.join(', ')}</p>
                          </div>
                          {order.fulfillment_mode === 'LocalDelivery' && (
                            <span className="text-xs font-medium text-gray-500 bg-gray-100 px-2 py-1 rounded-full">
                              {driverBadge(order)}
                            </span>
                          )}
                        </div>
                        <div className="mt-4">
                           {order.fulfillment_mode === 'LocalDelivery' && order.status !== 'DriverRequested' ? (
                             <button
                                onClick={() => handleAction(order.id, 'request_driver')}
                                className="w-full py-2.5 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg text-sm transition-colors shadow-sm mb-2"
                              >
                                Request Driver (DoorDash)
                              </button>
                           ) : null}
                           <button
                              onClick={() => handleAction(order.id, 'hand_off')}
                              className="w-full py-2.5 bg-green-600 hover:bg-green-700 text-white font-medium rounded-lg text-sm transition-colors shadow-sm"
                            >
                              Swipe to Hand Off
                            </button>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </section>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
