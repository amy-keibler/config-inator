package com.sonatype.configinator;

import com.sonatype.configinator.exceptions.ConfigurationFailedToLoadException;
import com.sonatype.configinator.exceptions.ConfigurationUsedAfterCleanupException;

import java.io.File;
import java.nio.file.Path;
import java.util.List;

public class Config implements AutoCloseable {

    private static native long loadConfigFromFile(String filePath);
    private static native long loadConfigFromFolder(String filePath);
    private static native void unloadConfig(long configPointer);
    private static native String configGetSetup(long configPointer);
    private static native String configGetBuild(long configPointer);
    private static native List<String> configGetImportantRules(long configPointer);
    private static native List<String> configGetIgnoreRules(long configPointer);
    private static native String configGetIgnoreFiles(long configPointer);
    private static native List<String> configGetTools(long configPointer);
    private static native List<String> configGetDisableTools(long configPointer);
    private static native List<String> configGetCustomTools(long configPointer);
    private static native List<String> configGetAllow(long configPointer);
    private static native Boolean configGetJdk11(long configPointer);
    private static native Integer configGetAndroidVersion(long configPointer);
    private static native List<String> configGetErrorproneBugPatterns(long configPointer);
    private static native Boolean configGetSummaryComments(long configPointer);

    static {
        System.loadLibrary("configinator_jni");
    }

    private long configPointer;

    // do not allow a user to construct a config in an invalid state
    private Config() {
    }

    public static Config loadFromFile(Path filePath) throws RuntimeException {
        var config = new Config();
        config.configPointer = loadConfigFromFile(filePath.toString());
        if (0 == config.configPointer) {
            throw new ConfigurationFailedToLoadException(filePath);
        }
        return config;
    }

    public static Config loadFromFolder(Path filePath) throws RuntimeException {
        var config = new Config();
        config.configPointer = loadConfigFromFolder(filePath.toString());
        if (0 == config.configPointer) {
            throw new ConfigurationFailedToLoadException(filePath);
        }
        return config;
    }

    public String getSetup() {
        assertConfigLoaded();
        return configGetSetup(configPointer);
    }

    public String getBuild() {
        assertConfigLoaded();
        return configGetBuild(configPointer);
    }

    public List<String> getImportantRules() {
        assertConfigLoaded();
        return configGetImportantRules(configPointer);
    }

    public List<String> getIgnoreRules() {
        assertConfigLoaded();
        return configGetIgnoreRules(configPointer);
    }

    public String getIgnoreFiles() {
        assertConfigLoaded();
        return configGetIgnoreFiles(configPointer);
    }

    public List<String> getTools() {
        assertConfigLoaded();
        return configGetTools(configPointer);
    }

    public List<String> getDisableTools() {
        assertConfigLoaded();
        return configGetDisableTools(configPointer);
    }

    public List<String> getCustomTools() {
        assertConfigLoaded();
        return configGetCustomTools(configPointer);
    }

    public List<String> getAllow() {
        assertConfigLoaded();
        return configGetAllow(configPointer);
    }

    public Boolean getJdk11() {
        assertConfigLoaded();
        return configGetJdk11(configPointer);
    }

    public Integer getAndroidVersion() {
        assertConfigLoaded();
        return configGetAndroidVersion(configPointer);
    }

    public List<String> getErrorproneBugPatterns() {
        assertConfigLoaded();
        return configGetErrorproneBugPatterns(configPointer);
    }

    public Boolean getSummaryComments() {
        assertConfigLoaded();
        return configGetSummaryComments(configPointer);
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
