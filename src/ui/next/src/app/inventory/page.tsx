'use client';
import React, { useState } from 'react';

// Using OHC Glassmorphism design system
export default function InventoryPage() {
  const [items, setItems] = useState([
    { id: 1, name: 'Blue Summer Dress (Size M)', stock: 2, status: 'Low Stock' },
    { id: 2, name: 'Red Blouse', stock: 15, status: 'In Stock' },
    { id: 3, name: 'Black Jeans', stock: 0, status: 'Sold Out' },
  ]);

  return (
    <div className="p-4" style={{ backdropFilter: 'blur(20px) saturate(200%)', backgroundColor: 'rgba(255, 255, 255, 0.05)' }}>
      <h1 className="text-2xl font-bold mb-4 font-outfit text-white">Inventory</h1>

      {/* AI Alert Card */}
      <div className="mb-6 p-4 rounded-xl border border-white/10 bg-white/5">
        <p className="text-sm text-white/90">
          ✨ Heads up Priya, your 'Blue Summer Dress (Size M)' has been selling fast this week. You have 2 left. Consider ordering more before Friday.
        </p>
      </div>

      <div className="space-y-4">
        {items.map(item => (
          <div key={item.id} className="p-4 rounded-xl border border-white/10 flex justify-between items-center bg-white/5">
            <div>
              <h3 className="font-semibold text-white">{item.name}</h3>
              <p className="text-sm text-white/70">Stock: {item.stock}</p>
            </div>
            <div className={`px-3 py-1 rounded-full text-sm ${
              item.status === 'In Stock' ? 'bg-green-500/20 text-green-300' :
              item.status === 'Low Stock' ? 'bg-yellow-500/20 text-yellow-300' :
              'bg-red-500/20 text-red-300'
            }`}>
              {item.status}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
