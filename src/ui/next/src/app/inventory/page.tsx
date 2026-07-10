"use client";

import { useEffect, useState, useMemo } from "react";
import { AppShell } from "../components/AppShell";

type Product = {
  id: string;
  name: string;
  stock: number;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function CentralizedInventory() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [posSimulateMsg, setPosSimulateMsg] = useState("");

  const fetchInventory = async () => {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/pos/inventory?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load inventory");
      const data = await res.json();
      setProducts(data.inventory || []);
    } catch (e: any) {
      setError(e.message || "Failed to fetch inventory");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchInventory();
  }, []);

  const handleAdjust = async (product: Product, change: number) => {
    // Optimistic update
    setProducts(prev => prev.map(p => {
      if (p.id === product.id) {
        return { ...p, stock: Math.max(0, p.stock + change) };
      }
      return p;
    }));

    try {
      const res = await fetch("/api/pos/inventory/adjust", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenantId()
        },
        body: JSON.stringify({ item_id: product.id, quantity_change: change })
      });
      if (!res.ok) throw new Error("Adjustment failed");
    } catch (e: any) {
      setError(e.message || "Failed to adjust inventory");
      fetchInventory(); // Revert
    }
  };

  const simulatePosSale = async (product: Product) => {
    // 1. Reserve
    try {
      const reserveRes = await fetch("/api/v1/payments/terminal/reserve", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenantId(),
          "x-spiffe-id": `spiffe://ohc/org/${tenantId()}/agent/browser`
        },
        body: JSON.stringify({
          tenant_id: tenantId(),
          product_id: product.id,
          quantity: 1,
          ttl_seconds: 15
        })
      });

      const reserveData = await reserveRes.json();
      if (!reserveRes.ok || !reserveData.success) {
        setPosSimulateMsg("Failed to reserve: " + (reserveData.error_message || "Item is currently being checked out"));
        return;
      }

      // Optimistically deduct stock
      setProducts(prev => prev.map(p => {
        if (p.id === product.id) {
          return { ...p, stock: Math.max(0, p.stock - 1) };
        }
        return p;
      }));
      setPosSimulateMsg(`Reserved! Simulating checkout...`);

      // 2. Commit
      const commitRes = await fetch("/api/v1/payments/terminal/commit", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenantId(),
          "x-spiffe-id": `spiffe://ohc/org/${tenantId()}/agent/browser`
        },
        body: JSON.stringify({
          tenant_id: tenantId(),
          product_id: product.id,
          quantity: 1,
          lock_id: reserveData.lock_id
        })
      });

      const commitData = await commitRes.json();
      if (commitRes.ok && commitData.success) {
        setPosSimulateMsg("Sale completed successfully.");
      } else {
        setPosSimulateMsg("Sale failed.");
        fetchInventory(); // Revert optimistic
      }
    } catch (e: any) {
      setPosSimulateMsg("Error: " + e.message);
      fetchInventory();
    }
  };

  const lowStockProducts = useMemo(
    () => products.filter(p => p.stock <= 5),
    [products]
  );

  return (
    <AppShell
      title="Inventory Ledger"
      subtitle="Centralized dynamic inventory. Mutating stock here syncs globally."
      statusItems={[
        { label: "Products", value: String(products.length), tone: products.length > 0 ? "good" : "neutral" },
        { label: "Low Stock", value: String(lowStockProducts.length), tone: lowStockProducts.length > 0 ? "warn" : "good" }
      ]}
    >
      <div className="app-grid two">
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Variants & Products</div>
              <div className="app-list-subtitle">Manage unified stock across online and in-store.</div>
            </div>
          </div>
          {error && <div className="app-empty" style={{color: 'red'}}>{error}</div>}

          <div className="app-table-wrap">
            <table className="app-table">
              <thead>
                <tr>
                  <th>Product</th>
                  <th>On Hand</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {loading && products.length === 0 ? (
                  <tr><td colSpan={3} className="app-empty">Loading inventory...</td></tr>
                ) : products.length === 0 ? (
                  <tr><td colSpan={3} className="app-empty">No products found.</td></tr>
                ) : (
                  products.map(p => (
                    <tr key={p.id}>
                      <td className="font-semibold">{p.name}</td>
                      <td>{p.stock}</td>
                      <td>
                        <button style={{marginRight: '8px', padding: '4px 12px', background: 'rgba(0,0,0,0.05)', borderRadius: '4px'}} onClick={() => handleAdjust(p, -1)}>-</button>
                        <button style={{padding: '4px 12px', background: 'rgba(0,0,0,0.05)', borderRadius: '4px'}} onClick={() => handleAdjust(p, 1)}>+</button>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">POS Simulator</div>
          </div>
          <div className="app-list" style={{padding: '16px'}}>
            <p style={{marginBottom: '16px', fontSize: '14px', color: '#666'}}>
              Test real-time unified checkout. Processing a sale here will immediately lock and deduct stock from the ledger above.
            </p>
            {posSimulateMsg && <p style={{color: '#0055ff', marginBottom: '16px'}}>{posSimulateMsg}</p>}
            {products.map(p => (
              <div key={p.id} style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px'}}>
                <span>{p.name}</span>
                <button
                  style={{padding: '8px 16px', background: '#0055ff', color: '#fff', borderRadius: '4px'}}
                  onClick={() => simulatePosSale(p)}
                >
                  Sell In-Person (POS)
                </button>
              </div>
            ))}
          </div>
        </section>
      </div>
    </AppShell>
  );
}
