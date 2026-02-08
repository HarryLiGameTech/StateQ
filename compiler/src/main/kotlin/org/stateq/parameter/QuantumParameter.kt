package org.stateq.parameter

import org.stateq.expression.IntExpr
import org.stateq.util.Location

interface QuantumParameter : QuantumVariable {
    val sizeInferenceVariable: IntVariable?
}

class QvarParameter(
    ident: String, size: IntExpr,
    override val sizeInferenceVariable: IntVariable?,
    override val location: Location,
) : QuantumParameter, QvarVariable(ident, size)

class QrefParameter(
    ident: String, size: IntExpr,
    override val sizeInferenceVariable: IntVariable?,
    override val location: Location,
) : QuantumParameter, QrefVariable(ident, size)
