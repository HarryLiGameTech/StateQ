package org.stateq.parser

import org.antlr.v4.runtime.RecognitionException
import org.antlr.v4.runtime.Recognizer
import org.stateq.util.CompileError
import java.nio.file.Path

data class ErrorRecord(
    val recognizer: Recognizer<*, *>,
    val offendingSymbol: Any?,
    val line: Int,
    val column: Int,
    val message: String,
    val cause: RecognitionException?
) {
    fun except(source: Path?): Nothing {
        CompileError.error(source, message, line, column).raise()
    }

    fun toCompileError(source: Path?): CompileError {
        return CompileError.error(source, message, line, column)
    }
}
