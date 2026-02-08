package org.stateq.polynomial

import org.stateq.expression.ClassicalExpr
import org.stateq.expression.FloatExpr
import org.stateq.expression.IntExpr
import org.stateq.module.classical.ExternIntFunction
import org.stateq.util.Location
import org.stateq.util.OptionalLocatable

class IndeterminateLikeFromFloat(
    val floatExpr: FloatExpr
) : IndeterminateBase() {

    override val name = "float_${floatExpr.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeFromFloat &&
        other.floatExpr == this.floatExpr
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + floatExpr.hashCode()
        return result
    }
}

class IndeterminateLikeDivision(
    val lhs: IntExpr,
    val rhs: IntExpr,
) : IndeterminateBase() {

    override val name = "div_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeDivision &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

class IndeterminateLikeModulo(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "mod_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeModulo &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

class IndeterminateLikeAnd(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "and_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeAnd &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

infix fun IntExpr.and(other: IntExpr) = IntExpr(IndeterminateLikeAnd(this, other))

class IndeterminateLikeOr(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "or_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeOr &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

infix fun IntExpr.or(other: IntExpr) = IntExpr(IndeterminateLikeOr(this, other))

class IndeterminateLikeXor(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "xor_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeXor &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

class IndeterminateLikePower(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "pow_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikePower &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

infix fun IntExpr.pow(other: IntExpr) = IntExpr(IndeterminateLikePower(this, other))

infix fun IntExpr.xor(other: IntExpr) = IntExpr(IndeterminateLikeXor(this, other))

class IndeterminateLikeShiftLeft(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "shl_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeShiftLeft &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

infix fun IntExpr.shl(other: IntExpr) = IntExpr(IndeterminateLikeShiftLeft(this, other))

class IndeterminateLikeShiftRight(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "shr_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeShiftRight &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

infix fun IntExpr.shr(other: IntExpr) = IntExpr(IndeterminateLikeShiftRight(this, other))

class IndeterminateLikeLogicalShiftRight(val lhs: IntExpr, val rhs: IntExpr) : IndeterminateBase() {

    override val name = "lshr_${lhs.hashCode() * rhs.hashCode()}"

    override fun equals(other: Any?) = (
        other is IndeterminateLikeLogicalShiftRight &&
        lhs == other.lhs && rhs == other.rhs
    )

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + lhs.hashCode()
        result = 31 * result + rhs.hashCode()
        return result
    }
}

infix fun IntExpr.lshr(other: IntExpr) = IntExpr(IndeterminateLikeLogicalShiftRight(this, other))

class IndeterminateLikeFuncCall(
    val function: ExternIntFunction,
    val args: List<ClassicalExpr>,
    override val location: Location?
) : IndeterminateBase(), OptionalLocatable {
    override val name = "call_${this.hashCode()}"
}
