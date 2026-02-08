package org.stateq.polynomial

import org.stateq.visitor.PolynomialVisitor

class IntMonomial(val coefficient: Int, terms: Map<IndeterminateBase, UInt>) {

    val terms: Map<IndeterminateBase, UInt> = if (coefficient == 0) mapOf() else terms

    constructor(constant: Int): this(constant, mapOf())

    constructor(monomial: IntMonomial): this(monomial.coefficient, monomial.terms)

    constructor(string: String): this(parse(string))

    constructor(indeterminate: IndeterminateBase, exponent: UInt = 1u) : this(
        1, mapOf(Pair(indeterminate, exponent))
    )

    override fun toString() = terms.map { "${it.key}^${it.value}" }.let {
        it.fold("$coefficient") { acc, term -> "${acc}*${term}" }
    }.trim('*')

    override fun hashCode() = coefficient * 31 + terms.hashCode()

    override operator fun equals(other: Any?): Boolean {
        return if (this.isConstant()) {
            if (other is IntMonomial) this.coefficient == other.coefficient
            else this.coefficient == other
        } else if (other is IntMonomial) {
            this.coefficient == other.coefficient && this.terms == other.terms
        } else false
    }

    fun isConstant() = this.terms.isEmpty()

    operator fun plus(other: IntMonomial): IntPolynomial {
        return if (this.equals(0)) {
            if (other.equals(0)) IntPolynomial.ZERO
            else IntPolynomial(other)
        } else {
            IntPolynomial(this, other)
        }
    }

    operator fun minus(other: IntMonomial) = this + other * -1

    operator fun times(coefficient: Int) = IntMonomial(
        this.coefficient * coefficient, terms
    )

    operator fun times(other: IntMonomial) = IntMonomial(
        this.coefficient * other.coefficient,
        this.terms.map { (indeterminate, exponent) ->
            Pair(indeterminate, other.terms[indeterminate]?.let { exponent + it } ?: exponent)
        }.toMap() + other.terms.filter { (indeterminate, _) ->
            !this.terms.contains(indeterminate)
        }
    )

    companion object {
        fun parse(string: String): IntMonomial = PolynomialVisitor.parseMonomial(string)
    }
}
