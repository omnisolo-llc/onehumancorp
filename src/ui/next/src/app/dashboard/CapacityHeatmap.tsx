"use client";

import { useEffect, useState } from "react";
import { WithTooltip } from "../../components/TooltipRegistry";

type Resource = {
  id: string;
  name: string;
  base_capacity: number;
};

type LedgerEntry = {
  id: string;
  resource_id: string;
  start_time: string;
  end_time: string;
  consumed_units: number;
  status: string;
};

export function CapacityHeatmap({ tenant }: { tenant: string }) {
  const [resources, setResources] = useState<Resource[]>([]);
  const [ledger, setLedger] = useState<LedgerEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function loadUCAL() {
      try {
        const start = new Date();
        const end = new Date();
        end.setDate(end.getDate() + 7);

        const [resResources, resLedger] = await Promise.all([
          fetch(`/api/v1/ucal/resources?tenant_id=${encodeURIComponent(tenant)}`),
          fetch(`/api/v1/ucal/ledger?tenant_id=${encodeURIComponent(tenant)}&start_time=${start.toISOString()}&end_time=${end.toISOString()}`)
        ]);

        if (resResources.ok && resLedger.ok) {
          const resData = await resResources.json();
          const ledData = await resLedger.json();
          setResources(Array.isArray(resData) ? resData : []);
          setLedger(Array.isArray(ledData) ? ledData : []);
        }
      } catch (e) {
        console.error("Failed to load UCAL data", e);
      } finally {
        setLoading(false);
      }
    }
    loadUCAL();
  }, [tenant]);

  const days = Array.from({ length: 7 }, (_, i) => {
    const d = new Date();
    d.setDate(d.getDate() + i);
    return d;
  });

  const getCapacityStatus = (date: Date) => {
    const dayStart = new Date(date);
    dayStart.setHours(0, 0, 0, 0);
    const dayEnd = new Date(date);
    dayEnd.setHours(23, 59, 59, 999);

    const dayEntries = ledger.filter(e => {
      const start = new Date(e.start_time);
      return start >= dayStart && start <= dayEnd;
    });

    const totalConsumed = dayEntries.reduce((sum, e) => sum + e.consumed_units, 0);
    const totalCapacity = resources.reduce((sum, r) => sum + r.base_capacity, 0) * 8; // Assume 8h workday for heatmap approx

    if (totalCapacity === 0) return "green";
    const ratio = totalConsumed / totalCapacity;

    if (ratio > 0.8) return "red";
    if (ratio > 0.4) return "amber";
    return "green";
  };

  const statusColors = {
    green: "bg-green-500/20 border-green-500/50 text-green-700 dark:text-green-400",
    amber: "bg-amber-500/20 border-amber-500/50 text-amber-700 dark:text-amber-400",
    red: "bg-red-500/20 border-red-500/50 text-red-700 dark:text-red-400",
  };

  return (
    <div className="p-6 rounded-[16px] mb-6 shadow-sm w-full relative overflow-hidden group bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10">
      <div className="flex flex-col gap-4">
        <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] flex items-center gap-2">
          <span className="text-xl">📊</span>
          <WithTooltip id="capacity-heatmap" defaultText="Real-time capacity across all resources.">
            <span>Workload Capacity</span>
          </WithTooltip>
        </h2>

        {loading ? (
          <div className="flex gap-2 animate-pulse">
            {Array.from({ length: 7 }).map((_, i) => (
              <div key={i} className="h-16 flex-1 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
            ))}
          </div>
        ) : (
          <div className="flex gap-2 overflow-x-auto pb-2 hide-scrollbar">
            {days.map((day, i) => {
              const status = getCapacityStatus(day);
              return (
                <div
                  key={i}
                  className={`flex-1 min-w-[50px] p-2 rounded-lg border flex flex-col items-center justify-center transition-all ${statusColors[status]}`}
                >
                  <span className="text-[10px] uppercase font-bold opacity-60">
                    {day.toLocaleDateString('en-US', { weekday: 'short' })}
                  </span>
                  <span className="text-sm font-bold">
                    {day.getDate()}
                  </span>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
