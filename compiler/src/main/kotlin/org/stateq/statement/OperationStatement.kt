package org.stateq.statement

import org.stateq.expression.QrefAtomicExpr
import org.stateq.expression.QvarExpr
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.ForLoop
import org.stateq.intermediate.decomposed.QuantumVariableAssignment
import org.stateq.parameter.BindingQvarVariable
import org.stateq.qubit.beginControl
import org.stateq.qubit.endControl
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location

open class OperationStatement(
    val expr: QvarExpr, val ctrl: QrefAtomicExpr?, override val location: Location
) : Statement() {
    override val transpiled: BasicBlock by lazy {
        ctrl?.let { ctrl ->
            BasicBlock(ctrl.accessor.beginControl()) + expr.toBasicBlock() + BasicBlock(ctrl.accessor.endControl())
        } ?: expr.toBasicBlock()
    }
}

class OperationStatementWithComprehension(
    expr: QvarExpr,
    ctrl: QrefAtomicExpr?,
    val comprehensions: List<Comprehension<ClassicalTrait>>,
    location: Location,
) : OperationStatement(expr, ctrl, location) {
    override val transpiled: BasicBlock by lazy {
        comprehensions.reversed().fold(super.transpiled) { block, compre ->
            ForLoop(compre.iterator, compre.iterable, block, location).asBasicBlock()
        }
    }
}

// note: it's "`OperationAs` Statement" not "Operation as Statement"
class OperationAsStatement(
    expr: QvarExpr, ident: String, location: Location,
) : OperationStatement(expr, null, location) {

    val variable = BindingQvarVariable(ident, expr, location)

    override val transpiled: BasicBlock by lazy {
        expr.toQubitAccessorWithBasicBlock().let { (accessor, block) ->
            block + QuantumVariableAssignment(variable, accessor, location).asBasicBlock()
        }
    }
}
