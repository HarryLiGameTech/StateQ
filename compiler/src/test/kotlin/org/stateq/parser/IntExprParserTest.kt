package org.stateq.parser

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import org.junit.jupiter.params.provider.ValueSource
import org.stateq.antlr.StateqParser.IntExprContext
import org.stateq.exception.CompileErrorException
import org.stateq.expression.IntExpr
import org.stateq.parameter.FloatVariable
import org.stateq.parameter.IntVariable
import org.stateq.parameter.QvarVariable
import org.stateq.util.assertNoCompileErrors
import org.stateq.visitor.StateqVisitor

class IntExprParserTest {

    @Throws(CompileErrorException::class)
    private fun parseIntExpr(code: String): IntExpr {
        return StateqParser(code).parse(::intExprParsingDelegate)
    }

    @Throws(CompileErrorException::class)
    private fun parseIntExprContext(code: String): IntExprContext {
        return StateqParser(code).parseToContextRule { parser ->
            parser.intExpr()
        }
    }

    private fun intExprParsingDelegate(parser: AntlrStateqParser, visitor: StateqVisitor): IntExpr {
        return visitor.visitIntExpr(parser.intExpr())
    }

    @ParameterizedTest
    @ValueSource(strings = [
        "0", "1", "-1",
        "(114+514)*1919",
        " \t (\t114 \n\n+\t514 \n)* 1919",
        "(114 + 514) * 1919",
        "((((((((1))))))))",
        "1 + 2 - 3 * 4 / 5 % 6 ^ 7 & 8 | 9",
        "((((((((1) + 2) - 3) * 4) / 5) % 6) ^ 7) & 8) | 9",
        "135792468",
        "asdfghj",
        "qwq",
        "id3ntW1thNum63rs",
        "thisIsAVeryLooooo0o0o0o0o0o0o0o0o0gIdentifier",
    ])
    fun `parse valid strings`(expr: String) {
        assertNoCompileErrors(expr) {
            parseIntExprContext(it)
        }
    }

    @ParameterizedTest
    @ValueSource(strings = [
        "this is an invalid IntExpr",
        "2f", "2.0", "2.5", "0.5", ".5",
        "PascalCaseIsInvalid",
        "snake_case_is_invalid",
        "UPPER_SNAKE_CASE_IS_ALSO_INVALID",
        "1dentifierCanNotStartWithNumber",
        "helloWorld!",
        "identifier cannot contains space",
        "中文不能做标识符",
        " ", "\n", "\t", "π",
        "!114514", "&23333",
        "(", ")", "()", "(((())))",
        "(1",
        "1+", "2*)",
        "*2", "/a",
        "((3 + 5)", "(6 * a))",
        "(2))",
        "(a))", "((((3)",
        "1 + 1 = 2",
        "!@#$%",
        "{}",
        "[[]]",
    ])
    fun `raise error on invalid strings`(code: String) {
        assertThrows<CompileErrorException> {
            parseIntExpr(code)
        }
    }

    @ParameterizedTest
    @CsvSource(value = [
        "1 + 1 = 2",
        "2 * 3 = 6",
        "4 - 2 = 2",
        "2 - 4 = -2",
        "-2 * (5 + 3) = -16",
        "1 * 1 = 1",
        "0 * 100 = 0",
        "(3 + 4) * 7 = 49",
        "2 * (5 + -3) = 4",
        "12 * (5 - 3) = 24",
        "(10 + 1) * -2 = -22",
        "5 * 3 + 2 = 17",
        "6 + 8 * 1 = 14",
        "1 + 2 * (11 + 2) = 27",
        "24 + 6 * (9 - 10) = 18",
        "88 + 1 * -1 = 87",
        "100 - 5 * -5 = 125",
        "8 + 8 * -2 = -8",
        "0 + 0 * -109 = 0",
        "22 * 2 + 3 * (4 - 5) = 41",
        "99 - 5 * -2 + 7 = 116",
        "13 + 14 * -1 + 6 * (-2 + 3) = 5",
        "6 * 6 * (6 - 5) = 36",
    ], delimiter = '=')
    fun `equality of two literal expressions (add, sub and mul only)`(
        lhs: String, rhs: String
    ) {
        assertEquals(parseIntExpr(lhs), parseIntExpr(rhs))
    }

    @ParameterizedTest
    @CsvSource(value = [
        "a: a",
        "a, b: a + b",
        "a, b: b * a",
        "a: a * 114",
        "a: a / 514",
        "a, b, c, d, e, f, g, h: a + b * c / d % e & f | g ^ h",
    ], delimiter = ':')
    fun `visiting valid expressions`(variables: String, expr: String) {
        assertNoCompileErrors(expr) { code ->
            StateqParser(code).also { parser ->
                parser.enterMockScope()
                variables.split(",").map(String::trim).forEach {
                    parser.putClassicalVariable(IntVariable(it))
                }
            }.run {
                this.parse(::intExprParsingDelegate)
            }
        }
    }

    @Test
    fun `different type cannot be add together`() {
        assertThrows<CompileErrorException> {
            StateqParser("a * b + 2").also { parser ->
                parser.enterMockScope()
                parser.putClassicalVariable(IntVariable("a"))
                parser.putClassicalVariable(FloatVariable("b"))
            }.run {
                this.parse(::intExprParsingDelegate)
            }
        }
    }

    @Test
    fun `quantum variables cannot appear in IntExpr`() {
        assertThrows<CompileErrorException> {
            StateqParser("a * b + 2").also { parser ->
                parser.enterMockScope()
                parser.putClassicalVariable(IntVariable("a"))
                parser.putQuantumVariable(QvarVariable("b", IntExpr(1)))
            }.run {
                this.parse(::intExprParsingDelegate)
            }
        }
    }
}
