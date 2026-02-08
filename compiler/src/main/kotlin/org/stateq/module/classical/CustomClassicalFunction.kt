package org.stateq.module.classical

import org.stateq.expression.ClassicalExpr
import org.stateq.parameter.ClassicalVariable

abstract class CustomClassicalFunction<E: ClassicalExpr>(
    ident: String,
    val params: List<ClassicalVariable>
) : ClassicalFunction<E>(ident) {
    override val paramTypes get() = params.map { it.type }
}
