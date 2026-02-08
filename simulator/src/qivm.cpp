#include "qivm-backend.h"
#include "instruction.hpp"
#include "bytecode.hpp"
#include "exception.hpp"
#include "logger.hpp"
#include "utils.hpp"

#include <cstring>
#include <bitset>
#include <map>
#include <random>

template <typename T, typename ... E>
auto array_of(E&& ... elements) -> std::array<T, sizeof...(E)> {
    return std::array<T, sizeof...(E)>{std::forward<E>(elements)... };
}

const auto GATES = array_of<const char*>(
    "I", "H", "X", "Y", "Z", "XPOW", "YPOW", "ZPOW", "S", "SD", "T", "TD", "V", "VD", "P", "RX", "RY", "RZ", "RN", "U",
    "CX", "CY", "CZ", "CH", "CP", "SWP", "SSWP", "SSWPD", "ISWP", "ISWPD", "SISWP", "SISWPD", "CAN", "CCX", "CSWP"
);

std::map<uint64_t, double> executeOnce(QuESTEnv & env, const ByteCode & bytecode)
{
    Qureg qubits {};
    bool isInitialized = false;
    uint64_t measureQubits = 0;
    std::map<uint64_t, double> probs;

    auto primitiveInstrExecutor = [&](const PrimitiveInstruction& instruction)
    {
        switch (instruction.opcode) {
            case PrimitiveOpCode::Alloc:
                qubits = createQureg((int) instruction.params[0].uint64, env);
                initZeroState(qubits);
                isInitialized = true;
                logger::info("Allocate " + std::to_string(instruction.params[0].uint64) + " qubits");
                break;
            case PrimitiveOpCode::Reset:
                for (const InstructionParam & qubit : instruction.params) {
                    if (measure(qubits, (int) qubit.uint64) == 1) {
                        pauliX(qubits, (int) qubit.uint64);
                    }
                    logger::debug("Reset qubit " + std::to_string(qubit.uint64));
                }
                break;
            case PrimitiveOpCode::Measure:
                if (!isInitialized) {
                    throw QivmBackendException("Qubits are not initialized");
                }
                for (const InstructionParam & qubit : instruction.params) {
                    measureQubits |= (1 << qubit.uint64);
                }
                for (int state = 0; state < (int) pow(2, qubits.numQubitsRepresented); state++) {
                    double prob = getProbAmp(qubits, state);
                    if (prob > 1e-10) {
                        if (!probs.contains(state & measureQubits)) {
                            probs[state & measureQubits] = prob;
                        } else {
                            probs[state & measureQubits] += prob;
                        }
                    }
                }
                break;
            default:
                throw QivmBackendException("Unknown primitive instruction");
        }
    };

    auto standardGateInstrExecutor = [&](const StandardGateInstruction& instruction)
    {
        if (!isInitialized) {
            throw QivmBackendException("Qubits are not initialized");
        }
        logger::debug("Executing instruction " + instruction.toString());
        instruction.execute(&qubits);
    };

    bytecode.forEach(primitiveInstrExecutor, standardGateInstrExecutor);

    destroyQureg(qubits, env);

    return probs;
}

extern "C" uint32_t qivm_available_qubits() {
    return 24;
}

extern "C" bool qivm_is_gate_available(const char* ident) {
    return std::any_of(GATES.begin(), GATES.end(), [&](const char* gate) {
        return std::strcmp(ident, gate) == 0;
    });
}

extern "C" ExecuteResult qivm_exec_bytecode(
    const uint8_t* rawBytecode, uint32_t bytecodeLength, uint32_t shots
) {
    logger::info("Initializing QuEST environment");
    std::mt19937 rng(std::chrono::system_clock::now().time_since_epoch().count());
    QuESTEnv env = createQuESTEnv();
    ExecuteResult result;
    result.measurement.measurements = nullptr;
    result.error = 0;

    std::map<uint64_t, uint64_t> measurements;

    logger::info(
        "Executing bytecode of length " + std::to_string(bytecodeLength) +
        " with " + std::to_string(shots) + " shots"
    );

    ByteVec bytes = ByteVec(rawBytecode, rawBytecode + bytecodeLength);

    // Print the bytecode in hex
    if constexpr (LOGLEVEL >= LogLevel::Debug) {
        logger::debug("Bytecode: " + bytesToHexString(bytes.begin(), bytes.end(), " ", " ", "\n  "));
    }

    try {
        ByteCode bytecode = ByteCode(bytes);
        try {
            // Execute the bytecode
            auto probs = executeOnce(env, bytecode);
            std::vector<uint64_t> states;
            for (auto [state, prob] : probs) {
                for (int i = 0; i < round(prob * (1 << 16)); i++) {
                    states.push_back(state);
                }
            }

            std::shuffle(states.begin(), states.end(), rng);

            for (int i = 0; i < shots; i++) {
                int idx = rng() % (1 << 16);
                while (idx >= states.size()) {
                    idx = rng() % (1 << 16);
                }
                uint64_t measurement = states[idx];
                if (measurements.count(measurement) == 0) {
                    measurements[measurement] = 1;
                } else {
                    measurements[measurement]++;
                }
            }

            // Collect the measurements
            result.measurement.shots = shots;
            result.measurement.result_size = measurements.size();
            result.measurement.measurements = new MeasurementResultEntry[measurements.size()];
            for (int i = 0; auto & [key, value] : measurements) {
                result.measurement.measurements[i++] = MeasurementResultEntry { .value = key, .count = value };
            }
        } catch (QivmBackendException& exception) {
            logger::error(exception.message);
            destroyQuESTEnv(env);
            return ExecuteResult { .error = 1, .measurement = { 0, 0, nullptr } };
        } catch (std::exception& exception) {
            logger::error("Unknown error: " + std::string(exception.what()));
            destroyQuESTEnv(env);
            return ExecuteResult { .error = 255, .measurement = { 0, 0, nullptr } };
        }
    } catch (BytecodeParseException& exception) {
        logger::error("Bytecode parse error: " + exception.message);
        destroyQuESTEnv(env);
        return ExecuteResult { .error = 2, .measurement = { 0, 0, nullptr } };
    }

    // Log the measurements
    if constexpr (LOGLEVEL >= LogLevel::Info) {
        std::stringstream measurementsStream;
        measurementsStream << "Measurements: {\n";
        for (auto & [value, count] : measurements) {
            measurementsStream << "    " << std::bitset<16>(value) << " : " << count << ",\n";
        }
        logger::info(measurementsStream.str() + "}");
    }

    destroyQuESTEnv(env);
    return result;
}
