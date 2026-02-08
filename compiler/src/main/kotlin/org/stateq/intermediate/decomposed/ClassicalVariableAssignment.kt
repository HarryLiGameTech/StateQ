package org.stateq.intermediate.decomposed

import org.stateq.expression.ClassicalExprBase
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.compiler.CodeGenerator
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location

class ClassicalVariableAssignment<out T: ClassicalTrait>(
    val variable: ClassicalVariableBase<T>,
    val expr: ClassicalExprBase<T>,
    override val location: Location?,
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.classicalVariableInitialization(variable, expr)
    }
}