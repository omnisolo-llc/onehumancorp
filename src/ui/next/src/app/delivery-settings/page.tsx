"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { WithTooltip } from "@/components/WithTooltip";

export default function DeliverySettingsPage() {
  const router = useRouter();
  const [flatFee, setFlatFee] = useState<number>(5.0);
  const [minOrder, setMinOrder] = useState<number>(20.0);
  const [maxDeliveries, setMaxDeliveries] = useState<number>(10);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleSave = () => {
    setIsProcessing(true);
    setTimeout(() => {
      setIsProcessing(false);
      alert("Delivery settings saved successfully.");
      router.push("/dashboard");
    }, 1000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Local Delivery Settings</h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        <p className="text-gray-700">Configure your local delivery zone and capacity.</p>

        <div className="p-6 shadow-sm flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-semibold text-gray-700">Flat Fee ($)</label>
            <input type="number" value={flatFee} onChange={e => setFlatFee(parseFloat(e.target.value))} className="border p-2 rounded-lg" />
          </div>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-semibold text-gray-700">Minimum Order Value ($)</label>
            <input type="number" value={minOrder} onChange={e => setMinOrder(parseFloat(e.target.value))} className="border p-2 rounded-lg" />
          </div>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-semibold text-gray-700">Max Deliveries Per Day</label>
            <input type="number" value={maxDeliveries} onChange={e => setMaxDeliveries(parseInt(e.target.value, 10))} className="border p-2 rounded-lg" />
          </div>

          <WithTooltip id="delivery-save-tooltip" defaultText="Save these settings for your delivery zone.">
            <button
              onClick={handleSave}
              disabled={isProcessing}
              className={`w-full mt-4 px-4 py-3 text-white rounded-lg font-medium transition-colors shadow-sm ${isProcessing ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700'}`}
            >
              {isProcessing ? 'Saving...' : 'Save Settings'}
            </button>
          </WithTooltip>
        </div>
      </main>
    </div>
  );
}
