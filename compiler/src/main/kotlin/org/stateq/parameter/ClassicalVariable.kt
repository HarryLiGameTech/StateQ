package org.stateq.parameter

import org.stateq.expression.*
import org.stateq.polynomial.Indeterminate
import org.stateq.type.*
import org.stateq.util.Location
import kotlin.reflect.KClass

typealias ClassicalVariable = ClassicalVariableBase<ClassicalTrait>

interface ClassicalVariableBase<out T: ClassicalTrait> : Variable {
    override val type: ClassicalType
    fun toClassicalExpr(location: Location): ClassicalExpr
}

inline val <reified K: ClassicalVariable> KClass<K>.typename: String get() {
    return this.simpleName!!.dropLast("Variable".length)
}

class IntVariable(
    ident: String,
    override val location: Location? = null
) : ClassicalVariableBase<IntTrait>, Indeterminate(ident) {
    override val type = ClassicalType.Int
    override fun toClassicalExpr(location: Location) = IntExpr(this, location)
    override val ident: String get() = name
    fun expr() = IntExpr(this)
}

class FloatVariable(
    override val ident: String,
    override val location: Location = Location.builtin
) : ClassicalVariableBase<FloatTrait> {
    override val type = ClassicalType.Float
    override fun toClassicalExpr(location: Location) = FloatExprVariable(this, location)
}

class ComplexVariable(
    override val ident: String,
    override val location: Location? = null
) : ClassicalVariableBase<ComplexTrait> {
    override val type = ClassicalType.Complex
    override fun toClassicalExpr(location: Location) = TODO()
}

class BoolVariable(
    override val ident: String,
    override val location: Location? = null
) : ClassicalVariableBase<BoolTrait> {
    override val type = ClassicalType.Bool
    override fun toClassicalExpr(location: Location) = BoolExprVariable(this, location)
}

class BitsVariable(
    override val ident: String,
    override val location: Location? = null
) : ClassicalVariableBase<BitsTrait> {
    override val type = ClassicalType.Bits
    override fun toClassicalExpr(location: Location) = BitsExprVariable(this, location)
}

class ListVariable(
    override val ident: String,
    elementType: ClassicalType,
    override val location: Location? = null
) : ClassicalVariableBase<ListTrait> {
    override val type = ClassicalListType(elementType)
    val elementType: ClassicalType get() = type.elementType
    override fun toClassicalExpr(location: Location) = ListExprVariable<ClassicalTrait>(this, elementType, location)
}
