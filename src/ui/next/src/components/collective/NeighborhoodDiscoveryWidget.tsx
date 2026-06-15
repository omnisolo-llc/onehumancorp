import React from 'react';

export interface PartnerBusiness {
  id: string;
  name: string;
  category: string;
  bookingUrl: string;
}

export interface NeighborhoodDiscoveryWidgetProps {
  partners?: PartnerBusiness[];
}

export const NeighborhoodDiscoveryWidget: React.FC<NeighborhoodDiscoveryWidgetProps> = ({
  partners = [
    { id: '1', name: "Carlos's Repairs", category: 'Handyman', bookingUrl: '#' },
    { id: '2', name: "Fatima's Food Cart", category: 'Food & Drink', bookingUrl: '#' },
  ],
}) => {
  return (
    <div className="w-full max-w-[375px] fixed bottom-0 left-0 right-0 p-4 backdrop-blur-lg bg-white/40 border-t border-white/50 rounded-t-2xl shadow-[0_-4px_15px_rgba(0,0,0,0.05)]">
      <h4 className="text-sm font-semibold text-gray-800 mb-3">Partner Businesses Nearby</h4>
      <div className="flex flex-col gap-2">
        {partners.map((partner) => (
          <div key={partner.id} className="flex items-center justify-between bg-white/50 p-2 rounded-lg backdrop-blur-sm">
            <div className="flex flex-col">
              <span className="text-sm font-medium text-gray-900">{partner.name}</span>
              <span className="text-xs text-gray-600">{partner.category}</span>
            </div>
            <a
              href={partner.bookingUrl}
              className="text-xs bg-gray-900 text-white px-3 py-1.5 rounded-md hover:bg-gray-800 transition-colors"
            >
              Book
            </a>
          </div>
        ))}
      </div>
    </div>
  );
};
