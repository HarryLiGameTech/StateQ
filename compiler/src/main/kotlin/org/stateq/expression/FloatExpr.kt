package org.stateq.expression

import org.stateq.exception.unreachable
import org.stateq.module.classical.ClassicalFunction
import org.stateq.parameter.FloatVariable
import org.stateq.util.Location
import org.stateq.type.ClassicalType
import org.stateq.type.FloatTrait

private typealias Operator = FloatExprBinary.Operator

abstract class FloatExpr : NumericExprBase<FloatTrait> {

    override val type: ClassicalType = ClassicalType.Float

    override operator fun plus(other: NumericExpr): FloatExpr = when (other) {
        is FloatExpr -> FloatExprBinary(this, other, Operator.Add, location ?: other.location)
        is IntExpr -> FloatExprBinary(this, other.toFloatExpr(), Operator.Add, location ?: other.location)
        else -> unreachable()
    }

    override operator fun minus(other: NumericExpr): FloatExpr = when (other) {
        is FloatExpr -> FloatExprBinary(this, other, Operator.Sub, location ?: other.location)
        is IntExpr -> FloatExprBinary(this, other.toFloatExpr(), Operator.Sub, location ?: other.location)
        else -> unreachable()
    }

    override operator fun times(other: NumericExpr): FloatExpr = when (other) {
        is FloatExpr -> FloatExprBinary(this, other, Operator.Mul, location ?: other.location)
        is IntExpr -> FloatExprBinary(this, other.toFloatExpr(), Operator.Mul, location ?: other.location)
        else -> unreachable()
    }

    override operator fun div(other: NumericExpr): FloatExpr = when (other) {
        is FloatExpr -> FloatExprBinary(this, other, Operator.Div, location ?: other.location)
        is IntExpr -> FloatExprBinary(this, other.toFloatExpr(), Operator.Div, location ?: other.location)
        else -> unreachable()
    }

    operator fun unaryMinus() = FloatExprNegative(this, this.location)

    override infix fun pow(exponent: NumericExpr): FloatExpr = when (exponent) {
        is FloatExpr -> FloatExprBinary(this, exponent, Operator.Pow, location ?: exponent.location)
        is IntExpr -> FloatExprBinary(this, exponent.toFloatExpr(), Operator.Pow, location ?: exponent.location)
        else -> unreachable()
    }

    override infix fun pow(exponent: UInt) = FloatExprBinary(this, FloatExprLiteral(exponent.toDouble()), Operator.Pow, location)
}

class FloatExprLiteral(
    val value: Double, override val location: Location? = null
) : FloatExpr()

class FloatExprVariable(
    val variable: FloatVariable, override val location: Location? = null
) : FloatExpr()

class FloatExprBinary(
    val lhs: FloatExpr, val rhs: FloatExpr, val op: Operator,
    override val location: Location?,
) : FloatExpr() {
    enum class Operator {
        Add, Sub, Mul, Div, Pow,
    }
}

class FloatExprNegative(
    val expr: FloatExpr,
    override val location: Location?,
) : FloatExpr()

class FloatExprFromInt(
    val inner: IntExpr, override val location: Location?
) : FloatExpr()

fun IntExpr.toFloatExpr() = FloatExprFromInt(this, this.location)

class FloatExprFuncCall(
    val function: ClassicalFunction<FloatExpr>,
    val args: List<ClassicalExpr>,
    override val location: Location?,
) : FloatExpr()
