package org.stateq.intermediate

import org.stateq.intermediate.decomposible.BasicBlock

abstract class Transpilable {
    protected abstract val transpiled: BasicBlock
    fun transpile(): BasicBlock = transpiled
}
