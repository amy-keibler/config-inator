package com.sonatype.configinator.exceptions;

import java.nio.file.Path;

public class ConfigurationFailedToLoadException extends RuntimeException {
    public ConfigurationFailedToLoadException(Path filePath) {
        super("Could not load configuration at " + filePath);
    }
}
