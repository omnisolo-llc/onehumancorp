// Implement image optimization
export async function optimizeImage(file: File, useWorker: boolean = true): Promise<Blob> {
    // Return original if not an image or very small (< 50KB)
    if (!file.type.startsWith('image/') || file.size < 50 * 1024) {
        return file;
    }

    if (useWorker && typeof Worker !== 'undefined') {
        return new Promise((resolve, reject) => {
            const worker = new Worker(new URL('./imageWorker.ts', import.meta.url), { type: 'module' });

            worker.onmessage = (e) => {
                worker.terminate();
                if (e.data.success) {
                    resolve(e.data.blob);
                } else {
                    reject(new Error(e.data.error));
                }
            };

            worker.onerror = (error) => {
                worker.terminate();
                reject(error);
            };

            worker.postMessage({ file, maxDimension: 2048, quality: 0.8 });
        });
    }

    // Fallback to main thread processing
    return new Promise((resolve, reject) => {
        const img = new Image();
        const url = URL.createObjectURL(file);

        img.onload = () => {
            URL.revokeObjectURL(url);

            // Calculate new dimensions (max 2048px on longest edge)
            const MAX_DIMENSION = 2048;
            let width = img.width;
            let height = img.height;

            if (width > height) {
                if (width > MAX_DIMENSION) {
                    height = Math.round((height * MAX_DIMENSION) / width);
                    width = MAX_DIMENSION;
                }
            } else {
                if (height > MAX_DIMENSION) {
                    width = Math.round((width * MAX_DIMENSION) / height);
                    height = MAX_DIMENSION;
                }
            }

            // Create canvas and draw resized image
            const canvas = document.createElement('canvas');
            canvas.width = width;
            canvas.height = height;

            const ctx = canvas.getContext('2d');
            if (!ctx) {
                reject(new Error('Failed to get canvas context'));
                return;
            }

            ctx.drawImage(img, 0, 0, width, height);

            // Convert to WebP (80% quality)
            canvas.toBlob(
                (blob) => {
                    if (blob) {
                        resolve(blob);
                    } else {
                        reject(new Error('Failed to convert canvas to blob'));
                    }
                },
                'image/webp',
                0.8
            );
        };

        img.onerror = () => {
            URL.revokeObjectURL(url);
            reject(new Error('Failed to load image'));
        };

        img.src = url;
    });
}
