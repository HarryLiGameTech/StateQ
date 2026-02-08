package org.stateq.intermediate

import org.stateq.compiler.CodeGenerator
import org.stateq.expression.IntExpr
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.parameter.ClassicalVariable
import org.stateq.parameter.QuantumVariable
import org.stateq.parameter.Variable
import org.stateq.type.ReturnType

interface IntermediateFunctionBase {
    val returnType: ReturnType?
    val ident: String
    val params: List<Variable>
    fun emit(codegen: CodeGenerator)
}

abstract class IntermediateFunction : IntermediateFunctionBase {
    protected abstract val doExport: Boolean
    abstract val body: DecomposedBasicBlock?
}

class IntermediateOperation(
    override val ident: String,
    override val doExport: Boolean,
    val classicalParams: List<ClassicalVariable>,
    val quantumParams: List<QuantumVariable>,
    override val body: DecomposedBasicBlock?,
) : IntermediateFunction() {

    override val returnType: ReturnType? = null

    override val params: List<Variable> get() = classicalParams + quantumParams

    override fun emit(codegen: CodeGenerator) {
        codegen.defOperation(ident, doExport, classicalParams, quantumParams) {
            body?.emit(codegen)
        }
    }
}

class IntermediateProgram(
    override val ident: String,
    override val params: List<ClassicalVariable>,
    val shots: IntExpr,
    override val body: DecomposedBasicBlock?,
) : IntermediateFunction() {
    override val returnType: ReturnType? = null
    override val doExport: Boolean = true

    override fun emit(codegen: CodeGenerator) {
        codegen.defProgram(ident, params, shots) {
            body?.emit(codegen)
        }
    }
}
