export async function optimizeImage(file: File): Promise<Blob> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = (e) => {
            if (!e.target?.result) {
                return reject(new Error('Failed to read file'));
            }
            const img = new Image();
            img.onload = () => {
                const maxDim = 2048;
                let width = img.width;
                let height = img.height;

                if (width > maxDim || height > maxDim) {
                    if (width > height) {
                        height = Math.round((height * maxDim) / width);
                        width = maxDim;
                    } else {
                        width = Math.round((width * maxDim) / height);
                        height = maxDim;
                    }
                }

                const canvas = document.createElement('canvas');
                canvas.width = width;
                canvas.height = height;

                const ctx = canvas.getContext('2d');
                if (!ctx) {
                    return reject(new Error('Failed to get canvas context'));
                }

                ctx.drawImage(img, 0, 0, width, height);

                canvas.toBlob((blob) => {
                    if (blob) {
                        resolve(blob);
                    } else {
                        reject(new Error('Canvas toBlob failed'));
                    }
                }, 'image/webp', 0.8);
            };
            img.onerror = () => reject(new Error('Failed to load image'));
            img.src = e.target.result as string;
        };
        reader.onerror = () => reject(new Error('File reading error'));
        reader.readAsDataURL(file);
    });
}
