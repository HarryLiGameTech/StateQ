package org.stateq.expression

import org.stateq.module.classical.ClassicalFunction
import org.stateq.util.Location
import org.stateq.util.OptionalLocatable
import org.stateq.type.ClassicalType
import org.stateq.type.ComplexTrait

abstract class ComplexExpr : OptionalLocatable, ClassicalExprBase<ComplexTrait> {
    override val type: ClassicalType = ClassicalType.Complex

    operator fun plus(other: ComplexExpr) = ComplexExprBinary(this, other, ComplexExprBinary.Operator.Add, location ?: other.location)
    operator fun minus(other: ComplexExpr) = ComplexExprBinary(this, other, ComplexExprBinary.Operator.Sub, location ?: other.location)
    operator fun times(other: ComplexExpr) = ComplexExprBinary(this, other, ComplexExprBinary.Operator.Mul, location ?: other.location)
    operator fun div(other: ComplexExpr) = ComplexExprBinary(this, other, ComplexExprBinary.Operator.Div, location ?: other.location)
    operator fun unaryMinus() = ComplexExprNegative(this, this.location)
    infix fun pow(other: ComplexExpr) = ComplexExprBinary(this, other, ComplexExprBinary.Operator.Pow, location ?: other.location)
}

class ComplexExprLiteral(
    val real: FloatExpr, val imaginary: FloatExpr,
    override val location: Location? = null,
) : ComplexExpr() {

    infix fun equalTo(other: ComplexExprLiteral): BoolExpr = (
        (this.real equalTo other.real) and (this.imaginary equalTo other.imaginary)
    )

    // (a+bi)+(c+di) = (a+c)+(b+d)i
    operator fun plus(other: ComplexExprLiteral) = ComplexExprLiteral(
        real + other.real,
        imaginary + other.imaginary
    )

    // (a+bi)−(c+di) = (a−c)+(b−d)i
    operator fun minus(other: ComplexExprLiteral) = ComplexExprLiteral(
        real - other.real,
        imaginary - other.imaginary
    )

    // (a+bi)∗(c+di) = (ac−bd)+(ad+bc)i
    operator fun times(other: ComplexExprLiteral) = ComplexExprLiteral(
        real * other.real - imaginary * other.imaginary,
        real * other.imaginary + imaginary * other.real
    )

    // (a+bi)/(c+di) = (ac+bd)/(c^2+d^2) + (bc−ad)/(c^2+d^2)i
    operator fun div(other: ComplexExprLiteral) = ComplexExprLiteral(
        (real * other.real + imaginary * other.imaginary) / (other.real.pow(2u) + other.imaginary.pow(2u)),
        (imaginary * other.real - real * other.imaginary) / (other.real.pow(2u) + other.imaginary.pow(2u))
    )
}

class ComplexExprBinary(
    val lhs: ComplexExpr, val rhs: ComplexExpr,
    val op: Operator,
    override val location: Location? = null,
) : ComplexExpr() {
    enum class Operator {
        Add, Sub, Mul, Div, Pow,
    }
}

class ComplexExprNegative(
    val expr: ComplexExpr,
    override val location: Location? = null
) : ComplexExpr()

class ComplexExprFuncCall(
    val func: ClassicalFunction<ComplexExpr>,
    val args: List<ClassicalExpr>,
    override val location: Location? = null
) : ComplexExpr()
