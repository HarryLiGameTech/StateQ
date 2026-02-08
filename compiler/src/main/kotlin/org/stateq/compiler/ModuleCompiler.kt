package org.stateq.compiler

import org.stateq.intermediate.IntermediateModule
import org.stateq.parser.StateqParser
import java.nio.file.Path

object ModuleCompiler {

    // TODO: imported functions

    @JvmOverloads
    fun compileModule(code: String, path: Path? = null): IntermediateModule {
        return StateqParser(code, source = path).parseModule().transpile()
    }

    fun clearImports() {
        TODO()
    }

    // TODO: fun importModule
}