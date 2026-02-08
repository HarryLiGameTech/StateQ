
#ifndef QIVMBE_SIMULATOR_EXCEPTION_HPP
#define QIVMBE_SIMULATOR_EXCEPTION_HPP

#include <exception>
#include <utility>
#include <string>

class QivmBackendException : std::exception
{
  public:

    explicit QivmBackendException(
        std::string message
    ) : message(std::move(message)) {}

    const std::string message;

    [[nodiscard]]
    const char* what() const noexcept override
    {
        return message.c_str();
    }
};

class BytecodeParseException : public QivmBackendException
{
  public:
    explicit BytecodeParseException(
        const std::string & message
    ) : QivmBackendException(message) {}

};

class QuantumProgramExecuteException : public QivmBackendException
{
  public:
    explicit QuantumProgramExecuteException(
        const std::string & message
    ) : QivmBackendException(message) {}

};

class MissingOrExtraParameterException : public QivmBackendException
{
  public:
    explicit MissingOrExtraParameterException(
        const std::string& gateIdent,
        size_t expectedNumParams, size_t actualNumParams
    ) : QivmBackendException(
        "Gate " + gateIdent + " expected " + std::to_string(expectedNumParams) +
        " parameters, got " + std::to_string(actualNumParams) + " parameters"
    ) {}
};

class TargetSizeNotMatchException : public QivmBackendException
{
  public:
    explicit TargetSizeNotMatchException(
        const std::string& gateIdent,
        size_t expectedTargetSize, size_t actualTargetSize
    ) : QivmBackendException(
        "The target size of gate " + gateIdent + " is " + std::to_string(expectedTargetSize) +
        ", got " + std::to_string(actualTargetSize)
    ) {}
};

class UnsupportedGateException : public QivmBackendException
{
  public:
    explicit UnsupportedGateException(const std::string& gateIdent) : QivmBackendException(
        "Unsupported gate " + gateIdent
    ) {}

};

#endif // QIVMBE_SIMULATOR_EXCEPTION_HPP
