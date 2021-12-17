
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <queue>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_split.h"

class BitStream {
 public:
  BitStream() {}

  static absl::StatusOr<BitStream> FromHex(absl::string_view hex) {
    BitStream stream;
    RETURN_IF_ERROR(stream.PushHex(hex));
    return stream;
  }

 public:
  BitStream(const BitStream&) = delete;
  BitStream& operator=(const BitStream&) = delete;
  BitStream(BitStream&&) = default;
  BitStream& operator=(BitStream&&) = default;

  void Print(std::ostream& out) const {
    for (bool b : bits_) {
      out << (b ? "1" : "0");
    }
    out << std::endl;
  }

  absl::StatusOr<bool> Read() {
    if (bits_.empty()) {
      return absl::InternalError("unexpected end-of-stream");
    }

    bool b = bits_.front();
    bits_.pop_front();
    return b;
  }

  void Write(bool bit) { bits_.push_back(bit); }

  bool Empty() { return bits_.empty(); }

 private:
  absl::Status PushHex(absl::string_view hex) {
    for (char c : hex) {
      RETURN_IF_ERROR(PushHexNibble(c));
    }
    return absl::OkStatus();
  }

  absl::Status PushHexNibble(char hex) {
    if (hex >= '0' && hex <= '9') {
      return PushNibble(hex - '0');
    }
    if (hex >= 'A' && hex <= 'Z') {
      return PushNibble((hex - 'A') + 10);
    }
    if (hex >= 'a' && hex <= 'z') {
      return PushNibble((hex - 'a') + 10);
    }
    return absl::InternalError(absl::StrFormat("invalid nibble char: %c", hex));
  }

  absl::Status PushNibble(uint8_t nibble) {
    if (nibble > 15) {
      return absl::InternalError(absl::StrCat("invalid nibble: ", nibble));
    }
    bits_.push_back((nibble & 0x08) != 0);
    bits_.push_back((nibble & 0x04) != 0);
    bits_.push_back((nibble & 0x02) != 0);
    bits_.push_back((nibble & 0x01) != 0);
    return absl::OkStatus();
  }

 private:
  std::deque<bool> bits_;
};

std::ostream& operator<<(std::ostream& out, const BitStream& bits) {
  bits.Print(out);
  return out;
}

class Packet {
 public:
  static absl::StatusOr<Packet> FromHex(absl::string_view hex) {
    Packet p;
    RETURN_IF_ERROR(p.ParseHex(hex));
    return p;
  }

 public:
  Packet() : version_(-1), type_(-1), value_(0) {}

  absl::Status ParseHex(absl::string_view hex) {
    ASSIGN_OR_RETURN(auto bits, BitStream::FromHex(hex));
    return ParseBits(&bits);
  }

  absl::Status ParseBits(BitStream* bits) {
    // std::cout << "Parsing " << *bits << std::endl;

    ASSIGN_OR_RETURN(version_, ParseInt<uint8_t>(bits, 3));
    ASSIGN_OR_RETURN(type_, ParseInt<uint8_t>(bits, 3));

    if (type_ == 4) {
      ASSIGN_OR_RETURN(value_, ParseVarInt(bits));
      return absl::OkStatus();
    }

    ASSIGN_OR_RETURN(bool length_is_packets, bits->Read());
    if (!length_is_packets) {
      ASSIGN_OR_RETURN(uint32_t subpacket_bits, ParseInt<uint32_t>(bits, 15));
      // std::cout << "Reading the next " << subpacket_bits
      //           << " bits as subpackets" << std::endl;
      BitStream copy;
      for (int i = 0; i < static_cast<int>(subpacket_bits); i++) {
        ASSIGN_OR_RETURN(bool bit, bits->Read());
        copy.Write(bit);
      }
      // std::cout << "Got " << copy << std::endl;
      while (!copy.Empty()) {
        Packet p;
        RETURN_IF_ERROR(p.ParseBits(&copy));
        subpackets_.emplace_back(std::move(p));
      }
      return absl::OkStatus();
    }

    ASSIGN_OR_RETURN(uint32_t subpacket_count, ParseInt<uint32_t>(bits, 11));
    for (int i = 0; i < static_cast<int>(subpacket_count); i++) {
      Packet p;
      RETURN_IF_ERROR(p.ParseBits(bits));
      subpackets_.emplace_back(std::move(p));
    }

    // std::cout << "Done parsing: " << *this << std::endl;

    return absl::OkStatus();
  }

  void Print(std::ostream& out, int depth) const {
    Indent indent(depth);
    out << "{\n"
        << indent << "  version: " << static_cast<int>(version_) << "\n"
        << indent << "  type: " << static_cast<int>(type_) << "\n"
        << indent << "  value: " << static_cast<unsigned long>(value_) << "\n"
        << indent << "  subpackets: [";
    for (const Packet& packet : subpackets_) {
      packet.Print(out, depth + 1);
      if (&packet != &(subpackets_.back())) {
        out << ", ";
      }
    }
    out << "]\n" << indent << "}";
  }

  int SumVersions() const {
    int version = version_;
    for (const auto& packet : subpackets_) {
      version += packet.SumVersions();
    }
    return version;
  }

  template <typename T>
  absl::StatusOr<T> Compute() const {
    std::vector<T> subresults;
    for (const Packet& packet : subpackets_) {
      ASSIGN_OR_RETURN(T val, packet.Compute<T>());
      subresults.emplace_back(std::move(val));
    }

    switch (type_) {
      case 0: {
        T sum = 0;
        for (auto sub : subresults) {
          sum += sub;
        }
        return sum;
      }

      case 1: {
        T product = 1;
        for (auto sub : subresults) {
          product *= sub;
        }
        return product;
      }

      case 2: {
        if (subpackets_.size() < 1) {
          return absl::InternalError("not enough args to min");
        }
        T min = subresults[0];
        for (auto sub : subresults) {
          if (sub < min) {
            min = sub;
          }
        }
        return min;
      }

      case 3: {
        if (subpackets_.size() < 1) {
          return absl::InternalError("not enough args to max");
        }
        T max = subresults[0];
        for (auto sub : subresults) {
          if (sub > max) {
            max = sub;
          }
        }
        return max;
      }

      case 4: {
        return static_cast<T>(value_);
      }

      case 5: {
        if (subpackets_.size() != 2) {
          return absl::InternalError("incorrect args to greater than");
        }
        return (subresults[0] > subresults[1]) ? 1 : 0;
      }

      case 6: {
        if (subpackets_.size() != 2) {
          return absl::InternalError("incorrect args to less than");
        }
        return (subresults[0] < subresults[1]) ? 1 : 0;
      }

      case 7: {
        if (subpackets_.size() != 2) {
          return absl::InternalError("incorrect args to equal to");
        }
        return (subresults[0] == subresults[1]) ? 1 : 0;
      }
    }
  }

 private:
  template <typename T>
  absl::StatusOr<T> ParseInt(BitStream* bits, int count) {
    T result = 0;
    for (int i = 0; i < count; i++) {
      ASSIGN_OR_RETURN(bool new_bit, bits->Read());
      result <<= 1;
      result |= (new_bit ? 1 : 0);
    }
    return result;
  }

  absl::StatusOr<uint64_t> ParseVarInt(BitStream* bits) {
    uint64_t result = 0;
    int nibbles = 0;
    bool more = true;
    while (more) {
      if (nibbles >= 16) {
        return absl::InternalError("var int overflow");
      }
      ASSIGN_OR_RETURN(more, bits->Read());
      ASSIGN_OR_RETURN(uint8_t nibble, ParseInt<uint8_t>(bits, 4));
      result <<= 4;
      result |= nibble;
      nibbles++;
    }
    return result;
  }

 private:
  uint8_t version_;
  uint8_t type_;
  uint64_t value_;
  std::vector<Packet> subpackets_;

  friend std::ostream& operator<<(std::ostream& os, const Packet& packet);
};

std::ostream& operator<<(std::ostream& out, const Packet& packet) {
  packet.Print(out, 0);
  return out;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/16/input.txt"));

  auto line = ReadLine(in);
  if (!line || *line == "") {
    return absl::InternalError("empty file");
  }

  ASSIGN_OR_RETURN(auto packet, Packet::FromHex(*line));

  std::cout << packet << std::endl;
  std::cout << "sum versions: " << packet.SumVersions() << std::endl;

  ASSIGN_OR_RETURN(auto compute, packet.Compute<int64_t>());
  std::cout << "compute: " << compute << std::endl;

  return absl::OkStatus();
}
