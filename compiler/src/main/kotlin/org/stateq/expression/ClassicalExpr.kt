package org.stateq.expression

import org.stateq.exception.unreachable
import org.stateq.type.ClassicalTrait
import org.stateq.type.ClassicalType
import org.stateq.type.NumericTrait
import org.stateq.util.OptionalLocatable

typealias ClassicalExpr = ClassicalExprBase<ClassicalTrait>

interface ClassicalExprBase<out T: ClassicalTrait> : OptionalLocatable {
    val type: ClassicalType
}

typealias NumericExpr = NumericExprBase<NumericTrait>

interface NumericExprBase<out T: NumericTrait> : BoolComparableExpr<T> {
    operator fun plus(other: NumericExpr): NumericExpr = when (this) {
        is IntExpr -> this as IntExpr + other
        is FloatExpr -> this as FloatExpr + other
        else -> unreachable()
    }

    operator fun minus(other: NumericExpr): NumericExpr = when (this) {
        is IntExpr -> this as IntExpr - other
        is FloatExpr -> this as FloatExpr - other
        else -> unreachable()
    }

    operator fun times(other: NumericExpr): NumericExpr = when (this) {
        is IntExpr -> this as IntExpr * other
        is FloatExpr -> this as FloatExpr * other
        else -> unreachable()
    }

    operator fun div(other: NumericExpr): NumericExpr = when (this) {
        is IntExpr -> this as IntExpr / other
        is FloatExpr -> this as FloatExpr / other
        else -> unreachable()
    }

    infix fun pow(exponent: UInt): NumericExpr = when (this) {
        is IntExpr -> this as IntExpr pow exponent
        is FloatExpr -> this as FloatExpr pow exponent
        else -> unreachable()
    }

    infix fun pow(exponent: NumericExpr): NumericExpr = when (this) {
        is IntExpr -> this as IntExpr pow exponent
        is FloatExpr -> this as FloatExpr pow exponent
        else -> unreachable()
    }
}
