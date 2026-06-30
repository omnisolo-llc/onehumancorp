"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type InventoryPrediction = {
  id: string;
  product_id: string;
  predicted_stockout_date: string;
  confidence_score: number;
  suggested_reorder_quantity: number;
  product_name?: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function PrepForecast() {
  const [predictions, setPredictions] = useState<InventoryPrediction[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function loadPredictions() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/prep-forecast?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load prep forecast from the database");
        const data = await res.json();
        setPredictions(Array.isArray(data?.predictions) ? data.predictions : []);
      } catch (e: any) {
        setError(e?.message || "Failed to load prep forecast");
      } finally {
        setLoading(false);
      }
    }
    loadPredictions();
  }, []);

  const approvePlan = async (prediction: InventoryPrediction) => {
    try {
      const res = await fetch(`/api/ui/prep-forecast/approve`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          tenant_id: tenantId(),
          prediction_id: prediction.id,
          product_id: prediction.product_id,
          quantity: prediction.suggested_reorder_quantity,
        }),
      });

      if (!res.ok) {
        throw new Error("Failed to approve plan");
      }

      // Filter out the approved prediction
      setPredictions(predictions.filter(p => p.id !== prediction.id));
    } catch (e: any) {
      alert(e.message || "Failed to approve plan");
    }
  };

  const adjustQuantity = (predictionId: string, delta: number) => {
    setPredictions(predictions.map(p => {
      if (p.id === predictionId) {
        return {
          ...p,
          suggested_reorder_quantity: Math.max(0, p.suggested_reorder_quantity + delta)
        };
      }
      return p;
    }));
  };

  return (
    <div className="p-4 sm:p-8 max-w-[1440px] mx-auto w-full">
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight mb-2">Daily Prep Plan</h1>
        <p className="text-muted-foreground opacity-70">AI-generated prep list based on historical sales and weather.</p>
      </div>

      <div className="app-grid one">
        <section className="app-panel" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", border: "1px solid rgba(255, 255, 255, 0.4)", borderRadius: "16px" }}>
          <div className="app-panel-header p-6 border-b border-[var(--border-subtle)]">
            <div>
              <div className="text-xl font-bold">Recommended Prep</div>
              <div className="text-sm opacity-70 mt-1">Review and approve today's prep quantities.</div>
            </div>
            {predictions.length > 0 && (
              <div className="bg-[#FF9500]/10 text-[#FF9500] px-3 py-1 rounded-full text-sm font-semibold">
                {predictions.length} Items to Prep
              </div>
            )}
          </div>

          <div className="p-6">
            {error && <div className="text-[#FF3B30] py-4">{error}</div>}

            {!error && predictions.length === 0 ? (
              <div className="text-center py-12 opacity-50">
                {loading ? "Loading prep forecast..." : "No prep items required for today."}
              </div>
            ) : (
              <div className="flex flex-col gap-4">
                {predictions.map((prediction) => (
                  <div key={prediction.id} className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between p-4 border border-[var(--border-subtle)] rounded-xl bg-white/40" data-testid={`prep-card-${prediction.id}`}>
                    <div>
                      <div className="text-lg font-semibold">{prediction.product_name || `Product ${prediction.product_id}`}</div>
                      <div className="text-sm opacity-70 mt-1">Stockout expected: {new Date(prediction.predicted_stockout_date).toLocaleDateString()} (Confidence: {Math.round(prediction.confidence_score * 100)}%)</div>
                    </div>
                    <div className="flex items-center gap-4 w-full sm:w-auto mt-2 sm:mt-0">
                      <div className="flex items-center border border-[var(--border-subtle)] overflow-hidden bg-white/80 w-full sm:w-auto">
                        <button
                          className="px-3 py-1 bg-black/5 hover:bg-black/10 transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center font-bold text-lg cursor-pointer"
                          onClick={() => adjustQuantity(prediction.id, -1)}
                        >
                          -
                        </button>
                        <div className="px-4 py-1 font-semibold min-h-[44px] flex items-center justify-center min-w-[48px] text-center w-full sm:w-auto">
                          {prediction.suggested_reorder_quantity}
                        </div>
                        <button
                          className="px-3 py-1 bg-black/5 hover:bg-black/10 transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center font-bold text-lg cursor-pointer"
                          onClick={() => adjustQuantity(prediction.id, 1)}
                        >
                          +
                        </button>
                      </div>
                      <button
                        className="app-btn primary min-h-[44px] px-6 font-semibold bg-[#0066FF] hover:bg-[#0052cc] text-white transition-colors cursor-pointer w-full sm:w-auto"
                        onClick={() => approvePlan(prediction)}
                      >
                        Approve
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}
