export const formatStorage = (bytes: number) => {
    const mb = bytes / (1024 * 1024);
    if (mb < 1) return '< 1 MB';
    if (mb >= 1024) return parseFloat((mb / 1024).toFixed(2)) + ' GB';
    return parseFloat(mb.toFixed(1)) + ' MB';
};
