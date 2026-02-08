package org.stateq.util

inline fun <reified T> Any.cast(): T? = this as? T

fun <T> List<T>.slice(start: Int, end: Int? = null, step: UInt = 1u, inclusive: Boolean): List<T> {
    val indexStart = if (start < 0) this.size + start else start
    val indexEnd = if (end == null) this.size else if (end < 0) this.size + end else end
    val indexStep = step.toInt()
    return if (inclusive) {
        this.slice(indexStart .. indexEnd step indexStep)
    } else {
        this.slice(indexStart until indexEnd step indexStep)
    }
}
