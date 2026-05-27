"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

interface Product {
  id: string;
  title: string;
  inventory_count: number;
  is_perishable: boolean;
  default_ttl_minutes: number;
  is_sold_out: boolean;
  status?: string;
  closes_in?: string;
}

export default function DailyPrep() {
  const [items, setItems] = useState<Product[]>([]);
  const [selectedItem, setSelectedItem] = useState<Product | null>(null);
  const [showFlashSaleModal, setShowFlashSaleModal] = useState(false);
  const [flashSaleSuccess, setFlashSaleSuccess] = useState(false);

  useEffect(() => {
    // Mock data for perishable inventory
    setItems([
      {
        id: "prod-1",
        title: "Chicken Over Rice",
        inventory_count: 15,
        is_perishable: true,
        default_ttl_minutes: 240,
        is_sold_out: false,
        status: "AVAILABLE",
        closes_in: "2h"
      },
      {
        id: "prod-2",
        title: "Vegan Cupcakes",
        inventory_count: 5,
        is_perishable: true,
        default_ttl_minutes: 360,
        is_sold_out: false,
        status: "LOW",
        closes_in: "4h"
      },
      {
         id: "prod-3",
         title: "Lamb Gyro",
         inventory_count: 0,
         is_perishable: true,
         default_ttl_minutes: 240,
         is_sold_out: true,
         status: "SOLD_OUT",
         closes_in: "0h"
      }
    ]);
  }, []);

  const handleClearOut = (item: Product) => {
    setSelectedItem(item);
    setShowFlashSaleModal(true);
    setFlashSaleSuccess(false);
  };

  const handleConfirmFlashSale = () => {
    setFlashSaleSuccess(true);
    setTimeout(() => {
      setShowFlashSaleModal(false);
      // Optimistically update inventory state
      setItems(items.map(i => i.id === selectedItem?.id ? { ...i, status: 'SOLD_OUT', is_sold_out: true, inventory_count: 0 } : i));
    }, 2000);
  };

  const handleMarkSoldOut = (item: Product) => {
    setItems(items.map(i => i.id === item.id ? { ...i, is_sold_out: !i.is_sold_out, inventory_count: i.is_sold_out ? 10 : 0, status: i.is_sold_out ? 'AVAILABLE' : 'SOLD_OUT' } : i));
  };


  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Today's Prep</h1>
        <Link href="/dashboard" className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors text-gray-800">
          Back
        </Link>
      </header>

      <main className="p-6 max-w-md mx-auto w-full flex-1 flex flex-col gap-4">
        {items.map(item => (
          <div key={item.id} className="p-4 shadow-sm flex flex-col gap-3" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex justify-between items-start">
              <div>
                <h3 className="text-lg font-bold font-outfit text-gray-900">{item.title}</h3>
                <p className={`text-sm font-medium ${item.is_sold_out ? 'text-red-500' : 'text-gray-600'}`}>
                  {item.is_sold_out ? 'Sold Out' : `${item.inventory_count} Remaining`}
                </p>
                {!item.is_sold_out && <p className="text-xs text-orange-500 mt-1">Closes in {item.closes_in}</p>}
              </div>
              <label className="flex items-center cursor-pointer">
                <div className="relative">
                  <input type="checkbox" className="sr-only" checked={item.is_sold_out} onChange={() => handleMarkSoldOut(item)} />
                  <div className={`block w-10 h-6 rounded-full transition-colors ${item.is_sold_out ? 'bg-red-500' : 'bg-gray-300'}`}></div>
                  <div className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform ${item.is_sold_out ? 'transform translate-x-4' : ''}`}></div>
                </div>
                <span className="ml-2 text-xs font-medium text-gray-600">86</span>
              </label>
            </div>

            {!item.is_sold_out && (
              <button
                onClick={() => handleClearOut(item)}
                className="w-full py-2 bg-indigo-600 text-white rounded-lg font-medium text-sm hover:bg-indigo-700 transition-colors shadow-sm"
              >
                Clear Out / Flash Sale
              </button>
            )}
          </div>
        ))}
      </main>

      {/* Flash Sale Modal */}
      {showFlashSaleModal && selectedItem && (
        <div className="fixed inset-0 bg-black bg-opacity-30 z-50 flex justify-center items-end sm:items-center p-4">
          <div className="bg-white rounded-t-2xl sm:rounded-2xl p-6 w-full max-w-sm shadow-xl transform transition-transform duration-300 ease-out translate-y-0" style={{ backdropFilter: 'blur(30px) saturate(210%)' }}>
            {!flashSaleSuccess ? (
              <>
                <h3 className="text-xl font-bold font-outfit text-gray-900 mb-2">Magic Action</h3>
                <p className="text-sm text-gray-700 mb-6 leading-relaxed">
                  I'll send a text to your 120 local followers offering these for $5 instead of $10 to sell them out before 3 PM. Sound good?
                </p>
                <div className="flex flex-col gap-3">
                  <button
                    onClick={handleConfirmFlashSale}
                    className="w-full py-3 bg-indigo-600 text-white rounded-xl font-semibold hover:bg-indigo-700 transition-all shadow-md"
                  >
                    Yes, send it
                  </button>
                  <button
                    onClick={() => setShowFlashSaleModal(false)}
                    className="w-full py-3 bg-gray-100 text-gray-800 rounded-xl font-medium hover:bg-gray-200 transition-colors"
                  >
                    Cancel
                  </button>
                </div>
              </>
            ) : (
              <div className="flex flex-col items-center justify-center py-6 text-center">
                <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-green-500 mb-4">
                  <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                </div>
                <h3 className="text-xl font-bold font-outfit text-gray-900 mb-2">Broadcast Sent!</h3>
                <p className="text-sm text-gray-600">Your followers have been notified. Inventory will auto-update as orders come in.</p>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
