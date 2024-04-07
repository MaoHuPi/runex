let a = new Integer({ digits: [Radix10Integer.fromDefaultNumber(1)] });
let b = new Integer({ digits: [Radix10Integer.fromDefaultNumber(1)] });
// console.log(Integer.add(a, b));
console.log(a.asRadix(Radix10Integer.fromDefaultNumber(2)))