"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type Product = {
  id: string;
  name: string;
  description: string;
  price_cents: number;
  stock: number;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function InventoryDashboard() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  async function loadInventory() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/ui/inventory?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load inventory from the database");
      const data = await res.json();
      setProducts(Array.isArray(data?.inventory) ? data.inventory : []);
    } catch (e: any) {
      setError(e?.message || "Failed to load inventory");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadInventory();
  }, []);

  async function adjustStock(productId: string, quantityChange: number) {
    try {
      setProducts(prev => prev.map(p => {
        if (p.id === productId) {
          return { ...p, stock: Math.max(0, p.stock + quantityChange) };
        }
        return p;
      }));

      const res = await fetch('/api/ui/inventory', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenantId()
        },
        body: JSON.stringify([
          {
            id: `mut-${Date.now()}`,
            payload: {
              item_id: productId,
              quantity_change: quantityChange,
              location_id: "default_loc"
            }
          }
        ])
      });
      if (!res.ok) {
        throw new Error("Failed to update stock");
      }
    } catch (e: any) {
      console.error(e);
      // Revert optimism? Simple refresh for now.
      loadInventory();
    }
  }

  const lowStockCount = products.filter(p => p.stock <= 5).length;

  return (
    <AppShell
      title="Inventory"
      subtitle="Centralized Inventory Ledger"
      statusItems={[
        { label: "Products", value: String(products.length), tone: products.length > 0 ? "good" : "neutral" },
        { label: "Low Stock", value: String(lowStockCount), tone: lowStockCount > 0 ? "warn" : "good" }
      ]}
    >
      <div className="app-grid">
        <section className="app-panel w-full max-w-4xl mx-auto backdrop-blur-[30px] bg-white/70 saturate-[200%] border border-white/50 shadow-xl rounded-2xl">
          <div className="app-panel-header p-6 border-b border-gray-200/50">
            <div>
              <div className="app-panel-title text-2xl font-bold font-outfit text-gray-900">Products & Variants</div>
              <div className="app-list-subtitle text-gray-500 mt-1">Live stock levels across all channels.</div>
            </div>
          </div>
          {error && <div className="p-6 text-red-500 font-medium">{error}</div>}
          {!error && products.length === 0 ? (
            <div className="p-12 text-center text-gray-500">
              {loading ? "Syncing ledger..." : "No products found in the inventory ledger."}
            </div>
          ) : (
            <div className="divide-y divide-gray-100">
              {products.map((product) => {
                const isLow = product.stock <= 5;
                return (
                  <div key={product.id} className="p-6 flex items-center justify-between hover:bg-white/50 transition-colors" data-testid={`product-row-${product.id}`}>
                    <div className="flex-1">
                      <h3 className="font-bold text-gray-900 text-lg">{product.name}</h3>
                      <p className="text-gray-500 text-sm mt-1">{product.description || "No description"}</p>
                      <div className="mt-2 flex items-center space-x-3">
                        <span className="font-medium text-gray-700">${(product.price_cents / 100).toFixed(2)}</span>
                        {isLow && (
                          <span className="px-2 py-0.5 rounded-full bg-red-100 text-red-700 text-xs font-bold border border-red-200">
                            Low Stock
                          </span>
                        )}
                      </div>
                    </div>

                    <div className="flex items-center space-x-4 bg-gray-50/50 p-2 rounded-xl border border-gray-200/50">
                      <button
                        onClick={() => adjustStock(product.id, -1)}
                        className="w-10 h-10 flex items-center justify-center rounded-lg bg-white border border-gray-200 text-gray-600 hover:bg-gray-50 hover:text-gray-900 active:scale-95 transition-all shadow-sm font-bold text-xl"
                        aria-label="Decrease stock"
                        data-testid={`decrease-btn-${product.id}`}
                      >
                        -
                      </button>
                      <div className="w-12 text-center">
                        <span className="font-bold text-xl text-gray-900" data-testid={`stock-count-${product.id}`}>
                          {product.stock}
                        </span>
                      </div>
                      <button
                        onClick={() => adjustStock(product.id, 1)}
                        className="w-10 h-10 flex items-center justify-center rounded-lg bg-[#0066FF] text-white hover:bg-blue-600 active:scale-95 transition-all shadow-sm font-bold text-xl"
                        aria-label="Increase stock"
                        data-testid={`increase-btn-${product.id}`}
                      >
                        +
                      </button>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}
