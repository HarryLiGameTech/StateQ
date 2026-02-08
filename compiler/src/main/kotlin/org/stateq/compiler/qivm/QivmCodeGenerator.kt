package org.stateq.compiler.qivm

import org.stateq.compiler.CodeGenerator
import org.stateq.exception.unreachable
import org.stateq.expression.IntExpr
import org.stateq.parameter.ClassicalVariable
import org.stateq.parameter.QuantumVariable
import org.stateq.parameter.Variable

abstract class QivmCodeGenerator : CodeGenerator() {
    protected val ctx: String get() = "ctx"
    abstract fun getProgramCtx()
    abstract fun destroyProgramCtx()
    abstract fun executeProgram(shots: IntExpr)
    abstract fun destroyProgramAndReturnResult()
    abstract fun pauseCtrl()
    abstract fun restoreCtrl()
    abstract fun enterStackFrame()
    abstract fun exitStackFrame()

    override val Variable.typename get() = when (this) {
        is ClassicalVariable -> this.type.ident
        is QuantumVariable -> "QubitAccessor"
        is ProgramContextVariable -> "ProgramContext"
        else -> unreachable()
    }
}
