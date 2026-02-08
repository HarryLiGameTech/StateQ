package org.stateq.intermediate.decomposable

import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import org.junit.jupiter.params.provider.ValueSource
import org.stateq.exception.SizeNotMatchException
import org.stateq.expression.*
import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.intermediate.Instruction
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.intermediate.decomposed.DecomposedForLoop
import org.stateq.intermediate.decomposible.OperationCall
import org.stateq.intermediate.decomposed.ElementaryOperationCall
import org.stateq.intermediate.decomposed.QubitAccessorDeclaration
import org.stateq.parameter.IntVariable
import org.stateq.qubit.QubitAccessor
import org.stateq.qubit.QubitAccessorAlloc
import org.stateq.qubit.QubitAccessorIndexing
import org.stateq.qubit.QubitAccessorSlicing
import org.stateq.type.ClassicalTrait
import org.stateq.type.ClassicalType
import org.stateq.type.IntTrait
import org.stateq.util.Location
import org.stateq.util.assertAs
import kotlin.test.Test
import kotlin.test.assertEquals

class OperationCallTest {

    companion object {
        private val stubLoc = Location(null, 0, 0)
    }

    class OperationExprStub(
        override val ident: String,
        override val size: IntExpr = IntExpr(1),
        override val classicalArgs: List<ClassicalExpr> = listOf(),
    ) : OperationExprElementary() {
        override val isSizeDetermined: Boolean = true
        override val location: Location = stubLoc
    }

    private fun qubits(size: Int = 1) = QubitAccessorAlloc(IntExpr(size))

    private val stubOpPool = mutableMapOf<String, OperationExprStub>()

    private fun op(ident: Any, size: IntExpr) = stubOpPool.getOrDefault(
        ident.toString(), OperationExprStub(ident.toString(), size)
    )

    private fun op(ident: Any, size: Int = 1) = this.op(ident, IntExpr(size))

    private fun List<OperationExpr>.seq() = OperationExprSequentialMatMul(this, stubLoc)

    private fun List<OperationExpr>.comb() = OperationExprCombined(this, stubLoc)

    private fun OperationExpr.extend(size: IntExpr) = OperationExprExtended(this, size, stubLoc)

    private fun OperationExpr.extend(size: Int) = OperationExprExtended(this, IntExpr(size), stubLoc)

    private infix fun OperationExprElementary.operate(target: QubitAccessor) = ElementaryOperationCall(this, target, stubLoc)

    private infix fun String.on(target: QubitAccessor) = OperationCall(op(this), target, stubLoc)

    private infix fun OperationExpr.on(target: QubitAccessor) = OperationCall(this, target, stubLoc)

    private fun seq(vararg ops: OperationExpr) = ops.toList().seq()

    private fun comb(vararg ops: OperationExpr) = ops.toList().comb()

    private fun Instruction.toList() = listOf(this)

    private fun DecomposedInstruction.toList() = listOf(this)

    private fun DecomposedBasicBlock.forLoop(
        start: IntExpr, end: IntExpr, step: IntExpr = IntExpr(1), inclusive: Boolean = true
    ): DecomposedForLoop<IntTrait> {
        return DecomposedForLoop(
            ClassicalType.Int.createVariable("", stubLoc) as IntVariable,
            IntListGenerator(start, end, inclusive, step, stubLoc),
            this, stubLoc
        )
    }

    private fun assertDecomposedOperationEquals(
        actual: DecomposableInstruction, expected: () -> List<DecomposedInstruction>
    ) {
        this.assertDecomposedOperationEquals(expected(), actual)
    }

    private fun assertDecomposedOperationEquals(
        expected: List<DecomposedInstruction>, actual: DecomposableInstruction
    ) {
        val decomposed = actual.decompose().instructions
        assertEquals(expected.size, decomposed.size)
        expected.zip(decomposed).forEach { (expected, actual) ->
            when (expected) {
                is ElementaryOperationCall -> {
                    assert(actual is ElementaryOperationCall)
                    actual as ElementaryOperationCall
                    assertEquals(expected.op.ident, actual.op.ident)
                }
                is QubitAccessorDeclaration -> {
                    assert(actual is QubitAccessorDeclaration)
                    actual as QubitAccessorDeclaration
                    assertEquals(expected.ident, actual.ident)
                    assertEquals(expected::class, actual::class)
                }
                is DecomposedForLoop<*> -> {
                    assert(actual is DecomposedForLoop<*>)
                    actual as DecomposedForLoop<*>
                    assertEquals(expected.iterableExpr, actual.iterableExpr)
                    assertEquals(expected.loopBody, actual.loopBody)
                }
                else -> {
                    assertEquals(expected, actual)
                }
            }
        }
    }

    @Nested
    inner class SequentialOperationTest {

        @Test
        fun `decompose sequential operation`() {
            val opList = ('A' .. 'H').map { op(it) }
            val target = qubits()
            assertDecomposedOperationEquals(
                expected = opList.map { ElementaryOperationCall(it, target, stubLoc) },
                actual = opList.seq() on target
            )
        }

        @Test
        fun `decompose sequential operation recursively`() {
            val target = qubits()
            val seqOperation = seq(
                seq(op("E"), op("B"), op("A")),
                seq(op("D"), seq(op("C"), op("A"), op("F"))),
                op("B"), op("E"), op("F")
            )
            assertDecomposedOperationEquals(
                expected = "EBADCAFBEF".toList().map { op(it) operate target },
                actual = seqOperation on target
            )
        }

        @ParameterizedTest
        @CsvSource("1, 1, 1, 2", "1, 2, 2, 2", "2, 2, 2, 1")
        fun `size not match when decomposing sequential operation`(
            op1Size: Int, op2Size: Int, op3Size: Int, targetSize: Int
        ) {
            assertThrows<SizeNotMatchException> {
                seq(op("A", op1Size), op("B", op2Size), op("C", op3Size)) on qubits(targetSize)
            }
        }
    }

    @Nested
    inner class CombinedOperationTest {

        @Test
        fun `decompose combined single-target operations`() {
            val opList = ('A' .. 'H').map { op(it) }
            val targets = qubits(opList.size)
            assertDecomposedOperationEquals(opList.comb() on targets) {
                opList.mapIndexed { i, op ->
                    listOf(targets[i].declare(), op operate targets[i])
                }.fold(listOf()) { acc, ins -> acc + ins }
            }
        }

        @ParameterizedTest
        @ValueSource(ints = [2, 5, 200, 100000, 19491001])
        fun `decompose combined multi-target operations`(size: Int) {
            val opList = ('A' .. 'H').map { op(it, size) }
            val targets = qubits(opList.size * size)
            assertDecomposedOperationEquals(opList.comb() on targets) {
                opList.mapIndexed { i, op ->
                    val target = targets.slice(IntExpr(i*size), IntExpr((i+1)*size))
                    listOf(target.declare()!!, op operate target)
                }.flatten()
            }
        }

        fun `decompose different size combined operations`() {

        }
    }

    @Nested
    inner class ExtendOperationTest {

        @Test
        fun `decompose simple extend operation`() {
            val targets = qubits(2)
            val opCall = op('H').extend(2) on targets
            val decomposed = opCall.decompose()
            assertEquals(1, decomposed.instructions.size)
            val forLoop = decomposed.instructions[0].assertAs<DecomposedForLoop<*>>()
            val decl = forLoop.loopBody.instructions[0].assertAs<QubitAccessorDeclaration>()
            assert(decl.qubitAccessor is QubitAccessorIndexing)
            val operationInstr = forLoop.loopBody.instructions[1].assertAs<ElementaryOperationCall>()
            val indexing = operationInstr.target.assertAs<QubitAccessorIndexing>()
            assert(indexing.subject == targets)
            assert(indexing.size == IntExpr(1))
        }

    }

}
