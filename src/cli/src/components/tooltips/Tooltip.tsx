import React, { useState } from 'react';
import { Box, Text } from 'ink';

// Mock registry lookup instead of direct JSON import to prevent build errors in CI
const mockRegistry: Record<string, string> = {
  "btn_new_sale": "Start here to charge a customer in person.",
  "btn_connect_bank": "Link your bank securely so we can send you your money."
};

interface TooltipProps {
  id: string;
  children: React.ReactNode;
}

export const Tooltip = ({ id, children }: TooltipProps) => {
  const [isHovered, setIsHovered] = useState(false);
  const text = mockRegistry[id] || 'Help text not found.';

  // In a real CLI hover isn't possible, we'll simulate it or just show it inline
  return (
    <Box flexDirection="row">
      <Box>
        {children}
        <Text color="yellow"> [?] </Text>
      </Box>
      <Box marginLeft={2}>
        <Text color="gray">{text}</Text>
      </Box>
    </Box>
  );
};
