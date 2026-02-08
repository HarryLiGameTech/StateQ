package org.stateq.statement

import org.stateq.expression.QrefExpr
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.QuantumMux
import org.stateq.util.Location

class QifStatement(
    val ctrl: QrefExpr,
    val ifBranch: StatementsBlock,
    val elseBranch: StatementsBlock,
    override val location: Location,
) : Statement() {
    override val transpiled: BasicBlock by lazy {
        ctrl.getQubitAccessors().toList().let { accessors ->
            val ifBlock = ifBranch.statements.flatMap { it.transpile().instructions }
            val elseBlock = elseBranch.statements.flatMap { it.transpile().instructions }
            (0 until (1 shl accessors.size)).map { state ->
                // for each state from 000..000 to 111..111
                // get the result of the boolean expression
                ctrl.getBoolValue(accessors.mapIndexed { idx, accessor ->
                    Pair(accessor, (state shr idx) and 1 == 1)
                }.toMap())
            }.map { boolResult ->
                // result of each row of the truth table
                if (boolResult) BasicBlock(ifBlock) else BasicBlock(elseBlock)
            }.let { blocks ->
                BasicBlock(QuantumMux(accessors, blocks, location))
            }
        }
    }
}
