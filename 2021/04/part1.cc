
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_split.h"

class Board {
 public:
  Board() {
    for (int i = 0; i < 5; i++) {
      std::vector<int> line;
      std::vector<bool> called;
      for (int j = 0; j < 5; j++) {
        line.push_back(0);
        called.push_back(false);
      }
      board_.push_back(line);
      called_.push_back(called);
    }
  }

  ~Board() = default;

  Board(const Board&) = delete;

  Board(Board&& other) {
    board_ = std::move(other.board_);
    called_ = std::move(other.called_);
  }

  Board& operator=(Board&& other) {
    if (this != &other) {
      board_ = std::move(other.board_);
      called_ = std::move(other.called_);
    }
    return *this;
  }

  void Set(int row, int column, int value) { board_[row][column] = value; }
  int Get(int row, int column) { return board_[row][column]; }

  std::string ToString() {
    std::string output;
    for (int i = 0; i < 5; i++) {
      for (int j = 0; j < 5; j++) {
        absl::StrAppend(&output, board_[i][j], " ");
      }
      absl::StrAppend(&output, "\n");
    }
    return output;
  }

  bool Mark(int n) {
    for (int i = 0; i < 5; i++) {
      for (int j = 0; j < 5; j++) {
        if (board_[i][j] == n) {
          called_[i][j] = true;
        }
      }
    }
    return Check();
  }

  int SumUnmarked() {
    int n = 0;
    for (int i = 0; i < 5; i++) {
      for (int j = 0; j < 5; j++) {
        if (!called_[i][j]) {
          n += board_[i][j];
        }
      }
    }
    return n;
  }

  bool Check() {
    for (int i = 0; i < 5; i++) {
      bool row_called = true;
      bool column_called = true;
      for (int j = 0; j < 5; j++) {
        if (!called_[i][j]) {
          row_called = false;
        }
        if (!called_[j][i]) {
          column_called = false;
        }
      }
      if (row_called || column_called) {
        return true;
      }
    }
    return false;
  }

 private:
  std::vector<std::vector<int>> board_;
  std::vector<std::vector<bool>> called_;
};

absl::StatusOr<absl::optional<Board>> ReadBoard(std::ifstream& in) {
  Board board;

  auto line = ReadLine(in);
  // Skip any blank lines.
  while (line == "") {
    line = ReadLine(in);
  }
  // Handle the end of the file.
  if (!line) {
    return absl::nullopt;
  }
  for (int i = 0; i < 5; i++) {
    // std::cout << "board line: " << *line << std::endl;
    std::vector<absl::string_view> parts =
        absl::StrSplit(*line, " ", absl::SkipEmpty());
    if (parts.size() != 5) {
      return absl::InternalError(
          absl::StrCat("line does not have 5 parts: ", *line));
    }

    for (int j = 0; j < static_cast<int>(parts.size()); j++) {
      int n = 0;
      if (!absl::SimpleAtoi(parts[j], &n)) {
        return absl::InternalError(absl::StrCat("invalid number: ", parts[j]));
      }
      // std::cout << "number[" << i << "," << j << "] = " << n << std::endl;
      board.Set(i, j, n);
    }

    line = ReadLine(in);
    if (!line) {
      return absl::InternalError("unexpected EOF");
    }
  }

  return board;
}

absl::StatusOr<std::vector<Board>> ReadBoards(std::ifstream& in) {
  std::vector<Board> boards;
  ASSIGN_OR_RETURN(auto board, ReadBoard(in));
  while (board) {
    std::cout << "board: \n" << board->ToString() << std::endl;
    boards.emplace_back(*std::move(board));
    ASSIGN_OR_RETURN(board, ReadBoard(in));
  }
  return boards;
}

absl::StatusOr<std::vector<int>> ReadNumbers(std::ifstream& in) {
  auto line1 = ReadLine(in);
  if (!line1) {
    return absl::InvalidArgumentError("expected list of numbers");
  }

  std::vector<absl::string_view> parts = absl::StrSplit(*line1, ",");
  std::vector<int> numbers;
  for (auto& part : parts) {
    int n;
    if (!absl::SimpleAtoi(part, &n)) {
      return absl::InternalError(absl::StrCat("invalid number: ", part));
    }
    numbers.push_back(n);
  }
  return numbers;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/04/input.txt"));
  ASSIGN_OR_RETURN(auto numbers, ReadNumbers(in));
  ASSIGN_OR_RETURN(auto boards, ReadBoards(in));

  for (auto number : numbers) {
    for (auto& board : boards) {
      if (board.Mark(number)) {
        std::cout << "winner: " << board.ToString() << std::endl;
        std::cout << "answer: " << board.SumUnmarked() * number << std::endl;
        return absl::OkStatus();
      }
    }
  }

  return absl::InternalError("no winner");
}
