package org.stateq.statement

import org.stateq.intermediate.Transpilable
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.util.Locatable
import org.stateq.util.Location

class StatementsBlock(
    val statements: List<Statement>,
    override val location: Location
) : Transpilable(), Locatable  {

    fun forEach(action: (Statement) -> Unit) = statements.forEach(action)

    fun <T> map(transform: (Statement) -> T) = statements.map(transform)

    override val transpiled: BasicBlock by lazy {
        statements.fold(BasicBlock()) { block, statement ->
            block + statement.transpile()
        }
    }
}
