package org.stateq.parser

import org.antlr.v4.runtime.BaseErrorListener
import org.antlr.v4.runtime.RecognitionException
import org.antlr.v4.runtime.Recognizer

class CommonErrorListener : BaseErrorListener() {
    val errorRecords: MutableList<ErrorRecord> = ArrayList()
    override fun syntaxError(
        recognizer: Recognizer<*, *>, offendingSymbol: Any?, line: Int,
        column: Int, message: String, cause: RecognitionException?
    ) = errorRecords.add(
        ErrorRecord(recognizer, offendingSymbol, line, column, message, cause)
    ).let { /* Unit() */ }
}
