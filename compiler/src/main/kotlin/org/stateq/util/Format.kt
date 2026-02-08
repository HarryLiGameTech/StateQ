package org.stateq.util

fun <E: Any> List<E>.format(
    separator: String = ", ",
    trim: Boolean = true,
    formatter: (E) -> String = { it.toString() }
): String {
    return this.fold("") { acc, e ->
        acc + separator + formatter(e)
    }.let {
        if (!trim) it else {
            it.substringBeforeLast(separator)
        }
    }
}
