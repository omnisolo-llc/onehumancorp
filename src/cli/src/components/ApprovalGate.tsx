import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';

export interface ApprovalRequest {
  toolName: string;
  argsJson: string;
  reason: string;
  isHighRisk: boolean;
}

interface ApprovalGateProps {
  request: ApprovalRequest;
  onApprove: () => void;
  onReject: () => void;
  onEdit?: (newArgsJson: string) => void;
}

export const ApprovalGate: React.FC<ApprovalGateProps> = ({ request, onApprove, onReject, onEdit }) => {
  const [mode, setMode] = useState<'view' | 'edit'>('view');

  useInput((input, key) => {
    if (mode === 'view') {
      if (input.toLowerCase() === 'y') {
        onApprove();
      } else if (input.toLowerCase() === 'n') {
        onReject();
      } else if (input.toLowerCase() === 'e' && onEdit) {
        setMode('edit');
      }
    }
  });

  return (
    <Box flexDirection="column" borderStyle="round" borderColor={request.isHighRisk ? 'red' : 'yellow'} padding={1}>
      <Box marginBottom={1}>
        <Text bold color={request.isHighRisk ? 'red' : 'yellow'}>
          {request.isHighRisk ? '⚠️ HIGH RISK APPROVAL REQUIRED ⚠️' : 'Approval Required'}
        </Text>
      </Box>

      <Box marginBottom={1}>
        <Text color="cyan">Tool: </Text>
        <Text>{request.toolName}</Text>
      </Box>

      <Box marginBottom={1} flexDirection="column">
        <Text color="cyan">Arguments:</Text>
        <Box borderStyle="single" borderColor="gray" padding={1}>
          <Text dimColor>{request.argsJson}</Text>
        </Box>
      </Box>

      <Box marginBottom={1}>
        <Text color="cyan">Reason: </Text>
        <Text>{request.reason}</Text>
      </Box>

      {mode === 'view' && (
        <Box marginTop={1}>
          <Text bold>
            Press <Text color="green">Y</Text> to Approve, <Text color="red">N</Text> to Reject{onEdit ? ', or E to Edit' : ''}.
          </Text>
        </Box>
      )}

      {mode === 'edit' && (
        <Box marginTop={1}>
            <Text color="blue">Editing arguments feature is not fully implemented in CLI mockup yet.</Text>
        </Box>
      )}
    </Box>
  );
};
