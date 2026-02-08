package org.stateq.exception

import org.stateq.expression.IntExpr
import org.stateq.util.CompileError
import org.stateq.util.Location
import org.stateq.util.Sized
import java.lang.RuntimeException

class UnreachableException(message: String? = null) : IllegalStateException(message)

fun unreachable(message: String? = null): Nothing {
    throw UnreachableException(message)
}

class InvalidCompileOptionException @JvmOverloads constructor(
    key: String, value: String, helpMessage: String? = null
) : RuntimeException(
    "Compile option {$key: $value} is invalid.".let {
        helpMessage?.let { help -> "$it\nhelp: $help" } ?: it
    }
)

class CompileErrorException(val errors: List<CompileError>) : Exception() {
    override fun toString(): String {
        return errors.fold("StateqCompileError [\n") {
            acc, err -> "$acc\t$err,\n"
        } + "]"
    }
}

abstract class InternalCompileException : Exception() {
    abstract val location: Location?
}

class SizeNotMatchException(
    val sized: Sized, val expectedSize: IntExpr,
    override val location: Location,
) : InternalCompileException()

class TypeNotMatchException(
    val item: Any, val expectedType: String,
    override val location: Location,
) : InternalCompileException()
