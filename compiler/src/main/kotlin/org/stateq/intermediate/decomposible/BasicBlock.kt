package org.stateq.intermediate.decomposible

import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.Instruction
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.intermediate.decomposed.DecomposedBasicBlock

class BasicBlock(val instructions: List<Instruction>) {

    operator fun plus(other: BasicBlock) = BasicBlock(this.instructions + other.instructions)

    constructor() : this(listOf())

    constructor(vararg instructions: Instruction) : this(instructions.toList())

    private val decomposed: DecomposedBasicBlock by lazy {
        instructions.map {
            if (it is DecomposableInstruction) {
                it.decompose().instructions
            } else /* it is TranspilableInstruction */ {
                listOf(it as DecomposedInstruction)
            }
        }.fold(listOf<DecomposedInstruction>()) {
            acc, instructions -> acc + instructions
        }.let { DecomposedBasicBlock(it) }
    }

    fun decompose(): DecomposedBasicBlock = decomposed

    companion object {
        fun of(vararg instructions: Instruction) = BasicBlock(instructions.toList())
    }
}
