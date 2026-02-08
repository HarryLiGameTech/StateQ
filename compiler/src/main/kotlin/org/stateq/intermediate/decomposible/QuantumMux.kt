package org.stateq.intermediate.decomposible

import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.Instruction
import org.stateq.intermediate.decomposed.BeginControl
import org.stateq.intermediate.decomposed.EndControl
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.qubit.QubitAccessor
import org.stateq.util.Location

class QuantumMux(
    val ctrlAccessors: List<QubitAccessor>, val blocks: List<BasicBlock>,
    override val location: Location
) : DecomposableInstruction() {

    init {
        require(1 shl ctrlAccessors.size == blocks.size) {
            "Invalid basic block list size"
        }
    }

    override val decomposed: DecomposedBasicBlock by lazy {
        // TODO: optimize for empty code blocks
        blocks.indices.drop(1).fold(blocks[0].instructions) { instructions, state ->
            (0 until (state xor (state - 1)).countOneBits()).map {
                // for each bit that need to be flipped from 0 to 1 or from 1 to 0
                listOf<Instruction>(
                    EndControl(ctrlAccessors[it], location),
                    // `it`: all bits that changed from last state to current state.
                    // `state and (1 shl it)`: if the bit is flipped from 0 to 1, returns 1 else 0.
                    BeginControl(ctrlAccessors[it], state and (1 shl it) == 1, location),
                )
            }.flatten().let { endBeginCtrl ->
                if (blocks[state].instructions.isEmpty()) listOf() else {
                    // concat previous instructions and add code block instructions
                    instructions + endBeginCtrl + blocks[state].instructions
                }
            }
        }.let { instructions ->
            // append begin control initial state (all false) to the front
            ctrlAccessors.map { BeginControl(it, false, location) } + instructions
        }.let { instructions ->
            // append end control all qubits to the back
            instructions + ctrlAccessors.map { EndControl(it, location) }
        }.run {
            return@lazy BasicBlock(this).decompose()
        }
    }
}