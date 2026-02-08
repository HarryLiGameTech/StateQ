package org.stateq.expression

import org.stateq.parameter.ListVariable
import org.stateq.type.*
import org.stateq.util.CompileError
import org.stateq.util.Location
import org.stateq.util.OptionalLocatable

abstract class ListExpr<out T: ClassicalTrait> : ClassicalExprBase<ListTrait>, IterableExpr<T> {
    final override val type by lazy { ClassicalListType(iterableType) }
}

class ListExprVariable<T: ClassicalTrait>(
    val variable: ListVariable,
    override val iterableType: ClassicalType,
    override val location: Location?
) : ListExpr<T>()

class ListExprLiteral<T: ClassicalTrait>(
    val elements: List<ClassicalExprBase<T>>,
    override val location: Location,
) : ListExpr<T>() {
    override val iterableType: ClassicalType get() = elements[0].type

    init {
        CompileError.error(location,
            "A list literal must have at least one element, " +
            "otherwise the compiler is unable to determine the type of the list"
        ).raise()
    }
}

class ListExprEmpty<T: ClassicalTrait>(
    override val iterableType: ClassicalType,
    override val location: Location?
) : ListExpr<T>()

class ListExprSlicing<T: ClassicalTrait>(
    val inner: ListExpr<T>,
    start: IntExpr?, val end: IntExpr?, step: IntExpr?,
    val inclusive: Boolean,
    override val location: Location?,
) : ListExpr<T>() {
    override val iterableType: ClassicalType get() = inner.iterableType

    val start = start ?: IntExpr(0)
    val step = step ?: IntExpr(1)
}

class IntListGenerator(
    val start: IntExpr,
    val end: IntExpr,
    val inclusive: Boolean = true,
    val step: IntExpr = IntExpr(1),
    override val location: Location?,
) : OptionalLocatable, IterableExpr<IntTrait>, ListExpr<IntTrait>() {

    override val iterableType = ClassicalType.Int

    constructor(end: IntExpr, location: Location? = null) : this(
        IntExpr(0), end, false, IntExpr(1), location ?: end.location
    )

    infix fun step(newStep: IntExpr) = IntListGenerator(
        start, end, inclusive, newStep, location
    )

    infix fun step(newStep: UInt) = IntListGenerator(
        start, end, inclusive, IntExpr(newStep.toInt()), location
    )
}

