self.onmessage = async (e: MessageEvent<{ file: File, maxDimension: number, quality: number }>) => {
    try {
        const { file, maxDimension, quality } = e.data;
        const bitmap = await createImageBitmap(file);

        let width = bitmap.width;
        let height = bitmap.height;

        if (width > height) {
            if (width > maxDimension) {
                height = Math.round((height * maxDimension) / width);
                width = maxDimension;
            }
        } else {
            if (height > maxDimension) {
                width = Math.round((width * maxDimension) / height);
                height = maxDimension;
            }
        }

        const canvas = new OffscreenCanvas(width, height);
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Failed to get 2d context');
        }

        ctx.drawImage(bitmap, 0, 0, width, height);

        const blob = await canvas.convertToBlob({
            type: 'image/webp',
            quality: quality
        });

        self.postMessage({ success: true, blob });
    } catch (error: any) {
        self.postMessage({ success: false, error: error.message });
    }
};
