fn main() {
  println(fmt!("%u", problem1(3, 5, 999)));
}

fn problem1(x: uint, y: uint, until: uint) -> uint {
  let mcm = x * y;
  let one_to_n = |n| { ((1 + n) * n) / 2 };
  y * one_to_n(until / y) + x * one_to_n(until / x) - mcm * one_to_n(until / mcm)
}
