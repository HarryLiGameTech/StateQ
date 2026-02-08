package org.stateq.parameter

import org.stateq.expression.*
import org.stateq.type.QuantumType
import org.stateq.util.Location
import org.stateq.util.Sized

interface QuantumVariable: Variable, Sized {
    override val type get() = QuantumType.QubitAccessor
    fun use(location: Location): QuantumExpr
}

open class QrefVariable(
    override val ident: String,
    override val size: IntExpr,
    override val location: Location? = null
): QuantumVariable {
    override fun use(location: Location): QuantumExpr = useAsQref(location)
    fun useAsQref(location: Location) = QrefExprVariable(this, location)
}

open class QvarVariable(
    ident: String, size: IntExpr,
    location: Location? = null
): QrefVariable(ident, size, location) {
    override fun use(location: Location): QuantumExpr = useAsQvar(location)
    fun useAsQvar(location: Location) = QvarExprVariable(this, location)
}

class BindingQvarVariable(
    override val ident: String,
    val expr: QvarExpr,
    override val location: Location,
): QvarVariable(ident, expr.size, location)
