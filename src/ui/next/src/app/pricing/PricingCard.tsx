import React from 'react';
import { ViralTrialExtensionWidget } from '../components/ViralTrialExtensionWidget';

export interface PricingCardProps {
  basePrice?: number;
  isAnnual?: boolean;
  tierName: string;
  price: string;
  priceSuffix?: string;
  isRecommended?: boolean;
  recommendationText?: string;
  features: string[];
  currentPlan: string | null;
  loading: boolean;
  onManageBilling: () => void;
  onUpgrade: (tier: string, isAnnual?: boolean) => void;
}

const PricingCardAction = ({
  loading,
  currentPlan,
  tierName,
  onManageBilling,
  isRecommended,
  onUpgrade,
  isAnnual
}: {
  loading: boolean;
  currentPlan: string | null;
  tierName: string;
  onManageBilling: () => void;
  isRecommended?: boolean;
  onUpgrade: (tier: string, isAnnual?: boolean) => void;
  isAnnual?: boolean;
}) => {
  if (loading) {
    return (
      <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-500 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
        Loading...
      </button>
    );
  }
  if (currentPlan === tierName || (!currentPlan && tierName === 'Free')) {
    if (tierName === 'Free') {
      return (
        <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
          Current Plan
        </button>
      );
    }
    return (
      <button onClick={onManageBilling} className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 hover:bg-gray-300 rounded-xl font-medium flex items-center justify-center transition-colors">
        Manage Plan
      </button>
    );
  }
  if (tierName === 'Free') {
    return (
      <button onClick={onManageBilling} className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 rounded-xl font-medium flex items-center justify-center hover:bg-gray-300 transition-colors">
        Downgrade to Free
      </button>
    );
  }
  if (isRecommended) {
    return (
      <button onClick={() => onUpgrade(tierName, isAnnual)} className="w-full min-h-[44px] px-4 py-2 bg-indigo-600 text-white hover:bg-indigo-700 rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center">
        Upgrade to {tierName} via Stripe
      </button>
    );
  }
  return (
    <button onClick={() => onUpgrade(tierName, isAnnual)} className="w-full min-h-[44px] px-4 py-2 bg-gray-900 text-white hover:bg-black rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center">
      Upgrade to {tierName} via Stripe
    </button>
  );
};

export const PricingCard: React.FC<PricingCardProps> = ({
  basePrice,
  isAnnual,
  tierName,
  price,
  priceSuffix = '/ month',
  isRecommended = false,
  recommendationText,
  features,
  currentPlan,
  loading,
  onManageBilling,
  onUpgrade,
}) => {
  return (
    <div className={`p-6 flex flex-col justify-between ${isRecommended ? 'relative shadow-xl' : 'shadow-lg'} app-card ohc-growth-card glass-card rounded-2xl hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 w-full`}>
      {isRecommended && (
        <div className="absolute top-0 right-0 bg-indigo-600 text-white text-xs font-bold px-3 py-1 rounded-bl-xl rounded-tr-2xl">Recommended</div>
      )}
      <div>
        <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">{tierName}</h3>
        <p className="text-xl font-semibold mb-4 text-gray-900">
          {basePrice !== undefined && basePrice > 0 ? (
            isAnnual ? `$${Math.floor(basePrice * 0.8)}` : `$${basePrice}`
          ) : (
            price
          )} <span className="text-sm font-normal text-gray-500">{isAnnual ? '/month, billed annually' : priceSuffix}</span>
        </p>
        {isRecommended && recommendationText && (
          <p className="text-xs text-indigo-600 font-medium mb-4">{recommendationText}</p>
        )}
        <ul className="text-sm text-gray-700 space-y-3 mb-6">
          {features.map((feature, index) => (
            <li key={index} className="flex items-center gap-2"><span>✓</span> {feature}</li>
          ))}
        </ul>
      </div>

      <PricingCardAction
        tierName={tierName}
        isAnnual={isAnnual}
        isRecommended={isRecommended}
        currentPlan={currentPlan}
        loading={loading}
        onManageBilling={onManageBilling}
        onUpgrade={onUpgrade}
      />

      {tierName === 'Free' && (!loading && (currentPlan === 'Free' || !currentPlan)) && (
        <ViralTrialExtensionWidget />
      )}
    </div>
  );
};
