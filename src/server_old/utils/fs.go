package utils

import (
    "os"
    "path/filepath"
)

// WriteFileAtomic writes data to a file atomically by writing to a temporary file first
// and then renaming it to the final path. This prevents file corruption on process crash.
func WriteFileAtomic(filename string, data []byte, perm os.FileMode) error {
    dir := filepath.Dir(filename)
    if err := os.MkdirAll(dir, 0755); err != nil {
        return err
    }

    tmpFile, err := os.CreateTemp(dir, filepath.Base(filename)+".*.tmp")
    if err != nil {
        return err
    }
    tmpName := tmpFile.Name()

    if _, err := tmpFile.Write(data); err != nil {
        tmpFile.Close()
        os.Remove(tmpName)
        return err
    }

    if err := tmpFile.Sync(); err != nil {
        tmpFile.Close()
        os.Remove(tmpName)
        return err
    }

    if err := tmpFile.Close(); err != nil {
        os.Remove(tmpName)
        return err
    }

    if err := os.Chmod(tmpName, perm); err != nil {
        os.Remove(tmpName)
        return err
    }

    if err := os.Rename(tmpName, filename); err != nil {
        os.Remove(tmpName)
        return err
    }

    return nil
}
