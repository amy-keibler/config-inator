package com.sonatype.configinator

import spock.lang.Specification

import java.nio.file.Path

class ConfigSpec extends Specification {

    def 'it should load and use a config'() {
        given: 'a path to a configuration'
        def path = Path.of('src/test/resources/examples/.lift.toml')

        when: 'the file is loaded'
        def subject = Config.loadFromFile(path)

        then: 'the config is available'
        def setup = subject.getSetup()
        setup == /echo 'Hello, Lift'/
    }

    def 'it should throw an exception when using the configuration after it is cleaned up'() {
        given: 'a path to a configuration'
        def path = Path.of('src/test/resources/examples/.lift.toml')

        when: 'the file is loaded, closed, and then accessed'
        def subject = Config.loadFromFile(path)
        subject.close()
        subject.getSetup()

        then: 'the config is available'
        def e = thrown(RuntimeException)
        e.message == 'Attempted to use a configuration after it was cleaned up'
    }
}
