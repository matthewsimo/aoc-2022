import { readFileSync } from "fs";
const INPUT = readFileSync("./input/13.txt", "utf8");

// const INPUT = `[1,1,3,1,1]
// [1,1,5,1,1]

// [[1],[2,3,4]]
// [[1],4]

// [9]
// [[8,7,6]]

// [[4,4],4,4]
// [[4,4],4,4,4]

// [7,7,7,7]
// [7,7,7]

// []
// [3]

// [[[]]]
// [[]]

// [1,[2,[3,[4,[5,6,7]]]],8,9]
// [1,[2,[3,[4,[5,6,0]]]],8,9]`;

const pairs = INPUT.split("\n\n").map((pair) =>
  pair.split("\n").map((p) => JSON.parse(p))
);

const ComparisonResult = {
  LESS: -1,
  SAME: 0,
  MORE: 1,
};

const compare = (left, right) => {
  if (typeof left === "object" && typeof right === "object") {
    let childrenResult = ComparisonResult.SAME;
    left.forEach((l, i) => {
      if (childrenResult !== ComparisonResult.SAME) {
        return childrenResult;
      }

      if (right[i] === undefined) {
        return ComparisonResult.MORE;
      }

      if (childrenResult === ComparisonResult.SAME) {
        childrenResult = compare(left[i], right[i]);
      }
    });

    if (
      childrenResult === ComparisonResult.SAME &&
      left.length < right.length
    ) {
      return ComparisonResult.LESS;
    }

    if (
      childrenResult === ComparisonResult.SAME &&
      left.length > right.length
    ) {
      return ComparisonResult.MORE;
    }

    return childrenResult;
  } else if (typeof left === "object" || typeof right === "object") {
    let cLeft, cRight;
    if (typeof left === "object") {
      cLeft = left;
      cRight = Array.of(right);
    } else {
      cLeft = Array.of(left);
      cRight = right;
    }

    return compare(cLeft, cRight);
  } else {
    const c = left - right;
    if (c < 0) return ComparisonResult.LESS;
    if (c === 0) return ComparisonResult.SAME;
    if (c > 0) return ComparisonResult.MORE;
  }
};

const part1 = () => {
  let count = 0;
  pairs.forEach(([left, right], i) => {
    const result = compare(left, right);
    if (result === ComparisonResult.LESS) {
      count += i + 1;
    }
  });
  console.log("Part 1:");
  console.log("------------");
  console.log({ count });
};

part1();

const part2 = () => {
  let dividerPackets = [[[2]], [[6]]];
  let packets = pairs.flat(1).concat(dividerPackets).sort(compare);

  let x = packets.findIndex((p) => p === dividerPackets[0]) + 1;
  let y = packets.findIndex((p) => p === dividerPackets[1]) + 1;
  console.log("Part 2:");
  console.log("------------");
  console.log({ x, y, t: x * y });
};

part2();
