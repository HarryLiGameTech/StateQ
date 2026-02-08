package org.stateq.polynomial

import org.stateq.visitor.PolynomialVisitor

private typealias Term = IntMonomial
private typealias TermsMap = Map<Map<IndeterminateBase, UInt>, Int>
private typealias MutTermsMap = MutableMap<Map<IndeterminateBase, UInt>, Int>

open class IntPolynomial(terms: Collection<Term>) {

    val terms: Set<Term> = terms.filter { !it.equals(0) }.let { terms ->
        val termsMap: MutTermsMap = mutableMapOf()
        terms.forEach { monomial ->
            termsMap.compute(monomial.terms) { _, coeff ->
                coeff?.plus(monomial.coefficient) ?: monomial.coefficient
            }
        }
        termsMap.map { (term, coeff) ->
            IntMonomial(coeff, term)
        }.filter { monomial -> !monomial.equals(0) }.toSet()
    }

    constructor(polynomial: IntPolynomial): this(polynomial.terms)

    constructor(vararg terms: Term): this(terms.toSet())

    constructor(terms: TermsMap): this(
        terms.map { (term, coeff) -> IntMonomial(coeff, term) }.toSet()
    )

    constructor(string: String): this(parse(string))

    private fun termsMap() = terms.associate { Pair(it.terms, it.coefficient) }

    fun map(transform: (Term) -> Term) = IntPolynomial(terms.map(transform).toSet())

    fun isZero() = this.terms.isEmpty()

    fun isConstant(): Boolean {
        return this.isZero() || (
            this.terms.size == 1 && this.terms.first().isConstant()
        )
    }

    fun getValueIfIsConstant(): Int? {
        return if (this.isZero()) 0 else {
            if (this.isConstant()) this.terms.first().coefficient else null
        }
    }

    private fun gcd(a: Int, b: Int): Int = if (b == 0) a else gcd(b, a % b)

    fun coefficientGcd() = if (isZero()) 0 else {
        terms.map { it.coefficient }.let { coefficients ->
            coefficients.fold(coefficients[0]) { acc, coeff -> gcd(acc, coeff) }
        }
    }

    override fun hashCode() = terms.hashCode()

    override fun equals(other: Any?): Boolean {
        return if (other == 0) {
            this.terms.isEmpty()
        } else {
            other is IntPolynomial && termsMap() == other.termsMap()
        }
    }

    override fun toString() = if (this.isZero()) "0" else {
        terms.map(IntMonomial::toString).fold("") { acc, s -> "$acc + $s" }.trim(' ', '+')
    }

    operator fun plus(other: IntPolynomial): IntPolynomial {
        if (this.isZero()) {
            return other
        } else if (other.isZero()) {
            return this
        }
        val selfTerms = this.termsMap()
        val otherTerms = other.termsMap()
        return (
            selfTerms.map { (term, coeff) ->
                Pair(term, otherTerms[term]?.let { coeff + it } ?: coeff)
            }.toMap() + otherTerms.filter { (term, _) ->
                !selfTerms.contains(term)
            }
        ).let { IntPolynomial(it) }
    }

    operator fun minus(other: IntPolynomial) = this + (other * -1)

    operator fun times(other: Int) = this.map { it * other }

    operator fun times(other: IntMonomial) = this.map { it * other }

    operator fun times(other: IntPolynomial): IntPolynomial {
        return this.terms.map { other * it }.fold(ZERO) { acc, monomial -> acc + monomial }
    }

    operator fun div(other: Int): IntPolynomial {
        return if (other == 1) {
            this
        } else if (this.coefficientGcd() % other == 0) {
            IntPolynomial(terms.map { Term(it.coefficient / other, it.terms) })
        } else {
            throw DivisionException(this, other)
        }
    }

    open fun pow(exponent: UInt): IntPolynomial {
        var result = ONE
        var base = this
        while (exponent > 0u) {
            if (exponent % 2u == 1u) result *= base
            base *= base
        }
        return result
    }

    companion object {
        class DivisionException(lhs: IntPolynomial, rhs: Any) : Exception(
            "`$lhs` cannot be divided by `$rhs`"
        )

        val ZERO: IntPolynomial by lazy { IntPolynomial() }
        val ONE: IntPolynomial by lazy { IntPolynomial(Term(1)) }

        fun parse(string: String): IntPolynomial = PolynomialVisitor.parsePolynomial(string)
    }
}
