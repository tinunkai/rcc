#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./target/release/rcc "$input" > tmp.s
  riscv-none-elf-gcc -march=rv64gc -mabi=lp64 -o tmp.elf tmp.s
  qemu-riscv64 ./tmp.elf
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 21 '5+20-4'
assert 41 ' 12 + 34 - 5 '

echo OK
