
#ifndef SIMULATOR_QIVMBE_UTILS_HPP
#define SIMULATOR_QIVMBE_UTILS_HPP

#include <cstdint>
#include <vector>
#include <string>
#include <sstream>
#include <iomanip>

inline std::string bytesToHexString(
    std::vector<uint8_t>::const_iterator begin,
    std::vector<uint8_t>::const_iterator end,
    const std::string & endOfByte  = " ",
    const std::string & endOfBlock = " ",
    const std::string & endOfChunk = "\n"
) {
    std::stringstream hexStream;
    for (size_t i = 0; begin != end; begin++, i++) {
        if (i % 4 == 0) {
            hexStream << endOfBlock;
            if (i % 8 == 0) hexStream << endOfChunk;
        }
        hexStream << std::hex << std::setw(2) << std::setfill('0');
        hexStream << (int) *begin << endOfByte;
    }
    return hexStream.str();
}

inline std::string padding(
    const std::string & str, int size, char paddingChar = ' '
) {
    std::stringstream paddingStream;
    paddingStream << std::setfill(paddingChar);
    paddingStream << std::left << std::setw(size) << str;
    return paddingStream.str();
}

#endif // SIMULATOR_QIVMBE_UTILS_HPP
