package org.stateq.intermediate.decomposible

import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.intermediate.decomposed.DecomposedWithBlock
import org.stateq.util.Location

class WithBlock(
    val withExprBlock: BasicBlock,
    val withBody: BasicBlock,
    override val location: Location
): DecomposableInstruction() {
    override val decomposed: DecomposedBasicBlock by lazy {
        DecomposedWithBlock(
            withExprBlock.decompose(), withBody.decompose(), location
        ).asDecomposedBasicBlock()
    }
}
