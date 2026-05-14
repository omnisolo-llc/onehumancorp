import React from 'react';
import { Box, Text } from 'ink';

interface WalkthroughOverlayProps {
  step: number;
  totalSteps: number;
  message: string;
}

export const WalkthroughOverlay = ({ step, totalSteps, message }: WalkthroughOverlayProps) => {
  return (
    <Box padding={1} borderStyle="round" borderColor="magenta" flexDirection="column" width={60}>
      <Text color="magenta" bold>Interactive Guide ({step}/{totalSteps})</Text>
      <Box marginTop={1} marginBottom={1}>
        <Text>{message}</Text>
      </Box>
      <Text color="gray">Press 'n' for Next, 'x' to Close</Text>
    </Box>
  );
};
