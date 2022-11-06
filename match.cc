#include <re2/re2.h>

extern "C" {
  bool is_match(const char* pattern, const char* string) {
    re2::StringPiece regex(pattern);
    re2::StringPiece input(string);
    return re2::RE2::FullMatch(input, regex);
  }
}
