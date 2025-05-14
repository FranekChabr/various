function guessNumber(guesses) {
  const target = Math.floor(Math.random() * 100) + 1; // generowanie losowej liczby 1-100
  //console.log(target); // do testu

  for (let i = 0; i < guesses.length; i++) {
    if (guesses[i] === target) {
      return {
        isCorrect: true,
        attempts: i + 1
      };
    }
  }

  return { // zadna proba sie nie powiodla
    isCorrect: false,
    attempts: guesses.length
  };
}

const myGuesses = [10,20,30,40,50,60,70,80,90];
const res = guessNumber(myGuesses);

console.log("czy odgadnieto?", res.isCorrect);
console.log("liczba prob:", res.attempts);
