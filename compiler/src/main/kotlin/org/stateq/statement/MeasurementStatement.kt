package org.stateq.statement

import org.stateq.expression.QvarExpr
import org.stateq.intermediate.decomposed.Measurement
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.util.Location

class MeasurementStatement(expr: QvarExpr, location: Location) : OperationStatement(expr, null, location) {
    override val transpiled: BasicBlock by lazy {
        val (accessor, basicBlock) = expr.toQubitAccessorWithBasicBlock()
        basicBlock + Measurement(accessor, expr.location).asBasicBlock()
    }
}
