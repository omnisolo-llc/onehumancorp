import React from 'react';

type PlatformInventory = {
  platform: string;
  product_id: string;
  quantity: number;
  location_id?: string;
};

type ReconciliationPayload = {
  product_name: string;
  sku: string;
  platform_counts: PlatformInventory[];
  recommended_quantity: number;
  discrepancy_reason: string;
};

interface ReconciliationCardProps {
  payload: string;
}

export const ReconciliationCard: React.FC<ReconciliationCardProps> = ({ payload }) => {
  let data: ReconciliationPayload;
  try {
    data = typeof payload === 'string' ? JSON.parse(payload) : payload;
  } catch (e) {
    console.error("Failed to parse reconciliation payload", e);
    return <div className="p-4 text-red-500">Invalid data format</div>;
  }

  return (
    <div className="w-full max-w-[375px] bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-[16px] overflow-hidden shadow-sm">
      <div className="p-5">
        {!data.platform_counts ? (
           <div className="text-sm text-gray-500 italic">No sync details available.</div>
        ) : (
          <>
        <div className="flex items-center gap-3 mb-4">
          <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
            </svg>
          </div>
          <div>
            <h3 className="text-lg font-bold font-outfit text-[#1D1D1F]">Inventory Sync</h3>
            <p className="text-xs text-gray-500">{data.sku}</p>
          </div>
        </div>

        <div className="mb-4">
          <p className="text-sm font-semibold text-gray-900 mb-1">{data.product_name}</p>
          <p className="text-xs text-gray-600 leading-relaxed italic border-l-2 border-orange-400 pl-3">
            {data.discrepancy_reason}
          </p>
        </div>

        <div className="space-y-2 mb-6">
          {data.platform_counts.map((pc, idx) => (
            <div key={idx} className="flex justify-between items-center py-2 border-b border-gray-100 last:border-0">
              <div className="flex items-center gap-2">
                <span className="text-xs font-medium text-gray-700">{pc.platform}</span>
                {pc.location_id && <span className="text-[10px] text-gray-400">({pc.location_id})</span>}
              </div>
              <span className="text-sm font-bold text-gray-900">{pc.quantity}</span>
            </div>
          ))}
        </div>

        <div className="bg-blue-50 rounded-xl p-4 mb-2 border border-blue-100">
          <div className="flex justify-between items-center">
            <span className="text-xs font-bold text-blue-800 uppercase tracking-wider">Recommended Sync</span>
            <span className="text-xl font-black text-blue-600">{data.recommended_quantity}</span>
          </div>
          <p className="text-[10px] text-blue-500 mt-1">This will update all channels to match the lowest reliable count.</p>
        </div>
          </>
        )}
      </div>
    </div>
  );
};
