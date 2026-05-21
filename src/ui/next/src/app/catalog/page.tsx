"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

type ItemType = 'PHYSICAL' | 'DIGITAL' | 'SERVICE';

interface UnifiedItem {
  id: string;
  name: string;
  description: string;
  price_cents: number;
  currency: string;
  type: ItemType;
  inventory_count?: number;
  digital_asset_url?: string;
  duration_minutes?: number;
}

export default function CatalogPage() {
  const [items, setItems] = useState<UnifiedItem[]>([]);
  const [isAdding, setIsAdding] = useState(false);
  const [newItem, setNewItem] = useState<Partial<UnifiedItem>>({
    type: 'PHYSICAL',
    currency: 'USD',
    price_cents: 0
  });

  useEffect(() => {
    fetchItems();
  }, []);

  const fetchItems = async () => {
    try {
      const orgId = localStorage.getItem('tenant') || 'system';
      const res = await fetch(`/api/v1/dashboard/snapshot?organization_id=${orgId}`);
      const data = await res.json();
      if (data.catalog_items) {
        setItems(data.catalog_items);
      }
    } catch (e) {
      console.error("Failed to fetch catalog", e);
    }
  };

  const handleSave = async () => {
    console.log("Saving item:", newItem);
    setIsAdding(false);
    const mockItem: UnifiedItem = {
        id: Math.random().toString(),
        name: newItem.name || "Unnamed Item",
        description: newItem.description || "",
        price_cents: newItem.price_cents || 0,
        currency: newItem.currency || "USD",
        type: newItem.type as ItemType,
        ...newItem
    };
    setItems([...items, mockItem]);
  };

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/80 backdrop-blur-xl border-gray-200">
        <div className="flex items-center gap-4">
            <Link href="/dashboard" className="text-blue-600 hover:text-blue-700 font-medium flex items-center gap-1">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                Dashboard
            </Link>
            <h1 className="text-2xl font-bold font-outfit text-gray-900">Catalog</h1>
        </div>
        <button
            onClick={() => setIsAdding(true)}
            className="bg-blue-600 text-white px-4 py-2 rounded-lg font-semibold shadow-sm hover:bg-blue-700 transition-all flex items-center gap-2"
        >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
            Add New
        </button>
      </header>

      <main className="p-6 md:p-8 max-w-5xl mx-auto w-full">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {items.map(item => (
                <div key={item.id} className="bg-white rounded-2xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div className="flex justify-between items-start mb-3">
                        <span className={`px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider ${
                            item.type === 'PHYSICAL' ? 'bg-orange-50 text-orange-600' :
                            item.type === 'DIGITAL' ? 'bg-purple-50 text-purple-600' :
                            'bg-blue-50 text-blue-600'
                        }`}>
                            {item.type}
                        </span>
                        <span className="font-bold text-gray-900 font-outfit">
                            ${(item.price_cents / 100).toFixed(2)}
                        </span>
                    </div>
                    <h3 className="font-bold text-lg text-gray-900 mb-1">{item.name}</h3>
                    <p className="text-sm text-gray-500 line-clamp-2 mb-4">{item.description}</p>

                    <div className="pt-4 border-t border-gray-50 flex items-center justify-between text-xs font-medium text-gray-400">
                        {item.type === 'PHYSICAL' && <span>Stock: {item.inventory_count || 0}</span>}
                        {item.type === 'SERVICE' && <span>Duration: {item.duration_minutes || 0} min</span>}
                        {item.type === 'DIGITAL' && <span>Digital Asset</span>}
                        <button className="text-blue-600 hover:underline">Edit</button>
                    </div>
                </div>
            ))}
            {items.length === 0 && !isAdding && (
                <div className="col-span-full py-20 text-center">
                    <div className="text-5xl mb-4">📦</div>
                    <h2 className="text-xl font-bold text-gray-900 mb-2">Your catalog is empty</h2>
                    <p className="text-gray-500 mb-8">Add your first product or service to start selling.</p>
                    <button
                        onClick={() => setIsAdding(true)}
                        className="bg-gray-900 text-white px-6 py-3 rounded-xl font-bold hover:bg-black transition-all"
                    >
                        Create My First Item
                    </button>
                </div>
            )}
        </div>
      </main>

      {isAdding && (
        <div className="fixed inset-0 bg-black/40 backdrop-blur-sm z-[60] flex items-center justify-center p-4">
            <div className="bg-white w-full max-w-lg rounded-3xl p-8 shadow-2xl animate-in fade-in zoom-in duration-200">
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Add New Item</h2>
                <div className="space-y-5">
                    <div>
                        <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Item Type</label>
                        <div className="flex gap-2 p-1 bg-gray-50 rounded-xl">
                            {(['PHYSICAL', 'DIGITAL', 'SERVICE'] as ItemType[]).map(t => (
                                <button key={t} onClick={() => setNewItem({...newItem, type: t})} className={`flex-1 py-2 rounded-lg text-sm font-bold transition-all ${newItem.type === t ? 'bg-white text-blue-600 shadow-sm' : 'text-gray-500 hover:text-gray-700'}`}>{t.charAt(0) + t.slice(1).toLowerCase()}</button>
                            ))}
                        </div>
                    </div>
                    <div>
                        <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Name</label>
                        <input type="text" placeholder="e.g. Signature Cupcake" className="w-full bg-gray-50 border-none rounded-xl p-4 text-gray-900 focus:ring-2 focus:ring-blue-500 transition-all outline-none" value={newItem.name || ''} onChange={e => setNewItem({...newItem, name: e.target.value})}/>
                    </div>
                    <div className="flex gap-4">
                        <div className="flex-1">
                            <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Price ($)</label>
                            <input type="number" placeholder="0.00" className="w-full bg-gray-50 border-none rounded-xl p-4 text-gray-900 focus:ring-2 focus:ring-blue-500 transition-all outline-none" onChange={e => setNewItem({...newItem, price_cents: Math.round(parseFloat(e.target.value) * 100)})}/>
                        </div>
                        {newItem.type === 'PHYSICAL' && (
                            <div className="flex-1 animate-in slide-in-from-right-4">
                                <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Inventory</label>
                                <input type="number" placeholder="0" className="w-full bg-gray-50 border-none rounded-xl p-4 text-gray-900 focus:ring-2 focus:ring-blue-500 transition-all outline-none" onChange={e => setNewItem({...newItem, inventory_count: parseInt(e.target.value)})}/>
                            </div>
                        )}
                        {newItem.type === 'SERVICE' && (
                            <div className="flex-1 animate-in slide-in-from-right-4">
                                <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Duration (min)</label>
                                <input type="number" placeholder="60" className="w-full bg-gray-50 border-none rounded-xl p-4 text-gray-900 focus:ring-2 focus:ring-blue-500 transition-all outline-none" onChange={e => setNewItem({...newItem, duration_minutes: parseInt(e.target.value)})}/>
                            </div>
                        )}
                    </div>
                    <div>
                        <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Description</label>
                        <textarea placeholder="Tell your customers about this..." rows={3} className="w-full bg-gray-50 border-none rounded-xl p-4 text-gray-900 focus:ring-2 focus:ring-blue-500 transition-all outline-none resize-none" value={newItem.description || ''} onChange={e => setNewItem({...newItem, description: e.target.value})}/>
                    </div>
                </div>
                <div className="flex gap-4 mt-8">
                    <button onClick={() => setIsAdding(false)} className="flex-1 py-4 text-gray-500 font-bold hover:bg-gray-50 rounded-2xl transition-all">Cancel</button>
                    <button onClick={handleSave} disabled={!newItem.name} className={`flex-[2] py-4 rounded-2xl font-bold shadow-lg transition-all ${newItem.name ? 'bg-blue-600 text-white hover:bg-blue-700 active:scale-[0.98]' : 'bg-gray-100 text-gray-400 cursor-not-allowed'}`}>Save Item</button>
                </div>
            </div>
        </div>
      )}
      <style dangerouslySetInnerHTML={{__html: `@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap'); .font-inter { font-family: 'Inter', sans-serif; } .font-outfit { font-family: 'Outfit', sans-serif; }`}} />
    </div>
  );
}
