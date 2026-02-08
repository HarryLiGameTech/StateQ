package org.stateq.util

import org.stateq.exception.CompileErrorException
import java.nio.file.Path

enum class CompileErrorType {
    ERROR, WARNING, NOTE;
}

data class CompileError(
    val type: CompileErrorType,
    val path: Path?,
    val line: Int,
    val column: Int,
    val message: String,
) {

    fun toList() = listOf(this)

    fun raise(): Nothing = throw CompileErrorException(this.toList())

    override fun toString(): String {
        return "${this.path?.toString() ?: ""}:$line:$column: $message"
    }

    companion object {
        fun error(source: Path?, message: String, line: Int = 0, column: Int = 0) = CompileError(
            CompileErrorType.ERROR, source, line, column, message
        )

        fun error(location: Location, message: String) = CompileError(
            CompileErrorType.ERROR, location.source, location.line, location.column, message
        )

        fun warning(source: Path?, message: String, line: Int = 0, column: Int = 0) = CompileError(
            CompileErrorType.WARNING, source, line, column, message
        )

        fun warning(location: Location, message: String) = CompileError(
            CompileErrorType.WARNING, location.source, location.line, location.column, message
        )

        fun note(source: Path?, message: String, line: Int = 0, column: Int = 0) = CompileError(
            CompileErrorType.NOTE, source, line, column, message
        )

        fun note(location: Location, message: String) = CompileError(
            CompileErrorType.NOTE, location.source, location.line, location.column, message
        )
    }
}

fun raiseCompileError(location: Location, message: () -> String): Nothing {
    CompileError.error(location, message()).raise()
}

fun raiseCompileErrorIf(condition: Boolean, location: Location, message: () -> String) {
    if (condition) {
        CompileError.error(location, message()).raise()
    }
}

