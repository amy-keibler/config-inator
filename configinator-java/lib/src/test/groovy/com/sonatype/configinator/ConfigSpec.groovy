package com.sonatype.configinator

import com.sonatype.configinator.exceptions.ConfigurationUsedAfterCleanupException
import com.sonatype.configinator.exceptions.JNIException
import spock.lang.Specification

import java.nio.file.Path

class ConfigSpec extends Specification {

    def 'it should load and use a config'() {
        given: 'a path to a configuration'
        def path = Path.of('src/test/resources/examples/.lift.toml')

        when: 'the file is loaded'
        def subject = Config.loadFromFile(path)

        then: 'the config is available'
        subject.getSetup() == /echo 'Hello, Lift'/
        subject.getBuild() == 'this string totally builds something'
        subject.getImportantRules() == ['rule A']
        subject.getIgnoreRules() == ['rule 2']
        subject.getIgnoreFiles() == '**/requirements.txt'
        subject.getTools() == ['clippy']
        subject.getDisableTools() == ['cobra']
        subject.getCustomTools() == ['custom']
        subject.getAllow() == ['amy']
        subject.getJdk11()
        subject.getAndroidVersion() == 28
        subject.getErrorproneBugPatterns() == ['bug pattern']
        subject.getSummaryComments()
    }

    def 'it should load and use a config when searching a folder'() {
        given: 'a path to a configuration'
        def path = Path.of('src/test/resources/examples/')

        when: 'the file is loaded'
        def subject = Config.loadFromFolder(path)

        then: 'the config is available'
        subject.getSetup() == /echo 'Hello, Lift'/
        subject.getBuild() == 'this string totally builds something'
        subject.getImportantRules() == ['rule A']
        subject.getIgnoreRules() == ['rule 2']
        subject.getIgnoreFiles() == '**/requirements.txt'
        subject.getTools() == ['clippy']
        subject.getDisableTools() == ['cobra']
        subject.getCustomTools() == ['custom']
        subject.getAllow() == ['amy']
        subject.getJdk11()
        subject.getAndroidVersion() == 28
        subject.getErrorproneBugPatterns() == ['bug pattern']
        subject.getSummaryComments()
    }

    def 'it should load a default config when none exists'() {
        given: 'a path to a configuration'
        def path = Path.of('src/test/resources/examples/no_configs/')

        when: 'the file is loaded'
        def subject = Config.loadFromFolder(path)

        then: 'the config is empty'
        subject.getSetup() == null
        subject.getBuild() == null
        subject.getImportantRules() == null
        subject.getIgnoreRules() == null
        subject.getIgnoreFiles() == null
        subject.getTools() == null
        subject.getDisableTools() == null
        subject.getCustomTools() == null
        subject.getAllow() == null
        subject.getJdk11() == null
        subject.getAndroidVersion() == null
        subject.getErrorproneBugPatterns() == null
        subject.getSummaryComments() == null
    }

    def 'it should throw an exception when using the configuration after it is cleaned up'() {
        given: 'a path to a configuration'
        def path = Path.of('src/test/resources/examples/.lift.toml')

        when: 'the file is loaded, closed, and then accessed'
        def subject = Config.loadFromFile(path)
        subject.close()
        subject.getSetup()

        then: 'the config is available'
        def e = thrown(ConfigurationUsedAfterCleanupException)
        e.message == 'Attempted to use a configuration after it was cleaned up'
    }

    def 'it should handle exceptions thrown from the Rust code'() {
        given: 'a path to invalid configuration'
        def path = Path.of('src/test/resources/examples/.lift.json')

        when: 'the file is loaded'
        Config.loadFromFile(path)

        then: 'an exception is thrown'
        def e = thrown(JNIException)
        e.message == 'Failed to parse file as a toml file'
    }
}
