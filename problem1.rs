fn main() {
  let x = 5;
  let y = 3;
  let until = 999;
  let mcm = x * y;
  let one_to_n = |n| { ((1 + n) * n) / 2 };

  let x_count = until / x;
  let y_count = until / y;
  let mcm_count = until / mcm;

  let sum = y * one_to_n(y_count) + x * one_to_n(x_count) - mcm * one_to_n(mcm_count);
  println(fmt!("%d", sum));
}
