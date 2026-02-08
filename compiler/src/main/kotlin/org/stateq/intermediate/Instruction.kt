package org.stateq.intermediate

import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.compiler.CodeGenerator
import org.stateq.util.OptionalLocatable

interface Instruction : OptionalLocatable {
    fun asBasicBlock() = BasicBlock.of(this)
}

interface DecomposedInstruction : Instruction {
    fun emit(codegen: CodeGenerator)
    fun asDecomposedBasicBlock() = DecomposedBasicBlock.of(this)
}

abstract class DecomposableInstruction : Instruction {
    protected abstract val decomposed: DecomposedBasicBlock
    fun decompose(): DecomposedBasicBlock = decomposed
}
