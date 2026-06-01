'use client';
import { useState, useEffect } from 'react';
import Head from 'next/head';

export default function LocalDelivery() {
  const [deliveryEnabled, setDeliveryEnabled] = useState(false);
  const [orders, setOrders] = useState([]);
  const [routes, setRoutes] = useState([]);

  return (
    <div className="flex h-screen bg-[#F5F5F7] font-inter">
      <Head>
        <title>Local Delivery - OHC</title>
      </Head>
      {/* Sidebar would go here */}

      <main className="flex-1 overflow-y-auto">
        <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", borderBottom: "1px solid rgba(255, 255, 255, 0.4)", position: "sticky", top: 0, zIndex: 50 }}>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: "#1D1D1F", letterSpacing: "-0.02em" }}>Local Delivery</h1>
        </header>

        <div className="p-8 max-w-6xl mx-auto space-y-8">
          <div className="bg-white/65 backdrop-blur-[30px] saturate-200 border border-white/40 p-6 rounded-2xl shadow-sm flex items-center justify-between">
            <div>
              <h2 className="text-xl font-semibold text-gray-900 font-outfit">Enable Local Delivery</h2>
              <p className="text-gray-500 mt-1">Allow customers to choose local delivery at checkout.</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" className="sr-only peer" checked={deliveryEnabled} onChange={() => setDeliveryEnabled(!deliveryEnabled)} />
              <div className="w-14 h-7 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="bg-white/65 backdrop-blur-[30px] saturate-200 border border-white/40 p-6 rounded-2xl shadow-sm">
                <h3 className="text-lg font-semibold text-gray-900 mb-4 font-outfit">Pending Orders</h3>
                <div className="space-y-4">
                  <div className="text-gray-500 text-center py-8">No pending orders</div>
                </div>
            </div>

            <div className="bg-white/65 backdrop-blur-[30px] saturate-200 border border-white/40 p-6 rounded-2xl shadow-sm">
                <h3 className="text-lg font-semibold text-gray-900 mb-4 font-outfit">Active Routes</h3>
                <div className="space-y-4">
                  <div className="text-gray-500 text-center py-8">No active routes</div>
                </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
