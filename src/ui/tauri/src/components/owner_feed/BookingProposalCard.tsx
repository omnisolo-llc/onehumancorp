import React, { useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { format } from 'date-fns';

export interface BookingProposalCardProps {
  proposalId: string;
  serviceDescription: string;
  timeOptions: string[];
  depositAmount: number;
  onApproveAndSend: (proposalId: string) => void;
  onReject?: (proposalId: string) => void;
}

export function BookingProposalCard({
  proposalId,
  serviceDescription,
  timeOptions,
  depositAmount,
  onApproveAndSend,
  onReject
}: BookingProposalCardProps) {
  const [isSending, setIsSending] = useState(false);
  const [hasSent, setHasSent] = useState(false);

  const handleSend = async () => {
    setIsSending(true);
    try {
      await onApproveAndSend(proposalId);
      setHasSent(true);
    } catch (e) {
      console.error(e);
    } finally {
      setIsSending(false);
    }
  };

  return (
    <Card className="w-full max-w-md mx-auto my-2 border-primary/20 shadow-md transition-all duration-300">
      <CardHeader className="pb-2">
        <CardTitle className="text-lg font-semibold flex items-center gap-2">
          <span className="bg-primary/10 text-primary px-2 py-1 rounded text-xs uppercase tracking-wider">Draft Proposal</span>
          Booking Request
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="bg-muted p-3 rounded-md">
          <h4 className="text-sm font-medium text-muted-foreground mb-1">Service</h4>
          <p className="text-sm">{serviceDescription}</p>
        </div>

        <div className="bg-muted p-3 rounded-md">
          <h4 className="text-sm font-medium text-muted-foreground mb-1">Time Options</h4>
          <ul className="text-sm space-y-1">
            {timeOptions.map((time, idx) => (
              <li key={idx} className="flex items-center before:content-[''] before:w-1.5 before:h-1.5 before:bg-primary before:rounded-full before:mr-2">
                {format(new Date(time), 'EEEE, MMMM do yyyy - h:mm a')}
              </li>
            ))}
          </ul>
        </div>

        <div className="flex justify-between items-center bg-primary/5 p-3 rounded-md border border-primary/10">
          <h4 className="text-sm font-medium text-muted-foreground">Required Deposit</h4>
          <span className="text-base font-bold text-primary">${depositAmount.toFixed(2)}</span>
        </div>
      </CardContent>
      <CardFooter className="flex gap-2 pt-2">
        {onReject && (
          <Button variant="outline" className="flex-1" onClick={() => onReject(proposalId)} disabled={isSending || hasSent}>
            Edit / Reject
          </Button>
        )}
        <Button
          className="flex-2 bg-primary hover:bg-primary/90 min-w-[140px] w-full touch-manipulation"
          onClick={handleSend}
          disabled={isSending || hasSent}
          style={{ minHeight: '44px' }}
        >
          {isSending ? 'Sending...' : hasSent ? 'Sent!' : 'Approve & Send'}
        </Button>
      </CardFooter>
    </Card>
  );
}
