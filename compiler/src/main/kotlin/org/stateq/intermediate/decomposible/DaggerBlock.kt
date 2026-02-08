package org.stateq.intermediate.decomposible

import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.decomposed.BeginDaggerInstruction
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.intermediate.decomposed.EndDaggerInstruction
import org.stateq.util.Location

class DaggerBlock(
    private val daggerBody: BasicBlock,
    override val location: Location?
) : DecomposableInstruction() {
    override val decomposed: DecomposedBasicBlock by lazy {
        BeginDaggerInstruction(location).asDecomposedBasicBlock() +
        daggerBody.decompose() +
        EndDaggerInstruction(location).asDecomposedBasicBlock()
    }
}
