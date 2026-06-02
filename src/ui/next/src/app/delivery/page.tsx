"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function DeliveryAdmin() {
  const [isEnabled, setIsEnabled] = useState(false);
  const [zipCodes, setZipCodes] = useState("");
  const [flatFee, setFlatFee] = useState(5.0);
  const [minOrder, setMinOrder] = useState(20);
  const [tasks, setTasks] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);


  useEffect(() => {
    // Mocking API call for itinerary
    setLoading(false);
    setTasks([
      { id: '1', status: 'PENDING', delivery_location: '123 Main St, New York, NY 10001' },
      { id: '2', status: 'IN_TRANSIT', delivery_location: '456 Broadway, New York, NY 10002' },
      { id: '3', status: 'DELIVERED', delivery_location: '789 5th Ave, New York, NY 10003' }
    ]);
  }, []);

  const saveZoneSettings = () => {
    alert(`Zone settings saved: ${zipCodes}, Fee: $${flatFee}, Min Order: $${minOrder}`);
  };

  const updateTaskStatus = (taskId: string, newStatus: string) => {
    setTasks(tasks.map(t => t.id === taskId ? { ...t, status: newStatus } : t));
    // Here we would call the gRPC UpdateDeliveryTaskStatus
  };


  return (
    <div className="flex flex-col min-h-screen bg-[#F5F5F7] font-inter">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>Local Delivery Dispatch</h1>
        <Link href="/dashboard" className="text-sm font-medium text-indigo-600 hover:text-indigo-800">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col md:flex-row gap-6">
        {/* Left Col: Setup Flow */}
        <div className="flex-1 flex flex-col gap-6">
          <div className="p-6 shadow-sm flex flex-col gap-4 rounded-2xl" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h2 className="text-xl font-semibold text-gray-900">Settings</h2>

            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={isEnabled}
                onChange={(e) => setIsEnabled(e.target.checked)}
                className="w-5 h-5 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
              />
              <span className="text-gray-800 font-medium">Enable Local Delivery</span>
            </label>

            {isEnabled && (
              <>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Delivery Zone (Zip Codes)</label>
                  <WithTooltip id="delivery-zip-tooltip" defaultText="Enter the zip codes you deliver to, separated by commas.">
                    <input
                      type="text"
                      value={zipCodes}
                      onChange={(e) => setZipCodes(e.target.value)}
                      placeholder="e.g. 10001, 10002, 10003"
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    />
                  </WithTooltip>
                </div>

                <div className="flex gap-4">
                  <div className="flex-1">
                    <label className="block text-sm font-medium text-gray-700 mb-1">Flat Fee ($)</label>
                    <input
                      type="number"
                      value={flatFee}
                      onChange={(e) => setFlatFee(parseFloat(e.target.value))}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    />
                  </div>
                  <div className="flex-1">
                    <label className="block text-sm font-medium text-gray-700 mb-1">Min Order ($)</label>
                    <input
                      type="number"
                      value={minOrder}
                      onChange={(e) => setMinOrder(parseInt(e.target.value))}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    />
                  </div>
                </div>

                <button onClick={saveZoneSettings} className="mt-2 w-full px-4 py-2 bg-black text-white rounded-lg font-medium hover:bg-gray-800 transition-colors shadow-sm">
                  Save Zone Settings
                </button>
              </>
            )}
          </div>
        </div>

        {/* Right Col: Driver App View */}
        <div className="flex-1">
          <div className="p-0 shadow-xl overflow-hidden flex flex-col rounded-[2rem] bg-white border-4 border-gray-100" style={{ maxWidth: '375px', margin: '0 auto', height: '700px' }}>
            {/* Phone Header */}
            <div className="bg-black text-white p-4 text-center">
              <h3 className="font-semibold font-outfit">Today's Route</h3>
              <p className="text-xs text-gray-300">Optimal AI Route</p>
            </div>

            {/* Phone Body */}
            <div className="flex-1 overflow-y-auto p-4 bg-gray-50 flex flex-col gap-4">
              <div className="w-full h-32 bg-indigo-100 rounded-xl flex items-center justify-center border border-indigo-200 text-indigo-500 mb-2">
                [ Mini Map Here ]
              </div>

              {loading ? (
                <p className="text-center text-gray-500 mt-10">Loading itinerary...</p>
              ) : tasks.length === 0 ? (
                <p className="text-center text-gray-500 mt-10">No deliveries scheduled for today.</p>
              ) : (
                tasks.map((task, idx) => (
                  <div key={task.id} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col gap-3">
                    <div className="flex justify-between items-start">
                      <span className="font-bold text-gray-900">Stop {idx + 1}</span>
                      <span className={`text-xs px-2 py-1 rounded-full ${task.status === 'DELIVERED' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'}`}>
                        {task.status}
                      </span>
                    </div>
                    <p className="text-sm text-gray-600">{task.delivery_location}</p>
                    <div className="flex gap-2 mt-1">
                      <button className="flex-1 bg-black text-white text-xs font-semibold py-2 rounded-lg" onClick={() => updateTaskStatus(task.id, "IN_TRANSIT")}>
                        Start Route
                      </button>
                      <button className="flex-1 bg-green-500 text-white text-xs font-semibold py-2 rounded-lg" onClick={() => updateTaskStatus(task.id, "DELIVERED")}>
                        Mark Delivered
                      </button>
                    </div>
                    <button className="w-full bg-gray-100 text-gray-700 text-xs font-semibold py-2 rounded-lg mt-1" onClick={() => {}}>
                      Call Customer
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
