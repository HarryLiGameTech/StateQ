package org.stateq.expression

import org.stateq.parameter.BitsVariable
import org.stateq.type.BitsTrait
import org.stateq.util.Locatable
import org.stateq.util.Location
import org.stateq.type.ClassicalType
import org.stateq.type.IntTrait

sealed class BitsExpr : Locatable, ClassicalExprBase<BitsTrait>, IterableExpr<IntTrait> {
    override val type = ClassicalType.Bits
    override val iterableType = ClassicalType.Int
    // TODO
}

class BitsExprVariable(
    val variable: BitsVariable, override val location: Location
) : BitsExpr()

class BitsExprBinary(
    val operator: Operator,
    val lhs: BitsExpr,
    val rhs: BitsExpr,
    override val location: Location,
) : BitsExpr() {
    enum class Operator {
        And, Or, Xor, Concat,
    }
}
