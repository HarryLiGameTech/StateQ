package org.stateq.expression

import org.stateq.parameter.BoolVariable
import org.stateq.type.BoolTrait
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location
import org.stateq.util.OptionalLocatable
import org.stateq.type.ClassicalType

abstract class BoolExpr : OptionalLocatable, ClassicalExprBase<BoolTrait> {
    override val type: ClassicalType = ClassicalType.Bool
    infix fun and(other: BoolExpr) = BoolExprBinary(this, other, BoolExprBinary.Operator.And, location)
    infix fun or(other: BoolExpr) = BoolExprBinary(this, other, BoolExprBinary.Operator.Or, location)
    operator fun not() = BoolExprNot(this)
}

abstract class BoolExprLiteral : BoolExpr()

object BoolExprLiteralTrue : BoolExprLiteral() {
    override val location: Location? = null
}

object BoolExprLiteralFalse : BoolExprLiteral() {
    override val location: Location? = null
}

class BoolExprVariable(
    val variable: BoolVariable,
    location: Location? = null,
) : BoolExpr() {
    override val location: Location? = location ?: variable.location
}


class BoolExprBinary(
    val lhs: BoolExpr, val rhs: BoolExpr, val op: Operator,
    location: Location? = null,
) : BoolExpr() {
    override val location: Location? = location ?: lhs.location ?: rhs.location
    enum class Operator { And, Or }
}

class BoolExprNot(
    val inner: BoolExpr,
    location: Location? = null,
) : BoolExpr() {
    override val location: Location? = location ?: inner.location
}

enum class CompareOperator {
    Equal, NotEqual, Greater, GreaterEqual, Less, LessEqual
}

interface BoolComparableExpr<out T: ClassicalTrait> : ClassicalExprBase<T>

class BoolExprCompare<T: ClassicalTrait>(
    val lhs: BoolComparableExpr<T>, val rhs: BoolComparableExpr<T>,
    val op: CompareOperator,
    override val location: Location? = null,
) : BoolExpr()

infix fun <T> BoolComparableExpr<T>.equalTo(other: BoolComparableExpr<T>)
    where T: ClassicalTrait = BoolExprCompare(this, other, CompareOperator.Equal)

infix fun <T> BoolComparableExpr<T>.notEqualTo(other: BoolComparableExpr<T>)
    where T: ClassicalTrait = BoolExprCompare(this, other, CompareOperator.NotEqual)

infix fun <T> BoolComparableExpr<T>.lessThan(other: BoolComparableExpr<T>)
    where T: ClassicalTrait = BoolExprCompare(this, other, CompareOperator.Less)

infix fun <T> BoolComparableExpr<T>.greaterThan(other: BoolComparableExpr<T>)
    where T: ClassicalTrait = BoolExprCompare(this, other, CompareOperator.Greater)

infix fun <T> BoolComparableExpr<T>.lessEqualTo(other: BoolComparableExpr<T>)
    where T: ClassicalTrait = (this lessThan other) or (this equalTo other)

infix fun <T> BoolComparableExpr<T>.greaterEqualTo(other: BoolComparableExpr<T>)
    where T: ClassicalTrait = (this greaterThan other) or (this equalTo other)
