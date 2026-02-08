package org.stateq.statement

import org.stateq.expression.BoolExpr
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.ClassicalIf
import org.stateq.util.Location

class CifStatement(
    val condition: BoolExpr,
    val ifBranch: StatementsBlock,
    val elseBranch: StatementsBlock?,
    override val location: Location,
) : Statement() {
    override val transpiled: BasicBlock by lazy {
        ClassicalIf(
            condition, ifBranch.transpile(), elseBranch?.transpile(), location
        ).asBasicBlock()
    }
}
