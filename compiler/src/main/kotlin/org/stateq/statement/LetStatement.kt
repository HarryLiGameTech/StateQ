package org.stateq.statement

import org.stateq.expression.ClassicalExprBase
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposed.ClassicalVariableAssignment
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location

class LetStatement<out T: ClassicalTrait>(
    val variable: ClassicalVariableBase<T>,
    val expr: ClassicalExprBase<T>,
    override val location: Location,
) : Statement() {
    override val transpiled: BasicBlock by lazy {
        ClassicalVariableAssignment(variable, expr, location).asBasicBlock()
    }
}