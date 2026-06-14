import React from 'react';

export interface SubscriptionUpsellWidgetProps {
  isSubscription: boolean;
  setIsSubscription: (val: boolean) => void;
}

export function SubscriptionUpsellWidget({ isSubscription, setIsSubscription }: SubscriptionUpsellWidgetProps) {
  return (
    <div className="flex items-center mb-4">
      <label htmlFor="subscribe" className="flex items-center cursor-pointer group">
        <div className="relative">
          <input
            type="checkbox"
            id="subscribe"
            className="sr-only"
            checked={isSubscription}
            onChange={(e) => setIsSubscription(e.target.checked)}
          />
          <div className={`block w-10 h-6 rounded-full transition-colors duration-300 ease-in-out ${isSubscription ? 'bg-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.5)]' : 'bg-gray-300'}`}></div>
          <div className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ease-in-out shadow-sm ${isSubscription ? 'transform translate-x-4' : ''}`}></div>
        </div>
        <div className="ml-3 text-sm font-medium text-gray-700 group-hover:text-gray-900 transition-colors">
          Subscribe & Save 10%
        </div>
      </label>
    </div>
  );
}