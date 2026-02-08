package org.stateq.intermediate.decomposed

import org.stateq.intermediate.DecomposedInstruction
import org.stateq.compiler.CodeGenerator

open class DecomposedBasicBlock(val instructions: List<DecomposedInstruction>) {
    fun emit(codegen: CodeGenerator): CodeGenerator.CodeBuilder {
        return codegen.CodeBuilder {
            instructions.forEach { it.emit(this) }
        }
    }

    operator fun plus(other: DecomposedBasicBlock): DecomposedBasicBlock {
        return DecomposedBasicBlock(this.instructions + other.instructions)
    }

    companion object {
        fun of(vararg instructions: DecomposedInstruction): DecomposedBasicBlock {
            return DecomposedBasicBlock(instructions.toList())
        }
    }
}
