"use client";

import { useEffect, useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";
import useSWR from "swr";
import { v4 as uuidv4 } from "uuid";

type RawMaterial = {
  id: string;
  name: string;
  current_quantity: number;
  reorder_threshold: number;
};

type Vendor = {
  id: string;
  name: string;
  contact_info?: string;
};

type SupplyPayload = {
  vendors: Vendor[];
  raw_materials: RawMaterial[];
  bom_items: unknown[];
};

type ProductInventory = {
  id: string;
  name: string;
  stock: number;
};

const fetcher = (url: string) => fetch(url, { headers: { 'x-tenant-id': tenantId() } }).then((res) => res.json());


function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function InventoryDashboard() {
  const [supply, setSupply] = useState<SupplyPayload>({ vendors: [], raw_materials: [], bom_items: [] });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  // Real-time Centralized Inventory using SWR with 2-second polling
  const { data: posData, mutate: mutatePosInventory } = useSWR(`/api/pos/inventory`, fetcher, { refreshInterval: 2000 });
  const products: ProductInventory[] = posData?.inventory || [];
  const isPosDataLoading = !posData && !error;

  const handleUpdateStock = async (productId: string, currentStock: number, delta: number) => {
    const newStock = Math.max(0, currentStock + delta);

    // Optimistic update
    mutatePosInventory(
      { inventory: products.map((p) => p.id === productId ? { ...p, stock: newStock } : p) },
      false
    );

    try {
      await fetch('/api/pos/inventory', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenantId(),
        },
        body: JSON.stringify([
          {
            id: uuidv4(),
            payload: { item_id: productId, new_stock: newStock, is_sold_out: newStock <= 0 }
          }
        ])
      });
    } catch (e) {
      console.error("Failed to update stock", e);
    } finally {
      // Re-fetch to ensure sync
      mutatePosInventory();
    }
  };

  useEffect(() => {
    async function loadSupply() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/supply?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load inventory from the database");
        const data = await res.json();
        setSupply({
          vendors: Array.isArray(data?.vendors) ? data.vendors : [],
          raw_materials: Array.isArray(data?.raw_materials) ? data.raw_materials : [],
          bom_items: Array.isArray(data?.bom_items) ? data.bom_items : [],
        });
      } catch (e: any) {
        setError(e?.message || "Failed to load inventory");
      } finally {
        setLoading(false);
      }
    }
    loadSupply();
  }, []);

  const lowStockMaterials = useMemo(
    () => supply.raw_materials.filter((item) => item.current_quantity <= item.reorder_threshold),
    [supply.raw_materials],
  );

  return (
    <AppShell
      title="Inventory"
      subtitle="Supply-chain records from the database, without fallback data."
      statusItems={[
        { label: "Materials", value: String(supply.raw_materials.length), tone: supply.raw_materials.length > 0 ? "good" : "neutral" },
        { label: "Low Stock", value: String(lowStockMaterials.length), tone: lowStockMaterials.length > 0 ? "warn" : "good" },
        { label: "Vendors", value: String(supply.vendors.length), tone: supply.vendors.length > 0 ? "good" : "neutral" },
      ]}
    >
      <div className="app-grid two">
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Centralized Inventory</div>
              <div className="app-list-subtitle">Real-time product stock across all channels.</div>
            </div>
          </div>
          {isPosDataLoading ? (
            <div className="p-8 text-center bg-white/60 backdrop-blur-[30px] saturate-[210%] rounded-2xl border border-white/50 shadow-sm text-gray-500 font-medium animate-pulse">Loading Centralized Inventory...</div>
          ) : products.length === 0 ? (
            <div className="p-8 text-center bg-white/60 backdrop-blur-[30px] saturate-[210%] rounded-2xl border border-white/50 shadow-sm text-gray-500 font-medium">No products found in Centralized Inventory.</div>
          ) : (
            <div className="space-y-3">
              {products.map((product) => (
                <div key={product.id} className="flex justify-between items-center p-4 bg-white/85 backdrop-blur-[40px] saturate-[210%] border border-white/60 rounded-2xl shadow-sm transition-all hover:bg-white hover:shadow-md">
                  <div className="flex flex-col">
                    <span className="font-bold text-gray-900 font-outfit">{product.name}</span>
                    <span className="text-sm font-semibold text-[#0066FF] bg-blue-50/80 inline-block px-2 py-1 rounded-md mt-1 w-fit">Stock: {product.stock}</span>
                  </div>
                  <div className="flex items-center space-x-2 bg-gray-100/50 p-1 rounded-xl border border-gray-200/50">
                    <button
                      onClick={() => handleUpdateStock(product.id, product.stock, -1)}
                      className="w-10 h-10 flex items-center justify-center bg-white hover:bg-gray-50 text-gray-700 font-bold rounded-lg transition-colors active:scale-95 shadow-sm border border-gray-200"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 12H4" /></svg>
                    </button>
                    <button
                      onClick={() => handleUpdateStock(product.id, product.stock, 1)}
                      className="w-10 h-10 flex items-center justify-center bg-white hover:bg-gray-50 text-gray-700 font-bold rounded-lg transition-colors active:scale-95 shadow-sm border border-gray-200"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Raw Materials</div>
              <div className="app-list-subtitle">Live material levels and reorder thresholds.</div>
            </div>
          </div>
          {error && <div className="app-empty">{error}</div>}
          {!error && supply.raw_materials.length === 0 ? (
            <div className="app-empty">{loading ? "Loading inventory from the database..." : "No raw material rows found for this tenant."}</div>
          ) : (
            <div className="app-table-wrap">
              <table className="app-table">
                <thead>
                  <tr>
                    <th>Material</th>
                    <th>On Hand</th>
                    <th>Threshold</th>
                    <th>Status</th>
                  </tr>
                </thead>
                <tbody>
                  {supply.raw_materials.map((material) => {
                    const low = material.current_quantity <= material.reorder_threshold;
                    return (
                      <tr key={material.id} data-testid={`alert-card-${material.id}`}>
                        <td className="font-semibold">{material.name}</td>
                        <td>{material.current_quantity}</td>
                        <td>{material.reorder_threshold}</td>
                        <td><span className={`app-badge ${low ? "warn" : "good"}`}>{low ? "Low Stock" : "Healthy"}</span></td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          )}
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">Vendors</div>
          </div>
          <div className="app-list">
            {supply.vendors.length === 0 ? (
              <div className="app-empty">{loading ? "Loading vendors from the database..." : "No vendor rows found for this tenant."}</div>
            ) : supply.vendors.map((vendor) => (
              <div key={vendor.id} className="app-list-item">
                <div>
                  <div className="app-list-title">{vendor.name}</div>
                  <div className="app-list-subtitle">{vendor.contact_info || "No contact info recorded"}</div>
                </div>
              </div>
            ))}
          </div>
        </section>
      </div>
    </AppShell>
  );
}
