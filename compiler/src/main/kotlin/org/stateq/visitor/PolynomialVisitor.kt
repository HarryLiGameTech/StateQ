package org.stateq.visitor

import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.stateq.parser.CommonErrorListener
import org.stateq.antlr.PolynomialBaseVisitor
import org.stateq.antlr.PolynomialLexer
import org.stateq.antlr.PolynomialParser
import org.stateq.antlr.PolynomialParser.*
import org.stateq.exception.unreachable
import org.stateq.polynomial.Indeterminate
import org.stateq.polynomial.IntMonomial
import org.stateq.polynomial.IntPolynomial
import java.lang.RuntimeException

class PolynomialVisitor : PolynomialBaseVisitor<Any>() {
    override fun visitMonomialTerm(ctx: MonomialTermContext) = Pair(
        ctx.indeterminate.text, ctx.exponent?.text?.toUInt() ?: 1u
    )

    override fun visitConstant(ctx: ConstantContext) = IntMonomial(
        ctx.value.text.toInt()
    )

    override fun visitMonomialWithCoeff(ctx: MonomialWithCoeffContext) = IntMonomial(
        ctx.coeff.text.toInt(),
        ctx.terms.associate(::visitMonomialTerm).map {
            Pair(Indeterminate(it.key), it.value)
        }.toMap()
    )

    override fun visitMonomialWithNoCoeff(ctx: MonomialWithNoCoeffContext) = IntMonomial(
        if (ctx.negative != null) -1 else 1,
        ctx.terms.associate(::visitMonomialTerm).map {
            Pair(Indeterminate(it.key), it.value)
        }.toMap()
    )

    private fun visitMonomial(ctx: MonomialContext) = when (ctx) {
        is ConstantContext -> visitConstant(ctx)
        is MonomialWithCoeffContext -> visitMonomialWithCoeff(ctx)
        is MonomialWithNoCoeffContext -> visitMonomialWithNoCoeff(ctx)
        else -> unreachable()
    }

    override fun visitPolynomial(ctx: PolynomialContext) = IntPolynomial(
        ctx.terms.map(::visitMonomial).toSet()
    )

    companion object {

        private fun parseString(string: String): Pair<PolynomialParser, CommonErrorListener> {
            val lexer = PolynomialLexer(CharStreams.fromString(string)).apply {
                this.removeErrorListeners()
            }
            val errorListener = CommonErrorListener()
            val parser = PolynomialParser(CommonTokenStream(lexer)).apply {
                this.removeErrorListeners()
                this.addErrorListener(errorListener)
            }
            return Pair(parser, errorListener)
        }

        fun parseMonomial(string: String): IntMonomial {
            val (parser, errorListener) = parseString(string)
            val ctx = parser.monomial().also {
                errorListener.errorRecords.forEach {
                    throw RuntimeException("`$string` is not a valid polynomial", it.cause)
                }
            }
            return PolynomialVisitor().visitMonomial(ctx)
        }

        fun parsePolynomial(string: String): IntPolynomial {
            val (parser, errorListener) = parseString(string)
            val ctx = parser.polynomial().also {
                errorListener.errorRecords.forEach { err -> err.cause?.let { throw it } }
            }
            return PolynomialVisitor().visitPolynomial(ctx)
        }
    }
}
