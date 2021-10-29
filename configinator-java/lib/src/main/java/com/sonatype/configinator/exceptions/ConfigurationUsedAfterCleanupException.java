package com.sonatype.configinator.exceptions;

public class ConfigurationUsedAfterCleanupException extends RuntimeException {
    public ConfigurationUsedAfterCleanupException() {
        super("Attempted to use a configuration after it was cleaned up");
    }
}
