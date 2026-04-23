import React, { useState } from 'react';
import { Box, Text } from 'ink';
import TextInput from 'ink-text-input';

export interface InteractivePromptProps {
  onSubmit: (input: string) => void;
}

export const InteractivePrompt: React.FC<InteractivePromptProps> = ({ onSubmit }) => {
  const [value, setValue] = useState('');

  const handleSubmit = (val: string) => {
    onSubmit(val);
    setValue('');
  };

  return (
    <Box>
      <Box marginRight={1}>
        <Text color="green">❯</Text>
      </Box>
      <TextInput
        value={value}
        onChange={setValue}
        onSubmit={handleSubmit}
        placeholder="Enter a command for the agent..."
      />
    </Box>
  );
};
