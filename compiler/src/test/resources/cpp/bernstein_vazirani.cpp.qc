#include <iostream>
#include <cstddef>
#include <stateq/runtime.hpp>

using stateq::Bits;

@stateq {
    operation BvOracle[bits: Bits] ($psi: ?n + 1) {
        i <- bits | qif &psi[i] then X $psi[n];
    }

    #name(bernsteinVazirani) // rename the function name to pascal case
    program BernsteinVazirani[bits: Bits | n = len(bits)] {
        Measure@n.I H@n.I BvOracle[bits] H@n.(Z H) |n+1'0⟩;
    }
}

const size_t BIT_STRING_LEN = 16;

int main() {
    Bits bitString;
    for (size_t i = 0; i < BIT_STRING_LEN; ++i) {
        bitString << rand() % 2;
    }
    auto result = bernsteinVazirani(bitString);
    std::cout << "result: " << result.getBitsByMaxProbability();
}
