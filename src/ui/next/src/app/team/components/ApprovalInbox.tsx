import React, { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Textarea } from '@/components/ui/textarea';

type ActionRisk = 'LOW' | 'HIGH';
type ApprovalStatus = 'PendingApproval' | 'Approved' | 'Rejected';

interface ApprovalRequest {
  id: string;
  department: string;
  description: string;
  status: ApprovalStatus;
  action_risk: ActionRisk;
  payload?: any;
}

interface ApprovalInboxProps {
  requests: ApprovalRequest[];
  onApprove: (id: string, customPayload?: any) => void;
  onReject: (id: string) => void;
}

export function ApprovalInbox({ requests, onApprove, onReject }: ApprovalInboxProps) {
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editContent, setEditContent] = useState<string>('');

  if (requests.length === 0) {
    return (
      <Card className="bg-white/30 backdrop-blur-xl border-white/20">
        <CardHeader>
          <CardTitle>Approvals Inbox</CardTitle>
          <CardDescription>No pending approvals. Your AI team has it handled.</CardDescription>
        </CardHeader>
      </Card>
    );
  }

  return (
    <div className="space-y-4">
      {requests.map((request) => (
        <Card key={request.id} className="bg-white/30 backdrop-blur-xl border-white/20 shadow-lg">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <div className="space-y-1">
              <CardTitle className="text-sm font-medium">{request.department} Agent</CardTitle>
              <CardDescription>{request.description}</CardDescription>
            </div>
            {request.action_risk === 'HIGH' && (
              <Badge variant="destructive">Action Required</Badge>
            )}
            {request.action_risk === 'LOW' && (
              <Badge variant="secondary">✨ Handled</Badge>
            )}
          </CardHeader>

          <CardContent>
            {request.payload?.feature_type === 'ambassador_reply' && (
              <div className="mt-4 p-4 bg-black/5 rounded-lg space-y-4">
                <div>
                   <p className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Customer Message ({request.payload.platform})</p>
                   <p className="text-sm italic border-l-2 border-gray-300 pl-3 py-1">"{request.payload.original_message}"</p>
                </div>

                {editingId === request.id ? (
                  <div>
                    <p className="text-xs font-semibold text-blue-600 uppercase tracking-wider mb-1">Edit Draft Response</p>
                    <Textarea
                      value={editContent}
                      onChange={(e) => setEditContent(e.target.value)}
                      className="min-h-[100px] text-sm"
                    />
                    <div className="flex gap-2 mt-2 justify-end">
                       <Button size="sm" variant="outline" onClick={() => setEditingId(null)}>Cancel</Button>
                       <Button size="sm" onClick={() => {
                          onApprove(request.id, { ...request.payload, generated_response: editContent });
                          setEditingId(null);
                       }}>Save & Send</Button>
                    </div>
                  </div>
                ) : (
                  <div>
                    <div className="flex justify-between items-end mb-1">
                      <p className="text-xs font-semibold text-blue-600 uppercase tracking-wider">AI Draft Response</p>
                      {request.payload.confidence_score && (
                         <span className="text-[10px] text-gray-500">Confidence: {request.payload.confidence_score}%</span>
                      )}
                    </div>
                    <div className="bg-white p-3 rounded border shadow-sm">
                      <p className="text-sm font-medium">{request.payload.generated_response}</p>
                    </div>

                    {request.status === 'PendingApproval' && request.action_risk === 'HIGH' && (
                       <div className="mt-4 grid grid-cols-2 gap-3">
                           <Button
                             variant="outline"
                             className="w-full h-11 bg-white/50 hover:bg-white/80"
                             onClick={() => {
                                setEditingId(request.id);
                                setEditContent(request.payload.generated_response || '');
                             }}
                           >
                             Edit Draft
                           </Button>
                           <Button
                             className="w-full h-11 bg-blue-600 hover:bg-blue-700 text-white"
                             onClick={() => onApprove(request.id)}
                           >
                             Approve & Send
                           </Button>
                       </div>
                    )}
                  </div>
                )}

                <div className="mt-3 pt-3 border-t border-black/10">
                   <p className="text-[10px] font-semibold text-gray-400 uppercase">Context Used</p>
                   <p className="text-xs text-gray-600 line-clamp-2 mt-1">{request.payload.context_used}</p>
                </div>
              </div>
            )}

            {request.payload?.feature_type !== 'ambassador_reply' && (
              <div className="mt-2 p-2 bg-black/5 rounded text-sm">
                <pre>{JSON.stringify(request.payload, null, 2)}</pre>
              </div>
            )}
          </CardContent>

          {request.status === 'PendingApproval' && request.payload?.feature_type !== 'ambassador_reply' && (
            <CardFooter className="flex justify-between">
              <Button variant="outline" onClick={() => onReject(request.id)}>Reject</Button>
              <Button onClick={() => onApprove(request.id)}>Approve</Button>
            </CardFooter>
          )}
        </Card>
      ))}
    </div>
  );
}
