const add = (x: number, y: number): number => x + y;
const multiply = (x: number, y: number): number => x * y;

const flockA = 4;
const flockB = 2;
const flockC = 0;

const result = add(
    multiply(add(flockA, flockC), flockB),
    multiply(flockA, flockB),
);

console.log(result);

const x = 1;
const y = 2;
const z = 3;

// Those properties hold true, using those we can simplify the calculation
// above.
const isAssociative = add(add(x, y), z) === add(x, add(y, z));
const isCommutative = add(x, y) === add(y, x);
const isIdentity = add(x, 0) === x;
const isDistributive =
    multiply(x, add(y, z)) === add(multiply(x, y), multiply(x, z));

console.log(isAssociative);
console.log(isCommutative);
console.log(isIdentity);
console.log(isDistributive);

const temp = add(
    multiply(flockA, flockB),
    multiply(flockA, flockB),
);
console.log(temp);

const res = multiply(flockA, add(flockB, flockB));
console.log(res);
