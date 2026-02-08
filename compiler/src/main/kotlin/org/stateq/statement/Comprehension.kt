package org.stateq.statement

import org.stateq.expression.BitsExpr
import org.stateq.expression.IterableExpr
import org.stateq.expression.ListExpr
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.parameter.IntVariable
import org.stateq.type.ClassicalTrait
import org.stateq.type.IntTrait
import org.stateq.util.Locatable
import org.stateq.util.Location

abstract class Comprehension<out T: ClassicalTrait> : Locatable {
    abstract val iterator: ClassicalVariableBase<T>
    abstract val iterable: IterableExpr<T>
}

class ListComprehension<out T: ClassicalTrait>(
    override val iterator: ClassicalVariableBase<T>,
    override val iterable: ListExpr<T>,
    override val location: Location,
) : Comprehension<T>()

class BitsComprehension(
    override val iterator: IntVariable,
    override val iterable: BitsExpr,
    override val location: Location,
) : Comprehension<IntTrait>()

// class IntArrayComprehension
// class FloatArrayComprehension
