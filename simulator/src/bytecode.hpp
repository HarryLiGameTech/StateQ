
#ifndef QIVMBE_SIMULATOR_BYTECODE_HPP
#define QIVMBE_SIMULATOR_BYTECODE_HPP

#include "instruction.hpp"

#include <functional>

using ByteVec = std::vector<uint8_t>;
using byte = std::uint8_t;

class ByteCode
{
  private:

    std::vector<Instruction> instructions;

  public:

    explicit ByteCode(const ByteVec& bytes);

    inline size_t size()
    {
        return instructions.size();
    }

    inline void forEach(
        const std::function<void(const PrimitiveInstruction&)> & primitiveInstrConsumer,
        const std::function<void(const StandardGateInstruction&)> & standardGateInstrConsumer
    ) const {
        for (const auto & instr: this->instructions) {
            if (auto primitiveInstr = std::get_if<PrimitiveInstruction>(&instr)) {
                primitiveInstrConsumer(*primitiveInstr);
            } else if (auto stdGateInstr = std::get_if<StandardGateInstruction>(&instr)) {
                standardGateInstrConsumer(*stdGateInstr);
            }
        }
    }
};


#endif // QIVMBE_SIMULATOR_BYTECODE_HPP
