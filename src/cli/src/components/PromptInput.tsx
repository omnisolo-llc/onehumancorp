import React, { useState } from 'react';
import { Box, Text } from 'ink';
import TextInput from 'ink-text-input';

export interface PromptInputProps {
  onSubmit: (value: string) => void;
  promptText?: string;
}

export const PromptInput: React.FC<PromptInputProps> = ({ onSubmit, promptText = '>' }) => {
  const [value, setValue] = useState('');

  return (
    <Box borderStyle="round" borderColor="cyan" paddingX={1} marginTop={1}>
      <Box marginRight={1}>
        <Text color="cyan" bold>{promptText}</Text>
      </Box>
      <TextInput
        value={value}
        onChange={setValue}
        onSubmit={(val) => {
          onSubmit(val);
          setValue('');
        }}
      />
    </Box>
  );
};
