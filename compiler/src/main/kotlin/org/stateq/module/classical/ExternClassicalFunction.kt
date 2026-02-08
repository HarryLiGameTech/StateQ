package org.stateq.module.classical

import org.stateq.expression.*
import org.stateq.polynomial.IndeterminateLikeFuncCall
import org.stateq.util.Location
import org.stateq.type.ClassicalType

abstract class ExternClassicalFunction<E: ClassicalExpr>(
    ident: String,
    override val paramTypes: List<ClassicalType>,
) : ClassicalFunction<E>(ident)

class ExternIntFunction(
    ident: String,
    paramTypes: List<ClassicalType>,
    override val location: Location,
) : ExternClassicalFunction<IntExpr>(ident, paramTypes) {
    override fun callInner(args: List<ClassicalExpr>, location: Location): IntExpr {
        return IntExpr(IndeterminateLikeFuncCall(this, args, location))
    }
}

class ExternFloatFunction(
    ident: String,
    paramTypes: List<ClassicalType>,
    override val location: Location,
) : ExternClassicalFunction<FloatExpr>(ident, paramTypes) {
    override fun callInner(args: List<ClassicalExpr>, location: Location): FloatExpr {
        return FloatExprFuncCall(this, args, location)
    }
}

class ExternComplexFunction(
    ident: String,
    paramTypes: List<ClassicalType>,
    override val location: Location,
) : ExternClassicalFunction<ComplexExpr>(ident, paramTypes) {
    override fun callInner(args: List<ClassicalExpr>, location: Location): ComplexExpr {
        return ComplexExprFuncCall(this, args, location)
    }
}
