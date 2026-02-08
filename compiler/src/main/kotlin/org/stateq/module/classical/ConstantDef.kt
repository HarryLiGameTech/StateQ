package org.stateq.module.classical

import org.stateq.expression.*
import org.stateq.module.Definition
import org.stateq.parameter.BoolVariable
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.parameter.FloatVariable
import org.stateq.parameter.IntVariable
import org.stateq.type.BoolTrait
import org.stateq.type.ClassicalTrait
import org.stateq.type.FloatTrait
import org.stateq.type.IntTrait
import org.stateq.util.Location

typealias ConstantDef = ConstantDefBase<ClassicalTrait>

abstract class ConstantDefBase<out T: ClassicalTrait> : Definition {
    abstract val variable: ClassicalVariableBase<T>
    abstract val value: ClassicalExprBase<T>
}

class ConstantDefBool(
    ident: String,
    override val value: BoolExpr,
    override val location: Location,
) : ConstantDefBase<BoolTrait>() {
    override val variable = BoolVariable(ident, location)
}

class ConstantDefInt(
    ident: String,
    override val value: IntExpr,
    override val location: Location,
) : ConstantDefBase<IntTrait>() {
    override val variable = IntVariable(ident, location)
}

class ConstantDefFloat(
    ident: String,
    override val value: FloatExpr,
    override val location: Location,
) : ConstantDefBase<FloatTrait>() {
    override val variable = FloatVariable(ident, location)
}
