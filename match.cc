#include <re2/re2.h>

extern "C" {
  bool is_match(const char* pattern, const char* string) {
    return re2::RE2::FullMatch(string, re2::RE2(pattern));
  }
}
