package org.stateq.expression

import org.stateq.parameter.QrefVariable
import org.stateq.qubit.QubitAccessor
import org.stateq.qubit.QubitAccessorIndexing
import org.stateq.qubit.QubitAccessorSlicing
import org.stateq.qubit.QubitAccessorVariable
import org.stateq.util.*

private typealias QrefBooleanMap = Map<QubitAccessor, Boolean>

abstract class QrefExpr : Locatable, QuantumExpr {
    abstract val size: IntExpr

    abstract fun getQubitAccessors(): Set<QubitAccessor>

    abstract fun getBoolValue(map: QrefBooleanMap): Boolean
}

abstract class QrefAtomicExpr : QrefExpr() {
    abstract val accessor: QubitAccessor
}

class QrefExprVariable(
    val variable: QrefVariable, override val location: Location
) : QrefAtomicExpr() {
    override val size: IntExpr get() = variable.size

    override fun getQubitAccessors(): Set<QubitAccessor> = setOf(this.accessor)

    override fun getBoolValue(map: QrefBooleanMap): Boolean = map[this.accessor]!!

    override val accessor: QubitAccessor by lazy { QubitAccessorVariable(variable) }

    fun slice(
        start: IntExpr?, end: IntExpr?, step: IntExpr = IntExpr(1),
        inclusive: Boolean = false, location: Location? = null
    ) = QrefExprSlice(this, start, end, step, inclusive, location)
}

class QrefExprIndexing(
    variable: QrefExprVariable, val index: IntExpr,
    location: Location? = null,
) : QrefAtomicExpr() {

    override val location: Location = location ?: variable.location

    override val size: IntExpr = IntExpr(1)

    override fun getQubitAccessors(): Set<QubitAccessor> = setOf(this.accessor)

    override fun getBoolValue(map: QrefBooleanMap): Boolean = map[this.accessor]!!

    override val accessor: QubitAccessor by lazy {
        QubitAccessorIndexing(variable.accessor, index)
    }
}

class QrefExprSlice(
    variable: QrefExprVariable,
    start: IntExpr?, end: IntExpr?, step: IntExpr?,
    val inclusive: Boolean,
    location: Location? = null,
) : QrefAtomicExpr() {

    override val location: Location = location ?: variable.location

    val start = start ?: IntExpr(0)
    val end by lazy { (end ?: variable.size) - if (inclusive) 1 else 0 }
    val step = step ?: IntExpr(1)

    override val size by lazy {
        ((this.end - this.start) + if (inclusive) 1 else 0) / this.step
    }

    override fun getQubitAccessors(): Set<QubitAccessor> = setOf(this.accessor)

    override fun getBoolValue(map: QrefBooleanMap): Boolean = map[this.accessor]!!

    override val accessor: QubitAccessor by lazy {
        QubitAccessorSlicing(variable.accessor, this.start, this.end, this.step, inclusive)
    }
}

class QrefExprNot(
    val subject: QrefExpr, location: Location?
) : QrefExpr() {

    override val location: Location = location ?:subject.location

    override val size: IntExpr by lazy { subject.size }

    override fun getQubitAccessors(): Set<QubitAccessor> = subject.getQubitAccessors()

    override fun getBoolValue(map: QrefBooleanMap): Boolean = !subject.getBoolValue(map)
}

abstract class QrefExprBinary(
    val lhs: QrefExpr, val rhs: QrefExpr, location: Location?
) : QrefExpr() {

    override val location: Location = location ?: lhs.location

    override val size: IntExpr by lazy {
        lhs.size.also {
            if (it != rhs.size) {
                CompileError.error(this.location,
                    "Qrefs on both sides of a binary operator are not equal in length: " +
                    "\tsize of LHS = ${lhs.size}, size of RHS = ${rhs.size}"
                ).raise()
            }
        }
    }

    override fun getQubitAccessors(): Set<QubitAccessor> {
        return lhs.getQubitAccessors() + rhs.getQubitAccessors()
    }
}

class QrefExprAnd(
    lhs: QrefExpr, rhs: QrefExpr, location: Location?
) : QrefExprBinary(lhs, rhs, location) {
    override fun getBoolValue(map: QrefBooleanMap): Boolean {
        return lhs.getBoolValue(map) && rhs.getBoolValue(map)
    }
}

class QrefExprOr(
    lhs: QrefExpr, rhs: QrefExpr, location: Location?
) : QrefExprBinary(lhs, rhs, location) {
    override fun getBoolValue(map: QrefBooleanMap): Boolean {
        return lhs.getBoolValue(map) || rhs.getBoolValue(map)
    }
}
