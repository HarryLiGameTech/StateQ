package org.stateq.parser

import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource
import org.stateq.util.assertNoCompileErrors
import org.stateq.util.readResourceFile

class ModuleParserTest {
    @ParameterizedTest
    @ValueSource(strings = [
        "bernstein_vazirani.qc"
    ])
    fun `test module parsing`(name: String) {
        val code = readResourceFile("stateq/$name")
        val parser = StateqParser(code)
        assertNoCompileErrors(code) {
            parser.parseToContextRule { instance -> instance.module() }
        }
    }
}
