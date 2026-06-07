"use client";

import { useEffect, useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";
import { WithTooltip } from "../../components/TooltipRegistry";

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

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function InventoryDashboard() {
  const [supply, setSupply] = useState<SupplyPayload>({ vendors: [], raw_materials: [], bom_items: [] });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

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
      subtitle="Supply-chain records from the database, without mock fallback data."
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
              <div className="app-panel-title">Raw Materials</div>
              <div className="app-list-subtitle">Loaded from `/api/ui/supply`.</div>
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
                    <th><WithTooltip id="inventory-on-hand" defaultText="The total count of items currently physically available.">On Hand</WithTooltip></th>
                    <th><WithTooltip id="inventory-threshold" defaultText="When stock drops below this number, you should reorder.">Threshold</WithTooltip></th>
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
