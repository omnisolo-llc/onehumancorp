import React from 'react';

interface QuoteLineItem {
  description: string;
  unit_price_cents: number;
  quantity: number;
  is_optional: boolean;
}

interface Quote {
  id: string;
  customer_id: string;
  total_amount_cents: number;
  required_deposit_cents: number;
  status: string;
  stripe_payment_link?: string;
  line_items: QuoteLineItem[];
}

interface QuoteReviewCardProps {
  quote: Quote;
  onApprove: (quoteId: string) => void;
  onReject: (quoteId: string) => void;
  onEdit: (quoteId: string) => void;
}

export const QuoteReviewCard: React.FC<QuoteReviewCardProps> = ({ quote, onApprove, onReject, onEdit }) => {
  return (
    <div
      className="p-4 rounded-xl border border-white/10"
      style={{
        width: '375px',
        maxWidth: '100%',
        backdropFilter: 'blur(20px) saturate(200%)',
        background: 'rgba(255, 255, 255, 0.05)',
        fontFamily: 'Outfit, Inter, sans-serif'
      }}
      data-testid="quote-review-card"
    >
      <h3 className="text-lg font-bold mb-2">Review Quote</h3>
      <div className="space-y-2 mb-4 text-sm">
        <p><strong>Customer ID:</strong> {quote.customer_id}</p>
        <p><strong>Status:</strong> {quote.status}</p>
        <div>
          <strong>Line Items:</strong>
          <ul className="list-disc pl-5 mt-1">
            {quote.line_items?.map((item, idx) => (
              <li key={idx}>
                {item.quantity}x {item.description} - ${(item.unit_price_cents / 100).toFixed(2)}
              </li>
            ))}
          </ul>
        </div>
        <p><strong>Total:</strong> ${(quote.total_amount_cents / 100).toFixed(2)}</p>
        <p><strong>Required Deposit (50%):</strong> ${(quote.required_deposit_cents / 100).toFixed(2)}</p>
      </div>

      <div className="flex flex-col space-y-2">
        <button
          onClick={() => onApprove(quote.id)}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-4 rounded-lg flex items-center justify-center transition-colors"
          style={{ minHeight: '44px', touchAction: 'manipulation' }}
          data-testid="approve-quote-button"
        >
          Approve & Send Link
        </button>
        <div className="flex space-x-2">
          <button
            onClick={() => onEdit(quote.id)}
            className="flex-1 bg-gray-700 hover:bg-gray-600 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
            style={{ minHeight: '44px' }}
          >
            Edit
          </button>
          <button
            onClick={() => onReject(quote.id)}
            className="flex-1 bg-red-600/80 hover:bg-red-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
            style={{ minHeight: '44px' }}
          >
            Reject
          </button>
        </div>
      </div>
    </div>
  );
};
