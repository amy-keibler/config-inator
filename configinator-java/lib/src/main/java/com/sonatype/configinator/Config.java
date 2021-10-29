package com.sonatype.configinator;

import com.sonatype.configinator.exceptions.ConfigurationFailedToLoadException;
import com.sonatype.configinator.exceptions.ConfigurationUsedAfterCleanupException;

import java.io.File;
import java.nio.file.Path;

public class Config implements AutoCloseable {

    private static native long loadConfig(String filePath);
    private static native String configGetSetup(long configPointer);
    private static native void unloadConfig(long configPointer);

    static {
        System.loadLibrary("configinator_jni");
    }

    private long configPointer;

    // do not allow a user to construct a config in an invalid state
    private Config() {
    }

    public static Config loadFromFile(Path filePath) throws RuntimeException {
        var config = new Config();
        config.configPointer = loadConfig(filePath.toString());
        if (0 == config.configPointer) {
            throw new ConfigurationFailedToLoadException(filePath);
        }
        return config;
    }

    public String getSetup() {
        assertConfigLoaded();
        return configGetSetup(configPointer);
    }

    private void assertConfigLoaded() {
        if (0 == configPointer) {
            throw new ConfigurationUsedAfterCleanupException();
        }
    }


    // Cleanup on GC
    @Override
    protected void finalize() throws Throwable {
        close();
        super.finalize();
    }

    // Cleanup
    @Override
    public void close() throws Exception {
        if(0 != configPointer) {
            unloadConfig(configPointer);
            configPointer = 0;
        }
    }
}
