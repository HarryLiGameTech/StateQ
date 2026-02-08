package org.stateq.qubit

import org.stateq.expression.IntExpr
import org.stateq.intermediate.decomposed.QubitAccessorDeclaration
import org.stateq.intermediate.decomposed.QubitAccessorEncoding
import org.stateq.parameter.QuantumVariable
import org.stateq.util.Location
import org.stateq.util.Sized
import kotlin.math.absoluteValue

abstract class QubitAccessor : Sized {

    abstract val ident: String
    abstract val hashcode: Int

    open fun declare(): QubitAccessorDeclaration? = null

    override fun equals(other: Any?) = other is QubitAccessor && this.ident == other.ident
    override fun hashCode(): Int = this.hashcode

    fun slice(
        start: IntExpr, end: IntExpr, step: IntExpr = IntExpr(1), inclusive: Boolean = false
    ) : QubitAccessor {
        return if ((inclusive && start == end) || (!inclusive && start + 1 == end)) {
            QubitAccessorIndexing(this, start)
        } else {
            QubitAccessorSlicing(this, start, end, step)
        }
    }

    operator fun get(index: IntExpr) = QubitAccessorIndexing(this, index)

    operator fun get(index: Int) = QubitAccessorIndexing(this, IntExpr(index))

    fun encode(
        value: IntExpr, location: Location? = null
    ) = QubitAccessorEncoding(this, value, location)

    fun encode(
        value: Int, location: Location? = null
    ) = QubitAccessorEncoding(this, IntExpr(value), location)
}

class QubitAccessorVariable(val variable: QuantumVariable) : QubitAccessor() {

    override val size: IntExpr get() = variable.size

    override val hashcode = variable.hashCode()

    override fun equals(other: Any?) = other is QubitAccessor && this.ident == other.ident

    override fun hashCode() = this.hashcode

    override val ident: String = variable.ident
}

abstract class QubitAccessorDeclarable : QubitAccessor() {
    override fun declare(): QubitAccessorDeclaration {
        return QubitAccessorDeclaration(this, null)
    }
}

class QubitAccessorAlloc(override val size: IntExpr, val init: IntExpr? = null) : QubitAccessorDeclarable() {

    override val hashcode = qubitAllocCounter++ + size.hashCode() * 131

    override fun equals(other: Any?) = other is QubitAccessor && this.ident == other.ident

    override fun hashCode() = this.hashcode

    override val ident: String by lazy {
        "qubitAlloc_${hashcode.toUInt().toString(16)}"
    }

    companion object {
        private var qubitAllocCounter: Int = 0
    }
}

class QubitAccessorConcat(val accessors: List<QubitAccessor>) : QubitAccessorDeclarable() {

    override val hashcode = accessors.hashCode()

    override fun equals(other: Any?) = other is QubitAccessor && this.ident == other.ident

    override fun hashCode() = this.hashcode

    override val size: IntExpr by lazy {
        accessors.fold(IntExpr(0)) { accumulator, accessor ->
            accumulator + accessor.size
        }
    }

    override val ident: String by lazy {
        "qubitConcat_${hashcode.toUInt().toString(16)}"
    }
}

class QubitAccessorIndexing(val subject: QubitAccessor, val index: IntExpr) : QubitAccessorDeclarable() {

    override val hashcode = subject.hashCode() * 131 + index.hashCode()

    override fun equals(other: Any?) = other is QubitAccessor && this.ident == other.ident

    override fun hashCode() = this.hashcode

    override val size = IntExpr(1)

    override val ident: String by lazy {
        "qubitIndexing_${this.hashcode.toUInt().toString(16)}"
    }
}

class QubitAccessorSlicing(
    val subject: QubitAccessor,
    val start: IntExpr, val end: IntExpr, val step: IntExpr,
    val inclusive: Boolean = false
) : QubitAccessorDeclarable() {

    override val size = (end - start + if (inclusive) 1 else 0) / step

    override val hashcode: Int by lazy {
        subject.hashCode().let {
            it * 131 + start.hashCode()
        }.let {
            it * 131 + end.hashCode()
        }.let {
            it * 131 + step.hashCode()
        }
    }

    override fun equals(other: Any?) = other is QubitAccessor && this.ident == other.ident

    override fun hashCode() = this.hashcode

    override val ident: String by lazy {
        "qubitSlicing_${this.hashcode.toUInt().toString(16)}"
    }
}
