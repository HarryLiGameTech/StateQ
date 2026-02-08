package org.stateq.intermediate.decomposible

import org.stateq.expression.BoolExpr
import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.intermediate.decomposed.DecomposedIfStatement
import org.stateq.util.Location

class ClassicalIf(
    val condition: BoolExpr,
    val ifBranch: BasicBlock,
    val elseBranch: BasicBlock?,
    override val location: Location?,
) : DecomposableInstruction() {
    override val decomposed: DecomposedBasicBlock by lazy {
        DecomposedIfStatement(
            condition,
            ifBranch.decompose(),
            elseBranch?.decompose(),
            this.location,
        ).asDecomposedBasicBlock()
    }
}
