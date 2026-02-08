package org.stateq.expression

import org.stateq.exception.unreachable
import org.stateq.parameter.IntVariable
import org.stateq.polynomial.*
import org.stateq.util.Location
import org.stateq.type.ClassicalType
import org.stateq.type.IntTrait
import kotlin.math.absoluteValue

class IntExpr(
    polynomial: IntPolynomial,
    override val location: Location? = null
) : IntPolynomial(polynomial), NumericExprBase<IntTrait> {

    override val type: ClassicalType = ClassicalType.Int

    constructor(value: Int, location: Location? = null): this(
        IntPolynomial(IntMonomial(value)), location
    )

    constructor(
        indeterminate: IndeterminateBase,
        location: Location? = null
    ): this(IntPolynomial(IntMonomial(indeterminate)), location)

    operator fun unaryMinus(): IntExpr = IntExpr(0) - this

    operator fun plus(other: Int) = IntExpr(this as IntPolynomial + IntExpr(other), location)

    operator fun plus(other: IntExpr) = IntExpr(this as IntPolynomial + other, location ?: other.location)

    override operator fun plus(other: NumericExpr) = when (other) {
        is IntExpr -> IntExpr(this as IntPolynomial + other, location ?: other.location)
        is FloatExpr -> this.toFloatExpr() + other
        else -> unreachable()
    }

    operator fun minus(other: Int) = IntExpr(this as IntPolynomial - IntExpr(other), location)

    operator fun minus(other: IntExpr) = IntExpr(this as IntPolynomial - other, location ?: other.location)

    override operator fun minus(other: NumericExpr): NumericExpr = when (other) {
        is IntExpr -> IntExpr(this as IntPolynomial - other, location ?: other.location)
        is FloatExpr -> this.toFloatExpr() - other
        else -> unreachable()
    }

    operator fun times(other: IntExpr) = IntExpr(this as IntPolynomial * other, location ?: other.location)

    override operator fun times(other: NumericExpr): NumericExpr = when (other) {
        is IntExpr -> IntExpr(this as IntPolynomial * other, location ?: other.location)
        is FloatExpr -> this.toFloatExpr() * other
        else -> unreachable()
    }

    operator fun rem(other: IntExpr) = IntExpr(IndeterminateLikeModulo(this, other), location ?: other.location)

    override infix fun pow(exponent: UInt) = IntExpr((this as IntPolynomial).pow(exponent), this.location)

    override infix fun pow(exponent: NumericExpr): NumericExpr = when (exponent) {
        is IntExpr -> IntExpr(IndeterminateLikePower(this, exponent), this.location)
        is FloatExpr -> this.toFloatExpr() pow exponent
        else -> unreachable()
    }

    operator fun div(other: IntExpr): IntExpr {
        return try {
            other.getValueIfIsConstant()?.let { IntExpr(this.div(it)) } ?: run {
                IntExpr(IndeterminateLikeDivision(this, other))
            }
        } catch (e: Companion.DivisionException) {
            IntExpr(IndeterminateLikeDivision(this, other))
        }
    }

    override operator fun div(other: NumericExpr): NumericExpr = when (other) {
        is IntExpr -> this / other
        is FloatExpr -> this.toFloatExpr() / other
        else -> unreachable()
    }

    operator fun rangeTo(other: IntExpr) = IntListGenerator(
        this, other, true, IntExpr(1), location
    )

    infix fun until(other: IntExpr) = IntListGenerator(
        this, other, false, IntExpr(1), location
    )

    @Throws(IllegalArgumentException::class)
    fun solve(rhs: IntExpr, variable: IntVariable): IntExpr {
        assert(this.terms.size <= 2)
        val constant = this.terms.find { it.isConstant() }?.coefficient ?: 0
        val coeff = this.terms.find {
            it.terms.size == 1 && it.terms[variable]?.equals(1u) ?: false
        }?.coefficient ?: run {
            throw IllegalArgumentException()
        }
        return IntExpr((rhs - constant) / coeff)
    }

    fun format(formatter: (IndeterminateBase, UInt) -> String): String {
        if (this.isZero()) return "0"
        return this.terms.fold("") { polyString, monomial ->
            "$polyString ${
                when (monomial.coefficient) {
                    1 -> "+"
                    -1 -> "-"
                    else -> "${
                        if (monomial.coefficient > 0) "+ " else "- "
                    } ${monomial.coefficient.absoluteValue} ${
                        if (monomial.terms.isEmpty()) "" else " *"
                    }"
                }
            } ${
                if (monomial.terms.isNotEmpty()) {
                    monomial.terms.entries.fold("") { monoString, (base, exponent) ->
                        "$monoString * ${formatter(base, exponent)}"
                    }.trimStart(' ', '*')
                } else {
                    if (monomial.coefficient.absoluteValue != 1) "" else "1"
                }
            }".trimStart(' ', '+')
        }.trim(' ', '+', '*').let {
            if (this.terms.size > 1) "($it)" else it
        }
    }
}
