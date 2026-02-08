
#ifndef QIVMBE_SIMULATOR_LOGGER_HPP
#define QIVMBE_SIMULATOR_LOGGER_HPP

#include <string>
#include <iostream>
#include <chrono>
#include <iomanip>

enum class LogLevel : uint8_t
{
    None    =   0,
    Error   =   1,
    Warning =   2,
    Info    =   3,
    Debug   =   4,
};

#ifndef LOGLEVEL
  #define LOGLEVEL LogLevel::Warning
#endif

namespace logger
{
    using std::string;

    namespace _io
    {
        using namespace std::chrono;
        using SysClock = std::chrono::system_clock;
        using StrStream = std::stringstream;

        static inline void printErrLn(StrStream & stream)
        {
            std::cerr << stream.str() << '\n';
        }

        template<typename T, typename ... Ts>
        static void printErrLn(StrStream & stream, const T & first, const Ts &... tail)
        {
            stream << first;
            printErrLn(stream, tail...);
        }

        #define COLOR_DEF(color, code)  static const char* color = "\033[0;" #code "m"

            COLOR_DEF(RED,      31);
            COLOR_DEF(GREEN,    32);
            COLOR_DEF(YELLOW,   33);
            COLOR_DEF(BLUE,     34);
            COLOR_DEF(CYAN,     36);

        #undef COLOR_DEF

        static inline string colorString(const string & s, const char* color)
        {
            return string(color) + s + "\033[0m";
        }

        static inline string timeString()
        {
            StrStream timeFormat;
            char buffer[64];
            auto now = SysClock::now();
            long millisecond = duration_cast<milliseconds>(now.time_since_epoch()).count() % 1000;
            timeFormat << "%H:%M:%S." << std::setw(3) << std::setfill('0') << millisecond;
            time_t timeNow = SysClock::to_time_t(now);
            std::strftime(buffer, sizeof(buffer), timeFormat.str().c_str(), std::localtime(&timeNow));
            return buffer;
        }

        template<typename ... Ts>
        static inline void log(const string & label, const char* color, const Ts &... args)
        {
            StrStream stream;
            stream << _io::colorString("[", _io::BLUE);
            stream << _io::colorString(label, color);
        #ifdef _GLIBCXX_THREAD
            stream << "T" << std::this_thread::get_id() << " ";
        #endif
            stream << _io::timeString() << " ";
            stream << _io::colorString("]", _io::BLUE) << " ";
            _io::printErrLn(stream, args...);
        }
    }

    template<typename ... Ts>
    static inline void debug(const Ts &... args)
    {
        if constexpr (LOGLEVEL >= LogLevel::Debug) {
            logger::_io::log(" DEBUG ", _io::CYAN, args...);
        }
    }

    template<typename ... Ts>
    static inline void info(const Ts &... args)
    {
        if constexpr (LOGLEVEL >= LogLevel::Info) {
            logger::_io::log(" INFO  ", _io::GREEN, args...);
        }
    }

    template<typename ... Ts>
    static inline void warning(const Ts &... args)
    {
        if constexpr (LOGLEVEL >= LogLevel::Warning) {
            logger::_io::log(" WARN  ", _io::YELLOW, args...);
        }
    }

    template<typename ... Ts>
    static inline void error(const Ts &... args)
    {
        if constexpr (LOGLEVEL >= LogLevel::Error) {
            logger::_io::log(" ERROR ", _io::RED, args...);
        }
    }
}

#endif //QIVMBE_SIMULATOR_LOGGER_HPP
