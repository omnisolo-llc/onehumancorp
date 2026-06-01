'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function EquipmentLeasePage({ params }: { params: { jobId: string } }) {
  const [approved, setApproved] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Simulated fetch of job requirements
  const equipmentDetails = {
    equipment: 'Cement Mixer',
    supplier: 'Local Hardware X',
    pickup: '123 Main St (Friday, 7:00 AM)',
    returnDeadline: 'Friday, 6:00 PM',
    rate: 150,
    deposit: 50,
    jobId: params.jobId
  };

  const handleApprove = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch('/api/v1/ledger/lease', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store',
          'X-User-ID': typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'anon' : 'anon',
        },
        body: JSON.stringify(equipmentDetails)
      });

      if (!response.ok) {
        throw new Error('Failed to secure lease on unified ledger.');
      }

      setApproved(true);
    } catch (err: any) {
      setError(err.message || 'An unknown error occurred.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-4 max-w-md mx-auto" style={{
        background: 'rgba(255, 255, 255, 0.65)',
        backdropFilter: 'blur(30px) saturate(210%)',
        border: '1px solid rgba(255, 255, 255, 0.4)',
        borderRadius: '16px'
    }}>
      <div className="flex items-center mb-6">
        <Link href="/dashboard" className="mr-4 text-[#0066FF] hover:underline">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F]">Equipment Leasing Engine</h1>
      </div>

      {!approved ? (
        <div className="space-y-4">
          <div className="bg-white p-4 rounded-xl border border-[#0066FF] shadow-sm relative overflow-hidden">
             <div className="absolute top-0 left-0 w-full h-1 bg-[#0066FF]"></div>
             <h2 className="font-outfit font-bold text-lg mb-2 text-[#1D1D1F]">Suggested Rental</h2>
             <p className="text-sm text-gray-600 mb-2 font-inter">You need a <span className="font-bold">{equipmentDetails.equipment}</span> for Friday&apos;s &quot;Concrete Pouring&quot; job.</p>
             <div className="bg-gray-50 p-3 rounded-lg mb-3 border border-gray-100">
                <p className="text-sm"><span className="text-gray-500">Supplier:</span> {equipmentDetails.supplier}</p>
                <p className="text-sm"><span className="text-gray-500">Pickup:</span> {equipmentDetails.pickup}</p>
                <p className="text-sm"><span className="text-gray-500">Return Deadline:</span> {equipmentDetails.returnDeadline}</p>
                <p className="text-sm mt-2 pt-2 border-t border-gray-200"><span className="text-gray-500">Rate:</span> ${equipmentDetails.rate} / day</p>
                <p className="text-sm"><span className="text-gray-500">Deposit:</span> ${equipmentDetails.deposit} (Unified Ledger)</p>
             </div>

             {error && <p className="text-sm text-[#FF3B30] mb-3">{error}</p>}

             <button
               onClick={handleApprove}
               disabled={loading}
               className="w-full bg-[#0066FF] text-white font-medium py-3 rounded-[8px] hover:bg-blue-700 transition-colors shadow-md active:scale-[0.98] disabled:opacity-50"
             >
               {loading ? 'Processing Ledger...' : 'Approve 1-Tap Lease'}
             </button>
          </div>
        </div>
      ) : (
        <div className="text-center p-6 bg-white rounded-xl border border-[#34C759] shadow-sm relative overflow-hidden">
             <div className="absolute top-0 left-0 w-full h-1 bg-[#34C759]"></div>
             <div className="w-12 h-12 rounded-full bg-green-100 flex items-center justify-center mx-auto mb-4">
               <svg className="w-6 h-6 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
             </div>
             <h2 className="font-outfit font-bold text-xl text-[#1D1D1F] mb-2">Lease Secured</h2>
             <p className="text-gray-600 font-inter text-sm mb-4">The {equipmentDetails.equipment} deposit has been processed. The ${equipmentDetails.rate} expense will be automatically deducted from the final job payout.</p>
             <Link href="/dashboard" className="text-[#0066FF] font-medium text-sm hover:underline">
               Return to Dashboard
             </Link>
        </div>
      )}
    </div>
  );
}
