package org.stateq.parser

import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.ParserRuleContext
import org.stateq.exception.CompileErrorException
import org.stateq.module.Module
import org.stateq.visitor.StateqVisitor
import java.nio.file.Path

typealias AntlrStateqLexer = org.stateq.antlr.StateqLexer
typealias AntlrStateqParser = org.stateq.antlr.StateqParser

class StateqParser(
    code: String,
    private val errorListener: CommonErrorListener = CommonErrorListener(),
    private val source: Path? = null
) {
    private val parser: AntlrStateqParser = code.lexer().let {
        it.apply {
            this.removeErrorListeners()
            this.addErrorListener(errorListener)
        }.let { lexer ->
            AntlrStateqParser(CommonTokenStream(lexer)).apply {
                this.removeErrorListeners()
                this.addErrorListener(errorListener)
            }
        }
    }

    private val visitor = StateqVisitor(source)

    private fun String.lexer() = AntlrStateqLexer(CharStreams.fromString(this))

    private fun getErrors(): List<ErrorRecord> = this.errorListener.errorRecords

    private fun throwExceptionIfGotError() {
        if (this.getErrors().isNotEmpty()) {
            throw CompileErrorException(this.getErrors().map {
                it.toCompileError(this.source)
            })
        }
    }

    @Throws(CompileErrorException::class)
    fun <C: ParserRuleContext> parseToContextRule(parsing: (AntlrStateqParser) -> C): C {
        return parsing(this.parser).also {
            this.throwExceptionIfGotError()
        }
    }

    @Throws(CompileErrorException::class)
    fun <T: Any> parse(parsing: (AntlrStateqParser, StateqVisitor) -> T): T {
        return parsing(this.parser, this.visitor).also {
            this.throwExceptionIfGotError()
        }
    }

    @Throws(CompileErrorException::class)
    fun <C: ParserRuleContext> parse(parsing: (AntlrStateqParser) -> C): Any? {
        val ctx = parsing(this.parser).also { this.throwExceptionIfGotError() }
        return visitor.visit(ctx).also { this.throwExceptionIfGotError() }
    }

    @Throws(CompileErrorException::class)
    fun parseModule(): Module {
        return this.parse { parser, visitor ->
            val ctx = parser.module()
            this.throwExceptionIfGotError()
            visitor.visitModule(ctx)
        }
    }
}
