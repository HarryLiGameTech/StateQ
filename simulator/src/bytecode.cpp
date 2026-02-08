#include "bytecode.hpp"
#include "exception.hpp"
#include "logger.hpp"
#include "utils.hpp"

using std::string;

enum struct InstructionType : std::uint8_t
{
    Nop, Primitive, Standard,
};

using ByteIter = ByteVec::const_iterator;

template <typename T>
inline T next(ByteIter& iterator, const ByteIter& end)
{
    if constexpr (sizeof(T) == 1) {
        if (iterator == end) {
            throw BytecodeParseException("Unexpected end of bytecode");
        }
        return (T) *(iterator++);
    } else {
        T data {};
        for (size_t i = 0; i < sizeof(T); i++) {
            if (iterator == end) {
                throw BytecodeParseException("Unexpected end of bytecode");
            }
            ((uint8_t*) &data)[i] = *(iterator++);
        }
        return data;
    }
}

ByteCode::ByteCode(const ByteVec& bytes)
{
    auto iter = bytes.begin();

    while (iter != bytes.end()) {
        auto iterBeginInstr = iter;
        switch (auto instrType = next<InstructionType>(iter, bytes.end())) {
            case InstructionType::Nop:
                break;
            case InstructionType::Primitive: {
                // Only `Alloc`, `Reset` and `Measure` primitive instructions
                //  are supported in the simulator backend
                auto opcode = next<PrimitiveOpCode>(iter, bytes.end());
                if ((int) opcode > 2) {
                    throw BytecodeParseException(
                        "Invalid primitive opcode: " + std::to_string((int) opcode)
                    );
                }
                std::vector<InstructionParam> params;
                for (size_t size = next<byte>(iter, bytes.end()); size > 0; size--) {
                    params.emplace_back(next<InstructionParam>(iter, bytes.end()));
                }
                auto instruction = PrimitiveInstruction(opcode, params);
                instructions.emplace_back(instruction);
                if constexpr (LOGLEVEL >= LogLevel::Debug) {
                    string hexString = bytesToHexString(iterBeginInstr, iter, "", " ", "");
                    logger::debug(padding(hexString, 40), instruction.toString());
                }
                break;
            }
            case InstructionType::Standard: {
                auto gate = next<StandardGate>(iter, bytes.end());
                auto numParams = next<uint8_t>(iter, bytes.end());
                std::vector<InstructionParam> params(numParams);
                for (int i = 0; i < numParams; i++) {
                    params[i] = next<InstructionParam>(iter, bytes.end());
                }
                auto numTargetQubits = next<uint8_t>(iter, bytes.end());
                std::vector<QubitAddr> target(numTargetQubits);
                for (int i = 0; i < numTargetQubits; i++) {
                    target[i] = next<QubitAddr>(iter, bytes.end());
                }
                auto instruction = StandardGateInstruction(gate, params, target);
                instructions.emplace_back(instruction);
                if constexpr (LOGLEVEL >= LogLevel::Debug) {
                    string hexString = bytesToHexString(iterBeginInstr, iter, "", " ", "");
                    logger::debug(padding(hexString, 40), instruction.toString());
                }
                break;
            }
            default: {
                throw BytecodeParseException(
                    "Invalid instruction type: " + std::to_string((int) instrType)
                );
            }
        }
    }
}
