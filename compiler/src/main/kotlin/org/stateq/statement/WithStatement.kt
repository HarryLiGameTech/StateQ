package org.stateq.statement

import org.stateq.expression.QvarExpr
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.WithBlock
import org.stateq.util.Location

class WithStatement(
    val expr: QvarExpr, val body: StatementsBlock,
    override val location: Location
): Statement() {
    override val transpiled: BasicBlock by lazy {
        WithBlock(expr.toBasicBlock(), body.transpile(), location).asBasicBlock()
    }
}
