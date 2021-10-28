package com.sonatype.configurator;

import java.io.File;
import java.nio.file.Path;

public class Config implements AutoCloseable {

    private static native long loadConfig(String filePath);
    private static native String configGetSetup(long configPointer);
    private static native void unloadConfig(long configPointer);

    static {
        System.loadLibrary("configurator_java");
    }

    private long configPointer;

    // do not allow a user to construct a config in an invalid state
    private Config() {
    }

    public static Config loadFromFile(Path filePath) throws RuntimeException {
        var config = new Config();
        config.configPointer = loadConfig(filePath.toString());
        if (0 == config.configPointer) {
            throw new RuntimeException("Could not load configuration at " + filePath);
        }
        return config;
    }

    public String getSetup() {
        assertConfigLoaded();
        return configGetSetup(configPointer);
    }

    private void assertConfigLoaded() {
        if (0 == configPointer) {
            throw new RuntimeException("Attempted to use a configuration after it was cleaned up");
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
