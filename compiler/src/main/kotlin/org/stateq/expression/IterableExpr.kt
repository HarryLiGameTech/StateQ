package org.stateq.expression

import org.stateq.parameter.ClassicalVariable
import org.stateq.type.ClassicalTrait
import org.stateq.type.ClassicalType
import org.stateq.util.Location

interface IterableExpr<out T: ClassicalTrait> {

    val iterableType: ClassicalType

    fun createIteratorVariable(ident: String, location: Location): ClassicalVariable {
        return iterableType.createVariable(ident, location)
    }
}
