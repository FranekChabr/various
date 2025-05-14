class NumbersAnalyzer {
  constructor(numbers) {
    this.numbers = numbers;
  }

  countElements() {
    return this.numbers.length;
  }

  sum() {
  let suma = 0;
  for (let i = 0; i < this.numbers.length; i++) {
    suma += this.numbers[i];
  }
  return suma;
  } 

  average() {
    if (this.numbers.length === 0) return 0;
    return this.sum() / this.countElements();
  }

  min() {
    return Math.min(...this.numbers);
  }

  max() {
    return Math.max(...this.numbers);
  }

  harmonicMean() {
    if (this.numbers.length === 0 || this.numbers.includes(0)) return 0;
    const reciprocalSum = this.numbers.reduce((acc, val) => acc + 1 / val, 0);
    return this.numbers.length / reciprocalSum;
  }
}

const analyzer = new NumbersAnalyzer([4, 8, 15, 16, 23, 42]);

console.log("liczba elementow:", analyzer.countElements());
console.log("suma liczb:", analyzer.sum());
console.log("srednia arytmetyczna:", analyzer.average());
console.log("najmniejsza liczba:", analyzer.min());
console.log("najwieksza liczba:", analyzer.max());
console.log("srednia harmoniczna:", analyzer.harmonicMean());
