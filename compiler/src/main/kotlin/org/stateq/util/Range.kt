package org.stateq.util

import org.stateq.expression.IntExpr

data class Range(
    val start: IntExpr, val end: IntExpr?,
    val step: IntExpr, val inclusive: Boolean,
)
